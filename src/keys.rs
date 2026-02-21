#[derive(Debug, Clone)]
pub enum Key {
    // modifiers
    Shift,
    Ctrl,
    Alt,
    Win,
    Command,
    // special
    Enter,
    Esc,
    Space,
    Tab,
    PrintScreen,
    Backspace,
    Del,
    Insert,
    Home,
    End,
    PageUp,
    PageDown,
    CapsLock,
    NumLock,
    // arrows
    Up,
    Down,
    Left,
    Right,
    // symbols
    Minus,
    Plus,
    Slash,
    Asterisk,
    BracketLeft,
    BracketRight,
    Semicolon,
    Quote,
    Tilde,
    Question,
    MarkLeft,
    MarkRight,
    // mouse
    MouseLeft,
    MouseRight,
    MouseMiddle,
    Mouse,
    // alphanumeric
    Letter(char),
    Digit(u8),
    FKey(u8),
}

impl Key {
    pub fn from_str(s: &str, mod_key: &str) -> Option<Self> {
        let k = s.to_lowercase();
        // resolve "mod" to whatever the user configured
        if k == "mod" {
            return Key::from_str(mod_key, mod_key);
        }
        match k.as_str() {
            "shift" => Some(Key::Shift),
            "ctrl" | "control" => Some(Key::Ctrl),
            "alt" => Some(Key::Alt),
            "win" | "super" | "windows" => Some(Key::Win),
            "command" | "cmd" => Some(Key::Command),
            "enter" | "return" => Some(Key::Enter),
            "esc" | "escape" => Some(Key::Esc),
            "space" => Some(Key::Space),
            "tab" => Some(Key::Tab),
            "print" | "print_screen" | "prtscn" => Some(Key::PrintScreen),
            "backspace" => Some(Key::Backspace),
            "del" | "delete" => Some(Key::Del),
            "insert" | "ins" => Some(Key::Insert),
            "home" => Some(Key::Home),
            "end" => Some(Key::End),
            "page_up" | "pageup" | "pgup" => Some(Key::PageUp),
            "page_down" | "pagedown" | "pgdown" => Some(Key::PageDown),
            "caps" | "caps_lock" | "capslock" => Some(Key::CapsLock),
            "num_lock" | "numlock" => Some(Key::NumLock),
            "up" | "arrow_up" | "up_arrow" => Some(Key::Up),
            "down" | "arrow_down" | "down_arrow" => Some(Key::Down),
            "left" | "arrow_left" | "left_arrow" => Some(Key::Left),
            "right" | "arrow_right" | "right_arrow" => Some(Key::Right),
            "minus" | "-" => Some(Key::Minus),
            "plus" | "+" => Some(Key::Plus),
            "slash" | "/" => Some(Key::Slash),
            "asterisk" | "*" => Some(Key::Asterisk),
            "bracket_left" | "[" => Some(Key::BracketLeft),
            "bracket_right" | "]" => Some(Key::BracketRight),
            "semicolon" | ";" => Some(Key::Semicolon),
            "quote" | "'" => Some(Key::Quote),
            "tilde" | "tilda" | "`" | "~" => Some(Key::Tilde),
            "question" | "?" => Some(Key::Question),
            "mark_left" | "<" => Some(Key::MarkLeft),
            "mark_right" | ">" => Some(Key::MarkRight),
            "mouse_left" => Some(Key::MouseLeft),
            "mouse_right" => Some(Key::MouseRight),
            "mouse_middle" => Some(Key::MouseMiddle),
            "mouse" => Some(Key::Mouse),
            k if k.len() == 1 && k.chars().next().unwrap().is_alphabetic() => {
                Some(Key::Letter(k.chars().next().unwrap()))
            }
            k if k.len() == 1 && k.chars().next().unwrap().is_numeric() => {
                Some(Key::Digit(k.parse().unwrap()))
            }
            k if k.starts_with('f') && k[1..].parse::<u8>().is_ok() => {
                Some(Key::FKey(k[1..].parse().unwrap()))
            }
            _ => None,
        }
    }

    pub fn to_path(&self, dark: bool) -> String {
        let theme = if dark { "Dark" } else { "Light" };
        let name = match self {
            Key::Shift => "Shift",
            Key::Ctrl => "Ctrl",
            Key::Alt => "Alt",
            Key::Win => "Win",
            Key::Command => "Command",
            Key::Enter => "Enter_Alt",
            Key::Esc => "Esc",
            Key::Space => "Space",
            Key::Tab => "Tab",
            Key::PrintScreen => "Print_Screen",
            Key::Backspace => "Backspace_Alt",
            Key::Del => "Del",
            Key::Insert => "Insert",
            Key::Home => "Home",
            Key::End => "End",
            Key::PageUp => "Page_Up",
            Key::PageDown => "Page_Down",
            Key::CapsLock => "Caps_Lock",
            Key::NumLock => "Num_Lock",
            Key::Up => "Arrow_Up",
            Key::Down => "Arrow_Down",
            Key::Left => "Arrow_Left",
            Key::Right => "Arrow_Right",
            Key::Minus => "Minus",
            Key::Plus => "Plus",
            Key::Slash => "Slash",
            Key::Asterisk => "Asterisk",
            Key::BracketLeft => "Bracket_Left",
            Key::BracketRight => "Bracket_Right",
            Key::Semicolon => "Semicolon",
            Key::Quote => "Quote",
            Key::Tilde => "Tilda",
            Key::Question => "Question",
            Key::MarkLeft => "Mark_Left",
            Key::MarkRight => "Mark_Right",
            Key::MouseLeft => "Mouse_Left",
            Key::MouseRight => "Mouse_Right",
            Key::MouseMiddle => "Mouse_Middle",
            Key::Mouse => "Mouse_Simple",
            Key::Letter(c) => {
                return format!("assets/{}/{}_Key_{}.png", theme, c.to_uppercase(), theme);
            }
            Key::Digit(n) => {
                return format!("assets/{}/{}_Key_{}.png", theme, n, theme);
            }
            Key::FKey(n) => {
                return format!("assets/{}/F{}_Key_{}.png", theme, n, theme);
            }
        };
        format!("assets/{}/{}_Key_{}.png", theme, name, theme)
    }
}
