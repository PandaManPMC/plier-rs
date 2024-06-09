extern crate crypto;
extern crate hex;

use crypto::aes::{cbc_decryptor, cbc_encryptor, KeySize};
use crypto::blockmodes::PkcsPadding;
use crypto::buffer::{BufferResult, ReadBuffer, RefReadBuffer, RefWriteBuffer, WriteBuffer};

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

pub fn aes256_decrypt(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let mut decryptor = cbc_decryptor(KeySize::KeySize256, key, iv, PkcsPadding);

    let mut read_buffer = RefReadBuffer::new(encrypted_data);
    let mut buffer = [0; 4096];
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);

    let mut result = Vec::new();
    loop {
        let res = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true).unwrap();
        result.extend(write_buffer.take_read_buffer().take_remaining().iter().copied());

        match res {
            BufferResult::BufferUnderflow => break,
            BufferResult::BufferOverflow => {}
        }
    }

    result
}
