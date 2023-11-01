use std::path::{Path, PathBuf};

pub fn get_parent_file_path(inl_file: &str) -> Option<PathBuf> {
    if inl_file.ends_with("-inl.h") {
        let parent = String::from(&inl_file[0..inl_file.len()-6]) + ".h";
        Some(PathBuf::from(parent))
    } else {
        None 
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_get_parent_file_path() {

    }
}
