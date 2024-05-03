use std::env;


/// get_current_dir_str 当前所在路径
pub fn get_current_dir_str() -> String {
    let p = env::current_dir();
    if let Ok(a) = p {
        return a.into_os_string().into_string().unwrap();;
    }
    return "".to_string();
}

