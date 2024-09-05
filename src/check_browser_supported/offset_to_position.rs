use ropey::Rope;
use tower_lsp::lsp_types::Position;

pub(crate) fn offset_to_position(
  offset: usize,
  source_text: &str,
) -> Option<Position> {
  let rope = Rope::from_str(source_text);
  let line = rope.try_byte_to_line(offset).ok()?;
  let first_char_of_line = rope.try_line_to_char(line).ok()?;
  // Original offset is byte, but Rope uses char offset
  let offset = rope.try_byte_to_char(offset).ok()?;
  let column = offset - first_char_of_line;
  Some(Position::new(line as u32, column as u32))
}
