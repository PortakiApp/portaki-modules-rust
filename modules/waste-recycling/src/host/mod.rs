//! Host dashboard surfaces.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::primitives::{Button, Field, Form, Page, Select, Text, TextArea, TextInput};
use portaki_sdk::sdui::surface::Surface;

use crate::config::{color_hex_to_name, load_config, BinRow, Localized};

const BIN_SLOTS: usize = 6;

/// Host configuration page — structured bin slots + collection schedule.
#[portaki_sdk::surface(host, id = "main")]
pub fn render_host_main(ctx: HostContext) -> Surface {
    let lang = Localized::lang_code(&ctx.locale);
    let config = load_config().unwrap_or_default();
    let bins = config.parse_bins();
    let collection_schedule = config.collection_schedule.get(&lang).to_string();

    let submit_args = crate::commands::UpdateConfigArgs {
        bins: bins_to_submit(&bins, &lang),
        bins_json: String::new(),
        collection_schedule: collection_schedule.clone(),
    };
    let save_action = crate::ids::module_id().command(crate::ids::UPDATE_CONFIG, submit_args);

    let mut form_children: Vec<Component> = Vec::new();
    for index in 0..BIN_SLOTS {
        push_bin_slot(&mut form_children, index, bins.get(index), &lang);
    }
    form_children.push(
        Field::new()
            .name("collection_schedule")
            .label("i18n:host.schedule.label")
            .child(
                TextArea::new()
                    .name("collection_schedule")
                    .value(collection_schedule)
                    .placeholder("i18n:host.schedule.placeholder"),
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

fn bins_to_submit(bins: &[BinRow], lang: &str) -> Vec<crate::commands::BinInput> {
    bins.iter()
        .map(|b| crate::commands::BinInput {
            title: b.title.get(lang).to_string(),
            title_fr: String::new(),
            title_en: String::new(),
            items: b
                .items
                .iter()
                .map(|i| i.get(lang))
                .collect::<Vec<_>>()
                .join(
                    "
",
                ),
            items_fr: String::new(),
            color: b.color.clone().unwrap_or_default(),
        })
        .collect()
}

fn push_bin_slot(children: &mut Vec<Component>, index: usize, bin: Option<&BinRow>, lang: &str) {
    let slot = index + 1;
    let title = bin.map(|b| b.title.get(lang)).unwrap_or("");
    let items = bin
        .and_then(|b| b.items.first())
        .map(|item| item.get(lang))
        .unwrap_or("");
    let color = color_hex_to_name(bin.and_then(|b| b.color.as_deref()));

    children.push(
        Text::new()
            .text(format!("i18n:host.bin.slot{slot}"))
            .variant(TextVariant::Caption)
            .into(),
    );
    children.push(
        Field::new()
            .name(format!("bins.{index}.title"))
            .label("i18n:host.bin.title")
            .child(
                TextInput::new()
                    .name(format!("bins.{index}.title"))
                    .value(title),
            )
            .into(),
    );
    children.push(
        Field::new()
            .name(format!("bins.{index}.items"))
            .label("i18n:host.bin.items")
            .child(
                TextInput::new()
                    .name(format!("bins.{index}.items"))
                    .value(items)
                    .placeholder("i18n:host.bin.items.placeholder"),
            )
            .into(),
    );
    children.push(
        Field::new()
            .name(format!("bins.{index}.color"))
            .label("i18n:host.bin.color")
            .child(
                Select::new()
                    .name(format!("bins.{index}.color"))
                    .options(vec![
                        ChoiceOption::new("", "i18n:host.bin.color.none"),
                        ChoiceOption::new("yellow", "i18n:host.bin.color.yellow"),
                        ChoiceOption::new("green", "i18n:host.bin.color.green"),
                        ChoiceOption::new("brown", "i18n:host.bin.color.brown"),
                        ChoiceOption::new("grey", "i18n:host.bin.color.grey"),
                    ])
                    .value(color),
            )
            .into(),
    );
}
