#![allow(non_upper_case_globals)]
use core::panic;
use std::{collections::HashMap, ffi::CString, mem::MaybeUninit, path::PathBuf};

use bitflags::bitflags;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    bindings,
    button::{Button, CheckBox, DockWidget},
    images::ImageHandleInternal,
    text::{IntInput, IntInputValues, Text, TextInput},
    viewport::WindowViewport,
    Dock, Image, ImageHandle, KeyBinding, KeyModifier,
};

use super::{ImGuiRaw, Window};

#[derive(Debug)]
pub struct Context {
    imgui: ImGuiRaw,
    name: String,
    raw: Option<bindings::Context>,
    flags: ContextFlags,
    image_handles: HashMap<Uuid, ImageHandleInternal>,
}
impl Context {
    pub fn new(imgui: ImGuiRaw, name: impl Into<String>) -> Self {
        Self {
            imgui,
            name: name.into(),
            raw: None,
            flags: ContextFlags::empty(),
            image_handles: HashMap::new(),
        }
    }
    pub fn window(&mut self, title: impl Into<String>) -> Window {
        Window::new(title, self)
    }
    pub fn window_viewport(&mut self) -> WindowViewport {
        WindowViewport::new(self)
    }
    pub fn error(&mut self, message: impl Into<String>) {
        panic!("{}", message.into());
    }
    pub fn dock_state(&mut self) -> Dock {
        let raw = self.raw();
        match unsafe { self.imgui().IsWindowDocked(raw) } {
            false => Dock::UnDocked,
            true => {
                let raw_state = unsafe { self.imgui().GetWindowDockID(raw) };
                Dock::from_raw(raw_state)
            }
        }
    }
    pub fn text(&mut self, text: impl Into<String>) -> Text {
        Text::new(text, self)
    }
    pub fn text_input(&mut self, label: impl Into<String>, text: impl Into<String>) -> TextInput {
        TextInput::new(label, text, self)
    }
    /// values can be either i32, either tuple from 2 to 4 i32 values
    pub fn int_input<T>(
        &mut self,
        label: impl Into<String>,
        values: impl Into<IntInputValues<T>>,
    ) -> IntInput<T> {
        IntInput::new(label, values.into(), self)
    }
    pub fn button(&mut self, text: impl Into<String>) -> Button {
        Button::new(text, self)
    }
    pub fn check_box(&mut self, text: impl Into<String>, value: bool) -> CheckBox {
        CheckBox::new(text, value, self)
    }

    pub fn capture_keyboard(&mut self, capture: bool) -> &mut Self {
        let raw = self.raw();
        unsafe {
            self.imgui().SetNextFrameWantCaptureKeyboard(raw, capture);
        }
        self
    }
    pub fn got_input(&mut self) -> Option<String> {
        let raw = self.raw();
        let mut idx = 0;
        let mut out = String::new();
        loop {
            let mut ch = MaybeUninit::new(0_i32);
            match unsafe {
                self.imgui()
                    .GetInputQueueCharacter(raw, idx, ch.as_mut_ptr())
            } {
                true => {
                    idx += 1;
                    out.push(
                        char::from_u32(unsafe { ch.assume_init() as u32 })
                            .expect("Can not convert ImGui character to char"),
                    );
                }
                false => break,
            }
        }
        match out.is_empty() {
            true => None,
            false => Some(out),
        }
    }
    pub fn got_key_binding(&mut self, key_binding: &KeyBinding, want_repeat: bool) -> bool {
        let raw = self.raw();
        let key = match key_binding.key_code.raw(self) {
            Ok(key) => key,
            Err(err) => {
                self.error(err);
                return false;
            }
        };
        let mut want_repeat = MaybeUninit::new(want_repeat);
        if unsafe {
            !self
                .imgui()
                .IsKeyPressed(raw, key, want_repeat.as_mut_ptr())
        } {
            return false;
        }
        let raw = self.raw();
        let modifiers = match KeyModifier::from_raw(&self, unsafe { self.imgui().GetKeyMods(raw) })
        {
            Ok(m) => m,
            Err(err) => {
                self.error(err);
                return false;
            }
        };

        for modifier in modifiers.iter() {
            if !key_binding.modifiers.contains(&modifier) {
                return false;
            }
        }
        for modifier in key_binding.modifiers.iter() {
            if !modifiers.contains(&modifier) {
                return false;
            }
        }
        return true;
    }

    /// CheckBox, that handles window docking.
    ///
    /// It requires to have 2 parameters of type [Dock]:
    /// - `app_dock_state: &mut Dock`, that is saved with app in
    /// Reaper ExtState
    /// - `next_dock`, which is returned by [DockWidget::next_dock],
    /// which should be passed to the [Window::dock] method.
    ///
    /// # Simplified example for [rea_rs]
    ///
    /// ```no_run
    /// use rea_rs::{PluginContext, Reaper, ExtState, Timer};
    /// use rea_rs_macros::reaper_extension_plugin;
    /// use std::error::Error;
    /// use serde::{Deserialize, Serialize};
    /// use reaper_imgui::{
    ///     Context, ContextFlags, Dock, ImGui, SetWidth, Size,
    /// };
    ///
    /// #[reaper_extension_plugin]
    /// fn plugin_main(context: PluginContext) -> Result<(), Box<dyn Error>> {
    ///     print!("rea_score extension... ");
    ///     Reaper::init_global(context);
    ///     let rpr = Reaper::get_mut();
    ///     let _id = rpr.register_action(
    ///         "test_window_dock",
    ///         "Test: window with dock widget",
    ///         move |_| Ok(Window::init(context)),
    ///         None,
    ///     );
    ///     println!("loaded! action result: {:?}", _id);
    ///     Ok(())
    /// }
    /// pub struct Window {
    ///     ctx: Context,
    ///     _imgui: ImGui,
    ///     app_dock: ExtState<'static, Dock, Reaper>,
    ///     next_dock:Dock
    /// }
    /// impl Window {
    ///     pub fn init(context: PluginContext) {
    ///         let imgui = ImGui::load(context);
    ///         let mut ctx = imgui
    ///             .create_context("ReaScore preview")
    ///             .with_flags(ContextFlags::DockingEnable);
    ///         let app_dock = ExtState::new(
    ///             "ReaScore",
    ///             "preview window",
    ///             Dock::UnDocked,
    ///             true,
    ///             Reaper::get(),
    ///         );
    ///         let next_dock =
    ///             app_dock.get().expect("can not load Preview Window app_dock");
    ///         Reaper::get_mut().register_timer(Box::new(Self {
    ///             ctx,
    ///             _imgui: imgui,
    ///             app_dock,
    ///             next_dock,
    ///         }));
    ///     }
    /// }
    ///
    /// impl Timer for Window {
    ///     fn run(&mut self) -> Result<(), Box<dyn Error>> {
    ///         let mut app_dock = self.app_dock.get()
    ///                            .expect("can not load Preview Window app_dock");
    ///         if !self
    ///             .ctx
    ///             .window("preview")
    ///             .dock(&self.next_dock)
    ///             .open(|ctx| {
    ///                 self.next_dock = ctx
    ///                     .dock_widget("dock", &mut app_dock)
    ///                     .set_width(80)
    ///                     .next_dock();
    ///                 self.app_dock.set(app_dock);
    ///             })
    ///         {
    ///             self.stop()
    ///         };    
    ///         Ok(())
    ///     }
    ///     fn id_string(&self) -> String {
    ///         "Dock example".to_string()
    ///     }
    /// }
    /// ```
    pub fn dock_widget<'a>(
        &'a mut self,
        text: impl Into<String>,
        app_dock_state: &'a mut Dock,
    ) -> DockWidget {
        DockWidget::new(text, app_dock_state, self)
    }
    pub fn image(&mut self, handle: ImageHandle) -> Option<Image> {
        Image::new(handle, self)
    }
    pub fn image_handle(&mut self, path: impl Into<PathBuf>) -> ImageHandle {
        let path = path.into();
        let handle = ImageHandleInternal::new(path.clone());
        let uuid = handle.uuid;
        self.image_handles.insert(handle.uuid, handle);
        ImageHandle::new(path, uuid)
    }
    pub fn sameline(
        &mut self,
        offset_from_start: impl Into<Option<u32>>,
        spacing: impl Into<Option<u32>>,
    ) {
        let mut offset_from_start = offset_from_start.into().unwrap_or(0) as f64;
        let mut spacing = spacing.into().unwrap_or(0) as f64;
        let raw = self.raw();
        unsafe {
            self.imgui().SameLine(
                raw,
                &mut offset_from_start as *mut _,
                &mut spacing as *mut _,
            )
        }
    }
    pub fn imgui(&self) -> &ImGuiRaw {
        &self.imgui
    }
    pub(crate) fn image_handles(&mut self) -> &mut HashMap<Uuid, ImageHandleInternal> {
        &mut self.image_handles
    }
    pub fn with_flags(mut self, flags: ContextFlags) -> Self {
        self.flags = flags;
        self
    }
    pub fn raw(&mut self) -> bindings::Context {
        match self.raw {
            None => {
                let name =
                    CString::new(self.name.as_str()).expect("Can not convert name to CString");
                self.raw = unsafe {
                    Some(
                        self.imgui
                            .CreateContext(name.as_ptr(), &mut self.flags.raw(self) as *mut _),
                    )
                };
                self.raw.unwrap()
            }
            Some(ctx) => ctx,
        }
    }
}

bitflags! {
    #[derive(Serialize, Deserialize)]
    pub struct ContextFlags:i32 {
        /// [BETA] Enable docking functionality.
        const DockingEnable=0b00000001;

        /// Master keyboard navigation enable flag.
        const NavEnableKeyboard=0b00000010;

        /// Instruct navigation to move the mouse cursor.
        const NavEnableSetMousePos=0b00000100;

        /// Instruct navigation to not capture global keyboard input
        /// when [ContextFlags::NavEnableKeyboard] is set
        /// (see SetNextFrameWantCaptureKeyboard).
        const NavNoCaptureKeyboard=0b00001000;

        /// Instruct imgui to ignore mouse position/buttons.
        const NoMouse=0b00010000;

        /// Instruct backend to not alter mouse cursor shape
        /// and visibility.
        const NoMouseCursorChange=0b00100000;

        /// Disable state restoration and persistence for the whole context.
        const NoSavedSettings=0b01000000;
    }
}
impl ContextFlags {
    pub fn raw(&self, ctx: &Context) -> i32 {
        let mut flags = 0_i32;
        let low = ctx.imgui();
        if self.contains(ContextFlags::DockingEnable) {
            flags |= low.ConfigFlags_DockingEnable.expect(
                "DockingEnable is not supported. Probably, ReaImGui extension is outdated.",
            );
        }
        if self.contains(ContextFlags::NavEnableKeyboard) {
            flags |= low.ConfigFlags_NavEnableKeyboard.expect(
                "NavEnableKeyboard is not supported. Probably, ReaImGui extension is outdated.",
            );
        }
        if self.contains(ContextFlags::NavEnableSetMousePos) {
            flags |= low.ConfigFlags_NavEnableSetMousePos.expect(
                "NavEnableSetMousePos is not supported. Probably, ReaImGui extension is outdated.",
            );
        }
        if self.contains(ContextFlags::NavNoCaptureKeyboard) {
            flags |= low.ConfigFlags_NavNoCaptureKeyboard.expect(
                "NavNoCaptureKeyboard is not supported. Probably, ReaImGui extension is outdated.",
            );
        }
        if self.contains(ContextFlags::NoMouse) {
            flags |= low
                .ConfigFlags_NoMouse
                .expect("NoMouse is not supported. Probably, ReaImGui extension is outdated.");
        }
        if self.contains(ContextFlags::NoMouseCursorChange) {
            flags |= low.ConfigFlags_NoMouseCursorChange.expect(
                "NoMouseCursorChange is not supported. Probably, ReaImGui extension is outdated.",
            );
        }
        if self.contains(ContextFlags::NoSavedSettings) {
            flags |= low.ConfigFlags_NoSavedSettings.expect(
                "NoSavedSettings is not supported. Probably, ReaImGui extension is outdated.",
            );
        }
        flags
    }
}
