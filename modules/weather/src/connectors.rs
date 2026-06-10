//! OpenWeather connector declaration for the weather module manifest.
//! HTTP paths are owned by the module; runtime executes generic egress from this metadata.

#[portaki_sdk::custom_connector(
    id = "open-weather",
    display_name_key = "connector.openWeather.name",
    base_url = "https://api.openweathermap.org",
    credential_provider_id = "open-weather"
)]
pub struct ModuleOpenWeather;

impl ModuleOpenWeather {
    #[portaki_sdk::connector_op(method = "GET", path = "/data/2.5/weather")]
    pub fn current() {}

    #[portaki_sdk::connector_op(method = "GET", path = "/data/2.5/forecast")]
    pub fn forecast() {}
}
