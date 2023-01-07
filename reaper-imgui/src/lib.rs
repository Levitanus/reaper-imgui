//! Simple bindings to ReaImGui Reaper extension.
//!
//! They are unsafe and hard to use because of all c-types.
//! But, at least, it works, and can be used raw as in `rea-rs` crate, as well
//! as in `reaper-rs`. But for the last one it should be published with the recent version,
//! to make it possible for selecting back-end with features.
//!
//! Minimal example crate can be found on GitHub repository:
//! https://github.com/Levitanus/reaper-imgui/tree/master/hello_world_example

pub mod bindings;

pub use bindings::ImGui as ImGuiRaw;
// pub use button::Button;
pub use context::{Context, ContextFlags};
pub use dimensions::*;
pub use images::{Image, ImageHandle};
use rea_rs_low::PluginContext;
// pub use text::Text;
// pub use viewport::WindowViewport;
pub use keyboard::{KeyBinding, KeyCode, KeyModifier};
pub use window::{Dock, Window};

pub mod button;
pub mod context;
pub mod dimensions;
pub mod images;
pub mod keyboard;
pub mod text;
pub mod viewport;
pub mod window;

#[derive(Debug, Clone)]
pub struct ImGui {
    raw: ImGuiRaw,
}
impl ImGui {
    pub fn load(context: PluginContext) -> Self {
        Self {
            raw: ImGuiRaw::load(context),
        }
    }
    pub fn create_context(&self, name: impl Into<String>) -> Context {
        let raw = self.raw.clone();
        Context::new(raw, name)
    }
}
