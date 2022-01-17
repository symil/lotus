#[allow(unused)]
pub fn get_type_name<T>() -> String {
    std::any::type_name::<T>().split("::").last().unwrap().to_string()
}