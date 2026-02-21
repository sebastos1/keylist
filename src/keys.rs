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
        let assets_path = option_env!("ASSET_PATH").unwrap_or("assets").to_string();
        let theme = if dark { "Dark" } else { "Light" };
        let name = match self {
            Key::Letter(c) => c.to_uppercase().to_string(),
            Key::Digit(n) => n.to_string(),
            Key::FKey(n) => format!("F{}", n),
            Key::Shift => "Shift".into(),
            Key::Ctrl => "Ctrl".into(),
            Key::Alt => "Alt".into(),
            Key::Win => "Win".into(),
            Key::Command => "Command".into(),
            Key::Enter => "Enter_Alt".into(),
            Key::Esc => "Esc".into(),
            Key::Space => "Space".into(),
            Key::Tab => "Tab".into(),
            Key::PrintScreen => "Print_Screen".into(),
            Key::Backspace => "Backspace_Alt".into(),
            Key::Del => "Del".into(),
            Key::Insert => "Insert".into(),
            Key::Home => "Home".into(),
            Key::End => "End".into(),
            Key::PageUp => "Page_Up".into(),
            Key::PageDown => "Page_Down".into(),
            Key::CapsLock => "Caps_Lock".into(),
            Key::NumLock => "Num_Lock".into(),
            Key::Up => "Arrow_Up".into(),
            Key::Down => "Arrow_Down".into(),
            Key::Left => "Arrow_Left".into(),
            Key::Right => "Arrow_Right".into(),
            Key::Minus => "Minus".into(),
            Key::Plus => "Plus".into(),
            Key::Slash => "Slash".into(),
            Key::Asterisk => "Asterisk".into(),
            Key::BracketLeft => "Bracket_Left".into(),
            Key::BracketRight => "Bracket_Right".into(),
            Key::Semicolon => "Semicolon".into(),
            Key::Quote => "Quote".into(),
            Key::Tilde => "Tilda".into(),
            Key::Question => "Question".into(),
            Key::MarkLeft => "Mark_Left".into(),
            Key::MarkRight => "Mark_Right".into(),
            Key::MouseLeft => "Mouse_Left".into(),
            Key::MouseRight => "Mouse_Right".into(),
            Key::MouseMiddle => "Mouse_Middle".into(),
            Key::Mouse => "Mouse_Simple".into(),
        };
        format!("{}/{}/{}_Key_{}.png", assets_path, theme, name, theme)
    }
}
