//! Integration-style unit tests with `portaki-test-utils`.

use nuki::{
    get_config, get_guest_credential, unlock, update_config, StayArgs, UpdateConfigArgs, SMART_LOCK,
};
use portaki_sdk::capability;
use portaki_sdk::contracts::smart_lock;
use portaki_test_utils::MockContext;
use serial_test::serial;

fn sample_config_bytes() -> Vec<u8> {
    serde_json::to_vec(&serde_json::json!({
        "smartlock_id": "lock-abc",
        "keypad_code": "482910",
        "device_name": "Front door"
    }))
    .expect("config json")
}

#[test]
fn module_declares_smart_lock_capability() {
    assert_eq!(SMART_LOCK, smart_lock::CAPABILITY.as_str());
    assert_eq!(smart_lock::UNLOCK.as_str(), "unlock");
    assert_eq!(
        smart_lock::GET_GUEST_CREDENTIAL.as_str(),
        "getGuestCredential"
    );
}

#[test]
#[serial]
fn get_guest_credential_returns_keypad_code() {
    MockContext::guest()
        .with_capabilities(&[capability::core::STORAGE])
        .with_kv("config", sample_config_bytes())
        .run(|ctx| {
            let cred =
                get_guest_credential(ctx, StayArgs { stay_id: None }).expect("getGuestCredential");
            assert_eq!(cred.credential_type, "keypad");
            assert_eq!(cred.code, "482910");
            assert_eq!(cred.smartlock_id, "lock-abc");
        });
}

#[test]
#[serial]
fn get_guest_credential_errors_without_keypad() {
    MockContext::guest()
        .with_capabilities(&[capability::core::STORAGE])
        .with_kv(
            "config",
            serde_json::to_vec(&serde_json::json!({
                "smartlock_id": "lock-abc",
                "keypad_code": "",
                "device_name": ""
            }))
            .unwrap(),
        )
        .run(|ctx| {
            let err =
                get_guest_credential(ctx, StayArgs { stay_id: None }).expect_err("expected error");
            assert!(err.to_string().contains("keypad_code"));
        });
}

#[test]
#[serial]
fn unlock_returns_credential_fallback() {
    MockContext::guest()
        .with_capabilities(&[capability::core::STORAGE])
        .with_kv("config", sample_config_bytes())
        .run(|ctx| {
            let result = unlock(
                ctx,
                StayArgs {
                    stay_id: Some("stay-1".into()),
                },
            )
            .expect("unlock");
            assert!(result.ok);
            assert_eq!(result.mode, "credential_fallback");
            assert_eq!(result.code, "482910");
        });
}

#[test]
#[serial]
fn update_config_roundtrip() {
    MockContext::host()
        .with_capabilities(&[capability::core::STORAGE])
        .run(|ctx| {
            update_config(
                ctx.clone(),
                UpdateConfigArgs {
                    smartlock_id: "nuki-99".into(),
                    keypad_code: "123456".into(),
                    device_name: "Entry".into(),
                },
            )
            .expect("updateConfig");
            let config = get_config(ctx).expect("getConfig");
            assert_eq!(config.smartlock_id, "nuki-99");
            assert_eq!(config.keypad_code, "123456");
            assert_eq!(config.device_name, "Entry");
        });
}
