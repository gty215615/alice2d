
#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum MouseButton {
    Left = 0,
    Right = 1,
    Middle = 2,
    Unknown = 4
}