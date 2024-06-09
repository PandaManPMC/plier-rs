use crypto::sha1::Sha1;
use crypto::sha2::Sha256;
use crypto::sha2::Sha512;
use crypto::digest::Digest;
use crypto::md5::Md5;


/// sha1 加密
pub fn sha1(plaintext: String) -> String {
    let mut hasher = Sha1::new();
    hasher.input_str(&plaintext);
    return hasher.result_str().to_string();
}

/// sha256 加密
pub fn sha256(plaintext: String) -> String{
    let mut hasher = Sha256::new();
    hasher.input_str(&plaintext);
    return hasher.result_str().to_string();
}

/// sha512 加密
pub fn sha512(plaintext: String) -> String{
    let mut hasher = Sha512::new();
    hasher.input_str(&plaintext);
    return hasher.result_str().to_string();
}

/// md5 加密
pub fn md5(plaintext: String) -> String{
    let mut hasher = Md5::new();
    hasher.input_str(&plaintext);
    return hasher.result_str().to_string();
}
