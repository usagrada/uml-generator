type GlobalStr = &'static str;

#[derive(Debug)]
pub enum ThemeName {
  Default,
}

pub static mut GLOBAL_THEME: ThemeName = ThemeName::Default;

#[derive(Debug)]
pub struct Theme {
  pub color: ColorScheme,
}

impl Theme {
  pub fn new(_name: ThemeName) -> Self {
    let color = DEFAULT_COLOR;
    Self { color }
  }
}

#[derive(Debug)]
pub struct ColorScheme {
  pub rect: RectColor,
  pub line: LineColor,
  pub text_primary: GlobalStr,
}
#[derive(Debug)]
pub struct LineColor {
  pub primary: GlobalStr,
  pub second: GlobalStr,
}
#[derive(Debug)]
pub struct RectColor {
  pub fill: GlobalStr,
  pub frame: GlobalStr,
  pub text: GlobalStr,
}

const DEFAULT_COLOR: ColorScheme = ColorScheme {
  line: LineColor {
    primary: "#e7afff",
    second: "#e74c3c",
  },
  rect: RectColor {
    fill: "#ffffff",
    frame: "#000000",
    text: "#000000",
  },
  text_primary: "#000000",
};
