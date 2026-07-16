//! Portaki guest-reviews module — post-stay thank-you and review CTAs.

mod commands;
mod config;
mod guest;
mod queries;
mod render_host;

pub use commands::{submit_review, update_config, SubmitReviewArgs, UpdateConfigArgs};
pub use config::{load_config, ModuleConfig, ReviewChannel};
pub use guest::render_home_card;
pub use queries::get_config;
pub use render_host::render_host_main;

portaki_sdk::portaki_module!(
    id = "guest-reviews",
    display_name_key = "module.displayName",
    description_key = "module.description",
    author = "Syntax Labs",
);

#[portaki_sdk::capability(required, id = "core.storage")]
pub const STORAGE: &str = "core.storage";
