//! Integration-style unit tests with `portaki-test-utils`.

use serial_test::serial;

use portaki_sdk::sdui::component::Component;
use portaki_sdk::sdui::surface::Surface;
use portaki_test_utils::{MockContext, Property};
use pre_arrival_form::{
    get_status, render_home_card, render_host_main, reset_test_store, submit, SubmitArgs,
};

fn contains_component_type(surface: &Surface, type_name: &str) -> bool {
    fn walk(node: &Component, type_name: &str) -> bool {
        let matches = match node {
            Component::Card(_) if type_name == "Card" => true,
            Component::Text(_) if type_name == "Text" => true,
            Component::EmptyState(_) if type_name == "EmptyState" => true,
            Component::Form(_) if type_name == "Form" => true,
            Component::Page(_) if type_name == "Page" => true,
            Component::Button(_) if type_name == "Button" => true,
            Component::TimePicker(_) if type_name == "TimePicker" => true,
            Component::TextArea(_) if type_name == "TextArea" => true,
            _ => false,
        };
        if matches {
            return true;
        }
        for child in child_components(node) {
            if walk(child, type_name) {
                return true;
            }
        }
        false
    }
    walk(&surface.root, type_name)
}

fn child_components(node: &Component) -> Vec<&Component> {
    match node {
        Component::Stack(inner) => inner.children.iter().collect(),
        Component::Card(inner) => inner.children.iter().collect(),
        Component::EmptyState(inner) => inner.children.iter().collect(),
        Component::Group(inner) => inner.children.iter().collect(),
        Component::Form(inner) => inner.children.iter().collect(),
        Component::Page(inner) => inner.children.iter().collect(),
        Component::Field(inner) => inner.children.iter().collect(),
        _ => Vec::new(),
    }
}

#[test]
#[serial]
fn home_card_renders_form_when_incomplete() {
    reset_test_store();
    MockContext::guest()
        .with_property(Property::default())
        .run(|ctx| {
            let surface = render_home_card(ctx);
            assert!(contains_component_type(&surface, "Card"));
            assert!(contains_component_type(&surface, "Form"));
            assert!(contains_component_type(&surface, "TimePicker"));
            assert!(contains_component_type(&surface, "TextArea"));
            assert!(contains_component_type(&surface, "Button"));
            let json = serde_json::to_string(&surface).expect("surface json");
            assert!(json.contains("home.card.intro"));
            assert!(json.contains("submit"));
        });
}

#[test]
#[serial]
fn submit_then_status_and_thanks_card() {
    reset_test_store();
    MockContext::guest()
        .with_property(Property::default())
        .run(|ctx| {
            let before = get_status(ctx.clone()).expect("status");
            assert!(!before.completed);

            submit(
                ctx.clone(),
                SubmitArgs {
                    arrival_time_estimated: Some("17:30".into()),
                    guest_occasion: Some("Anniversaire".into()),
                    guest_allergies: None,
                    message_to_host: Some("Merci !".into()),
                },
            )
            .expect("submit");

            let after = get_status(ctx.clone()).expect("status after");
            assert!(after.completed);
            assert_eq!(
                after.arrival_time_estimated.as_deref(),
                Some("17:30")
            );
            assert_eq!(after.guest_occasion.as_deref(), Some("Anniversaire"));

            let surface = render_home_card(ctx);
            let json = serde_json::to_string(&surface).expect("surface json");
            assert!(json.contains("home.card.thanks"));
            assert!(!json.contains("TimePicker"));
        });
}

#[test]
#[serial]
fn host_main_renders_page() {
    reset_test_store();
    MockContext::host()
        .with_property(Property::default())
        .run(|ctx| {
            let surface = render_host_main(ctx);
            assert!(contains_component_type(&surface, "Page"));
            assert!(contains_component_type(&surface, "Text"));
        });
}
