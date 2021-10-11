use crate::CLIENT;
use aes::Aes128;
use block_modes::{block_padding::Pkcs7, BlockMode, Cbc};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use regex::Regex;
use reqwest::blocking::Response;
use std::collections::HashMap;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

const URL_AUTH: &str = "https://authserver.nju.edu.cn/authserver/login";
const AES_BLOCK_SIZE: usize = 128;

pub struct Auth {
    form: HashMap<&'static str, String>,
}

impl Auth {
    pub fn new(username: String, password: String) -> Auth {
        let text = CLIENT.get(URL_AUTH).send().unwrap().text().unwrap();
        let mut form = HashMap::with_capacity(8);
        form.insert("username", username);
        form.insert(
            "password",
            Self::encrypt_password(
                password,
                Regex::new(r#"<input type="hidden" id="pwdDefaultEncryptSalt" value="(.*)""#)
                    .unwrap()
                    .captures(&text)
                    .unwrap()[1]
                    .to_string(),
            ),
        );
        form.insert(
            "lt",
            Regex::new(r#"<input type="hidden" name="lt" value="(.*)"/>"#)
                .unwrap()
                .captures(&text)
                .unwrap()[1]
                .to_string(),
        );
        form.insert("dllt", "userNamePasswordLogin".to_string());
        form.insert(
            "execution",
            Regex::new(r#"<input type="hidden" name="execution" value="(.*)"/>"#)
                .unwrap()
                .captures(&text)
                .unwrap()[1]
                .to_string(),
        );
        form.insert(
            "_eventId",
            Regex::new(r#"<input type="hidden" name="_eventId" value="(.*)"/>"#)
                .unwrap()
                .captures(&text)
                .unwrap()[1]
                .to_string(),
        );
        form.insert(
            "rmShown",
            Regex::new(r#"<input type="hidden" name="rmShown" value="(.*)""#)
                .unwrap()
                .captures(&text)
                .unwrap()[1]
                .to_string(),
        );
        Auth { form }
    }

    fn encrypt_password(password: String, pwd_default_encrypt_salt: String) -> String {
        let random_iv: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();
        let random_str: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(64)
            .map(char::from)
            .collect();
        let data = format!("{}{}", random_str, password);
        let cipher =
            Aes128Cbc::new_from_slices(pwd_default_encrypt_salt.as_bytes(), random_iv.as_bytes())
                .unwrap();
        let pos = data.len();
        let mut buffer = [0u8; AES_BLOCK_SIZE];
        buffer[..pos].copy_from_slice(data.as_bytes());
        base64::encode(cipher.encrypt(&mut buffer, pos).unwrap())
    }

    pub fn login(&self) -> Response {
        CLIENT.post(URL_AUTH).form(&self.form).send().unwrap()
    }
}
