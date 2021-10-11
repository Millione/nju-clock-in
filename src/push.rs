use std::collections::HashMap;

use crate::CLIENT;

const URL_PUSH: &str = "https://sctapi.ftqq.com";

pub struct Push {
    send_key: Option<String>,
}

impl Push {
    pub fn new(send_key: Option<String>) -> Push {
        Self { send_key }
    }

    pub fn ok(self) {
        if let Some(key) = self.send_key {
            CLIENT
                .post(format!("{}/{}.send", URL_PUSH, key))
                .form(&Self::map(
                    "今日打卡成功".to_string(),
                    r#"[点击查看打卡详细信息](http://ehallapp.nju.edu.cn/xgfw/sys/yqfxmrjkdkappnju/apply/getApplyInfoList.do)"#.to_string(),
                ))
                .send()
                .unwrap();
        }
    }

    pub fn err(self) {
        if let Some(key) = self.send_key {
            CLIENT
                .post(format!("{}/{}.send", URL_PUSH, key))
                .form(&Self::map(
                    "今日打卡失败".to_string(),
                    r#"请自行前往APP手动打卡或重新触发Github Actions"#.to_string(),
                ))
                .send()
                .unwrap();
        }
    }

    fn map(title: String, desp: String) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("title".to_string(), title);
        map.insert("desp".to_string(), desp);
        map
    }
}
