
use plier::files;

#[cfg(test)]
mod files_test {
    use super::*;

    #[test]
    fn test_get_current_dir() {
        let path = files::get_current_dir_str();
        println!("{:?}", path);
    }
}