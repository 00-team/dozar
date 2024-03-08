// use std::borrow::Cow;
// use sqlx::{encode::IsNull, sqlite::{SqliteArgumentValue, SqliteTypeInfo}, Sqlite};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub name: Option<String>,
    pub phone: String,
    pub wallet: i64,
    pub in_hold: i64,
    pub token: String,
    pub photo: Option<String>,
    pub admin: bool,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
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

// Clone, PartialEq, Eq,
// #[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
// pub struct GG<T>(T);

// impl<'q, T> sqlx::Encode<'q, Sqlite> for Vec<String> {
//     fn encode_by_ref(
//         &self,
//         buf: &mut <Sqlite as sqlx::database::HasArguments<'q>>::ArgumentBuffer,
//     ) -> IsNull {
//         let result = serde_json::to_string(self).unwrap_or("{}".to_string());
//         buf.push(SqliteArgumentValue::Text(Cow::Owned(result)));
//
//         IsNull::No
//     }
// }

// impl<T> sqlx::Type<Sqlite> for GG<T> {
//     fn type_info() -> SqliteTypeInfo {
//         <&str as sqlx::Type<Sqlite>>::type_info()
//     }
// }

// impl<T> From<String> for GG<T> {
//     fn from(value: String) -> Self {
//         let result: Vec<String> = serde_json::from_str(&value).unwrap();
//         result
//     }
// }
