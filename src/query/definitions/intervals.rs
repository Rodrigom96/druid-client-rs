use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug)]
pub struct Interval {
    pub from: chrono::NaiveDateTime,
    pub to: chrono::NaiveDateTime,
}

impl Serialize for Interval {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let from_str = serde_json::to_string(&self.from)
            .map_err(serde::ser::Error::custom)?
            .replace('\"', "");
        let to_str = serde_json::to_string(&self.to)
            .map_err(serde::ser::Error::custom)?
            .replace('\"', "");

        serializer.collect_str(&format!("{from_str}/{to_str}"))
    }
}

impl<'de> serde::de::Deserialize<'de> for Interval {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        let split: Vec<&str> = s.split('/').collect();
        if !split.len() == 2 {
            return Err(serde::de::Error::custom("Time must be separted by /"));
        }

        Ok(Interval {
            from: serde_json::from_str(&format!("\"{}\"", &split[0]))
                .map_err(serde::de::Error::custom)?,
            to: serde_json::from_str(&format!("\"{}\"", &split[1]))
                .map_err(serde::de::Error::custom)?,
        })
    }
}
