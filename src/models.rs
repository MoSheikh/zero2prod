use chrono::prelude::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Deserialize, PartialEq, Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::subscriptions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Subscription {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub subscribed_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Insertable, Validate)]
#[diesel(table_name = crate::schema::subscriptions)]
pub struct NewSubscription {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1))]
    pub name: String,
}
