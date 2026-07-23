//! Host dashboard surfaces.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::primitives::{Button, Field, Form, Page, Text, TextArea, TextInput};
use portaki_sdk::sdui::surface::Surface;

use crate::config::{load_config, Localized, SpotRow};

const SPOT_SLOTS: usize = 6;

#[portaki_sdk::surface(host, id = "main")]
pub fn render_host_main(ctx: HostContext) -> Surface {
    let lang = Localized::lang_code(&ctx.locale);
    let config = load_config().unwrap_or_default();
    let spots = config.parse_spots();
    let disclaimer = config.disclaimer.get(&lang).to_string();

    let submit_args = crate::commands::UpdateConfigArgs {
        spots: spots_to_submit(&spots, &lang),
        spots_json: String::new(),
        disclaimer: disclaimer.clone(),
    };
    let save_action = crate::ids::module_id().command(crate::ids::UPDATE_CONFIG, submit_args);

    let mut form_children: Vec<Component> = Vec::new();
    for index in 0..SPOT_SLOTS {
        push_spot_slot(&mut form_children, index, spots.get(index), &lang);
    }
    form_children.push(
        Field::new()
            .name("disclaimer")
            .label("i18n:host.disclaimer.label")
            .child(
                TextArea::new()
                    .name("disclaimer")
                    .value(disclaimer)
                    .placeholder("i18n:host.disclaimer.placeholder"),
            )
            .into(),
    );
    form_children.push(
        Text::new()
            .text("i18n:host.main.help")
            .variant(TextVariant::Caption)
            .into(),
    );
    form_children.push(
        Button::new()
            .label("i18n:host.save")
            .action(save_action)
            .into(),
    );

    Surface::new(
        Page::new()
            .title("i18n:surface.host.main.title")
            .child(
                Text::new()
                    .text("i18n:surface.host.main.subtitle")
                    .variant(TextVariant::Body),
            )
            .child(Form::new().children(form_children)),
    )
    .with_id(crate::ids::HOST_MAIN)
}

fn spots_to_submit(spots: &[SpotRow], lang: &str) -> Vec<crate::commands::SpotInput> {
    spots
        .iter()
        .map(|s| crate::commands::SpotInput {
            name: s.title.get(lang).to_string(),
            category: s.category.clone().unwrap_or_default(),
            distance: s.distance.clone().unwrap_or_default(),
            tag: s.tag.clone().unwrap_or_default(),
            description: s
                .detail
                .as_ref()
                .map(|d| d.get(lang).to_string())
                .unwrap_or_default(),
        })
        .collect()
}

fn push_spot_slot(children: &mut Vec<Component>, index: usize, spot: Option<&SpotRow>, lang: &str) {
    let slot = index + 1;
    let name = spot.map(|s| s.title.get(lang)).unwrap_or("");
    let category = spot.and_then(|s| s.category.as_deref()).unwrap_or("");
    let distance = spot.and_then(|s| s.distance.as_deref()).unwrap_or("");
    let tag = spot.and_then(|s| s.tag.as_deref()).unwrap_or("");
    let description = spot
        .and_then(|s| s.detail.as_ref())
        .map(|d| d.get(lang))
        .unwrap_or("");

    children.push(
        Text::new()
            .text(format!("i18n:host.spot.slot{slot}"))
            .variant(TextVariant::Caption)
            .into(),
    );
    children.push(
        Field::new()
            .name(format!("spots.{index}.name"))
            .label("i18n:host.spot.name")
            .child(
                TextInput::new()
                    .name(format!("spots.{index}.name"))
                    .value(name),
            )
            .into(),
    );
    children.push(
        Field::new()
            .name(format!("spots.{index}.category"))
            .label("i18n:host.spot.category")
            .child(
                TextInput::new()
                    .name(format!("spots.{index}.category"))
                    .value(category),
            )
            .into(),
    );
    children.push(
        Field::new()
            .name(format!("spots.{index}.distance"))
            .label("i18n:host.spot.distance")
            .child(
                TextInput::new()
                    .name(format!("spots.{index}.distance"))
                    .value(distance),
            )
            .into(),
    );
    children.push(
        Field::new()
            .name(format!("spots.{index}.tag"))
            .label("i18n:host.spot.tag")
            .child(
                TextInput::new()
                    .name(format!("spots.{index}.tag"))
                    .value(tag),
            )
            .into(),
    );
    children.push(
        Field::new()
            .name(format!("spots.{index}.description"))
            .label("i18n:host.spot.description")
            .child(
                TextArea::new()
                    .name(format!("spots.{index}.description"))
                    .value(description),
            )
            .into(),
    );
}
