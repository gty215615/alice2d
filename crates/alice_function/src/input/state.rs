

#[derive(Debug, PartialEq, Eq,Clone, Copy)]
pub enum AliceInputState {
    Pressed,
    Released,
    Unknown
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]

pub struct  Modifiers {
    /// Either of the alt keys are down (option ⌥ on Mac).
    pub alt: bool,

    /// Either of the control keys are down.
    /// When checking for keyboard shortcuts, consider using [`Self::command`] instead.
    pub ctrl: bool,

    /// Either of the shift keys are down.
    pub shift: bool,

    /// The Mac ⌘ Command key. Should always be set to `false` on other platforms.
    pub mac_cmd: bool,

    /// On Windows and Linux, set this to the same value as `ctrl`.
    /// On Mac, this should be set whenever one of the ⌘ Command keys are down (same as `mac_cmd`).
    /// This is so that egui can, for instance, select all text by checking for `command + A`
    /// and it will work on both Mac and Windows.
    pub command: bool,
}


impl Modifiers {
    pub fn new() -> Self {
        Default::default()
    }

    pub const NONE: Self = Self {
        alt: false,
        ctrl: false,
        shift: false,
        mac_cmd: false,
        command: false,
    };

    pub const ALT: Self = Self {
        alt: true,
        ctrl: false,
        shift: false,
        mac_cmd: false,
        command: false,
    };
    pub const CTRL: Self = Self {
        alt: false,
        ctrl: true,
        shift: false,
        mac_cmd: false,
        command: false,
    };
    pub const SHIFT: Self = Self {
        alt: false,
        ctrl: false,
        shift: true,
        mac_cmd: false,
        command: false,
    };
    /// The Mac ⌘ Command key
    pub const MAC_CMD: Self = Self {
        alt: false,
        ctrl: false,
        shift: false,
        mac_cmd: true,
        command: false,
    };
    /// On Mac: ⌘ Command key, elsewhere: Ctrl key
    pub const COMMAND: Self = Self {
        alt: false,
        ctrl: false,
        shift: false,
        mac_cmd: false,
        command: true,
    };

    #[inline(always)]
    pub fn is_none(&self) -> bool {
        self == &Self::default()
    }

    #[inline(always)]
    pub fn any(&self) -> bool {
        !self.is_none()
    }

    /// Is shift the only pressed button?
    #[inline(always)]
    pub fn shift_only(&self) -> bool {
        self.shift && !(self.alt || self.command)
    }

    /// true if only [`Self::ctrl`] or only [`Self::mac_cmd`] is pressed.
    #[inline(always)]
    pub fn command_only(&self) -> bool {
        !self.alt && !self.shift && self.command
    }

    /// Check for equality but with proper handling of [`Self::command`].
    ///
    /// ```
    /// # use egui::Modifiers;
    /// assert!(Modifiers::CTRL.matches(Modifiers::CTRL));
    /// assert!(!Modifiers::CTRL.matches(Modifiers::CTRL | Modifiers::SHIFT));
    /// assert!(!(Modifiers::CTRL | Modifiers::SHIFT).matches(Modifiers::CTRL));
    /// assert!((Modifiers::CTRL | Modifiers::COMMAND).matches(Modifiers::CTRL));
    /// assert!((Modifiers::CTRL | Modifiers::COMMAND).matches(Modifiers::COMMAND));
    /// assert!((Modifiers::MAC_CMD | Modifiers::COMMAND).matches(Modifiers::COMMAND));
    /// assert!(!Modifiers::COMMAND.matches(Modifiers::MAC_CMD));
    /// ```
    pub fn matches(&self, pattern: Modifiers) -> bool {
        // alt and shift must always match the pattern:
        if pattern.alt != self.alt || pattern.shift != self.shift {
            return false;
        }

        if pattern.mac_cmd {
            // Mac-specific match:
            if !self.mac_cmd {
                return false;
            }
            if pattern.ctrl != self.ctrl {
                return false;
            }
            return true;
        }

        if !pattern.ctrl && !pattern.command {
            // the pattern explicitly doesn't want any ctrl/command:
            return !self.ctrl && !self.command;
        }

        // if the pattern is looking for command, then `ctrl` may or may not be set depending on platform.
        // if the pattern is looking for `ctrl`, then `command` may or may not be set depending on platform.

        if pattern.ctrl && !self.ctrl {
            return false;
        }
        if pattern.command && !self.command {
            return false;
        }

        true
    }
}

impl std::ops::BitOr for Modifiers {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self {
            alt: self.alt | rhs.alt,
            ctrl: self.ctrl | rhs.ctrl,
            shift: self.shift | rhs.shift,
            mac_cmd: self.mac_cmd | rhs.mac_cmd,
            command: self.command | rhs.command,
        }
    }
}
