use google_authenticator::GoogleAuthenticator;

pub fn google_secret(length: u8) -> String {
    let auth = GoogleAuthenticator::new();
    let secret = auth.create_secret(length);
    return secret;
}

pub fn google_code(secret: String, times_slice: u64) -> String {
    let auth = GoogleAuthenticator::new();
    let code = auth.get_code(&secret, times_slice).unwrap();
    return code;
}

pub fn google_verify_code(secret: String, code: String, times_slice: u64) -> bool {
    let auth = GoogleAuthenticator::new();
    let is_valid = auth.verify_code(&secret, &code, 1, times_slice);
    return is_valid;
}

pub fn google_qr_url(secret: String, name: String, title: String) -> String {
    let auth = GoogleAuthenticator::new();
    let qr_code_url = auth.qr_code_url(&secret, &name, &title, 200, 200, google_authenticator::ErrorCorrectionLevel::Medium);
    return qr_code_url;
}

