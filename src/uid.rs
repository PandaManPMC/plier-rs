use uuid::Uuid;

pub fn uid_v4() -> String {
    let u = Uuid::new_v4();
    return u.to_string();
}
