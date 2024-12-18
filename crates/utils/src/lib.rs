mod glob;
mod semantic_builder;

use std::{
  fs,
  io::{BufReader, Read},
};

pub use glob::{glob_by, GlobArgs, GlobJsArgs};
pub use semantic_builder::{
  source_type_from_path, SemanticBuilder, SemanticHandler,
};

pub fn read_file_content(path: &std::path::Path) -> anyhow::Result<String> {
  let file = fs::File::open(path)?;
  let mut reader = BufReader::with_capacity(1024 * 1024, file); // 1MB buffer
  let mut content = String::new();
  reader.read_to_string(&mut content)?;
  Ok(content)
}

pub fn is_ts_video(path: &std::path::Path) -> bool {
  if let Ok(mut file) = fs::File::open(path) {
    let mut buffer = [0; 4];
    if file.read_exact(&mut buffer).is_ok() {
      // TS 视频文件的魔数是 0x47
      return buffer[0] == 0x47;
    }
  }
  false
}
