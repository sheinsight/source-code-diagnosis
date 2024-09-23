mod ast_node;
mod location;
mod position;
mod span;

pub use ast_node::AstNode;
pub use location::Location;
pub use position::Position;
pub use span::Span;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_position() {
    let pos = Position { line: 1, col: 1 };
    assert_eq!(pos.line, 1);
    assert_eq!(pos.col, 1);
  }

  #[test]
  fn test_location() {
    let loc = Location {
      start: Position { line: 1, col: 1 },
      end: Position { line: 2, col: 2 },
    };
    assert_eq!(loc.start.line, 1);
    assert_eq!(loc.start.col, 1);
    assert_eq!(loc.end.line, 2);
    assert_eq!(loc.end.col, 2);
  }

  #[test]
  fn test_span() {
    let span = Span { start: 0, end: 10 };
    assert_eq!(span.start, 0);
    assert_eq!(span.end, 10);
  }

  #[test]
  fn test_ast_node() {
    let loc = Location {
      start: Position { line: 1, col: 1 },
      end: Position { line: 2, col: 2 },
    };
    let node = AstNode::new((0, 10), loc);
    assert_eq!(node.span.start, 0);
    assert_eq!(node.span.end, 10);
    assert_eq!(node.loc.start.line, 1);
    assert_eq!(node.loc.start.col, 1);
    assert_eq!(node.loc.end.line, 2);
    assert_eq!(node.loc.end.col, 2);
  }
}
