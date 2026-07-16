//! Integration-style unit tests with `portaki-test-utils`.

use serial_test::serial;
use uuid::Uuid;

use portaki_sdk::sdui::component::Component;
use portaki_sdk::sdui::surface::Surface;
use portaki_test_utils::{MockContext, Property};

use sections::{
    delete_section, list_sections, render_explore_sheet, render_home_card, reorder, reset_test_store,
    save_section, DeleteSectionArgs, ListSectionsArgs, ReorderArgs, SaveSectionArgs,
    SectionLocaleInput,
};

fn contains_component_type(surface: &Surface, type_name: &str) -> bool {
    fn walk(node: &Component, type_name: &str) -> bool {
        let matches = match node {
            Component::Card(_) if type_name == "Card" => true,
            Component::Markdown(_) if type_name == "Markdown" => true,
            Component::EmptyState(_) if type_name == "EmptyState" => true,
            Component::Stack(_) if type_name == "Stack" => true,
            Component::Text(_) if type_name == "Text" => true,
            Component::Divider(_) if type_name == "Divider" => true,
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
        Component::Form(inner) => inner.children.iter().collect(),
        Component::Page(inner) => inner.children.iter().collect(),
        Component::Group(inner) => inner.children.iter().collect(),
        _ => Vec::new(),
    }
}

fn seed_two_sections(ctx: portaki_sdk::Context) -> (Uuid, Uuid) {
    let first = save_section(
        ctx.clone(),
        SaveSectionArgs {
            id: None,
            sort_order: Some(0),
            locales: vec![
                SectionLocaleInput {
                    lang: "fr".into(),
                    title: "Bienvenue".into(),
                    body_markdown: "Bienvenue à L'Islette !".into(),
                },
                SectionLocaleInput {
                    lang: "en".into(),
                    title: "Welcome".into(),
                    body_markdown: "Welcome to L'Islette!".into(),
                },
            ],
            title_fr: String::new(),
            title_en: String::new(),
            body_markdown_fr: String::new(),
            body_markdown_en: String::new(),
        },
    )
    .expect("save first");
    let second = save_section(
        ctx,
        SaveSectionArgs {
            id: None,
            sort_order: Some(1),
            locales: vec![SectionLocaleInput {
                lang: "fr".into(),
                title: "L'appartement".into(),
                body_markdown: "2 chambres, terrasse vue mer.".into(),
            }],
            title_fr: String::new(),
            title_en: String::new(),
            body_markdown_fr: String::new(),
            body_markdown_en: String::new(),
        },
    )
    .expect("save second");
    (first.id, second.id)
}

#[test]
#[serial]
fn home_card_empty_without_content() {
    reset_test_store();
    MockContext::guest()
        .with_property(Property::default())
        .with_capabilities(&["core.storage"])
        .run(|ctx| {
            let surface = render_home_card(ctx);
            assert!(contains_component_type(&surface, "EmptyState"));
        });
}

#[test]
#[serial]
fn home_card_and_sheet_with_sections() {
    reset_test_store();
    MockContext::guest()
        .with_property(Property::default())
        .with_capabilities(&["core.storage"])
        .run(|ctx| {
            seed_two_sections(ctx.clone());
            let card = render_home_card(ctx.clone());
            assert!(contains_component_type(&card, "Card"));
            assert!(contains_component_type(&card, "Markdown"));
            let sheet = render_explore_sheet(ctx);
            assert!(contains_component_type(&sheet, "Markdown"));
            let json = serde_json::to_string(&sheet).expect("json");
            assert!(json.contains("L'appartement"));
        });
}

#[test]
#[serial]
fn list_reorder_delete() {
    reset_test_store();
    MockContext::guest()
        .with_property(Property::default())
        .with_capabilities(&["core.storage"])
        .run(|ctx| {
            let (a, b) = seed_two_sections(ctx.clone());
            let listed = list_sections(
                ctx.clone(),
                ListSectionsArgs {
                    locale: Some("fr-FR".into()),
                },
            )
            .expect("list");
            assert_eq!(listed.len(), 2);
            assert_eq!(listed[0].id, a);

            reorder(
                ctx.clone(),
                ReorderArgs {
                    ordered_ids: vec![b, a],
                },
            )
            .expect("reorder");
            let listed = list_sections(
                ctx.clone(),
                ListSectionsArgs {
                    locale: Some("fr-FR".into()),
                },
            )
            .expect("list after reorder");
            assert_eq!(listed[0].id, b);

            delete_section(ctx.clone(), DeleteSectionArgs { id: b }).expect("delete");
            let listed = list_sections(
                ctx,
                ListSectionsArgs {
                    locale: Some("fr-FR".into()),
                },
            )
            .expect("list after delete");
            assert_eq!(listed.len(), 1);
            assert_eq!(listed[0].id, a);
        });
}
