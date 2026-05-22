//! Platform event subscriptions.

use portaki_sdk::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::queries::{get_current, get_forecast, GetCurrentArgs, GetForecastArgs};
use crate::weather::has_open_weather;

/// Payload for `core.booking.confirmed`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookingConfirmedEvent {
    /// Booking identifier.
    pub id: Uuid,
    /// Property id (informational).
    pub property_id: Uuid,
}

#[portaki_sdk::event_handler(event_type = "core.booking.confirmed")]
pub fn on_booking_confirmed(ctx: Context, _event: BookingConfirmedEvent) -> Result<()> {
    if !has_open_weather(&ctx) {
        return Ok(());
    }

    let lat = ctx.property.lat;
    let lng = ctx.property.lng;
    let _ = get_current(
        ctx.clone(),
        GetCurrentArgs {
            lat: Some(lat),
            lng: Some(lng),
        },
    )?;
    let _ = get_forecast(
        ctx,
        GetForecastArgs {
            lat: Some(lat),
            lng: Some(lng),
            days: Some(5),
        },
    )?;
    Ok(())
}
