use napi_derive::napi;
use serde::{Deserialize, Serialize};

#[napi(object)]
#[derive(
  Debug, Default, PartialEq, Eq, Hash, Clone, Serialize, Deserialize, Copy,
)]
pub struct Position {
  pub line: u32,
  pub col: u32,
}

impl Position {
  pub fn with_source(source_text: &str, offset: usize) -> Self {
    // Unicode 换行符
    // \u{000A}    // LF (Line Feed)
    // \u{000B}    // VT (Vertical Tab)
    // \u{000C}    // FF (Form Feed)
    // \u{000D}    // CR (Carriage Return)
    // \u{0085}    // NEL (Next Line)
    // \u{2028}    // LS (Line Separator)
    // \u{2029}    // PS (Paragraph Separator)

    // let normalized_text = source_text
    //   .replace('\u{85}', " ") // NEL
    //   .replace('\u{2028}', " ") // LS
    //   .replace('\u{2029}', " "); // PS

    let rope = ropey::Rope::from_str(&source_text);
    let line = rope.try_byte_to_line(offset).unwrap_or(0);

    let first_char_of_line = rope.try_line_to_char(line).unwrap_or(0);
    let offset = rope.try_byte_to_char(offset).unwrap_or(0);
    let col = offset - first_char_of_line;

    Self {
      line: (line + 1) as u32,
      col: (col + 1) as u32,
    }
  }
}
