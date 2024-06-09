use plier::aes;
use std::str;

#[cfg(test)]
mod aes_test {
    use plier::aes::{aes256_decrypt, aes256_encrypt};
    use super::*;

    #[test]
    fn test1() {
        // 256 位密钥 (32 bytes)
        let key = b"an example very very secret key.";
        // 128 位初始化向量 (16 bytes)
        let iv = b"unique init vect";

        // 原始数据
        let data = b"hello world";

        // 加密数据
        let encrypted_data = aes256_encrypt(data, key, iv);
        println!("Encrypted data: {}", hex::encode(&encrypted_data));

        // 解密数据
        let decrypted_data = aes256_decrypt(&encrypted_data, key, iv);
        println!("Decrypted data: {}", str::from_utf8(&decrypted_data).unwrap());
    }
}