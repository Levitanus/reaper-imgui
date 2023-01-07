// #![allow(clippy::float_cmp)]

use rea_rs::{
    keys::{FVirt, KeyBinding, VKeys},
    IntEnum, PluginContext, Reaper, Timer,
};
use rea_rs_macros::reaper_extension_plugin;
use reaper_imgui::{Context, ImGui, ImageHandle};
use std::error::Error;

#[reaper_extension_plugin]
fn test_main(context: PluginContext) -> Result<(), Box<dyn Error>> {
    println!("test plugin main");
    let rpr = Reaper::init_global(context);
    rpr.register_action(
        "test_imgui_window",
        "reaper-imgui preview window",
        run_imgui,
        KeyBinding::new(
            FVirt::FCONTROL | FVirt::FVIRTKEY,
            VKeys::VK_NUMPAD9.int_value() as u16,
        ),
    )?;
    println!("Registered action");
    Ok(())
}

fn run_imgui(_: i32) -> Result<(), Box<dyn Error>> {
    let imgui = ImGui::load(Reaper::get().plugin_context());
    Reaper::get_mut().register_timer(Box::new(ImGuiRunner::new(imgui)));
    Ok(())
}

struct ImGuiRunner {
    _imgui: ImGui,
    ctx: Context,
    count: i32,
    text: String,
    image: ImageHandle,
    int_val: i32,
    int_vals: (i32, i32),
}
impl ImGuiRunner {
    fn new(imgui: ImGui) -> Self {
        let mut ctx = imgui.create_context("main context");
        let image = ctx.image_handle("/home/levitanus/gits/reaper-imgui/test/test.png");
        Self {
            _imgui: imgui,
            ctx,
            count: 0,
            text: String::from("c'"),
            image,
            int_val: 2,
            int_vals: (2, 5),
        }
    }
}
impl Timer for ImGuiRunner {
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.ctx.capture_keyboard(true);
        if !self.ctx.window("n window").open(|ctx| {
            if ctx.got_key_binding(
                &reaper_imgui::KeyBinding::new(
                    vec![
                        reaper_imgui::KeyModifier::Ctrl,
                        reaper_imgui::KeyModifier::Shift,
                    ],
                    reaper_imgui::KeyCode::B,
                ),
                false,
            ) {
                println!("Pressed Ctrl+Shift+B");
            }
            if let Some(s) = ctx.got_input() {
                println!("got char input: {s}");
            }
            ctx.text_input("type lilypond here", &self.text)
                .changed(|text| {
                    self.text = text;
                });
            ctx.image(self.image.clone())
                .expect("Handle not from this context")
                .show();
            {
                if ctx.button("dec").clicked() {
                    self.count -= 1;
                }
                ctx.sameline(50, None);
                if ctx.button("inc").clicked() {
                    self.count += 1;
                }
            }
            ctx.text(format!("current count: {}", self.count)).show();
            ctx.int_input("one value input", self.int_val)
                .changed(|v| self.int_val = v);
            ctx.int_input("two value input", self.int_vals)
                .changed(|v| self.int_vals = v);
        }) {
            self.stop();
        }
        Ok(())
    }
    fn id_string(&self) -> String {
        "imgui runner".to_string()
    }
}
