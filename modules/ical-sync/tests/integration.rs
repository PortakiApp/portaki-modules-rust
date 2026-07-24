//! Integration-style unit tests with `portaki-test-utils`.

use ical_sync::{
    apply_feeds, get_config, list_sources, parse_stay_rows, update_config, ApplyFeedsArgs,
    CalendarInput, FeedBody, UpdateConfigArgs,
};
use portaki_sdk::capability;
use portaki_test_utils::MockContext;
use serial_test::serial;

#[test]
#[serial]
fn update_config_and_list_sources_many_calendars() {
    MockContext::host()
        .with_capabilities(&[capability::core::STORAGE, capability::core::ICAL_IMPORT])
        .run(|ctx| {
            update_config(
                ctx.clone(),
                UpdateConfigArgs {
                    calendars: vec![
                        CalendarInput {
                            id: "airbnb".into(),
                            url: "https://www.airbnb.com/calendar/ical/1.ics".into(),
                            label: "Airbnb".into(),
                        },
                        CalendarInput {
                            id: "".into(),
                            url: "  ".into(),
                            label: "".into(),
                        },
                        CalendarInput {
                            id: "booking".into(),
                            url: "https://admin.booking.com/hotel/hoteladmin/ical.html?t=abc"
                                .into(),
                            label: "Booking".into(),
                        },
                        CalendarInput {
                            id: "vrbo".into(),
                            url: "https://www.vrbo.com/calendar/ical/9.ics".into(),
                            label: "".into(),
                        },
                    ],
                    ..Default::default()
                },
            )
            .expect("update");

            let sources = list_sources(ctx.clone()).expect("sources");
            assert_eq!(sources.sources.len(), 3);
            assert_eq!(sources.sources[0].id, "airbnb");
            assert_eq!(sources.sources[0].provider.as_deref(), Some("airbnb"));
            assert_eq!(sources.sources[1].provider.as_deref(), Some("booking"));
            assert_eq!(sources.sources[2].provider.as_deref(), Some("vrbo"));

            let config = get_config(ctx).expect("config");
            assert_eq!(config.calendars.len(), 3);
            assert!(config.ical_url_primary.contains("airbnb.com"));
        });
}

#[test]
#[serial]
fn legacy_primary_secondary_still_accepted() {
    MockContext::host()
        .with_capabilities(&[capability::core::STORAGE, capability::core::ICAL_IMPORT])
        .run(|ctx| {
            update_config(
                ctx.clone(),
                UpdateConfigArgs {
                    ical_url_primary: "https://example.com/a.ics".into(),
                    ical_url_secondary: "https://example.com/b.ics".into(),
                    ..Default::default()
                },
            )
            .expect("update");

            let sources = list_sources(ctx).expect("sources");
            assert_eq!(sources.sources.len(), 2);
        });
}

#[test]
#[serial]
fn apply_feeds_parses_ics_and_updates_summary() {
    MockContext::host()
        .with_capabilities(&[capability::core::STORAGE, capability::core::ICAL_IMPORT])
        .run(|ctx| {
            update_config(
                ctx.clone(),
                UpdateConfigArgs {
                    calendars: vec![CalendarInput {
                        id: "primary".into(),
                        url: "https://example.com/a.ics".into(),
                        label: "".into(),
                    }],
                    ..Default::default()
                },
            )
            .expect("update");

            let ics = "BEGIN:VCALENDAR\nBEGIN:VEVENT\nUID:u1\n\
DTSTART;VALUE=DATE:20260801\nDTEND;VALUE=DATE:20260805\n\
SUMMARY:Sofia Rossi\nEND:VEVENT\nEND:VCALENDAR\n";

            let result = apply_feeds(
                ctx,
                ApplyFeedsArgs {
                    guest_lang: "fr".into(),
                    feeds: vec![FeedBody {
                        id: "primary".into(),
                        provider: Some("airbnb".into()),
                        ics_body: ics.into(),
                    }],
                },
            )
            .expect("apply");

            assert!(result.ok);
            assert_eq!(result.rows.len(), 1);
            assert_eq!(result.rows[0].guest_name, "Sofia Rossi");
            assert_eq!(result.rows[0].ical_uid, "u1");
            assert!(result.updated_plain_config.last_sync_at.is_some());
            assert!(result
                .updated_plain_config
                .sync_summary
                .as_deref()
                .unwrap_or("")
                .contains("1 stay"));
        });
}

#[test]
fn parse_stay_rows_unit() {
    let rows = parse_stay_rows(
        "BEGIN:VEVENT\nUID:x\nDTSTART;VALUE=DATE:20260101\nDTEND;VALUE=DATE:20260103\nSUMMARY:A\nEND:VEVENT\n",
        "en",
        10,
    );
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].guest_name, "A");
}
