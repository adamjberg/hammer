use std::env;
use regex::Regex;
use std::fs;

pub fn bundle(filename: &str, outfile: &str, platform: &str) {
  let initial_dir = env::current_dir().unwrap();

  let path = std::path::Path::new(filename);
  let parent = path.parent().unwrap();

  println!("{}", parent.display());
  let contents = fs::read_to_string(path).expect("Cannot read file");

  let imports = get_imports(&contents);

  let mut output = transpile(&contents, platform);

  let _ = env::set_current_dir(parent);
  for import in imports {
    let import_with_ext = format!("{}{}", import, ".ts");
    let import_contents = fs::read_to_string(import_with_ext).expect("Cannot read file");
    let cleaned_contents = transpile(&import_contents, &platform);

    output = format!("{}{}", cleaned_contents, output);
  }
  
  let _ = env::set_current_dir(initial_dir);
  fs::write(outfile, output).expect("Failed to write output");
}

pub fn get_import_regex() -> Regex {
  // https://docs.rs/regex/latest/regex/
  // lazy_static! {
  let re = Regex::new(r#"import .* from ["'](.*)['"];?[\n]?"#).unwrap();
  // }

  return re;
}

pub fn get_imports(contents: &str) -> std::vec::Vec<String> {
  let mut imports: std::vec::Vec<String> = vec![];

  let re = get_import_regex();
  for caps in re.captures_iter(contents) {
    let import = String::from(&caps[1]);
    imports.push(import);
  }

  return imports;
}

pub fn transpile(contents: &str, platform: &str) -> String {
  if platform == "node" {
    return String::from(contents);
  }
  let cleaned = get_import_regex().replace_all(&contents, "");

  let export_re = Regex::new(r"export ").unwrap();
  let without_exports = export_re.replace_all(&cleaned, "");

  return String::from(without_exports);
}

#[cfg(test)]
mod tests {
  use super::*;

    #[test]
    fn it_should_return_single_import() {
        let imports = get_imports("import test from './test';");
        assert_eq!(imports.len(), 1);
        assert_eq!(imports[0], "./test")
    }

    #[test]
    fn it_should_return_with_imports() {
        let imports = get_imports("import test from './test';\nimport test2 from './test2';");
        assert_eq!(imports.len(), 2);
        assert_eq!(imports[0], "./test");
        assert_eq!(imports[1], "./test2");
    }

    #[test]
    fn it_should_remove_imports() {
        let cleaned = transpile("import test from './test';\nimport test2 from './test2';", "browser");
        assert_eq!(cleaned, "");
    }
}