use std::env;
use regex::Regex;
use std::fs;
use std::collections::HashMap;


pub fn bundle(filename: &str, outfile: &str, platform: &str) {
  let initial_dir = env::current_dir().unwrap();
  let mut output = String::new();

  let mut imported_files_map = HashMap::new();

  let path = std::path::Path::new(&filename);

  let mut imports = vec![String::from(path.file_stem().unwrap().to_str().unwrap())];
    
  let parent = path.parent().unwrap();
  let _ = env::set_current_dir(parent);

  while imports.len() > 0 {
    let import = imports.pop().unwrap();

    let import_with_ext = format!("{}{}", import, ".js");

    let import_path = std::path::Path::new(&import_with_ext);
    let contents = fs::read_to_string(import_path).expect(format!("Cannot read {}", import_with_ext).as_str());

    let path = std::path::Path::new(&import);
    let parent = path.parent().unwrap();

    let imports_to_add = get_imports(&contents);
    for import_to_add in imports_to_add {
      let full_path = parent.join(import_to_add);

      if imported_files_map.contains_key(&import) {
        continue;
      }

      let full_import_path = String::from(full_path.to_str().unwrap());
      imports.push(full_import_path.clone());
      imported_files_map.insert(full_import_path.clone(), true);
    }

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

  // let param_types_re = Regex::new(r": [\w]*").unwrap();
  // let without_param_types = param_types_re.replace_all(&without_imports, "");

  return String::from(without_imports);
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