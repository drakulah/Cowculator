use crate::utils::CoerceAtLeast;

use super::ErrorConfig;

impl ErrorConfig {
  pub fn split_preview(&self, start: usize, endd: usize) -> (String, String, String) {
    let text_len = self.text.len();
    let mut total_chnk_size = 0;
    let mut err_chnk = String::new();
    let mut err_chnk_left = String::new();
    let mut err_chnk_right = String::new();

    let end;

    if endd >= text_len {
      end = text_len;
    } else {
      end = endd;
    }

    if start >= end || start >= text_len {
      return (err_chnk_left, err_chnk, err_chnk_right);
    }

    /* Get Err Chunk */
    if let Some(e) = self.text.get(start..end) {
      err_chnk = e.to_string();
      total_chnk_size = err_chnk.len();
    }

    let mut l = (start as i32 - 1).coerce_at_least(0) as usize;
    let mut r = end;

    if total_chnk_size >= self.err_pre_size {
    } else {
      loop {
        if total_chnk_size >= self.err_pre_size || (l < 1 && r >= text_len) {
          break;
        }

        /* Get Left Side */
        if l > 0 {
          l -= 1;
          total_chnk_size += 1;
        }

        /* Get Right Side */
        if r < text_len {
          r += 1;
          total_chnk_size += 1;
        }
      }

      err_chnk_left = self.text.get(l..start).unwrap().to_string();
      err_chnk_right = self.text.get(end..r).unwrap().to_string();
    }

    if l > 0 {
      err_chnk_left.replace_range(0..1, "…");
    }

    if r < text_len - 1 {
      err_chnk_right.pop();
      err_chnk_right.push('…');
    }

    return (err_chnk_left, err_chnk, err_chnk_right);
  }
}
