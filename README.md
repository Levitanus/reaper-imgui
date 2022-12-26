# reaper-imgui

Bindings for cfillion [ReaImGui](https://github.com/cfillion/reaimgui) Reaper Extension.

They are unsafe and hard to use because of all c-types.
But, at least, it works, and can be used raw as in `rea-rs` crate, as well
as in `reaper-rs`. But for the last one it should be published with the recent version,
to make it possible for selecting back-end with features.
Minimal example crate can be found on GitHub repository:
<https://github.com/Levitanus/reaper-imgui/tree/master/hello_world_example>

```rust
use rea_rs::{PluginContext, Reaper, Timer};
use rea_rs_macros::reaper_extension_plugin;
use reaper_imgui::{
    Context, DrawList, DrawListSplitter, Font, ImGui, Image, ImageSet, ListClipper, Resource,
    TextFilter, Viewport,
};
use std::{error::Error, mem::MaybeUninit};
use c_str_macro::c_str;
#[derive(Debug)]
struct GuiRunner {
    imgui: ImGui,
    ctx: Context,
}
impl Timer for GuiRunner {
    fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let (mut open, mut flags) = (MaybeUninit::new(true), MaybeUninit::zeroed());
        unsafe {
            self.imgui.Begin(
                self.ctx,
                c_str!("my window").as_ptr(),
                open.as_mut_ptr(),
                flags.as_mut_ptr(),
            )
        };
        let open = unsafe { open.assume_init() };
        println!("ctx: {:?} open: {:?}", self.ctx, open);
        if open {
            unsafe {
                self.imgui.Text(self.ctx, c_str!("Hello World!").as_ptr());
            }
            unsafe { self.imgui.End(self.ctx) };
        } else {
            unsafe { self.imgui.End(self.ctx) };
            self.stop();
        }
        Ok(())
    }
    fn id_string(&self) -> String {
        "im_gui_example".to_string()
    }
}
#[reaper_extension_plugin]
fn plugin_main(context: PluginContext) -> Result<(), Box<dyn Error>> {
    println!("plugin main");
    Reaper::init_global(context);
    let rpr = Reaper::get_mut();
    let imgui = ImGui::load(context);
    let mut zero = MaybeUninit::zeroed();
    let ctx = unsafe { imgui.CreateContext(c_str!("my context").as_ptr(), zero.as_mut_ptr()) };
    rpr.register_timer(Box::new(GuiRunner { imgui, ctx }));
    Ok(())
}
```
