# Capmonster
Capmonster.cloud captcha solvers implemented in Rust

## Usage
```rust
use capmonster::CapMonster;
use capmonster::solver::{RecaptchaV2, RecaptchaV2Task};

pub async fn recaptcha_v2() {
    let client = wreq::Client::new();
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
- **Recaptcha**
  - _V2_
  - _V3_
  - _V2 Enterprise_
  - _V3 Enterprise_
- **GeeTest**
- **Cloudflare**
  - _Turnstile_
  - _Challenge_
  - _Waiting_ Room
- **Datadome**
- **Basilisk**
- **TenDI**
- **Amazon AWS WAF**
  - _Captcha (Option 1)_
  - _Captcha and Challenge (Option 2)_
  - _Challenge (Option 3)_
- **Binance**
- **Imperva** (Incapsula)
- **Prosopo**
- **Yidun**
- **MTCaptcha**
- **Altcha**
- **FunCaptcha**
- **TSPD**
- **FriendlyCaptcha**

## To Be Implemented
- **Recaptcha**
  - _Click_
- **Complex Image**
  - **Audio**
    - _Bills_audio_
  - **Coordinate**
    - _Shein_
  - **Grid**
    - _Bls_
  - **Rotation**
    - _Baidu_
    - _Betpunch_3x3_rotate_
    - _oocl_rotate_double_new_
    - _oocl_rotate_new_
  - **Text**
    - _Dli_ensemble_
    - _Mathsum_
    - _portugal_text_find_icon_
- **ImageToText**
    - _All Variants_
- **Hunt Captcha**
- **Alibaba Captcha**