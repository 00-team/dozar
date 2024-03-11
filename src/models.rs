// use std::borrow::Cow;
// use sqlx::{encode::IsNull, sqlite::{SqliteArgumentValue, SqliteTypeInfo}, Sqlite};

use std::{future::Future, ops, pin::Pin};

use actix_web::{dev::Payload, error, web::Data, FromRequest, HttpRequest};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sha2::Digest;
use sqlx::{
    encode::IsNull,
    sqlite::{SqliteArgumentValue, SqliteTypeInfo},
    Sqlite,
};
use utoipa::ToSchema;

use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
pub struct Address {
    pub latitude: f32,
    pub longitude: f32,
    pub country: String,
    pub state: String,
    pub city: String,
    pub postal: String,
    pub detail: String
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema, Default)]
pub struct User {
    pub id: i64,
    pub name: Option<String>,
    pub phone: String,
    pub wallet: i64,
    pub in_hold: i64,
    pub token: String,
    pub photo: Option<String>,
    pub admin: bool,
    #[schema(value_type = Address)]
    pub addr: JsonStr<Address>,
}

fn parse_token(token: &str) -> Option<(i64, String)> {
    let mut token = token.splitn(2, ':');
    let id = match token.next() {
        Some(s) => match s.parse::<i64>() {
            Ok(v) => v,
            Err(_) => return None,
        },
        None => return None,
    };

    let token = match token.next() {
        Some(s) => s.to_string(),
        None => return None,
    };

    Some((id, token))
}

impl FromRequest for User {
    type Error = error::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        let state = req.app_data::<Data<AppState>>().unwrap();
        let pool = state.sql.clone();
        let token = BearerAuth::from_request(req, pl);

        Box::pin(async move {
            let (id, token) = match token.await {
                Ok(t) => match parse_token(t.token()) {
                    Some(t) => t,
                    None => return Err(error::ErrorForbidden("invalid token")),
                },
                Err(e) => return Err(e.into()),
            };

            let token = hex::encode(sha2::Sha512::digest(&token));

            let result = sqlx::query_as! {
                User,
                "select * from users where id = ? and token = ?",
                id, token
            }
            .fetch_one(&pool)
            .await;

            match result {
                Ok(mut user) => {
                    user.token = user.token[..32].to_string();
                    Ok(user)
                }
                Err(_) => Err(error::ErrorForbidden("not found")),
            }
        })
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Transaction {
    pub id: i64,
    pub user: i64,
    pub kind: i64,   // in OR out | withdrawl OR deposit
    pub status: i64, // success | failed | in progress
    pub amount: i64,
    pub idpay_id: Option<String>,
    pub idpay_track_id: Option<i64>,
    pub card_no: Option<String>,
    pub hashed_card_no: Option<String>,
    pub date: Option<i64>,
    pub bank_track_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Product {
    pub id: i64,
    pub title: String,
    pub detail: Option<String>,
    pub end: i64,
    pub start: i64,
    pub base_price: i64,
    pub photos: Vec<String>,
    pub buy_now_opens: Option<i64>,
    pub buy_now_price: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Offer {
    pub id: i64,
    pub product: i64,
    pub user: Option<i64>,
    pub amount: i64,
    pub timestamp: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, PartialEq, Eq)]
pub enum Action {
    Login,
    Delete,
}

impl From<String> for Action {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "login" => Action::Login,
            "delete" => Action::Delete,
            _ => Action::Login,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Verification {
    pub id: i64,
    pub action: Action,
    pub phone: String,
    pub code: String,
    pub expires: i64,
    pub tries: i64,
}

#[derive(Debug, Deserialize, Default)]
pub struct JsonStr<T>(pub T);

// impl<T> JsonStr<T> {
//     pub fn into_inner(self) -> T {
//         self.0
//     }
// }

impl<T> ops::Deref for JsonStr<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> ops::DerefMut for JsonStr<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: Serialize> Serialize for JsonStr<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'q, T> sqlx::Encode<'q, Sqlite> for JsonStr<T>
where
    T: Serialize,
{
    fn encode_by_ref(
        &self,
        buf: &mut <Sqlite as sqlx::database::HasArguments<'q>>::ArgumentBuffer,
    ) -> IsNull {
        let result = serde_json::to_string(&self.0).unwrap_or("{}".to_string());
        buf.push(SqliteArgumentValue::Text(result.into()));

        IsNull::No
    }
}

impl<T> sqlx::Type<Sqlite> for JsonStr<T> {
    fn type_info() -> SqliteTypeInfo {
        <&str as sqlx::Type<Sqlite>>::type_info()
    }
}

impl<T> From<String> for JsonStr<T>
where
    T: DeserializeOwned + Default,
{
    fn from(value: String) -> Self {
        Self(serde_json::from_str::<T>(&value).unwrap_or(T::default()))
    }
}
