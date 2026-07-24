//! Host dashboard surfaces — config, create-found form, stay-scoped list.
//!
//! Create + status UI lives here (Wasm SDUI). Host shells only embed surfaces.

mod create;
mod main;
mod stay;
mod status_ui;

pub use main::render_host_main;
pub use stay::render_host_stay;

pub(crate) use create::build_create_found_form;
pub(crate) use status_ui::status_choice_options;
