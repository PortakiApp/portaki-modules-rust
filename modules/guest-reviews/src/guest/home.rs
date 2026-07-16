//! Guest home card — inline post-stay review (no overlay).

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::action::Action;
use portaki_sdk::sdui::primitives::{
    Button, Card, Field, Form, QRCode, Select, Stack, Text, TextArea,
};
use portaki_sdk::sdui::surface::Surface;
use serde_json::json;

use super::load::GuestData;

pub fn build_home_card(data: &GuestData) -> Surface {
    let mut children = Vec::new();

    let prompt = if data.locale.to_ascii_lowercase().starts_with("en") {
        format!("How was your stay at {}?", data.property_name)
    } else {
        format!(
            "Comment s'est passé votre séjour à {} ?",
            data.property_name
        )
    };
    children.push(Component::Text(
        Text::new().text(json!(prompt)).variant(json!("title")),
    ));

    if !data.thank_you.is_empty() {
        children.push(Component::Text(
            Text::new()
                .text(json!(data.thank_you.clone()))
                .variant(json!("body")),
        ));
    } else {
        children.push(Component::Text(
            Text::new()
                .text(json!("i18n:guest.defaultThanks"))
                .variant(json!("body")),
        ));
    }

    let show_airbnb = data.channel.show_airbnb() && data.airbnb_url.is_some();
    let show_portaki = data.channel.show_portaki();

    if show_airbnb {
        let url = data.airbnb_url.clone().unwrap_or_default();
        let action =
            serde_json::to_value(Action::External { url: url.clone() }).unwrap_or(json!({}));
        children.push(Component::Button(
            Button::new()
                .label(json!("i18n:guest.airbnbCta"))
                .action(action),
        ));
        if data.show_qr && !url.is_empty() {
            children.push(Component::QRCode(
                QRCode::new().value(json!(url)).size(json!(144)),
            ));
            children.push(Component::Text(
                Text::new()
                    .text(json!("i18n:guest.scanQr"))
                    .variant(json!("caption")),
            ));
        }
    }

    if show_portaki {
        if show_airbnb {
            children.push(Component::Text(
                Text::new()
                    .text(json!("i18n:guest.orPortaki"))
                    .variant(json!("caption")),
            ));
        }

        let submit_action = serde_json::to_value(Action::command(
            "guest-reviews",
            "submitReview",
            json!({ "rating": 5, "comment": "" }),
        ))
        .unwrap_or(json!({}));

        children.push(Component::Form(
            Form::new()
                .child(
                    Field::new()
                        .name(json!("rating"))
                        .label(json!("i18n:guest.rating"))
                        .child(
                            Select::new()
                                .name(json!("rating"))
                                .options(json!([
                                    {"value": "1", "label": "★"},
                                    {"value": "2", "label": "★★"},
                                    {"value": "3", "label": "★★★"},
                                    {"value": "4", "label": "★★★★"},
                                    {"value": "5", "label": "★★★★★"}
                                ]))
                                .value(json!("5")),
                        ),
                )
                .child(
                    Field::new()
                        .name(json!("comment"))
                        .label(json!("i18n:guest.comment"))
                        .child(
                            TextArea::new()
                                .name(json!("comment"))
                                .placeholder(json!("i18n:guest.commentPlaceholder")),
                        ),
                )
                .child(
                    Button::new()
                        .label(json!("i18n:guest.submit"))
                        .action(submit_action),
                ),
        ));
    }

    Surface::new(
        Card::new()
            .icon(json!("star"))
            .title(json!("i18n:home.card.title"))
            .child(Stack::new().gap(json!(12)).children(children)),
    )
    .with_id("home.card")
}
