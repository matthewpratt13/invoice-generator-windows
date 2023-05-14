use crate::data::hours::Hours;

use chrono::Weekday;
use csv::{ByteRecord, Reader};

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;

pub type DailySchedule = HashMap<
    Weekday,
    (
        Option<Vec<u32>>,
        Vec<u32>,
        Option<Vec<u32>>,
        Option<Vec<u32>>,
    ),
>;

#[allow(dead_code)]
// connect weekday to hours
pub fn from_hours(mut rdr: Reader<File>) -> Result<DailySchedule, Box<dyn Error>> {
    let mut daily_schedule: DailySchedule = HashMap::new();

    let headers = ByteRecord::from(vec![
        "Weekday",
        "Peak Start",
        "Peak End",
        "Off-peak Start",
        "Off-peak End",
        "Secondary Peak Start",
        "Secondary Peak End",
        "Secondary Off-peak Start",
        "Secondary Off-peak End",
    ]);

    for result in rdr.byte_records() {
        let byte_record: ByteRecord = result?;
        let hours: Hours = byte_record.deserialize(Some(&headers))?;
        let weekday: Weekday = hours.weekday_enum()?;

        let times: (
            Option<Vec<u32>>,
            Vec<u32>,
            Option<Vec<u32>>,
            Option<Vec<u32>>,
        ) = (
            hours.peak_hours()?,
            hours.off_peak_hours()?,
            hours.sec_peak_hours()?,
            hours.sec_off_peak_hours()?,
        );

        daily_schedule.insert(weekday, times);
    }

    Ok(daily_schedule)
}
