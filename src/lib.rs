use std::env;
use regex::Regex;
use std::fs;
use std::collections::HashMap;


pub fn bundle(filename: &str, outfile: &str, platform: &str) {
  let initial_dir = env::current_dir().unwrap();
  let mut output = String::new();

  let mut imports = vec![String::from(filename)];

  let mut imported_files_map = HashMap::new();

  while imports.len() > 0 {
    let import = imports.pop().unwrap();

    if imported_files_map.contains_key(&import) {
      continue;
    }

    imported_files_map.insert(import.clone(), true);

    let path = std::path::Path::new(&import);
    let parent = path.parent().unwrap();
    let _ = env::set_current_dir(parent);

    let import_with_ext = format!("{}{}", import, ".ts");

    let import_path = std::path::Path::new(&import_with_ext);
    let contents = fs::read_to_string(import_path).expect("Cannot read file");

    let mut imports_to_add = get_imports(&contents);
    imports.append(&mut imports_to_add);

    let cleaned_contents = transpile(&contents, &platform);

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
  let without_imports = get_import_regex().replace_all(&contents, "");

  let param_types_re = Regex::new(r": [\w]*").unwrap();
  let without_param_types = param_types_re.replace_all(&without_imports, "");

  return String::from(without_param_types);
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

    #[test]
    fn it_should_remove_param_types() {
        let cleaned = transpile("export function Text(text: string) {}", "browser");
        assert_eq!(cleaned, "function Text(text) {}");
    }
}