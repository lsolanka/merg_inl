use faccess::PathExt;
use std::error::Error;
use std::io;
use std::path::{Path, PathBuf};
use std::process;

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

    // TODO: process error list

    Ok(())
}

pub fn merge_one(inl_file: &Path) -> Result<(), Box<dyn Error>> {
    let Some(parent_path) = get_parent_file_path(inl_file) else {
        eprintln!(
            "{} is not a file with `-inl.h` or `_inl.h` suffix; skipping",
            inl_file.display()
        );

        return Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Test",
        )));
    };

    println!(
        "{} parent: {}",
        inl_file.display(),
        parent_path.to_str().unwrap()
    );

    Ok(())
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
}
