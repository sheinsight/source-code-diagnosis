use tokei::{Config, Languages};

fn main() {
  // The paths to search. Accepts absolute, relative, and glob paths.
  let paths = &["."];
  // Exclude any path that contains any of these strings.
  let excluded = &["target", "node_modules"];
  // `Config` allows you to configure what is searched and counted.
  let config = Config::default();

  let mut languages = Languages::new();
  languages.get_statistics(paths, excluded, &config);
  // let rust = &languages[&LanguageType::Rust];

  languages.iter().for_each(|(lang, stats)| {
    println!("{}: {:?}", lang, stats.code);
  });

  // println!("Lines of code: {:?}", languages);
}
