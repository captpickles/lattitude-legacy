use std::str::from_utf8;
use chrono::{DateTime, Local, NaiveDate};

const URLS: [&str; 2] = [
    //"https://ics.calendarlabs.com/76/201ebb0e/US_Holidays.ics",
    "https://www.thunderbird.net/media/caldata/autogen/United-States-Holidays.ics",
    "https://api.open.fec.gov/v1/calendar-dates/export/?api_key=28Y8q8XFocq8yhKfBzzhUJXjFj2JHCZzIv4P2KIK&per_page=500&calendar_category_id=36",
];

pub struct CalendarClient {}

impl CalendarClient {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn events(&self) -> Result<Vec<Event>, anyhow::Error> {
        let mut events = Vec::new();
        for url in URLS {
            let result = reqwest::Client::new()
                .get(url)
                .send()
                .await?
                .text()
                .await?;

            let bytes = &*result.into_bytes();
            for line in ical::IcalParser::new(bytes) {
                if let Ok(line) = line {
                    for event in line.events {
                        let summary = event.properties.iter().find(|e| e.name == "SUMMARY");
                        let date = event.properties.iter().find(|e| e.name == "DTSTART");
                        match (summary, date) {
                            (Some(summary), Some(date)) => {
                                match (&summary.value, &date.value) {
                                    (Some(summary), Some(date)) => {
                                        let (date, _) = chrono::NaiveDate::parse_and_remainder(date, "%Y%m%d").unwrap();
                                        events.push(
                                            Event {
                                                summary: summary.clone(),
                                                date,
                                            }
                                        );
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
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