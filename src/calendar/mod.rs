use crate::state::CalendarState;
use chrono::{NaiveDate};
use regex::Regex;

pub struct CalendarClient {}

impl CalendarClient {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn events(&self, calendar: &CalendarState) -> Result<Vec<Event>, anyhow::Error> {
        let mut events: Vec<Event> = Vec::new();

        let parens = Regex::new( "\\(.*\\)")?;
        for url in &calendar.urls {
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
                            let summary = parens.replace_all(summary, "");

                            if events.iter().find(|e| {
                                e.summary == summary && e.date == date
                            }).is_none() {
                                events.push(Event {
                                    summary: summary.to_string(),
                                    date,
                                });
                            }
                        }
                    }
                }
            }
        }

        Ok(events)
    }
}

#[derive(Debug, Clone, Hash, PartialEq)]
pub struct Event {
    pub summary: String,
    pub date: NaiveDate,
}
