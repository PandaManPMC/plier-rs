
use plier::md;

#[cfg(test)]
mod files_md {
    use super::*;

    #[test]
    fn test_sha() {
        let ciphertext = md::sha1("123456".to_string());
        println!("{:?}", ciphertext);

        let ciphertext = md::sha256("123456".to_string());
        println!("{:?}", ciphertext);

        let ciphertext = md::sha512("123456".to_string());
        println!("{:?}", ciphertext);

        let ciphertext = md::md5("123456".to_string());
        println!("{:?}", ciphertext);
    }

}