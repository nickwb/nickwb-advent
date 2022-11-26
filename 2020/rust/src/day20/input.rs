mod expanded;
mod raw;

pub use self::expanded::*;
pub use self::raw::*;

#[derive(Debug, Clone, Copy)]
pub enum Flip {
    None,
    Horizontal,
    Vertical,
    Both,
}

#[derive(Debug, Clone, Copy)]
pub enum Rotation {
    R0,
    R90,
    R180,
    R270,
}
