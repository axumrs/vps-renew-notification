use crate::{Error, Result};

pub fn chrono_from_str(s: &str) -> Result<chrono::DateTime<chrono::Local>> {
    let dt = chrono::DateTime::parse_from_rfc3339(s).map_err(Error::from)?;
    Ok(dt.into())
}

// pub fn chrono_date_from_str(s: &str) -> Result<chrono::DateTime<chrono::Local>> {
//     let dt = chrono_from_str(s)?;
//     dt.
// }
// pub fn chrono_time_from_str(s: &str) -> Result<chrono::DateTime<chrono::Local>> {
//     chrono_from_str(s, "%H:%M:%S")
// }
// pub fn chrono_datetime_from_str(s: &str) -> Result<chrono::DateTime<chrono::Local>> {
//     chrono_from_str(s, "%H:%M:%S")
// }
