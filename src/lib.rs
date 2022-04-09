use regex::Regex;
use std::fs;

pub fn bundle(filename: &str) {
  let contents = fs::read_to_string(filename).expect("Cannot read file");

  let re = Regex::new(r#"import .* from ["'](.*)['"];?"#).unwrap();
  let cap = re.captures(&contents).unwrap();

  let cleaned_main = re.replace_all(&contents, "");

  let import = &cap[1];
  let import_with_ext = format!("{}{}", import, ".ts");
  let import_contents = fs::read_to_string(import_with_ext).expect("Cannot read file");
  let export_re = Regex::new(r"export ").unwrap();
  let cleaned_contents = export_re.replace_all(&import_contents, "");

  let output = format!("{}{}", cleaned_contents, cleaned_main);
  fs::write("app.js", output).expect("Failed to write output");
}

pub fn process_input_file_contents(contents: &str) {
  let imports = get_imports(contents);
}

pub fn get_import_regex() -> Regex {
  // https://docs.rs/regex/latest/regex/
  // lazy_static! {
  let re = Regex::new(r#"import .* from ["'](.*)['"];?"#).unwrap();
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

pub fn transpile(contents: &str) {
  let cleaned = get_import_regex().replace_all(&contents, "");

  // return cleaned;
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
}