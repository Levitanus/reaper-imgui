use serde::{Deserialize, Serialize};

use super::Context;
use std::{ffi::CString, mem::MaybeUninit, ptr::null_mut};

pub struct Window<'a> {
    name: String,
    ctx: &'a mut Context,
    show_close_button: bool,
}
impl<'a> Window<'a> {
    pub fn new(name: impl Into<String>, ctx: &'a mut Context) -> Self {
        Self {
            name: name.into(),
            ctx,
            show_close_button: true,
        }
    }
    pub fn show_close_button(&mut self, show: bool) -> &mut Self {
        self.show_close_button = show;
        self
    }
    pub fn dock(self, dock: &Dock) -> Self {
        if dock == &Dock::UnChanged {
            return self;
        }
        let ctx_raw = self.ctx.raw();
        unsafe {
            self.ctx
                .imgui()
                .SetNextWindowDockID(ctx_raw, dock.raw(), null_mut());
        }
        self
    }
    pub fn open(self, mut inner: impl FnMut(&mut Context)) -> bool {
        let raw_ctx = self.ctx.raw();
        let imgui = self.ctx.imgui();
        let mut opened = MaybeUninit::new(self.show_close_button);
        let mut flags = MaybeUninit::new(0);
        let name = CString::new(self.name).expect("Can not convert name to CString");
        let visible = unsafe {
            imgui.Begin(
                raw_ctx,
                name.as_ptr(),
                opened.as_mut_ptr(),
                flags.as_mut_ptr(),
            )
        };
        if visible {
            inner(self.ctx);
            let imgui = self.ctx.imgui();
            unsafe {
                imgui.End(raw_ctx);
            }
        }
        let opened = unsafe { opened.assume_init() };
        opened
    }
    pub fn close(&mut self) {
        todo!()
    }
}

#[derive(
    Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize, Default,
)]
pub enum Dock {
    #[default]
    UnDocked,
    UnChanged,
    Reaper(u8),
    ImGui(u32),
}
impl Dock {
    pub fn raw(&self) -> i32 {
        match self {
            Self::UnDocked => 0,
            Self::Reaper(id) => -(*id as i32),
            Self::ImGui(id) => *id as i32,
            Self::UnChanged => panic!("Can not convert Dock:Unchanged to i32"),
        }
    }
    pub fn from_raw(id: i32) -> Self {
        match id {
            x if x < 0 => Self::Reaper(x.abs() as u8),
            0 => Self::UnDocked,
            _ => Self::ImGui(id as u32),
        }
    }
}
