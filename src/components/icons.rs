const DART  : &str = include_str!("../assets/dart.svg");
const FIGMA : &str = include_str!("../assets/figma.svg");
const NIM   : &str = include_str!("../assets/nim.svg");
const PYTHON: &str = include_str!("../assets/python.svg");
const RUST  : &str = include_str!("../assets/rust.svg");
const VIM   : &str = include_str!("../assets/vim.svg");

pub enum Icon {
    Dart, Figma, Nim,
    Python, Rust, Vim
}

impl Icon {
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
