use crate::color::StringColor;

use super::ErrorView;

impl ErrorView {
  pub fn display(&self) {
    let h;

    if let Some(h_m) = self.config.help_msg.clone() {
      h = format!("\n{}", h_m);
    } else {
      h = String::new();
    }

    println!(
      "{}:{}: {}: {}\n    |\n  {} | {}\n    | {}{}",
      self.config.row_index.to_string().c_bright_white(),
      self.config.col_index.to_string().c_bright_white(),
      self.config.err_kind.to_string().c_red(),
      self.config.err_msg,
      self.config.row_index,
      self.config.input_pre,
      self.config.err_marker.c_red(),
      h
    );
  }

  pub fn to_string(&self) -> String {
    let h;

    if let Some(h_m) = self.config.help_msg.clone() {
      h = format!("\n{}", h_m);
    } else {
      h = String::new();
    }

    format!(
      "{}:{}: {}: {}\n    |\n  {} | {}\n    | {}{}",
      self.config.row_index.to_string().c_bright_white(),
      self.config.col_index.to_string().c_bright_white(),
      self.config.err_kind.to_string().c_red(),
      self.config.err_msg,
      self.config.row_index,
      self.config.input_pre,
      self.config.err_marker.c_red(),
      h
    )
  }
}