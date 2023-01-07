use serde::{Deserialize, Serialize};

use crate::Context;

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct KeyBinding {
    pub modifiers: Vec<KeyModifier>,
    pub key_code: KeyCode,
}
impl KeyBinding {
    pub fn new(modifiers: impl IntoIterator<Item = KeyModifier>, key_code: KeyCode) -> Self {
        Self {
            modifiers: modifiers.into_iter().collect(),
            key_code,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum KeyModifier {
    Alt,
    Ctrl,
    None,
    Shift,
    // Shortcut,
    Super,
}

impl KeyModifier {
    pub fn from_raw(ctx: &Context, value: i32) -> Result<Vec<Self>, String> {
        let low = ctx.imgui();
        let mut result = Vec::new();
        if value
            & low
                .Mod_Alt
                .ok_or("Mod_Alt constant is not loaded! Probably, bad version of ReaImGui")?
            > 0
        {
            result.push(Self::Alt)
        }
        if value
            & low
                .Mod_Ctrl
                .ok_or("Mod_Ctrl constant is not loaded! Probably, bad version of ReaImGui")?
            > 0
        {
            result.push(Self::Ctrl)
        }
        if value
            & low
                .Mod_None
                .ok_or("Mod_None constant is not loaded! Probably, bad version of ReaImGui")?
            > 0
        {
            result.push(Self::None)
        }
        if value
            & low
                .Mod_Shift
                .ok_or("Mod_Shift constant is not loaded! Probably, bad version of ReaImGui")?
            > 0
        {
            result.push(Self::Shift)
        }
        // if value
        //     & low
        //         .Mod_Shortcut
        //         .ok_or("Mod_Shortcut constant is not loaded! Probably, bad version of ReaImGui")?
        //     > 0
        // {
        //     result.push(Self::Shortcut)
        // }
        if value
            & low
                .Mod_Super
                .ok_or("Mod_Super constant is not loaded! Probably, bad version of ReaImGui")?
            > 0
        {
            result.push(Self::Super)
        }
        Ok(result)
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum KeyCode {
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    A,
    Apostrophe,
    B,
    Backslash,
    Backspace,
    C,
    CapsLock,
    Comma,
    D,
    Delete,
    DownArrow,
    E,
    End,
    Enter,
    Equal,
    Escape,
    F,
    F1,
    F10,
    F11,
    F12,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    G,
    GraveAccent,
    H,
    Home,
    I,
    Insert,
    J,
    K,
    Keypad0,
    Keypad1,
    Keypad2,
    Keypad3,
    Keypad4,
    Keypad5,
    Keypad6,
    Keypad7,
    Keypad8,
    Keypad9,
    KeypadAdd,
    KeypadDecimal,
    KeypadDivide,
    KeypadEnter,
    KeypadEqual,
    KeypadMultiply,
    KeypadSubtract,
    L,
    LeftAlt,
    LeftArrow,
    LeftBracket,
    LeftCtrl,
    LeftShift,
    LeftSuper,
    M,
    Menu,
    Minus,
    N,
    NumLock,
    O,
    P,
    PageDown,
    PageUp,
    Pause,
    Period,
    PrintScreen,
    Q,
    R,
    RightAlt,
    RightArrow,
    RightBracket,
    RightCtrl,
    RightShift,
    RightSuper,
    S,
    ScrollLock,
    Semicolon,
    Slash,
    Space,
    T,
    Tab,
    U,
    UpArrow,
    V,
    W,
    X,
    Y,
    Z,
}
impl KeyCode {
    pub fn raw(&self, context: &Context) -> Result<i32, String> {
        let imgui = context.imgui();
        match self {
            Self::_0 => imgui.Key_0.ok_or(
                "Key_0 constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::_1 => imgui.Key_1.ok_or(
                "Key_1 constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::_2 => imgui.Key_2.ok_or(
                "Key_2 constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::_3 => imgui.Key_3.ok_or(
                "Key_3 constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::_4 => imgui.Key_4.ok_or(
                "Key_4 constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::_5 => imgui.Key_5.ok_or(
                "Key_5 constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::_6 => imgui.Key_6.ok_or(
                "Key_6 constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::_7 => imgui.Key_7.ok_or(
                "Key_7 constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::_8 => imgui.Key_8.ok_or(
                "Key_8 constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::_9 => imgui.Key_9.ok_or(
                "Key_9 constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::A => imgui.Key_A.ok_or(
                "Key_A constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::Apostrophe => imgui.Key_Apostrophe.ok_or(
                "Key_Apostrophe constant is not loaded. Probably, wrong version of ImGui"
                    .to_string(),
            ),
            Self::B => imgui.Key_B.ok_or(
                "Key_B constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::Backslash => imgui.Key_Backslash.ok_or(
                "Key_Backslash constant is not loaded. Probably, wrong version of ImGui"
                    .to_string(),
            ),
            Self::Backspace => imgui.Key_Backspace.ok_or(
                "Key_Backspace constant is not loaded. Probably, wrong version of ImGui"
                    .to_string(),
            ),
            Self::C => imgui.Key_C.ok_or(
                "Key_C constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::CapsLock => imgui.Key_CapsLock.ok_or(
                "Key_CapsLock constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::Comma => imgui.Key_Comma.ok_or(
                "Key_Comma constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::D => imgui.Key_D.ok_or(
                "Key_D constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::Delete => imgui.Key_Delete.ok_or(
                "Key_Delete constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::DownArrow => imgui.Key_DownArrow.ok_or(
                "Key_DownArrow constant is not loaded. Probably, wrong version of ImGui"
                    .to_string(),
            ),
            Self::E => imgui.Key_E.ok_or(
                "Key_E constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::End => imgui.Key_End.ok_or(
                "Key_End constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::Enter => imgui.Key_Enter.ok_or(
                "Key_Enter constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::Equal => imgui.Key_Equal.ok_or(
                "Key_Equal constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::Escape => imgui.Key_Escape.ok_or(
                "Key_Escape constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::F => imgui.Key_F.ok_or(
                "Key_F constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::F1 => imgui.Key_F1.ok_or(
                "Key_F1 constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::F10 => imgui.Key_F10.ok_or(
                "Key_F10 constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::F11 => imgui.Key_F11.ok_or(
                "Key_F11 constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::F12 => imgui.Key_F12.ok_or(
                "Key_F12 constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::F2 => imgui.Key_F2.ok_or(
                "Key_F2 constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::F3 => imgui.Key_F3.ok_or(
                "Key_F3 constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::F4 => imgui.Key_F4.ok_or(
                "Key_F4 constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::F5 => imgui.Key_F5.ok_or(
                "Key_F5 constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::F6 => imgui.Key_F6.ok_or(
                "Key_F6 constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::F7 => imgui.Key_F7.ok_or(
                "Key_F7 constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::F8 => imgui.Key_F8.ok_or(
                "Key_F8 constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::F9 => imgui.Key_F9.ok_or(
                "Key_F9 constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::G => imgui.Key_G.ok_or(
                "Key_G constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::GraveAccent => imgui.Key_GraveAccent.ok_or(
                "Key_GraveAccent constant is not loaded. Probably, wrong version of ImGui"
                    .to_string(),
            ),
            Self::H => imgui.Key_H.ok_or(
                "Key_H constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::Home => imgui.Key_Home.ok_or(
                "Key_Home constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::I => imgui.Key_I.ok_or(
                "Key_I constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::Insert => imgui.Key_Insert.ok_or(
                "Key_Insert constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::J => imgui.Key_J.ok_or(
                "Key_J constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::K => imgui.Key_K.ok_or(
                "Key_K constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::Keypad0 => imgui.Key_Keypad0.ok_or(
                "Key_Keypad0 constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::Keypad1 => imgui.Key_Keypad1.ok_or(
                "Key_Keypad1 constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::Keypad2 => imgui.Key_Keypad2.ok_or(
                "Key_Keypad2 constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::Keypad3 => imgui.Key_Keypad3.ok_or(
                "Key_Keypad3 constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::Keypad4 => imgui.Key_Keypad4.ok_or(
                "Key_Keypad4 constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::Keypad5 => imgui.Key_Keypad5.ok_or(
                "Key_Keypad5 constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::Keypad6 => imgui.Key_Keypad6.ok_or(
                "Key_Keypad6 constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::Keypad7 => imgui.Key_Keypad7.ok_or(
                "Key_Keypad7 constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::Keypad8 => imgui.Key_Keypad8.ok_or(
                "Key_Keypad8 constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::Keypad9 => imgui.Key_Keypad9.ok_or(
                "Key_Keypad9 constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::KeypadAdd => imgui.Key_KeypadAdd.ok_or(
                "Key_KeypadAdd constant is not loaded. Probably, wrong version of ImGui"
                    .to_string(),
            ),
            Self::KeypadDecimal => imgui.Key_KeypadDecimal.ok_or(
                "Key_KeypadDecimal constant is not loaded. Probably, wrong version of ImGui"
                    .to_string(),
            ),
            Self::KeypadDivide => imgui.Key_KeypadDivide.ok_or(
                "Key_KeypadDivide constant is not loaded. Probably, wrong version of ImGui"
                    .to_string(),
            ),
            Self::KeypadEnter => imgui.Key_KeypadEnter.ok_or(
                "Key_KeypadEnter constant is not loaded. Probably, wrong version of ImGui"
                    .to_string(),
            ),
            Self::KeypadEqual => imgui.Key_KeypadEqual.ok_or(
                "Key_KeypadEqual constant is not loaded. Probably, wrong version of ImGui"
                    .to_string(),
            ),
            Self::KeypadMultiply => imgui.Key_KeypadMultiply.ok_or(
                "Key_KeypadMultiply constant is not loaded. Probably, wrong version of ImGui"
                    .to_string(),
            ),
            Self::KeypadSubtract => imgui.Key_KeypadSubtract.ok_or(
                "Key_KeypadSubtract constant is not loaded. Probably, wrong version of ImGui"
                    .to_string(),
            ),
            Self::L => imgui.Key_L.ok_or(
                "Key_L constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::LeftAlt => imgui.Key_LeftAlt.ok_or(
                "Key_LeftAlt constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::LeftArrow => imgui.Key_LeftArrow.ok_or(
                "Key_LeftArrow constant is not loaded. Probably, wrong version of ImGui"
                    .to_string(),
            ),
            Self::LeftBracket => imgui.Key_LeftBracket.ok_or(
                "Key_LeftBracket constant is not loaded. Probably, wrong version of ImGui"
                    .to_string(),
            ),
            Self::LeftCtrl => imgui.Key_LeftCtrl.ok_or(
                "Key_LeftCtrl constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::LeftShift => imgui.Key_LeftShift.ok_or(
                "Key_LeftShift constant is not loaded. Probably, wrong version of ImGui"
                    .to_string(),
            ),
            Self::LeftSuper => imgui.Key_LeftSuper.ok_or(
                "Key_LeftSuper constant is not loaded. Probably, wrong version of ImGui"
                    .to_string(),
            ),
            Self::M => imgui.Key_M.ok_or(
                "Key_M constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::Menu => imgui.Key_Menu.ok_or(
                "Key_Menu constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::Minus => imgui.Key_Minus.ok_or(
                "Key_Minus constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::N => imgui.Key_N.ok_or(
                "Key_N constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::NumLock => imgui.Key_NumLock.ok_or(
                "Key_NumLock constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::O => imgui.Key_O.ok_or(
                "Key_O constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::P => imgui.Key_P.ok_or(
                "Key_P constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::PageDown => imgui.Key_PageDown.ok_or(
                "Key_PageDown constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::PageUp => imgui.Key_PageUp.ok_or(
                "Key_PageUp constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::Pause => imgui.Key_Pause.ok_or(
                "Key_Pause constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::Period => imgui.Key_Period.ok_or(
                "Key_Period constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::PrintScreen => imgui.Key_PrintScreen.ok_or(
                "Key_PrintScreen constant is not loaded. Probably, wrong version of ImGui"
                    .to_string(),
            ),
            Self::Q => imgui.Key_Q.ok_or(
                "Key_Q constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::R => imgui.Key_R.ok_or(
                "Key_R constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::RightAlt => imgui.Key_RightAlt.ok_or(
                "Key_RightAlt constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::RightArrow => imgui.Key_RightArrow.ok_or(
                "Key_RightArrow constant is not loaded. Probably, wrong version of ImGui"
                    .to_string(),
            ),
            Self::RightBracket => imgui.Key_RightBracket.ok_or(
                "Key_RightBracket constant is not loaded. Probably, wrong version of ImGui"
                    .to_string(),
            ),
            Self::RightCtrl => imgui.Key_RightCtrl.ok_or(
                "Key_RightCtrl constant is not loaded. Probably, wrong version of ImGui"
                    .to_string(),
            ),
            Self::RightShift => imgui.Key_RightShift.ok_or(
                "Key_RightShift constant is not loaded. Probably, wrong version of ImGui"
                    .to_string(),
            ),
            Self::RightSuper => imgui.Key_RightSuper.ok_or(
                "Key_RightSuper constant is not loaded. Probably, wrong version of ImGui"
                    .to_string(),
            ),
            Self::S => imgui.Key_S.ok_or(
                "Key_S constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::ScrollLock => imgui.Key_ScrollLock.ok_or(
                "Key_ScrollLock constant is not loaded. Probably, wrong version of ImGui"
                    .to_string(),
            ),
            Self::Semicolon => imgui.Key_Semicolon.ok_or(
                "Key_Semicolon constant is not loaded. Probably, wrong version of ImGui"
                    .to_string(),
            ),
            Self::Slash => imgui.Key_Slash.ok_or(
                "Key_Slash constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::Space => imgui.Key_Space.ok_or(
                "Key_Space constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::T => imgui.Key_T.ok_or(
                "Key_T constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::Tab => imgui.Key_Tab.ok_or(
                "Key_Tab constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::U => imgui.Key_U.ok_or(
                "Key_U constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::UpArrow => imgui.Key_UpArrow.ok_or(
                "Key_UpArrow constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::V => imgui.Key_V.ok_or(
                "Key_V constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::W => imgui.Key_W.ok_or(
                "Key_W constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::X => imgui.Key_X.ok_or(
                "Key_X constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::Y => imgui.Key_Y.ok_or(
                "Key_Y constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
            Self::Z => imgui.Key_Z.ok_or(
                "Key_Z constant is not loaded. Probably, wrong version of ImGui".to_string(),
            ),
        }
    }
}
