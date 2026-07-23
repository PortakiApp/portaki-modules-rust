//! Guest home booklet card — mixed-destination departure board glance.

use portaki_sdk::prelude::*;
use portaki_sdk::sdui::common::Emphasis;
use portaki_sdk::sdui::primitives::{Card, Text, TimedEntry};
use portaki_sdk::sdui::surface::Surface;

use crate::content::{home_board, station_caption, MODULE_ICON};

pub fn build_home_card(ctx: &GuestContext) -> Surface {
    let mut children: Vec<Component> = vec![Component::Text(
        Text::new()
            .text(station_caption())
            .variant(TextVariant::Caption)
            .emphasis(Emphasis::Subtle),
    )];
    children.extend(home_board().into_iter().map(board_entry_component));

    Surface::new(
        Card::new()
            .icon(MODULE_ICON)
            .title("i18n:home.card.title")
            .action(Action::open_overlay(
                OverlayPresentation::Fullscreen,
                crate::ids::EXPLORE_DETAIL,
                OverlayArgs::new()
                    .icon(MODULE_ICON)
                    .title("i18n:home.card.title"),
            ))
            .children(children),
    )
    .with_id(crate::ids::HOME_CARD)
}

fn board_entry_component(entry: crate::content::BoardEntry) -> Component {
    Component::TimedEntry(
        TimedEntry::new()
            .time(entry.time)
            .title(entry.destination)
            .subtitle(entry.platform),
    )
}
