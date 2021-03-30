use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url;

#[derive(Debug, Deserialize, Serialize)]
pub struct WhoAmI {
    authenticated: bool,
    client_id: String,
    user_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WebhookInner {
    account_id: String,
    id: String,
    url: url::Url,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Webhook {
    webhook: WebhookInner,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Webhooks {
    webhooks: Vec<WebhookInner>,
}

/// Monzo gives us `type` and `data` keys in json for webhooks, presumably for expandability.
#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "data")]
#[non_exhaustive]
pub enum WebhookEvent {
    #[serde(rename = "transaction.created")]
    TransactionCreated(TransactionCreated),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransactionCreated {
    account_id: String,
    amount: isize,
    created: DateTime<Utc>,
    currency: String,
    description: String,
    id: String,
    category: String,
    is_load: bool,
    settled: DateTime<Utc>,
    merchant: Merchant,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Address {
    address: String,
    city: String,
    country: String,
    latitude: f64,
    longitude: f64,
    postcode: String,
    region: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Merchant {
    address: Address,
    created: DateTime<Utc>,
    group_id: String,
    id: String,
    logo: String,
    emoji: char,
    name: String,
    category: String,
}

/// From the Monzo Docs
pub mod example_objects {
    pub const TRANSACTION_CREATED: &str = r#"{
        "type": "transaction.created",
        "data": {
            "account_id": "acc_00008gju41AHyfLUzBUk8A",
            "amount": -350,
            "created": "2015-09-04T14:28:40Z",
            "currency": "GBP",
            "description": "Ozone Coffee Roasters",
            "id": "tx_00008zjky19HyFLAzlUk7t",
            "category": "eating_out",
            "is_load": false,
            "settled": "2015-09-05T14:28:40Z",
            "merchant": {
                "address": {
                    "address": "98 Southgate Road",
                    "city": "London",
                    "country": "GB",
                    "latitude": 51.54151,
                    "longitude": -0.08482400000002599,
                    "postcode": "N1 3JD",
                    "region": "Greater London"
                },
                "created": "2015-08-22T12:20:18Z",
                "group_id": "grp_00008zIcpbBOaAr7TTP3sv",
                "id": "merch_00008zIcpbAKe8shBxXUtl",
                "logo": "https://pbs.twimg.com/profile_images/527043602623389696/68_SgUWJ.jpeg",
                "emoji": "🍞",
                "name": "The De Beauvoir Deli Co.",
                "category": "eating_out"
            }
        }
    }"#;

    pub const REGISTER_WEBHOOK: &str = r#"{
        "webhook": {
            "account_id": "account_id",
            "id": "webhook_id",
            "url": "http://example.com"
        }
    }"#;

    pub const LIST_WEBHOOKS: &str = r#"{
        "webhooks": [
            {
                "account_id": "acc_000091yf79yMwNaZHhHGzp",
                "id": "webhook_000091yhhOmrXQaVZ1Irsv",
                "url": "http://example.com/callback"
            },
            {
                "account_id": "acc_000091yf79yMwNaZHhHGzp",
                "id": "webhook_000091yhhzvJSxLYGAceC9",
                "url": "http://example2.com/anothercallback"
            }
        ]
    }"#;
}

#[cfg(test)]
mod tests {
    use super::*;
    use example_objects;
    #[test]
    fn transaction_created() {
        serde_json::from_str::<WebhookEvent>(example_objects::TRANSACTION_CREATED).unwrap();
    }
    #[test]
    fn webhooks() {
        serde_json::from_str::<Webhook>(example_objects::REGISTER_WEBHOOK).unwrap();
        serde_json::from_str::<Webhooks>(example_objects::LIST_WEBHOOKS).unwrap();
    }
}
