use chrono::{NaiveDate, NaiveDateTime};
use serde::{self, Deserialize, Deserializer, Serializer};
use bson::DateTime as BsonDateTime;

/// Serializer for `Option<BsonDateTime>` that serializes Some(...) as formatted string,
/// and None as null (or skips if `skip_serializing_if` is used).
pub fn serialize_option_bson_datetime_as_chrono_date<S>(
    opt_dt: &Option<BsonDateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match opt_dt {
        Some(dt) => {
            // Convert dt to chrono::NaiveDateTime and format as string
            let chrono_dt = dt.to_chrono();
            let naive_dt = chrono_dt.naive_utc();
            let formatted = naive_dt.format("%d/%m/%y %H:%M:%S").to_string();
            serializer.serialize_str(&formatted)
        }
        None => serializer.serialize_none(),
    }
}

pub fn serialize_bson_datetime_as_chrono_date<S>(
    dt: &BsonDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Convert bson::DateTime -> chrono::DateTime<Utc>
    let chrono_dt = dt.to_chrono();

    // Convert chrono::DateTime<Utc> to NaiveDateTime (drops timezone)
    let naive_dt: NaiveDateTime = chrono_dt.naive_utc();

    // Format as "dd/MM/yy"
    let formatted = naive_dt.format("%d/%m/%y %H:%M:%S").to_string();

    serializer.serialize_str(&formatted)
}

pub fn deserialize_multiple_formats<'de, D>(
    deserializer: D,
) -> Result<Option<NaiveDate>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<&str> = Option::deserialize(deserializer)?;
    if s.is_none() {
        return Ok(None);
    }
    let s = s.unwrap().trim();

    // List of allowed date formats, ordered by priority
    let formats = [
        "%Y-%m-%d",     // 2025-07-27
        "%m/%d/%Y",     // 07/27/2025
        "%d/%m/%Y",     // 27/07/2025
        "%d/%m/%y",     // 27/07/25
        "%m-%d-%Y",     // 07-27-2025
        "%d-%m-%Y",     // 27-07-2025
        "%Y/%m/%d",     // 2025/07/27
    ];

    for fmt in formats.iter() {
        if let Ok(date) = NaiveDate::parse_from_str(s, fmt) {
            return Ok(Some(date));
        }
    }

    Err(serde::de::Error::custom(format!(
        "invalid date format: '{}', expected one of: {:?}",
        s, formats
    )))
}
