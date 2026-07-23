//! Guest home booklet card.

use portaki_sdk::prelude::*;


use portaki_sdk::sdui::primitives::Card;
use portaki_sdk::sdui::surface::Surface;

use super::body::build_weather_glance;
use super::load::GuestWeatherData;

pub fn build_home_card(data: &GuestWeatherData) -> Surface {
    Surface::new(
        Card::new()
            .icon("cloud-sun")
            .title("i18n:nav.weather")
            .action(Action::open_overlay(
                OverlayPresentation::BottomSheet,
                crate::ids::EXPLORE_FORECAST,
                OverlayArgs::new().icon("cloud-sun").title("i18n:nav.weather"),
            ))
            .children(build_weather_glance(
                &data.current,
                &data.forecast,
                &data.units,
                data.city.as_deref(),
            )),
    )
    .with_id(crate::ids::HOME_CARD)
}
