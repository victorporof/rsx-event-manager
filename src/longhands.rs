/*
Copyright 2016 Mozilla
Licensed under the Apache License, Version 2.0 (the "License"); you may not use
this file except in compliance with the License. You may obtain a copy of the
License at http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.
*/

use enum_str_derive::EnumStrCamelCase;

pub type KeyCodeId = u8;
pub type MouseButtonId = u8;

// See https://w3c.github.io/uievents-code/#code-value-tables
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash, Serialize, Deserialize, Primitive, EnumStrCamelCase)]
#[repr(u8)]
pub enum KeyCode {
    Unidentified = 0,

    // Alphanumeric Section Writing System Keys
    Backquote = 1,
    Backslash = 2,
    BracketLeft = 3,
    BracketRight = 4,
    Comma = 5,
    Digit0 = 6,
    Digit1 = 7,
    Digit2 = 8,
    Digit3 = 9,
    Digit4 = 10,
    Digit5 = 11,
    Digit6 = 12,
    Digit7 = 13,
    Digit8 = 14,
    Digit9 = 15,
    Equal = 16,
    IntlBackslash = 17,
    IntlRo = 18,
    IntlYen = 19,
    KeyA = 20,
    KeyB = 21,
    KeyC = 22,
    KeyD = 23,
    KeyE = 24,
    KeyF = 25,
    KeyG = 26,
    KeyH = 27,
    KeyI = 28,
    KeyJ = 29,
    KeyK = 30,
    KeyL = 31,
    KeyM = 32,
    KeyN = 33,
    KeyO = 34,
    KeyP = 35,
    KeyQ = 36,
    KeyR = 37,
    KeyS = 38,
    KeyT = 39,
    KeyU = 40,
    KeyV = 41,
    KeyW = 42,
    KeyX = 43,
    KeyY = 44,
    KeyZ = 45,
    Minus = 46,
    Period = 47,
    Quote = 48,
    Semicolon = 49,
    Slash = 50,

    // Alphanumeric Section Functional Keys
    AltLeft = 51,
    AltRight = 52,
    Backspace = 53,
    CapsLock = 54,
    ContextMenu = 55,
    ControlLeft = 56,
    ControlRight = 57,
    Enter = 58,
    MetaLeft = 59,
    MetaRight = 60,
    ShiftLeft = 61,
    ShiftRight = 62,
    Space = 63,
    Tab = 64,

    // Alphanumeric Section Japanese and Korean keyboards
    Convert = 65,
    KanaMode = 66,
    Lang1 = 67,
    Lang2 = 68,
    Lang3 = 69,
    Lang4 = 70,
    Lang5 = 71,
    NonConvert = 72,

    // Control Pad Section
    Delete = 73,
    End = 74,
    Help = 75,
    Home = 76,
    Insert = 77,
    PageDown = 78,
    PageUp = 79,

    // Arrow Pad Section
    ArrowDown = 80,
    ArrowLeft = 81,
    ArrowRight = 82,
    ArrowUp = 83,

    // Numpad Section
    NumLock = 84,
    Numpad0 = 85,
    Numpad1 = 86,
    Numpad2 = 87,
    Numpad3 = 88,
    Numpad4 = 89,
    Numpad5 = 90,
    Numpad6 = 91,
    Numpad7 = 92,
    Numpad8 = 93,
    Numpad9 = 94,
    NumpadAdd = 95,
    NumpadBackspace = 96,
    NumpadClear = 97,
    NumpadClearEntry = 98,
    NumpadComma = 99,
    NumpadDecimal = 100,
    NumpadDivide = 101,
    NumpadEnter = 102,
    NumpadEqual = 103,
    NumpadHash = 104,
    NumpadMemoryAdd = 105,
    NumpadMemoryClear = 106,
    NumpadMemoryRecall = 107,
    NumpadMemoryStore = 108,
    NumpadMemorySubtract = 109,
    NumpadMultiply = 110,
    NumpadParenLeft = 111,
    NumpadParenRight = 112,
    NumpadStar = 113,
    NumpadSubtract = 114,

    // Function Section
    Escape = 115,
    F1 = 116,
    F2 = 117,
    F3 = 118,
    F4 = 119,
    F5 = 120,
    F6 = 121,
    F7 = 122,
    F8 = 123,
    F9 = 124,
    F10 = 125,
    F11 = 126,
    F12 = 127,
    Fn = 128,
    FnLock = 129,
    PrintScreen = 130,
    ScrollLock = 131,
    Pause = 132,

    // Media Keys
    BrowserBack = 133,
    BrowserFavorites = 134,
    BrowserForward = 135,
    BrowserHome = 136,
    BrowserRefresh = 137,
    BrowserSearch = 138,
    BrowserStop = 139,
    Eject = 140,
    LaunchApp1 = 141,
    LaunchApp2 = 142,
    LaunchMail = 143,
    MediaPlayPause = 144,
    MediaSelect = 145,
    MediaStop = 146,
    MediaTrackNext = 147,
    MediaTrackPrevious = 148,
    Power = 149,
    Sleep = 150,
    AudioVolumeDown = 151,
    AudioVolumeMute = 152,
    AudioVolumeUp = 153,
    WakeUp = 154
}

// See https://developer.mozilla.org/en-US/docs/Web/API/MouseEvent/button
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash, Serialize, Deserialize, Primitive, EnumStrCamelCase)]
#[repr(u8)]
pub enum MouseButton {
    Main = 0,
    Aux = 1,
    Secondary = 2,
    Fourth = 3,
    Fifth = 4
}
