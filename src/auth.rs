use crate::CLIENT;
use aes::Aes128;
use block_modes::{block_padding::Pkcs7, BlockMode, Cbc};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use regex::Regex;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

pub const URL_AUTH: &str = "https://authserver.nju.edu.cn/authserver/login";
const AES_BLOCK_SIZE: usize = 128;

pub struct Auth {
    username: String,
    password: String,
}

impl Auth {
    pub fn new(username: String, password: String) -> Auth {
        Auth { username, password }
    }

    pub fn with_form(&self) -> HashMap<&'static str, String> {
        let mut form: HashMap<&'static str, String> = HashMap::with_capacity(8);
        let text = CLIENT.get(URL_AUTH).send().unwrap().text().unwrap();
        form.insert("username", self.username.clone());
        form.insert(
            "password",
            self.encrypt_password(
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
        if self.need_captcha() {
            form.insert("captchaResponse", self.get_captcha());
        }
        form
    }

    pub fn login(&self) -> bool {
        for _ in 1..=10 {
            let resp = CLIENT
                .post(URL_AUTH)
                .form(&self.with_form())
                .send()
                .unwrap();
            if resp.url().as_str() != URL_AUTH {
                return true;
            }
            sleep(Duration::from_secs(1));
        }
        false
    }

    fn encrypt_password(&self, pwd_default_encrypt_salt: String) -> String {
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
        let data = format!("{}{}", random_str, self.password);
        let cipher =
            Aes128Cbc::new_from_slices(pwd_default_encrypt_salt.as_bytes(), random_iv.as_bytes())
                .unwrap();
        let pos = data.len();
        let mut buffer = [0u8; AES_BLOCK_SIZE];
        buffer[..pos].copy_from_slice(data.as_bytes());
        base64::encode(cipher.encrypt(&mut buffer, pos).unwrap())
    }

    fn need_captcha(&self) -> bool {
        CLIENT
            .get(format!(
                "https://authserver.nju.edu.cn/authserver/needCaptcha.html?username={}",
                self.username
            ))
            .send()
            .unwrap()
            .text()
            .unwrap()
            .contains("true")
    }

    fn get_captcha(&self) -> String {
        let res = CLIENT
            .get("https://authserver.nju.edu.cn/authserver/captcha.html")
            .send()
            .unwrap()
            .bytes()
            .unwrap();

        let mut ocr = leptess::LepTess::new(Some("./data"), "eng").unwrap();

        let _ = ocr.set_image_from_mem(res.as_ref());
        ocr.get_utf8_text()
            .unwrap()
            .replace('\n', "")
            .replace(' ', "")
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_ocr() {
        let mut ocr = leptess::LepTess::new(Some("./data"), "eng").unwrap();
        let _ = ocr.set_image("./data/captcha.jpg");
        assert_eq!(
            "Eb43",
            ocr.get_utf8_text()
                .unwrap()
                .replace('\n', "")
                .replace(' ', "")
        );
    }
}
