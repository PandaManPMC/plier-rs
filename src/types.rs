
/// type_of_string 获取指针对象的类型 - String
pub fn type_of_string<T>(_: &T) -> String {
    return format!("{}", std::any::type_name::<T>());
}
