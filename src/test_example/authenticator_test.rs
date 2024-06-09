use google_authenticator::GoogleAuthenticator;

#[cfg(test)]
mod authenticator_test {
    use plier::authenticator;
    use super::*;

    #[test]
    fn test_get_current_dir() {
        let secret = authenticator::google_secret(32);
        println!("Secret: {}", secret);

        let code = authenticator::google_code(secret.clone(), 0);
        println!("Code: {}", code);

        let is_valid = authenticator::google_verify_code(secret.clone(), code, 1);
        println!("Is code valid? {}", is_valid);

        let qr_code_url = authenticator::google_qr_url(secret, "your-app".to_string(), "your-email@example.com".to_string());
        println!("QR Code URL: {}", qr_code_url);
    }
}
