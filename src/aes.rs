extern crate crypto;
extern crate hex;

use std::fmt::format;
use crypto::aes::{cbc_decryptor, cbc_encryptor, KeySize};
use crypto::blockmodes::PkcsPadding;
use crypto::buffer::{BufferResult, ReadBuffer, RefReadBuffer, RefWriteBuffer, WriteBuffer};

pub fn aes256_encrypt_string(data: String, key: String, iv: String) -> String {
    let encrypted_data = aes256_encrypt(data.as_bytes(), key.as_bytes(), iv.as_bytes());
    return hex::encode(&encrypted_data);
}

pub fn aes256_encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let mut encryptor = cbc_encryptor(KeySize::KeySize256, key, iv, PkcsPadding);

    let mut read_buffer = RefReadBuffer::new(data);
    let mut buffer = [0; 4096];
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);

    let mut result = Vec::new();
    loop {
        let res = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true).unwrap();
        result.extend(write_buffer.take_read_buffer().take_remaining().iter().copied());

        match res {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    result
}

pub fn aes256_decrypt_string(encrypted_data: String, key: String, iv: String) -> Result<String, String> {
    let res_decode = hex::decode(encrypted_data);
    if res_decode.is_err() {
        return Err(format!("{:?}", res_decode.err()))
    }
    let ve = res_decode.unwrap();

    let b = aes256_decrypt(&ve, key.as_bytes(), iv.as_bytes());
    if b.is_err() {
        return Err(b.err().unwrap());
    }

    let bi = b.unwrap();
    let r = std::str::from_utf8(&bi);
    return match r {
        Ok(s) => {
            Ok(s.to_string())
        },
        Err(e) => {
            Err(e.to_string())
        },
    }
}

pub fn aes256_decrypt(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Result<Vec<u8>, String> {
    let mut decryptor = cbc_decryptor(KeySize::KeySize256, key, iv, PkcsPadding);

    let mut read_buffer = RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);

    let mut result = Vec::new();
    loop {
        let res_o = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true);
        if res_o.is_err() {
            return Err(format!("{:?}",res_o.err()))
        }

        let res = res_o.unwrap();

        result.extend(write_buffer.take_read_buffer().take_remaining().iter().copied());

        match res {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    Ok(result)
}
