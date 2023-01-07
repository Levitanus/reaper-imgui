use std::{ffi::CString, mem::MaybeUninit};

use crate::{Dock, SetWidth, Widget};

use super::Context;

pub struct Button<'a> {
    text: String,
    ctx: &'a mut Context,
    width: Option<u32>,
    height: Option<u32>,
}
impl<'a> Widget for Button<'a> {
    fn ctx(&mut self) -> &mut Context {
        self.ctx
    }
}
impl<'a> SetWidth for Button<'a> {}
impl<'a> Button<'a> {
    pub fn new(name: impl Into<String>, ctx: &'a mut Context) -> Self {
        Self {
            text: name.into(),
            ctx,
            width: None,
            height: None,
        }
    }
    pub fn clicked(&mut self) -> bool {
        let raw = self.ctx.raw();
        let text = CString::new(self.text.as_str()).expect("Can not convert text to CString");
        let mut width = match self.width {
            None => 0.0,
            Some(width) => width as f64,
        };
        let mut height = match self.height {
            None => 0.0,
            Some(height) => height as f64,
        };
        unsafe {
            self.ctx.imgui().Button(
                raw,
                text.as_ptr(),
                &mut width as *mut _,
                &mut height as *mut _,
            )
        }
    }
    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }
    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }
}

pub struct CheckBox<'a> {
    text: String,
    value: bool,
    ctx: &'a mut Context,
}
impl<'a> Widget for CheckBox<'a> {
    fn ctx(&mut self) -> &mut Context {
        self.ctx
    }
}
impl<'a> SetWidth for CheckBox<'a> {}
impl<'a> CheckBox<'a> {
    pub fn new(name: impl Into<String>, value: bool, ctx: &'a mut Context) -> Self {
        Self {
            text: name.into(),
            value,
            ctx,
        }
    }
    pub fn clicked(&mut self, mut inner: impl FnMut(bool)) -> bool {
        let raw = self.ctx.raw();
        let text = CString::new(self.text.as_str()).expect("Can not convert text to CString");
        let mut v = MaybeUninit::new(self.value);
        let clicked = unsafe {
            self.ctx
                .imgui()
                .Checkbox(raw, text.as_ptr(), v.as_mut_ptr())
        };
        if clicked {
            inner(unsafe { v.assume_init() });
        }
        clicked
    }
}

pub struct DockWidget<'a> {
    text: String,
    app_dock: &'a mut Dock,
    ctx: &'a mut Context,
}
impl<'a> Widget for DockWidget<'a> {
    fn ctx(&mut self) -> &mut Context {
        self.ctx
    }
}
impl<'a> SetWidth for DockWidget<'a> {}
impl<'a> DockWidget<'a> {
    pub fn new(
        name: impl Into<String>,
        app_dock_state: &'a mut Dock,
        ctx: &'a mut Context,
    ) -> Self {
        Self {
            text: name.into(),
            app_dock: app_dock_state,
            ctx,
        }
    }
    pub fn next_dock(&mut self) -> Dock {
        let raw = self.ctx.raw();
        let text = CString::new(self.text.as_str()).expect("Can not convert text to CString");
        let current_dock = self.ctx.dock_state();
        let mut v = MaybeUninit::new(current_dock != Dock::UnDocked);
        if current_dock != Dock::UnDocked {
            *self.app_dock = current_dock;
        }
        match unsafe {
            self.ctx
                .imgui()
                .Checkbox(raw, text.as_ptr(), v.as_mut_ptr())
        } {
            true => match unsafe { v.assume_init() } {
                false => Dock::UnDocked,
                true => *self.app_dock,
            },
            false => Dock::UnChanged,
        }
    }
}
