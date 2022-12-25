use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    path::PathBuf,
    process::Command,
};

use once_cell::sync::Lazy;
use quote::{quote, ToTokens, __private::TokenStream};
use regex::Regex;

pub struct FuncDef {
    name: String,
    returns: String,
    args: Vec<String>,
    arg_types: Vec<String>,
}
impl FuncDef {
    pub fn new(name: String, returns: String, args: Vec<String>) -> Result<FuncDef, String> {
        let (arg_types, args) = Self::parse_args(args)?;
        Ok(Self {
            name,
            returns: Self::parse_type(&returns)?.0,
            args,
            arg_types,
        })
    }
    fn parse_type(arg: &String) -> Result<(String, Option<String>), String> {
        static C_TYPES: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
            let mut m = HashMap::new();
            m.insert("void", "()");
            m.insert("int", stringify!(std::ffi::c_int));
            m.insert("int*", stringify!(*mut std::ffi::c_int));
            m.insert("const char*", stringify!(*const std::ffi::c_char));
            m.insert("char*", stringify!(*mut std::ffi::c_char));
            m.insert("bool", stringify!(bool));
            m.insert("bool*", stringify!(*mut bool));
            m.insert("double", stringify!(f64));
            m.insert("double*", stringify!(*mut f64));
            m
        });

        let c_type = C_TYPES.iter().find_map(|(s, t)| {
            match arg.starts_with(s) && !arg.starts_with(&format!("{}*", s)) {
                true => Some((s, t)),
                false => None,
            }
        });
        let (c_type, mut name) = match c_type {
            None => {
                let mut s = arg.split(" ");
                let t = s.next().unwrap();
                if t.is_empty() {
                    return Err(format!("Can not take type from arg: {}", arg));
                }
                let name = match s.next() {
                    None => None,
                    Some(s) => Some(s.to_string()),
                };
                (format!("{}", t.trim_end_matches("*")), name)
            }
            Some(s) => {
                let t = s.1;
                let name = match arg.strip_prefix(s.0) {
                    None => None,
                    Some(n) => Some(n.to_string()),
                };
                (t.to_string(), name)
            }
        };
        if name == Some("type".to_string()) {
            name = Some("type_".to_string());
        }
        if c_type.contains("reaper_array") {
            return Err("reaper_array".into());
        }
        Ok((c_type, name))
    }
    fn parse_args(args: Vec<String>) -> Result<(Vec<String>, Vec<String>), String> {
        let args: Result<(Vec<String>, Vec<String>), _> = args
            .into_iter()
            .map(|arg| -> Result<(String, String), String> {
                let (k, v) = Self::parse_type(&arg)?;
                match v {
                    None => Err(format!("No Argument name in arg: {}", arg)),
                    Some(v) => Ok((k, v)),
                }
            })
            .fold(Ok((Vec::new(), Vec::new())), |vecs, arg| {
                let (t, a) = arg?;
                let (mut v1, mut v2) = vecs?;
                v1.push(t);
                v2.push(a);
                Ok((v1, v2))
            });
        args
    }
}
impl Display for FuncDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fn {}({:?}) -> {}", self.name, self.args, self.returns)
    }
}

pub struct Headers {
    pub class_defs: Vec<String>,
    pub constants: Vec<String>,
    pub func_defs: Vec<FuncDef>,
}

pub fn walk_header(header: impl Into<PathBuf>) -> Result<Headers, String> {
    let file = File::open(header.into()).unwrap();
    let reader = BufReader::new(file);
    let cls = Regex::new(r"class (\w+);").unwrap();
    let func = Regex::new(r"REAIMGUIAPI_EXTERN ReaImGuiFunc<(\S+)\((.+)\)> (\w+)").unwrap();
    let int_const = Regex::new(r"REAIMGUIAPI_EXTERN ReaImGuiEnum (\w+)").unwrap();

    let mut class_defs = Vec::new();
    let mut func_defs = Vec::new();
    let mut constants = Vec::new();

    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        // if index >= 100 {
        //     break;
        // }
        let line = line.unwrap(); // Ignore errors.
                                  // Show the line and its number.
                                  // println!("{}. {}", index + 1, line);
        if let Some(s) = cls.captures(&line) {
            let s = s
                .get(1)
                .expect(format!("Can not get the first capture at source line {}", index).as_str())
                .as_str()
                .trim_start_matches("ImGui_")
                .to_string();
            if !s.contains("ReaImGuiFunc") {
                class_defs.push(s);
            }
        }
        if let Some(s) = func.captures(&line) {
            let name = s
                .get(3)
                .expect(format!("Can not parse name of func def: {:?}", s).as_str())
                .as_str()
                .trim_start_matches("ImGui_")
                .to_string();
            if name.contains("ValidatePtr") {
                continue;
            }
            let returns = s
                .get(1)
                .expect(format!("Can not parse returns of func def: {:?}", s).as_str())
                .as_str()
                .trim_start_matches("ImGui_")
                .to_string();
            let args = s
                .get(2)
                .expect(format!("Can not parse args of func def: {:?}", s).as_str());
            func_defs.push(
                match FuncDef::new(
                    name,
                    returns,
                    args.as_str()
                        .to_string()
                        .split(", ")
                        .map(|s| s.trim_start_matches("ImGui_").into())
                        .collect(),
                ) {
                    Ok(fd) => fd,
                    Err(err) => {
                        if err.contains("reaper_array") {
                            continue;
                        } else {
                            return Err(err);
                        }
                    }
                },
            )
        }
        if let Some(s) = int_const.captures(&line) {
            constants.push(
                s.get(1)
                    .expect(
                        format!("Can not get the first capture at source line {}", index).as_str(),
                    )
                    .as_str()
                    .trim_start_matches("ImGui_")
                    .to_string(),
            );
        }
    }
    // println!("Class Defs:");
    // class_defs.iter().map(|s| println!("---- {}", s)).count();
    // println!("\nConstants:");
    // constants.iter().map(|s| println!("---- {}", s)).count();
    // println!("\n Function Defs:");
    // func_defs.iter().map(|s| println!("---- {}", s)).count();
    Ok(Headers {
        class_defs,
        constants,
        func_defs,
    })
}

pub fn func_extern(fdef: &FuncDef) -> TokenStream {
    let returns = fdef.returns.parse::<TokenStream>().expect("Can no parse");
    let c_types = fdef
        .arg_types
        .iter()
        .map(|c_type| c_type.parse::<TokenStream>().expect("Can no parse"));
    quote!(fn(#(#c_types),*) -> #returns)
}
pub fn func_method(fdef: &FuncDef) -> TokenStream {
    let returns = fdef.returns.parse::<TokenStream>().expect("Can no parse");
    let name = fdef.name.parse::<TokenStream>().expect("Can no parse");
    let c_types = fdef
        .arg_types
        .iter()
        .map(|c_type| c_type.parse::<TokenStream>().expect("Can no parse"));
    let args = fdef.args.iter().map(|arg| {
        let arg = if arg.contains("type") {
            arg.clone() + "_"
        } else {
            arg.clone()
        };
        arg.parse::<TokenStream>().expect("Can no parse")
    });
    let args1 = args.clone();
    quote!(
        pub unsafe fn #name(&self, #(#args: #c_types),*) -> #returns{
            match self.pointers.#name{
                None => panic!(
                    "Attempt to use a function that has not been loaded: {}",
                    stringify!(#name)
                ),
                Some(f) => f(#(#args1),*)
            }
        }
    )
}

pub fn build_bindings(headers: Headers) -> String {
    let class_defs = headers
        .class_defs
        .into_iter()
        .map(|s| s.parse::<TokenStream>().expect("Can no parse"));
    let names = headers
        .func_defs
        .iter()
        .map(|fd| fd.name.parse::<TokenStream>().expect("Can no parse"));
    let names1 = headers.func_defs.iter().map(|fd| {
        let name = "ImGui_".to_string() + &fd.name;
        name.parse::<TokenStream>().expect("Can no parse")
    });

    let names2 = names.clone();
    let names3 = names.clone();
    // let returns = headers
    //     .func_defs
    //     .iter()
    //     .map(|fd| fd.returns.parse::<TokenStream>().expect("Can no parse"));
    let declarations = headers.func_defs.iter().map(|fd| func_extern(fd));
    let methods = headers.func_defs.iter().map(|fd| func_method(fd));
    // let total_fn_ptr_count = names.len() as u32;
    let const_init_names = headers.constants.iter().map(|name| {
        let name = "ImGui_".to_string() + name;
        name
    });
    let const_names = headers
        .constants
        .iter()
        .map(|name| name.parse::<TokenStream>().expect("Can no parse"));
    let const_names1 = const_names.clone();

    let bindings = quote!(
        //! This file is automatically generated by executing `cargo build --features generate`.
        //!
        //! **Make adjustments in `parser.rs`, not in this file!**
        #![allow(non_upper_case_globals)]
        #![allow(non_camel_case_types)]
        #![allow(non_snake_case)]
        use std::ffi::c_void;
        use reaper_low::PluginContext;
        use std::fmt;

        #(
            pub type #class_defs = *mut c_void;
        )*

        #[derive(Clone)]
        pub struct ImGui{
            pointers: FunctionPointers,
            plugin_context: Option<PluginContext>,
            #(pub #const_names1: i32,)*
        }
        impl std::fmt::Debug for ImGui {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct("ImGui")
                    .field("plugin_context", &self.plugin_context)
                    .finish()
            }
        }

        impl ImGui {
            /// Loads all available REAPER functions from the given plug-in context.
            ///
            /// Returns a low-level `Reaper` instance which allows you to call these functions.
            pub fn load(plugin_context: PluginContext) -> Self {
                let mut loaded_count = 0;
                let mut pointers = unsafe {
                    FunctionPointers {
                        loaded_count: 0,
                        #(
                            #names: std::mem::transmute(plugin_context.GetFunc(c_str_macro::c_str!(stringify!(#names1)).as_ptr())),
                        )*
                    }
                };
                #(
                    if pointers.#names2.is_some() {
                        loaded_count += 1;
                    }
                )*
                pointers.loaded_count = loaded_count;
                Self {
                    pointers,
                    plugin_context: Some(plugin_context),
                    #(
                        #const_names: unsafe{(ConstLoader{f: std::mem::transmute(plugin_context.GetFunc(c_str_macro::c_str!(#const_init_names).as_ptr()))}.f)()},
                    )*
                }
            }

            #(
                #methods
            )*
        }

        #[derive(Clone)]
        struct FunctionPointers{
            loaded_count: u32,
            #(
                #names3: Option<
                    unsafe extern "C" #declarations
                >,
            )*
        }
        impl std::fmt::Debug for FunctionPointers {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.debug_struct("ImGui")
                    .field("loaded_count", &self.loaded_count)
                    .finish()
            }
        }
        // impl FunctionPointers {
        //     const TOTAL_COUNT: u32 = #total_fn_ptr_count;
        // }

        struct ConstLoader {
            f: fn() -> i32,
        }

    );
    bindings.to_token_stream().to_string()
}

fn main() {
    let mut res = reqwest::blocking::get(
        "https://github.com/cfillion/reaimgui/releases/latest/download/reaper_imgui_functions.h",
    )
    .expect("Can not get release");
    let mut body = String::new();
    res.read_to_string(&mut body)
        .expect("Can not read to string");
    let h_path = PathBuf::from("./src/reaper_imgui_functions.h");
    File::create(&h_path)
        .expect("Can not create headers file")
        .write_all(body.as_bytes())
        .expect("Can not write header to file.");
    let headers = walk_header(h_path).expect("Can not walk header");
    let bindings = build_bindings(headers);
    let path = PathBuf::from("./src/bindings.rs");
    let mut f = File::create(path.clone()).expect("Can not create file");
    f.write_all(bindings.as_bytes())
        .expect("Can not write bindings");

    Command::new("rustfmt")
        .arg(format!(
            "{}",
            path.to_str().expect("Can not convert out path to string")
        ))
        .output()
        .expect("Error while formatting");
}
