use crate::{SetWidth, Widget};

use super::Context;
use std::{ffi::CString, mem::MaybeUninit, ptr::null_mut};

pub struct Text<'a> {
    text: String,
    ctx: &'a mut Context,
}
impl<'a> Widget for Text<'a> {
    fn ctx(&mut self) -> &mut Context {
        self.ctx
    }
}
impl<'a> SetWidth for Text<'a> {}
impl<'a> Text<'a> {
    pub fn new(name: impl Into<String>, ctx: &'a mut Context) -> Self {
        Self {
            text: name.into(),
            ctx,
        }
    }
    pub fn show(&mut self) {
        let raw = self.ctx.raw();
        let text = CString::new(self.text.as_str()).expect("Can not convert text to CString");
        unsafe { self.ctx.imgui().Text(raw, text.as_ptr()) }
    }
}

pub struct TextInput<'a> {
    label: String,
    text: String,
    ctx: &'a mut Context,
}
impl<'a> Widget for TextInput<'a> {
    fn ctx(&mut self) -> &mut Context {
        self.ctx
    }
}
impl<'a> SetWidth for TextInput<'a> {}
impl<'a> TextInput<'a> {
    pub fn new(label: impl Into<String>, text: impl Into<String>, ctx: &'a mut Context) -> Self {
        Self {
            label: label.into(),
            text: text.into(),
            ctx,
        }
    }
    pub fn changed(&mut self, mut inner: impl FnMut(String)) -> bool {
        let raw = self.ctx.raw();
        let label = CString::new(self.label.as_str()).expect("Can not convert text to CString");
        let mut vec = Vec::from(self.text.as_bytes());
        vec.resize(400, 0);
        let buf = unsafe { CString::from_vec_unchecked(vec) }.into_raw();
        let changed = unsafe {
            self.ctx
                .imgui()
                .InputText(raw, label.as_c_str().as_ptr(), buf, 400, null_mut())
        };
        if changed {
            inner(String::from(
                unsafe { CString::from_raw(buf) }
                    .to_str()
                    .expect("Can not convert back to string"),
            ))
        };
        changed
    }
}

pub struct IntInputValues<T: Sized> {
    values: T,
    len: u8,
}
impl<T: Sized> IntInputValues<T> {
    pub fn len(&self) -> u8 {
        self.len
    }
}
impl From<i32> for IntInputValues<i32> {
    fn from(values: i32) -> Self {
        Self { values, len: 1 }
    }
}
impl From<(i32, i32)> for IntInputValues<(i32, i32)> {
    fn from(values: (i32, i32)) -> Self {
        Self { values, len: 2 }
    }
}
impl From<(i32, i32, i32)> for IntInputValues<(i32, i32, i32)> {
    fn from(values: (i32, i32, i32)) -> Self {
        Self { values, len: 3 }
    }
}
impl From<(i32, i32, i32, i32)> for IntInputValues<(i32, i32, i32, i32)> {
    fn from(values: (i32, i32, i32, i32)) -> Self {
        Self { values, len: 4 }
    }
}

pub struct IntInput<'a, T> {
    label: String,
    values: IntInputValues<T>,
    ctx: &'a mut Context,
}
impl<'a, T> Widget for IntInput<'a, T> {
    fn ctx(&mut self) -> &mut Context {
        self.ctx
    }
}
impl<'a, T> SetWidth for IntInput<'a, T> {}
impl<'a, T> IntInput<'a, T> {
    pub fn new(label: impl Into<String>, values: IntInputValues<T>, ctx: &'a mut Context) -> Self {
        Self {
            label: label.into(),
            values,
            ctx,
        }
    }
}
impl<'a> IntInput<'a, i32> {
    pub fn changed(&mut self, mut inner: impl FnMut(i32)) -> bool {
        let raw = self.ctx.raw();
        let label = CString::new(self.label.as_str()).expect("Can not convert text to CString");

        let mut v1 = MaybeUninit::new(self.values.values);
        let changed = unsafe {
            self.ctx.imgui().InputInt(
                raw,
                label.as_c_str().as_ptr(),
                v1.as_mut_ptr(),
                null_mut(),
                null_mut(),
                null_mut(),
            )
        };
        if changed {
            inner(unsafe { v1.assume_init() })
        };
        changed
    }
}
impl<'a> IntInput<'a, (i32, i32)> {
    pub fn changed(&mut self, mut inner: impl FnMut((i32, i32))) -> bool {
        let raw = self.ctx.raw();
        let label = CString::new(self.label.as_str()).expect("Can not convert text to CString");

        let mut v1 = MaybeUninit::new(self.values.values.0);
        let mut v2 = MaybeUninit::new(self.values.values.1);
        let changed = unsafe {
            self.ctx.imgui().InputInt2(
                raw,
                label.as_c_str().as_ptr(),
                v1.as_mut_ptr(),
                v2.as_mut_ptr(),
                null_mut(),
            )
        };
        if changed {
            inner(unsafe { (v1.assume_init(), v2.assume_init()) })
        };
        changed
    }
}
impl<'a> IntInput<'a, (i32, i32, i32)> {
    pub fn changed(&mut self, mut inner: impl FnMut((i32, i32, i32))) -> bool {
        let raw = self.ctx.raw();
        let label = CString::new(self.label.as_str()).expect("Can not convert text to CString");

        let mut v1 = MaybeUninit::new(self.values.values.0);
        let mut v2 = MaybeUninit::new(self.values.values.1);
        let mut v3 = MaybeUninit::new(self.values.values.2);
        let changed = unsafe {
            self.ctx.imgui().InputInt3(
                raw,
                label.as_c_str().as_ptr(),
                v1.as_mut_ptr(),
                v2.as_mut_ptr(),
                v3.as_mut_ptr(),
                null_mut(),
            )
        };
        if changed {
            inner(unsafe { (v1.assume_init(), v2.assume_init(), v3.assume_init()) })
        };
        changed
    }
}
impl<'a> IntInput<'a, (i32, i32, i32, i32)> {
    pub fn changed(&mut self, mut inner: impl FnMut((i32, i32, i32, i32))) -> bool {
        let raw = self.ctx.raw();
        let label = CString::new(self.label.as_str()).expect("Can not convert text to CString");

        let mut v1 = MaybeUninit::new(self.values.values.0);
        let mut v2 = MaybeUninit::new(self.values.values.1);
        let mut v3 = MaybeUninit::new(self.values.values.2);
        let mut v4 = MaybeUninit::new(self.values.values.3);
        let changed = unsafe {
            self.ctx.imgui().InputInt4(
                raw,
                label.as_c_str().as_ptr(),
                v1.as_mut_ptr(),
                v2.as_mut_ptr(),
                v3.as_mut_ptr(),
                v4.as_mut_ptr(),
                null_mut(),
            )
        };
        if changed {
            inner(unsafe {
                (
                    v1.assume_init(),
                    v2.assume_init(),
                    v3.assume_init(),
                    v4.assume_init(),
                )
            })
        };
        changed
    }
}
