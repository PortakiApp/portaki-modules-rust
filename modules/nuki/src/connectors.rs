//! Nuki Cloud connector — remote unlock via host egress (Bearer + POST).

#[portaki_sdk::custom_connector(
    id = "nuki",
    display_name_key = "connector.nuki.name",
    base_url = "https://api.nuki.io",
    credential_provider_id = "nuki"
)]
#[allow(dead_code)]
pub struct ModuleNuki;

#[allow(dead_code)]
impl ModuleNuki {
    #[portaki_sdk::connector_op(method = "POST", path = "/smartlock/{smartlockId}/action/unlock")]
    pub fn remote_unlock() {}
}
