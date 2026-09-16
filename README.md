# Capmonster
Capmonster.cloud captcha solvers implemented in Rust

[[Crate](https://crates.io/crates/capmonster)] [[Docs](https://docs.rs/capmonster/latest/capmonster/)]

## Disclaimer
**This library is not endorsed by Capmonster.cloud and is an unofficial implementation.**

## Installation
`cargo add capmonster`

## Usage
```rust
use capmonster::CapMonster;
use capmonster::solver::{RecaptchaV2, RecaptchaV2Task};

pub async fn recaptcha_v2() {
    let client = reqwest::Client::new();
    let solver = CapMonster::new(client, "api_key", RecaptchaV2);
    
    let task_data = RecaptchaV2Task::new("www.example.com", "site_key");
    let task = solver
        .create_task(task_data)
        .await
        .expect("Failed to create task");
    
    let solution = task
        .wait_for_result(None, None)
        .await
        .expect("Failed to get solution");
    
    println!("Solution: {}", solution.g_recaptcha_response);
}
```

## Documentation
All solver fields are documented as required on [Capmonster's Documentation page](https://docs.capmonster.cloud/docs/getting-start/)

## Supported Captchas
- [x] **Recaptcha**
  - [x] _V2_
  - [x] _V3_
  - [x] _V2 Enterprise_
  - [x] _V3 Enterprise_
  - [x] _Click_
- [x] **GeeTest**
- [x] **Cloudflare**
  - [x] _Turnstile_
  - [x] _Challenge_
  - [x] _Waiting_ Room
- [x] **Datadome**
- [x] **Basilisk**
- [x] **TenDI**
- [x] **Amazon AWS WAF**
  - [x] _Captcha (Option 1)_
  - [x] _Captcha and Challenge (Option 2)_
  - [x] _Challenge (Option 3)_
- [x] **Binance**
- [x] **Imperva** (Incapsula)
- [x] **Prosopo**
- [x] **Yidun**
- [x] **MTCaptcha**
- [x] **Altcha**
- [x] **FunCaptcha**
- [x] **TSPD**
- [x] **FriendlyCaptcha**
- [x] **Complex Image**
  - [x] **Audio**
    - [x] _Bills_audio_
  - [x] **Coordinate**
    - [x] _Shein_
  - [x] **Grid**
    - [x] _Bls_
  - [x] **Rotation**
    - [x] _Baidu_
    - [x] _Betpunch_3x3_rotate_
    - [x] _oocl_rotate_double_new_
    - [x] _oocl_rotate_new_
  - [x] **Text**
    - [x] _Dli_ensemble_
    - [x] _Mathsum_
    - [x] _portugal_text_find_icon_
- [x] **ImageToText**
  - [x] _All Variants_
- [x] **Hunt Captcha**
- [x] **Alibaba Captcha**