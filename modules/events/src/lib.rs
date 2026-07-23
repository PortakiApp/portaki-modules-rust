//! Portaki events module — local happenings near the property.

mod commands;
mod config;
mod email_context;
mod guest;
mod host;
mod ids;
mod queries;
mod time_format;

pub use commands::{update_config, EventInput, UpdateConfigArgs};
pub use config::{load_config, ModuleConfig};
pub use email_context::{email_context, EmailContextArgs, EmailContextResponse};
pub use guest::{render_explore_detail, render_home_card};
pub use host::render_host_main;
pub use queries::get_config;

portaki_sdk::portaki_module!(
    id = "events",
    display_name_key = "module.displayName",
    description_key = "module.description",
    author = "Syntax Labs",
);

#[portaki_sdk::capability(required, id = "core.storage")]
pub const STORAGE: &str = "core.storage";
