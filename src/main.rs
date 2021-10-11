use crate::auth::Auth;
use crate::push::Push;
use lazy_static::lazy_static;
use log::{error, info, warn};
use reqwest::blocking::{Client, ClientBuilder};
use serde_json::Value;
use std::env;
use std::thread::sleep;
use std::time::Duration;

mod auth;
mod push;

lazy_static! {
    pub static ref CLIENT: Client = ClientBuilder::new()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; WOW64; rv:56.0) Gecko/20100101 Firefox/56.0")
        .cookie_store(true)
        .build()
        .unwrap();
}

const URL_INFO_LIST: &str =
    "http://ehallapp.nju.edu.cn/xgfw/sys/yqfxmrjkdkappnju/apply/getApplyInfoList.do";
const URL_INFO_APPLY: &str =
    "http://ehallapp.nju.edu.cn/xgfw/sys/yqfxmrjkdkappnju/apply/saveApplyInfos.do";

fn main() {
    env_logger::init();

    info!("try to login in");
    let username = env::var("USERNAME")
        .expect("please add secret USERNAME follow the instructions in README.md");
    let password = env::var("PASSWORD")
        .expect("please add secret PASSWORD follow the instructions in README.md");
    let resp = Auth::new(username, password).login();
    let push = Push::new(env::var("SENDKEY").ok());
    if resp.status() != 200 {
        error!("login in failed, maybe caused by password error or network bad, please try again");
        push.err();
        return;
    }
    info!("login in successfully");

    for i in 1..=3 {
        info!("try to clock in, times: {}", i);
        let resp = CLIENT.get(URL_INFO_LIST).send().unwrap();
        if resp.status() != 200 {
            warn!("get clokck in info-list failed");
            sleep(Duration::from_secs(5));
            continue;
        }
        let value: Value = serde_json::from_str(&resp.text().unwrap()).unwrap();
        let clock_in_info = &value["data"][0];
        if clock_in_info["TBZT"] == "0" {
            let location = env::var("LOCATION")
                .expect("please add secret LOCATION follow the instructions in README.md");
            CLIENT
                .get(format!(
                    "{}?WID={}&IS_TWZC=1&CURR_LOCATION={}&JRSKMYS=1&IS_HAS_JKQK=1&JZRJRSKMYS=1",
                    URL_INFO_APPLY,
                    clock_in_info["WID"].as_str().unwrap(),
                    location
                ))
                .send()
                .unwrap();
            sleep(Duration::from_secs(1));
        } else {
            info!("clock in successfully");
            push.ok();
            return;
        }
    }
    error!("clock in failed");
    push.err();
}
