use plier::aes;
use std::str;

#[cfg(test)]
mod aes_test {
    use plier::aes::{aes256_decrypt, aes256_decrypt_string, aes256_encrypt, aes256_encrypt_string};
    use super::*;

    #[test]
    fn test1() {
        // 256 位密钥 (32 bytes)
        let key = b"0123456789abcdef0123456789abcdef";
        // 128 位初始化向量 (16 bytes)
        let iv = b"abcdef9876543210";

        // 原始数据
        let data = b"hello world";

        // 加密数据
        let encrypted_data = aes256_encrypt(data, key, iv);
        println!("Encrypted data: {}", hex::encode(&encrypted_data));

        // 解密数据
        let decrypted_data = aes256_decrypt(&encrypted_data, key, iv);
        let d = decrypted_data.unwrap();
        println!("Decrypted data: {}", str::from_utf8(&d).unwrap());
    }

    #[test]
    fn test2() {
        // 256 位密钥 (32 bytes)
        let key = "0123456789abcdef0123456789abcdef";
        // 128 位初始化向量 (16 bytes)
        let iv = "abcdef9876543210";

        // 原始数据
        let data = "hello world";

        // 加密数据
        let encrypted_data = aes256_encrypt_string(data.to_string(), key.to_string(), iv.to_string());
        println!("Encrypted data: {}", hex::encode(&encrypted_data));

        // 解密数据
        let decrypted_data = aes256_decrypt_string(encrypted_data, key.to_string(), iv.to_string());

        let d = decrypted_data;
        println!("Decrypted data: {:?}", d);
    }

    #[test]
    fn test3() {
        // 使用字节字符串字面量
        let byte_literal = b"hello world";
        println!("{:?}", byte_literal);

        // 使用 String 并转换为字节数组
        let s = String::from("hello world");
        let bytes = s.as_bytes();
        println!("{:?}", bytes);
    }
}