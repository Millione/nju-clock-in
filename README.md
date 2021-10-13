<p align = "center">
    <img width = "100px" src = "imgs/nju.svg" align = "center" alt = "南京大学健康打卡" />
    <h3 align= "center">南京大学健康打卡</h3>
</p>

<p align = "center">
    <a href = "https://www.rust-lang.org/">
        <img alt = "rust" src = "https://img.shields.io/badge/language-rust-brightgreen" />
    </a>
    <a href = "LICENSE">
        <img alt = "license" src = "https://img.shields.io/badge/license-MIT-blue.svg" />
    </a>
    <a href = "https://GitHub.com/Millione/nju-clock-in/network/">
        <img alt = "stars" src = "https://badgen.net/github/stars/Millione/nju-clock-in/" />
    </a>
    <a href = "https://GitHub.com/Millione/nju-clock-in/network/">
        <img alt = "forks" src = "https://badgen.net/github/forks/Millione/nju-clock-in/" />
    </a>
</p>


## 使用
1. Fork 本仓库

2. 点击 Settings -> Secrets -> New repository secret

![创建密钥](imgs/setup.png)

3. 分别添加3个secret，如下 `Name`: `Value`

    * `USERNAME`: 统一认证账号
    * `PASSWORD`: 统一认证密码
    * `LOCATION`: 打卡位置

4. 完成，打卡将在每日北京时间18:00开始


## 打卡状态微信通知
1. 点击[Server酱](https://sct.ftqq.com/login)，微信扫码登录后关注，并在网页上点此继续

2. 获取[SENDKEY](https://sct.ftqq.com/sendkey)

3. 按[使用](#使用)步骤2、3将SENDKEY添加进密钥中

    * `SENDKEY`: 网页获取值


## 打卡时间修改
1. 修改`.github/workflows/main.yml`中如下参数

![修改打卡时间](imgs/cron.png)

2. 10表示UTC时间10:00，对应北京时间18:00，修改此值即可修改打卡时间

3. 如若有复杂的定时需求，可自行查阅[cron](https://www.gairuo.com/p/cron-expression-sheet)规则进行相应修改


---
Related to [kottory](https://github.com/kottory/NJU-health-report)