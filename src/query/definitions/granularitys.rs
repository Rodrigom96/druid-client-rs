use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum Granularity {
    Base(GranularityBase),
    Typed(GranularityTyped),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum GranularityBase {
    All,
    None,
    Second,
    Minute,
    FifteenMinute,
    ThirtyMinute,
    Hour,
    Day,
    Week,
    Month,
    Quarter,
    Year,
    Null,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum GranularityTyped {
    #[serde(rename_all = "camelCase")]
    Duration { duration: usize },
    #[serde(rename_all = "camelCase")]
    Period { period: String, time_zone: String },
}

impl Default for Granularity {
    fn default() -> Self {
        Granularity::Base(GranularityBase::Null)
    }
}

impl Granularity {
    pub fn all() -> Granularity {
        Granularity::Base(GranularityBase::All)
    }
    pub fn node() -> Granularity {
        Granularity::Base(GranularityBase::None)
    }
    pub fn second() -> Granularity {
        Granularity::Base(GranularityBase::Second)
    }
    pub fn minute() -> Granularity {
        Granularity::Base(GranularityBase::Minute)
    }
    pub fn fifteen_minute() -> Granularity {
        Granularity::Base(GranularityBase::FifteenMinute)
    }
    pub fn thirty_minute() -> Granularity {
        Granularity::Base(GranularityBase::ThirtyMinute)
    }
    pub fn hour() -> Granularity {
        Granularity::Base(GranularityBase::Hour)
    }
    pub fn day() -> Granularity {
        Granularity::Base(GranularityBase::Day)
    }
    pub fn week() -> Granularity {
        Granularity::Base(GranularityBase::Week)
    }
    pub fn month() -> Granularity {
        Granularity::Base(GranularityBase::Month)
    }
    pub fn querter() -> Granularity {
        Granularity::Base(GranularityBase::Quarter)
    }
    pub fn year() -> Granularity {
        Granularity::Base(GranularityBase::Year)
    }
    pub fn duration(duration: usize) -> Granularity {
        Granularity::Typed(GranularityTyped::Duration { duration })
    }
    pub fn period(period: &str, time_zone: &str) -> Granularity {
        Granularity::Typed(GranularityTyped::Period {
            period: period.to_string(),
            time_zone: time_zone.to_string(),
        })
    }
}
