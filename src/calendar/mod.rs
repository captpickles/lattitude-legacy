use crate::state::state;
use chrono::{NaiveDate};

pub struct CalendarClient {}

impl CalendarClient {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn events(&self) -> Result<Vec<Event>, anyhow::Error> {
        let state = state().calendar;
        let mut events = Vec::new();
        for url in state.urls {
            let result = reqwest::Client::new().get(url).send().await?.text().await?;

            let bytes = &*result.into_bytes();
            for line in ical::IcalParser::new(bytes).flatten() {
                for event in line.events {
                    let summary = event.properties.iter().find(|e| e.name == "SUMMARY");
                    let date = event.properties.iter().find(|e| e.name == "DTSTART");
                    if let (Some(summary), Some(date)) = (summary, date) {
                        if let (Some(summary), Some(date)) = (&summary.value, &date.value) {
                            let (date, _) =
                                chrono::NaiveDate::parse_and_remainder(date, "%Y%m%d")
                                    .unwrap();
                            events.push(Event {
                                summary: summary.clone(),
                                date,
                            });
                        }
                    }
                }
            }
        }

        Ok(events)
    }
}

#[derive(Debug, Clone)]
pub struct Event {
    pub summary: String,
    pub date: NaiveDate,
}
