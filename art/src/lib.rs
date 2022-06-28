//! # Art
//!
//! 调色库

pub use self::kinds::{PrimaryColor, SecondaryColor};
pub use self::utils::mix;

pub mod kinds {
    //! 定义颜色类型
    /// 主色
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// 副色
    #[derive(Debug, PartialEq)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    //! 工具 目前只实现了调色板
    use crate::kinds::*;
    /// 将两种主色调成副色
    /// ''' rust
    /// use art::utils::mix;
    /// use art::kinds::{PrimaryColor,SecondaryColor};
    /// assert!(matches!(mix(PrimaryColor::Yellow, PrimaryColor::Blue), SecondaryColor::Green));
    /// ```
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        SecondaryColor::Green
    }
}
