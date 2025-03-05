#[cfg(not(target_os = "windows"))]
pub mod color {
    pub const RESET: &str = "\x1B[0m";
    pub const RED: &str = "\x1B[31m";
    pub const GREEN: &str = "\x1B[32m";
    pub const YELLOW: &str = "\x1B[33m";
    pub const BLUE: &str = "\x1B[34m";
    pub const PURPLE: &str = "\x1B[35m";
    pub const CYAN: &str = "\x1B[36m";
    pub const GRAY: &str = "\x1B[37m";
    pub const WHITE: &str = "\x1B[97m";

    pub fn wrap_color(msg: &str, color: &str) -> String {
        format!("{}{}{}", color, msg, RESET)
    }
}

#[cfg(target_os = "windows")]
pub mod color {
    pub const RESET: &str = "";
    pub const RED: &str = "";
    pub const GREEN: &str = "";
    pub const YELLOW: &str = "";
    pub const BLUE: &str = "";
    pub const PURPLE: &str = "";
    pub const CYAN: &str = "";
    pub const GRAY: &str = "";
    pub const WHITE: &str = "";

    pub fn wrap_color(msg: &str, color: &str) -> String {
        msg.to_string()
    }
}

pub use color::*;
