use std::mem::MaybeUninit;

use crate::{bindings::Viewport, Context, Position, Rect, Size};

pub struct WindowViewport<'a> {
    ctx: &'a mut Context,
    ptr: Viewport,
}
impl<'a> WindowViewport<'a> {
    pub fn new(ctx: &'a mut Context) -> Self {
        let raw = ctx.raw();
        let ptr = unsafe { ctx.imgui().GetWindowViewport(raw) };
        Self { ctx, ptr }
    }
    pub fn position(&self) -> Position {
        let (mut x, mut y) = (MaybeUninit::zeroed(), MaybeUninit::zeroed());
        unsafe {
            self.ctx
                .imgui()
                .Viewport_GetPos(self.ptr, x.as_mut_ptr(), y.as_mut_ptr());
            Position::new(x.assume_init() as u32, y.assume_init() as u32)
        }
    }
    pub fn size(&self) -> Size {
        let (mut width, mut height) = (MaybeUninit::zeroed(), MaybeUninit::zeroed());
        unsafe {
            self.ctx
                .imgui()
                .Viewport_GetSize(self.ptr, width.as_mut_ptr(), height.as_mut_ptr());
            Size::new(width.assume_init() as u32, height.assume_init() as u32)
        }
    }
    pub fn rect(&self) -> Rect {
        Rect::from((self.position(), self.size()))
    }
    pub fn work_position(&self) -> Position {
        let (mut x, mut y) = (MaybeUninit::zeroed(), MaybeUninit::zeroed());
        unsafe {
            self.ctx
                .imgui()
                .Viewport_GetWorkPos(self.ptr, x.as_mut_ptr(), y.as_mut_ptr());
            Position::new(x.assume_init() as u32, y.assume_init() as u32)
        }
    }
    pub fn work_size(&self) -> Size {
        let (mut width, mut height) = (MaybeUninit::zeroed(), MaybeUninit::zeroed());
        unsafe {
            self.ctx.imgui().Viewport_GetWorkSize(
                self.ptr,
                width.as_mut_ptr(),
                height.as_mut_ptr(),
            );
            Size::new(width.assume_init() as u32, height.assume_init() as u32)
        }
    }
    pub fn work_rect(&self) -> Rect {
        Rect::from((self.work_position(), self.work_size()))
    }
}
