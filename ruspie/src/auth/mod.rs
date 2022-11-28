use hmac::{Hmac, Mac};
use serde_json::Value;
use sha2::Sha256;
use time::{OffsetDateTime, PrimitiveDateTime, Date, format_description::well_known::Rfc3339};
use time::macros::{format_description, time};
use uuid::Uuid;
use uuid::fmt::Hyphenated;

use self::error::AuthError;

pub type Result<T> = std::result::Result<T, AuthError>;

pub mod error;
pub mod keys;
pub mod controller;
mod store;

fn parse_expiration_date(value: &Value) -> Result<Option<OffsetDateTime>> {
    match value {
        Value::String(string) => OffsetDateTime::parse(string, &Rfc3339)
            .or_else(|_| {
                PrimitiveDateTime::parse(
                    string,
                    format_description!(
                        "[year repr:full base:calendar]-[month repr:numerical]-[day]T[hour]:[minute]:[second]"
                    ),
                ).map(|datetime| datetime.assume_utc())
            })
            .or_else(|_| {
                PrimitiveDateTime::parse(
                    string,
                    format_description!(
                        "[year repr:full base:calendar]-[month repr:numerical]-[day] [hour]:[minute]:[second]"
                    ),
                ).map(|datetime| datetime.assume_utc())
            })
            .or_else(|_| {
                    Date::parse(string, format_description!(
                        "[year repr:full base:calendar]-[month repr:numerical]-[day]"
                    )).map(|date| PrimitiveDateTime::new(date, time!(00:00)).assume_utc())
            })
            .map_err(|_| AuthError::invalid_api_key_expires_at(value.clone()))
            // check if the key is already expired.
            .and_then(|d| {
                if d > OffsetDateTime::now_utc() {
                    Ok(d)
                } else {
                    Err(AuthError::invalid_api_key_expires_at(value.clone()))
                }
            })
            .map(Option::Some),
        Value::Null => Ok(None),
        _otherwise => Err(AuthError::invalid_api_key_expires_at(value.clone())),
    }
}

/// Divides one slice into an array and the tail at an index,
/// returns `None` if `N` is out of bounds.
pub fn try_split_array_at<T, const N: usize>(slice: &[T]) -> Option<(&[T; N], &[T])>
where
    [T; N]: for<'a> TryFrom<&'a [T]>,
{
    let (head, tail) = try_split_at(slice, N)?;
    let head = head.try_into().ok()?;
    Some((head, tail))
}

// Divides one slice into two at an index, returns `None` if mid is out of bounds.
pub fn try_split_at<T>(slice: &[T], mid: usize) -> Option<(&[T], &[T])> {
    if mid <= slice.len() {
        Some(slice.split_at(mid))
    } else {
        None
    }
}

pub fn generate_key_as_hexa(uid: Uuid, master_key: &[u8]) -> String {
    // format uid as hyphenated allowing user to generate their own keys.
    let mut uid_buffer = [0; Hyphenated::LENGTH];
    let uid = uid.hyphenated().encode_lower(&mut uid_buffer);

    // new_from_slice function never fail.
    let mut mac = Hmac::<Sha256>::new_from_slice(master_key).unwrap();
    mac.update(uid.as_bytes());

    let result = mac.finalize();
    format!("{:x}", result.into_bytes())
}