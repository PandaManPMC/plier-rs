use uuid::Uuid;

/// uid_v4 uid v4 不带 -
pub fn uid_v4() -> String {
    let u = Uuid::new_v4();
    return u.to_string().replace("-", "");
}

pub fn uid_v4_original() -> String {
    let u = Uuid::new_v4();
    return u.to_string();
}

