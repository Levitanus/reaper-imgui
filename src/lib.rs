pub mod bindings;
mod parser;

use std::{error::Error, mem::MaybeUninit};

pub use bindings::{
    Context, DrawList, DrawListSplitter, Font, ImGui, Image, ImageSet, ListClipper, Resource,
    TextFilter, Viewport,
};
use rea_rs::{ControlSurface, Reaper};
use reaper_low::PluginContext;
use reaper_macros;

use c_str_macro::c_str;

#[derive(Debug)]
struct GuiRunner {
    imgui: ImGui,
    ctx: Context,
}
impl ControlSurface for GuiRunner {
    fn run(&mut self) {
        let (mut open, mut flags) = (MaybeUninit::new(true), MaybeUninit::zeroed());
        let opened = unsafe {
            self.imgui.Begin(
                self.ctx,
                c_str!("my window").as_ptr(),
                open.as_mut_ptr(),
                flags.as_mut_ptr(),
            )
        };
        if opened {
            unsafe {
                self.imgui.Text(self.ctx, c_str!("Hello World!").as_ptr());
            }
            unsafe { self.imgui.End(self.ctx) };
        }
    }
}

#[reaper_macros::reaper_extension_plugin]
fn plugin_main(context: PluginContext) -> Result<(), Box<dyn Error>> {
    Reaper::load(context);
    let rpr = Reaper::get_mut();
    let imgui = ImGui::load(context);
    let mut zero = MaybeUninit::zeroed();
    let ctx = unsafe { imgui.CreateContext(c_str!("my context").as_ptr(), zero.as_mut_ptr()) };

    rpr.medium_session_mut()
        .plugin_register_add_csurf_inst(Box::new(GuiRunner { ctx, imgui }))
        .expect("Can not register SCurf");

    Ok(())
}
