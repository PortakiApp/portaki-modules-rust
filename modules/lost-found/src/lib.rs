//! Portaki lost-found module — guest reports + host-declared found items.

mod commands;
mod config;
mod description;
mod email_context;
mod entities;
mod guest;
mod host;
mod ids;
mod kind;
mod queries;
mod status;
mod storage;

pub use commands::{
    submit, submit_found, update_config, update_status, SubmitArgs, SubmitFoundArgs,
    UpdateConfigArgs, UpdateStatusArgs,
};
pub use config::{load_config, ModuleConfig};
pub use email_context::{
    build_email_context, email_context, EmailContextArgs, EmailContextResponse,
};
pub use entities::LostFoundReport;
pub use guest::render_home_card;
pub use host::render_host_main;
pub use queries::{list_for_stay, list_recent, ListForStayArgs, LostFoundReportRow};
pub use status::{DEFAULT as STATUS_DEFAULT, WIRE_VALUES as STATUS_WIRE_VALUES};
pub use storage::reset_test_store;

portaki_sdk::portaki_module!(
    id = "lost-found",
    display_name_key = "module.displayName",
    description_key = "module.description",
    author = "Portaki",
);

#[portaki_sdk::capability(required, id = "core.storage")]
pub const STORAGE: &str = "core.storage";
