use serde::{Deserialize, Serialize};
use serde_json::{from_value, Value};
use time::{
    format_description::well_known::Rfc3339,
    macros::{format_description, time},
    Date, OffsetDateTime, PrimitiveDateTime,
};
use uuid::Uuid;

use super::error::Error;

pub type KeyId = uuid::Uuid;
type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct Key {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub uid: KeyId,
    #[serde(with = "time::serde::rfc3339::option")]
    pub expires_at: Option<OffsetDateTime>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

impl Key {
    pub fn create_from_value(value: Value) -> Result<Self> {
        let name = match value.get("name") {
            None | Some(Value::Null) => None,
            Some(des) => from_value::<String>(des.clone())
                .map(Some)
                .map_err(|_| Error::InvalidApiKeyName(des.clone()))?,
        };

        let description = match value.get("description") {
            None | Some(Value::Null) => None,
            Some(des) => from_value::<String>(des.clone())
                .map(Some)
                .map_err(|_| Error::InvalidApiKeyDescription(des.clone()))?,
        };

        let uid = value.get("uid").map_or_else(
            || Ok(Uuid::new_v4()),
            |uid| from_value(uid.clone()).map_err(|_| Error::InvalidApiKeyUid(uid.clone())),
        )?;
        let expires_at = value
            .get("expiresAt")
            .map(parse_expiration_date)
            .ok_or(Error::MissingParameter("expiresAt"))??;

        let created_at = OffsetDateTime::now_utc();
        let updated_at = created_at;

        Ok(Self {
            name,
            description,
            uid,
            expires_at,
            created_at,
            updated_at,
        })
    }
    
    pub fn update_from_value(&mut self, value: Value) -> Result<()> {
        if let Some(des) = value.get("description") {
            let des =
                from_value(des.clone()).map_err(|_| Error::InvalidApiKeyDescription(des.clone()));
            self.description = des?;
        }

        if let Some(des) = value.get("name") {
            let des = from_value(des.clone()).map_err(|_| Error::InvalidApiKeyName(des.clone()));
            self.name = des?;
        }

        if value.get("uid").is_some() {
            return Err(Error::ImmutableField("uid".to_string()));
        }

        if value.get("expiresAt").is_some() {
            return Err(Error::ImmutableField("expiresAt".to_string()));
        }

        if value.get("createdAt").is_some() {
            return Err(Error::ImmutableField("createdAt".to_string()));
        }

        if value.get("updatedAt").is_some() {
            return Err(Error::ImmutableField("updatedAt".to_string()));
        }

        self.updated_at = OffsetDateTime::now_utc();

        Ok(())
    }
}

fn parse_expiration_date(value: &Value) -> Result<Option<OffsetDateTime>> {
    let e = match value {
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
            .map_err(|_| Error::InvalidApiKeyExpiresAt(value.clone()))
            // check if the key is already expired.
            .and_then(|d| {
                if d > OffsetDateTime::now_utc() {
                    Ok(d)
                } else {
                    Err(Error::InvalidApiKeyExpiresAt(value.clone()))
                }
            })
            .map(Option::Some),
        Value::Null => Ok(None),
        _otherwise => Err(Error::InvalidApiKeyExpiresAt(value.clone())),
    };
    println!("{:?}", e);
    e
}
