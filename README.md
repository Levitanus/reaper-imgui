# reaper-imgui

Bindings for cfillion [ReaImGui](https://github.com/cfillion/reaimgui) Reaper Extension.

Currently, this is work in progress, that requires a lot of hand-work. From the generation (which currently is made from unit test) to usage.

But, at least, it makes possible to run native Reaper imgui window in Reaper with `reaper-rs`, or with `rea-rs` crates. Note, that ReaImGui has to be installed in Raper first.

Minimal example in 1hello_world_example` folder:

```rust
use std::{error::Error, mem::MaybeUninit};

use rea_rs::{ControlSurface, Reaper};
use reaper_imgui::{
    Context, DrawList, DrawListSplitter, Font, ImGui, Image, ImageSet, ListClipper, Resource,
    TextFilter, Viewport,
};
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
```
