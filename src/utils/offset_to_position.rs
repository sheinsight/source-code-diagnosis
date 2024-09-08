use ropey::Rope;

use super::ast_node::Position;

// use tower_lsp::lsp_types::Position;

pub fn offset_to_position(
  offset: usize,
  source_text: &str,
) -> Option<Position> {
  let rope = Rope::from_str(source_text);
  let line = rope.try_byte_to_line(offset).ok()?;
  let first_char_of_line = rope.try_line_to_char(line).ok()?;
  // Original offset is byte, but Rope uses char offset
  let offset = rope.try_byte_to_char(offset).ok()?;
  let col = offset - first_char_of_line;
  Some(Position {
    line: line as u32,
    col: col as u32,
  })
}
