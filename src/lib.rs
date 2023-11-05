use std::path::{Path, PathBuf};

pub fn get_parent_file_path(inl_file: &Path) -> Option<PathBuf> {
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
}
