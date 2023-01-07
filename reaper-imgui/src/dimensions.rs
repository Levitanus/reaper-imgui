use crate::Context;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Default, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}
impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}
impl From<Rect> for Position {
    fn from(value: Rect) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

#[derive(
    Debug, Default, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}
impl Size {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
impl From<Rect> for Size {
    fn from(value: Rect) -> Self {
        Self {
            width: value.width,
            height: value.height,
        }
    }
}

#[derive(
    Debug, Default, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}
impl Rect {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
    pub fn center(&self) -> Position {
        Position::new(self.x + self.width / 2, self.y + self.height / 2)
    }
}
impl From<(Position, Size)> for Rect {
    fn from(value: (Position, Size)) -> Self {
        let (pos, size) = value;
        Self::new(pos.x, pos.y, size.width, size.height)
    }
}

pub trait Widget {
    fn ctx(&mut self) -> &mut Context;
}

pub trait SetWidth: Widget + Sized {
    fn set_width(mut self, width: u32) -> Self {
        let ctx_raw = self.ctx().raw();
        unsafe { self.ctx().imgui().SetNextItemWidth(ctx_raw, width as f64) };
        self
    }
}
