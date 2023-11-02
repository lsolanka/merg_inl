use std::path::PathBuf;

pub fn get_parent_file_path(inl_file: &str) -> Option<PathBuf> {
    if inl_file.ends_with("-inl.h") || inl_file.ends_with("_inl.h") {
        let parent = String::from(&inl_file[0..inl_file.len()-6]) + ".h";
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
        let parent_path = get_parent_file_path("");
        assert!(parent_path.is_none());
    }

    #[test]
    fn correct_input_with_dash() {
        let parent_path = get_parent_file_path("dir/fancy-inl.h");
        assert!(parent_path.is_some());
        assert_eq!(parent_path.unwrap(), PathBuf::from("dir/fancy.h"));
    }

    #[test]
    fn correct_input_with_underscore() {
        let parent_path = get_parent_file_path("dir/fancy_inl.h");
        assert!(parent_path.is_some());
        assert_eq!(parent_path.unwrap(), PathBuf::from("dir/fancy.h"));
    }

    #[test]
    fn no_inl_in_input() {
        let parent_path = get_parent_file_path("dir/");
        assert!(parent_path.is_none());

        let parent_path = get_parent_file_path("dir/fancy.h");
        assert!(parent_path.is_none());
    }
}
}
