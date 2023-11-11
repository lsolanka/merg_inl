use faccess::PathExt;
use regex::RegexBuilder;
use std::fmt;
use std::{
    error::Error,
    fs, io,
    path::{Path, PathBuf},
    process,
};

#[derive(Debug)]
struct ErrorList {
    preamble: String,
    errors: Vec<Box<dyn Error>>,
}

impl ErrorList {
    fn new(preamble: &str, errors: Vec<Box<dyn Error>>) -> ErrorList {
        let preamble = String::from(preamble);
        ErrorList { preamble, errors }
    }
}

impl fmt::Display for ErrorList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.preamble)?;
        for error in self.errors.iter() {
            writeln!(f, "{}", error)?;
        }

        Ok(())
    }
}

impl Error for ErrorList {}

#[derive(Debug)]
struct MergeError {
    msg: String,
}

impl MergeError {
    fn new<S: Into<String>>(msg: S) -> MergeError {
        MergeError { msg: msg.into() }
    }
}

impl fmt::Display for MergeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.msg)?;
        Ok(())
    }
}

impl Error for MergeError {}

pub fn merge(inl_files: &Vec<PathBuf>) -> Result<(), Box<dyn Error>> {
    if let Err(error) = check_if_args_exist(inl_files) {
        eprint!("{}", error.to_string());
        process::exit(1);
    }

    let mut error_list = vec![];
    for inl_file in inl_files.iter() {
        if let Err(error) = merge_one(inl_file) {
            error_list.push(error);
        }
    }

    if error_list.len() == 0 {
        Ok(())
    } else {
        Err(Box::new(ErrorList::new(
            "Some files failed to merge:",
            error_list,
        )))
    }
}

pub fn merge_one(inl_file: &Path) -> Result<(), Box<dyn Error>> {
    log::trace!("merge_one: inl_file: {}", inl_file.display());
    let Some(parent_path) = get_parent_file_path(inl_file) else {
        let msg = format!(
            "{} is not a file with `-inl.h` or `_inl.h` suffix; skipping",
            inl_file.display()
        );
        log::warn!("{}", msg);
        return Err(Box::new(MergeError::new(msg)));
    };

    let Ok(parent_file) = fs::read_to_string(&parent_path) else {
        let msg = std::format!(
            "cannot open parent file: {} ; skipping",
            parent_path.display()
        );
        log::warn!("{}", msg);
        return Err(Box::new(MergeError::new(msg)));
    };
    let Some(inl_file_relative) = get_include_relative_path(inl_file) else {
        let msg = std::format!(
            "{} is not in any include folder; skipping",
            inl_file.display()
        );
        log::warn!("{}", msg);
        return Err(Box::new(MergeError::new(msg)));
    };

    if !contains_include(&parent_file, &inl_file_relative.to_string_lossy()) {
        let msg = std::format!(
            "{} does not contain the requested -inl.h file: {}; skipping",
            parent_path.display(),
            inl_file_relative.display()
        );
        log::warn!("{}", msg);
        return Err(Box::new(MergeError::new(msg)));
    }

    Ok(())
}

fn contains_include(file: &str, include_path: &str) -> bool {
    let re = RegexBuilder::new(&std::format!(
        r#"^[ \t]*#include\s+[<"]{}[>"]"#,
        include_path
    ))
    .multi_line(true)
    .build()
    .unwrap();
    re.is_match(file)
}

/// Get the path of `inl_file` relative to the include folder.
/// Returns None if there is no `include` in the path
fn get_include_relative_path(inl_file: &Path) -> Option<PathBuf> {
    use std::ffi::OsStr;
    use std::path::Component;

    let mut include_relative_path = PathBuf::new();
    let mut include_found = false;
    for component in inl_file.components() {
        if include_found {
            include_relative_path.push(component);
        } else {
            if component == Component::Normal(OsStr::new("include")) {
                include_found = true;
            }
        }
    }

    if include_found {
        Some(include_relative_path)
    } else {
        None
    }
}

fn get_parent_file_path(inl_file: &Path) -> Option<PathBuf> {
    if inl_file.to_string_lossy().ends_with("-inl.h")
        || inl_file.to_string_lossy().ends_with("_inl.h")
    {
        let file_as_str = inl_file.to_string_lossy();
        let parent = String::from(&file_as_str[0..file_as_str.len() - 6]) + ".h";
        Some(PathBuf::from(parent))
    } else {
        None
    }
}

fn check_if_args_exist(file_paths: &Vec<PathBuf>) -> Result<(), io::Error> {
    let mut bad_files: Vec<&Path> = vec![];

    for file_path in file_paths.iter() {
        if !(file_path.is_file() && file_path.readable()) {
            bad_files.push(file_path);
        }
    }

    if !bad_files.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            String::from("The following files are not readable:\n")
                + &file_list_to_string(&bad_files),
        ));
    }

    Ok(())
}

fn file_list_to_string(files: &Vec<&Path>) -> String {
    let mut file_string = String::new();

    for file in files.iter() {
        file_string += &format!("{}\n", file.to_string_lossy());
    }

    file_string
}

#[cfg(test)]
mod test {
    mod test_get_parent_file_path {

        use crate::get_parent_file_path;
        use std::path::PathBuf;

        #[test]
        fn empty_input() {
            let parent_path = get_parent_file_path(&PathBuf::from(""));
            assert!(parent_path.is_none());
        }

        #[test]
        fn correct_input_with_dash() {
            let parent_path = get_parent_file_path(&PathBuf::from("dir/fancy-inl.h"));
            assert!(parent_path.is_some());
            assert_eq!(parent_path.unwrap(), PathBuf::from("dir/fancy.h"));
        }

        #[test]
        fn correct_input_with_underscore() {
            let parent_path = get_parent_file_path(&PathBuf::from("dir/fancy_inl.h"));
            assert!(parent_path.is_some());
            assert_eq!(parent_path.unwrap(), PathBuf::from("dir/fancy.h"));
        }

        #[test]
        fn no_inl_in_input() {
            let parent_path = get_parent_file_path(&PathBuf::from("dir/"));
            assert!(parent_path.is_none());

            let parent_path = get_parent_file_path(&PathBuf::from("dir/fancy.h"));
            assert!(parent_path.is_none());
        }
    }

    mod test_include_relative_path {
        use crate::get_include_relative_path;
        use std::path::PathBuf;

        #[test]
        fn empty_file_returns_none() {
            assert!(get_include_relative_path(&PathBuf::new()).is_none());
        }

        #[test]
        fn correct_file_but_no_include_returns_none() {
            assert!(
                get_include_relative_path(&PathBuf::from("this/is/my/include-inl.h")).is_none()
            );
        }

        #[test]
        fn correct_file_correct_result() {
            let relative_path = get_include_relative_path(&PathBuf::from(
                "/abs/path/include/my-package/my-include-inl.h",
            ));
            assert!(relative_path.is_some());
            assert_eq!(
                relative_path.unwrap(),
                PathBuf::from("my-package/my-include-inl.h")
            );
        }
    }

    mod test_contains_include {
        use crate::contains_include;

        #[test]
        fn file_contains_include() {
            let file_content = r#"
                class Foo {};

                #include <test/include-inl.h>
                "#;
            assert!(contains_include(file_content, "test/include-inl.h"));
        }

        #[test]
        fn does_not_contain_any_include() {
            let file_content = r#"
                class Foo {};

                1 + 2 + 3;
                "#;
            assert!(!contains_include(file_content, "test/include-inl.h"));
        }

        #[test]
        fn contains_different_include() {
            let file_content = r#"
                class Foo {};

                #include <test/include-inl.h>
                "#;
            assert!(!contains_include(file_content, "test/other-include-inl.h"));
        }

        #[test]
        fn contains_include_with_apostrophes() {
            let file_content = r#"
                class Foo {};

                #include "test/include-inl.h"
                "#;
            assert!(contains_include(file_content, "test/include-inl.h"));
        }
    }
}
