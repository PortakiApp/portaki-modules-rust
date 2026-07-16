//! Portaki rules module — structured bilingual house rules.

mod commands;
mod content;
mod entities;
mod guest;
mod queries;
mod render_host;
mod store;

pub use commands::{save_content, SaveContentArgs};
pub use content::{RuleItem, RulesPayload};
pub use entities::RulesContent;
pub use guest::{render_explore_detail, render_home_card};
pub use queries::{get_content, GetContentArgs, RulesContentView};
pub use render_host::render_host_main;
pub use store::reset_test_store;

portaki_sdk::portaki_module!(
    id = "rules",
    display_name_key = "module.displayName",
    description_key = "module.description",
    author = "Syntax Labs",
);

#[portaki_sdk::capability(required, id = "core.storage")]
pub const STORAGE: &str = "core.storage";
