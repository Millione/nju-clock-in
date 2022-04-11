use crate::auth::{Auth, URL_AUTH};
use crate::pcr::Pcr;
use crate::push::Push;
use lazy_static::lazy_static;
use log::{error, info, warn};
use reqwest::blocking::{Client, ClientBuilder};
use serde_json::Value;
use std::env;
use std::thread::sleep;
use std::time::Duration;

mod auth;
mod pcr;
mod push;

lazy_static! {
    static ref CLIENT: Client = ClientBuilder::new()
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
    match env::var("DISABLE_CLOCK_IN").unwrap().as_str() {
        "false" => {}
        _ => {
            warn!("DISABLE_CLOCK_IN is true, skip");
            return;
        }
    }

    env_logger::init();

    info!("try to login in");
    let username = env::var("USERNAME").unwrap();
    let password = env::var("PASSWORD").unwrap();
    let resp = Auth::new(username.clone(), password).login();

    let push = Push::new(env::var("SENDKEY").unwrap());
    if resp.url().as_str() == URL_AUTH {
        error!("login in failed, maybe caused by password error or network bad, please check and try again");
        push.err();
        return;
    }
    info!("login in successfully");

    let location = env::var("LOCATION").unwrap();

    let pcr_time = Pcr::new(username, env::var("PCR_TIME").unwrap()).calc();

    for i in 1..=3 {
        info!("try to clock in, times: {}", i);
        let resp = CLIENT.get(URL_INFO_LIST).send().unwrap();
        if resp.status() != 200 {
            warn!("get clock in info-list failed");
            sleep(Duration::from_secs(5));
            continue;
        }
        let value: Value = match serde_json::from_str(&resp.text().unwrap()) {
            Ok(v) => v,
            Err(e) => {
                error!("resp.text: {}", e.to_string());
                break;
            }
        };
        let clock_in_info = &value["data"][0];
        if clock_in_info["TBZT"] == "0" {
            CLIENT
                .get(format!(
                    "{}?WID={}&IS_TWZC=1&CURR_LOCATION={}&ZJHSJCSJ={}&JRSKMYS=1&IS_HAS_JKQK=1&JZRJRSKMYS=1&SFZJLN=0",
                    URL_INFO_APPLY,
                    clock_in_info["WID"].as_str().unwrap(),
                    location,
                    pcr_time
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
