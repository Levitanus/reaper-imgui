use std::{ffi::CString, mem::MaybeUninit, path::PathBuf, ptr::null_mut};

use crate::{bindings, Context, ImGuiRaw};
use c_str_macro::c_str;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct Image<'a> {
    ctx: &'a mut Context,
    handle: ImageHandle,
    reload: bool,
}
impl<'a> Image<'a> {
    pub fn new(handle: ImageHandle, ctx: &'a mut Context) -> Option<Self> {
        ctx.image_handles()
            .contains_key(&handle.uuid)
            .then_some(Self {
                ctx,
                handle,
                reload: false,
            })
    }
    fn handle(&mut self) -> &mut ImageHandleInternal {
        self.ctx.image_handles().get_mut(&self.handle.uuid).unwrap()
    }
    pub fn force_reload(mut self, reload: bool) -> Self {
        self.reload = reload;
        self
    }
    pub fn show(&mut self) {
        let reload = self.reload;
        let imgui = self.ctx.imgui().clone();
        self.handle().invalidate(&imgui);
        let raw = self.ctx.raw();
        let handle = self.handle();
        match handle.exists() {
            false => unsafe { self.ctx.imgui().Text(raw, c_str!("No image!").as_ptr()) },
            true => {
                //
                unsafe {
                    imgui.Image(
                        raw,
                        match reload {
                            true => handle.load(&imgui),
                            false => handle.raw(&imgui),
                        },
                        handle.w() as f64,
                        handle.h() as f64,
                        null_mut(),
                        null_mut(),
                        null_mut(),
                        null_mut(),
                        null_mut(),
                        null_mut(),
                    )
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImageHandle {
    pub path: PathBuf,
    pub uuid: Uuid,
}
impl ImageHandle {
    pub fn new(path: impl Into<PathBuf>, uuid: Uuid) -> Self {
        Self {
            path: path.into(),
            uuid,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageHandleInternal {
    path: PathBuf,
    w: u32,
    h: u32,
    raw: Option<bindings::Image>,
    pub uuid: Uuid,
}
impl ImageHandleInternal {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            w: 0,
            h: 0,
            raw: None,
            uuid: Uuid::new_v4(),
        }
    }
    pub fn w(&self) -> u32 {
        self.w
    }
    pub fn h(&self) -> u32 {
        self.h
    }
    pub fn exists(&self) -> bool {
        self.path.exists()
    }
    pub fn invalidate(&mut self, imgui: &ImGuiRaw) {
        if self.exists() {
            self.raw(imgui);
        } else {
            self.w = 0;
            self.h = 0;
        }
    }
    pub fn raw(&mut self, imgui: &ImGuiRaw) -> bindings::Image {
        match self.raw {
            Some(img) => match img.is_null() {
                false => match self.w {
                    x if x < 2 => self.load(imgui),
                    _ => img,
                },
                true => self.load(imgui),
            },
            None => self.load(imgui),
        }
    }
    pub fn load(&mut self, imgui: &ImGuiRaw) -> bindings::Image {
        if !self.path.exists() {
            panic!("path does not exist")
        };
        let path = CString::new(self.path.to_str().expect("can not convert path to str"))
            .expect("Can not convert path string to CString");
        let ptr = unsafe { imgui.CreateImage(path.as_c_str().as_ptr(), null_mut()) };
        let (mut w, mut h) = (MaybeUninit::zeroed(), MaybeUninit::zeroed());
        unsafe {
            imgui.Image_GetSize(ptr, w.as_mut_ptr(), h.as_mut_ptr());
            self.w = w.assume_init() as u32;
            self.h = h.assume_init() as u32;
        }
        self.raw = Some(ptr);
        ptr
    }
}
