#[derive(Debug, Clone, Copy)]
pub enum Key {
    Printable(PrintableKey),
    Navigation(NavigationKey),
    Control(ControlKey),
}

#[derive(Debug, Clone, Copy)]
pub enum PrintableKey {
    Letter(Letter),
    Number(Number),
    Space,
    Enter,
}

#[derive(Debug, Clone, Copy)]
pub enum Letter {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Copy)]
pub enum Number {
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    N0,
}

#[derive(Debug, Clone, Copy)]
pub enum NavigationKey {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
pub enum ControlKey {
    CtrlDown,
    CtrlUp,
}

impl Key {
    pub fn from_scancode(code: u8) -> Option<Key> {
        match code {
            // Letters
            0x1E => Some(Key::Printable(PrintableKey::Letter(Letter::A))),
            0x30 => Some(Key::Printable(PrintableKey::Letter(Letter::B))),
            0x2E => Some(Key::Printable(PrintableKey::Letter(Letter::C))),
            0x20 => Some(Key::Printable(PrintableKey::Letter(Letter::D))),
            0x12 => Some(Key::Printable(PrintableKey::Letter(Letter::E))),
            0x21 => Some(Key::Printable(PrintableKey::Letter(Letter::F))),
            0x22 => Some(Key::Printable(PrintableKey::Letter(Letter::G))),
            0x23 => Some(Key::Printable(PrintableKey::Letter(Letter::H))),
            0x17 => Some(Key::Printable(PrintableKey::Letter(Letter::I))),
            0x24 => Some(Key::Printable(PrintableKey::Letter(Letter::J))),
            0x25 => Some(Key::Printable(PrintableKey::Letter(Letter::K))),
            0x26 => Some(Key::Printable(PrintableKey::Letter(Letter::L))),
            0x32 => Some(Key::Printable(PrintableKey::Letter(Letter::M))),
            0x31 => Some(Key::Printable(PrintableKey::Letter(Letter::N))),
            0x18 => Some(Key::Printable(PrintableKey::Letter(Letter::O))),
            0x19 => Some(Key::Printable(PrintableKey::Letter(Letter::P))),
            0x10 => Some(Key::Printable(PrintableKey::Letter(Letter::Q))),
            0x13 => Some(Key::Printable(PrintableKey::Letter(Letter::R))),
            0x1F => Some(Key::Printable(PrintableKey::Letter(Letter::S))),
            0x14 => Some(Key::Printable(PrintableKey::Letter(Letter::T))),
            0x16 => Some(Key::Printable(PrintableKey::Letter(Letter::U))),
            0x2F => Some(Key::Printable(PrintableKey::Letter(Letter::V))),
            0x11 => Some(Key::Printable(PrintableKey::Letter(Letter::W))),
            0x2D => Some(Key::Printable(PrintableKey::Letter(Letter::X))),
            0x15 => Some(Key::Printable(PrintableKey::Letter(Letter::Y))),
            0x2C => Some(Key::Printable(PrintableKey::Letter(Letter::Z))),

            // Numbers
            0x02 => Some(Key::Printable(PrintableKey::Number(Number::N1))),
            0x03 => Some(Key::Printable(PrintableKey::Number(Number::N2))),
            0x04 => Some(Key::Printable(PrintableKey::Number(Number::N3))),
            0x05 => Some(Key::Printable(PrintableKey::Number(Number::N4))),
            0x06 => Some(Key::Printable(PrintableKey::Number(Number::N5))),
            0x07 => Some(Key::Printable(PrintableKey::Number(Number::N6))),
            0x08 => Some(Key::Printable(PrintableKey::Number(Number::N7))),
            0x09 => Some(Key::Printable(PrintableKey::Number(Number::N8))),
            0x0A => Some(Key::Printable(PrintableKey::Number(Number::N9))),
            0x0B => Some(Key::Printable(PrintableKey::Number(Number::N0))),

            // Special printable
            0x1C => Some(Key::Printable(PrintableKey::Enter)),
            0x39 => Some(Key::Printable(PrintableKey::Space)),

            // Navigation
            0x48 => Some(Key::Navigation(NavigationKey::Up)),
            0x50 => Some(Key::Navigation(NavigationKey::Down)),
            0x4B => Some(Key::Navigation(NavigationKey::Left)),
            0x4D => Some(Key::Navigation(NavigationKey::Right)),

            // Control
            0x1D => Some(Key::Control(ControlKey::CtrlDown)),
            0x9D => Some(Key::Control(ControlKey::CtrlUp)),

            _ => None,
        }
    }
}

impl PrintableKey {
    pub fn to_char(&self) -> char {
        match self {
            PrintableKey::Letter(l) => match l {
                Letter::A => 'a',
                Letter::B => 'b',
                Letter::C => 'c',
                Letter::D => 'd',
                Letter::E => 'e',
                Letter::F => 'f',
                Letter::G => 'g',
                Letter::H => 'h',
                Letter::I => 'i',
                Letter::J => 'j',
                Letter::K => 'k',
                Letter::L => 'l',
                Letter::M => 'm',
                Letter::N => 'n',
                Letter::O => 'o',
                Letter::P => 'p',
                Letter::Q => 'q',
                Letter::R => 'r',
                Letter::S => 's',
                Letter::T => 't',
                Letter::U => 'u',
                Letter::V => 'v',
                Letter::W => 'w',
                Letter::X => 'x',
                Letter::Y => 'y',
                Letter::Z => 'z',
            },
            PrintableKey::Number(n) => match n {
                Number::N0 => '0',
                Number::N1 => '1',
                Number::N2 => '2',
                Number::N3 => '3',
                Number::N4 => '4',
                Number::N5 => '5',
                Number::N6 => '6',
                Number::N7 => '7',
                Number::N8 => '8',
                Number::N9 => '9',
            },
            PrintableKey::Space => ' ',
            PrintableKey::Enter => '\n',
        }
    }
}
