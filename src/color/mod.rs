const ALL_RESET: &str = "\x1B[0m";

const STYLE_DIM: &str = "\x1B[2m";
const STYLE_BOLD: &str = "\x1B[1m";

const COLOR_RED: &str = "\x1B[31m";
const COLOR_CYAN: &str = "\x1B[36m";
const COLOR_WHITE: &str = "\x1B[37m";

const COLOR_B_WHITE: &str = "\x1B[97m";

pub trait StringColor {
  fn reset(&self) -> String;

  fn s_dim(&self) -> String;
  fn s_bold(&self) -> String;

  fn c_red(&self) -> String;
  fn c_cyan(&self) -> String;
  fn c_white(&self) -> String;

  fn c_bright_white(&self) -> String;
}

impl StringColor for String {
  fn reset(&self) -> String {
    format!("{}{}", ALL_RESET, self)
  }

  fn s_dim(&self) -> String {
    format!("{}{}{}", STYLE_DIM, self, ALL_RESET)
  }

  fn s_bold(&self) -> String {
    format!("{}{}{}", STYLE_BOLD, self, ALL_RESET)
  }

  fn c_red(&self) -> String {
    format!("{}{}{}", COLOR_RED, self, ALL_RESET)
  }

  fn c_cyan(&self) -> String {
    format!("{}{}{}", COLOR_CYAN, self, ALL_RESET)
  }

  fn c_white(&self) -> String {
    format!("{}{}{}", COLOR_WHITE, self, ALL_RESET)
  }

  fn c_bright_white(&self) -> String {
    format!("{}{}{}", COLOR_B_WHITE, self, ALL_RESET)
  }
}
