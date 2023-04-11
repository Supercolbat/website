const DART  : &str = include_str!("../assets/dart.svg");
const FIGMA : &str = include_str!("../assets/figma.svg");
const NIM   : &str = include_str!("../assets/nim.svg");
const PYTHON: &str = include_str!("../assets/python.svg");
const RUST  : &str = include_str!("../assets/rust.svg");
const VIM   : &str = include_str!("../assets/vim.svg");

/// Enum representing icons in the assets folder
pub enum Icon {
    Dart, Figma, Nim,
    Python, Rust, Vim
}

impl Icon {
    /// Returns the appropriate SVG as a string
    pub fn as_svg(&self) -> &str {
        match self {
            Icon::Dart   => DART,
            Icon::Figma  => FIGMA,
            Icon::Nim    => NIM,
            Icon::Python => PYTHON,
            Icon::Rust   => RUST,
            Icon::Vim    => VIM,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_svg() {
        assert_eq!(Icon::Dart.as_svg(), DART);
        assert_eq!(Icon::Figma.as_svg(), FIGMA);
        assert_eq!(Icon::Nim.as_svg(), NIM);
        assert_eq!(Icon::Python.as_svg(), PYTHON);
        assert_eq!(Icon::Rust.as_svg(), RUST);
        assert_eq!(Icon::Vim.as_svg(), VIM);
    }
}
