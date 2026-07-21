//! Portaki checklist module — checkout tasks with stay-scoped completions.

mod commands;
mod entities;
mod guest;
mod labels;
mod queries;
mod render_host;
mod storage;

pub use commands::{
    complete_item, replace_items, uncomplete_item, ChecklistItemInput, ItemIdArgs, ReplaceItemsArgs,
};
pub use entities::{ChecklistCompletion, ChecklistItem};
pub use guest::render_home_card;
pub use queries::{list_completions, list_items, ChecklistItemDto};
pub use render_host::render_host_main;
pub use storage::reset_test_store;

portaki_sdk::portaki_module!(
    id = "checklist",
    display_name_key = "module.displayName",
    description_key = "module.description",
    author = "Syntax Labs",
);

#[portaki_sdk::capability(required, id = "core.storage")]
pub const STORAGE: &str = "core.storage";
