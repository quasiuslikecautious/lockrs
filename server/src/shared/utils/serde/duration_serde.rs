use chrono::Duration;
use serde::{Deserializer, Serializer};
use serde::de::{self, Visitor};

pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer 
{
    serializer.serialize_i64(duration.num_seconds())
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>
{
    struct DurationVisitor;

    impl<'de> Visitor<'de> for DurationVisitor {
        type Value = Duration;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a duration in seconds")
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E> 
        where
            E: de::Error,
        {
            Ok(Duration::seconds(value))
        }
    }

    Ok(deserializer.deserialize_i64(DurationVisitor)?)
}

