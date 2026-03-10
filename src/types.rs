#[doc = r" Error types."]
use std::collections::HashMap;

pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(std::borrow::Cow<'static, str>);
    impl std::error::Error for ConversionError {}
    impl std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}

macro_rules! validated_string {
    // min + max length validation
    ($name:ident, min=$min:expr, max=$max:expr) => {
        #[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[serde(transparent)]
        pub struct $name(String);
        impl std::ops::Deref for $name {
            type Target = String;
            fn deref(&self) -> &String { &self.0 }
        }
        impl From<$name> for String {
            fn from(value: $name) -> Self { value.0 }
        }
        impl std::str::FromStr for $name {
            type Err = self::error::ConversionError;
            fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
                if value.chars().count() > $max {
                    return Err(concat!("longer than ", stringify!($max), " characters").into());
                }
                if value.chars().count() < $min {
                    return Err(concat!("shorter than ", stringify!($min), " characters").into());
                }
                Ok(Self(value.to_string()))
            }
        }
        validated_string!(@try_from $name);
        validated_string!(@deserialize $name);
    };
    // min-only validation
    ($name:ident, min=$min:expr) => {
        #[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[serde(transparent)]
        pub struct $name(String);
        impl std::ops::Deref for $name {
            type Target = String;
            fn deref(&self) -> &String { &self.0 }
        }
        impl From<$name> for String {
            fn from(value: $name) -> Self { value.0 }
        }
        impl std::str::FromStr for $name {
            type Err = self::error::ConversionError;
            fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
                if value.chars().count() < $min {
                    return Err(concat!("shorter than ", stringify!($min), " characters").into());
                }
                Ok(Self(value.to_string()))
            }
        }
        validated_string!(@try_from $name);
        validated_string!(@deserialize $name);
    };
    // max-only validation
    ($name:ident, max=$max:expr) => {
        #[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[serde(transparent)]
        pub struct $name(String);
        impl std::ops::Deref for $name {
            type Target = String;
            fn deref(&self) -> &String { &self.0 }
        }
        impl From<$name> for String {
            fn from(value: $name) -> Self { value.0 }
        }
        impl std::str::FromStr for $name {
            type Err = self::error::ConversionError;
            fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
                if value.chars().count() > $max {
                    return Err(concat!("longer than ", stringify!($max), " characters").into());
                }
                Ok(Self(value.to_string()))
            }
        }
        validated_string!(@try_from $name);
        validated_string!(@deserialize $name);
    };
    // UUID validation
    ($name:ident, uuid) => {
        #[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[serde(transparent)]
        pub struct $name(String);
        impl std::ops::Deref for $name {
            type Target = String;
            fn deref(&self) -> &String { &self.0 }
        }
        impl From<$name> for String {
            fn from(value: $name) -> Self { value.0 }
        }
        impl std::str::FromStr for $name {
            type Err = self::error::ConversionError;
            fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
                uuid::Uuid::parse_str(value)
                    .map_err(|_| self::error::ConversionError::from("invalid UUID format"))?;
                Ok(Self(value.to_string()))
            }
        }
        validated_string!(@try_from $name);
        validated_string!(@deserialize $name);
    };
    // no validation (pub field, From<String>, Display, Infallible)
    ($name:ident) => {
        #[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[serde(transparent)]
        pub struct $name(pub String);
        impl std::ops::Deref for $name {
            type Target = String;
            fn deref(&self) -> &String { &self.0 }
        }
        impl From<$name> for String {
            fn from(value: $name) -> Self { value.0 }
        }
        impl From<String> for $name {
            fn from(value: String) -> Self { Self(value) }
        }
        impl std::str::FromStr for $name {
            type Err = ::std::convert::Infallible;
            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Ok(Self(value.to_string()))
            }
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
    // shared TryFrom impls
    (@try_from $name:ident) => {
        impl TryFrom<&str> for $name {
            type Error = self::error::ConversionError;
            fn try_from(value: &str) -> Result<Self, self::error::ConversionError> { value.parse() }
        }
        impl TryFrom<&String> for $name {
            type Error = self::error::ConversionError;
            fn try_from(value: &String) -> Result<Self, self::error::ConversionError> { value.parse() }
        }
        impl TryFrom<String> for $name {
            type Error = self::error::ConversionError;
            fn try_from(value: String) -> Result<Self, self::error::ConversionError> { value.parse() }
        }
    };
    // shared Deserialize impl
    (@deserialize $name:ident) => {
        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                String::deserialize(deserializer)?
                    .parse()
                    .map_err(|e: self::error::ConversionError| {
                        <D::Error as ::serde::de::Error>::custom(e.to_string())
                    })
            }
        }
    };
}

macro_rules! string_enum {
    (pub enum $name:ident { $($variant:ident => $str:expr),+ $(,)? }) => {
        #[derive(serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub enum $name {
            $(#[serde(rename = $str)] $variant,)+
        }
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                match *self {
                    $(Self::$variant => f.write_str($str),)+
                }
            }
        }
        impl std::str::FromStr for $name {
            type Err = self::error::ConversionError;
            fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
                match value {
                    $($str => Ok(Self::$variant),)+
                    _ => Err("invalid value".into()),
                }
            }
        }
        impl TryFrom<&str> for $name {
            type Error = self::error::ConversionError;
            fn try_from(value: &str) -> Result<Self, self::error::ConversionError> { value.parse() }
        }
        impl TryFrom<&String> for $name {
            type Error = self::error::ConversionError;
            fn try_from(value: &String) -> Result<Self, self::error::ConversionError> { value.parse() }
        }
        impl TryFrom<String> for $name {
            type Error = self::error::ConversionError;
            fn try_from(value: String) -> Result<Self, self::error::ConversionError> { value.parse() }
        }
    };
}

#[doc = "`Account`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct Account {
    #[doc = "Analytics source for event tracking Javascript."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analytics_src: Option<String>,
    #[doc = "Company Name associated with the account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[doc = "The ETag HTTP header"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_etag: Option<String>,
    #[doc = "The unique ID for the account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "The link to the [3rd party service Integrations](#tag/Integrations/paths/~1accounts~1{accountId}~1integrations/get)"]
    #[serde(default, skip_serializing)]
    pub integrations_collection_link: Option<String>,
    #[doc = "The link to the [Lists for the account](#tag/Lists/paths/~1accounts~1{accountId}~1lists/get)"]
    #[serde(default, skip_serializing)]
    pub lists_collection_link: Option<String>,
    #[doc = "The link to the account type"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<String>,
    #[doc = "The link to the account resource"]
    #[serde(default, skip_serializing)]
    pub self_link: Option<String>,
    #[doc = "The UUID (universally unique identifier) for the account"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<::uuid::Uuid>,
}
#[doc = "`Accounts`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct Accounts {
    #[doc = "A list of account entries"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<Account>,
    #[doc = "A link to the next page of entries if more entries exist. This attribute is omitted from the collection if there are no more entries."]
    #[serde(default, skip_serializing)]
    pub next_collection_link: Option<String>,
    #[doc = "A link to the previous page of entries if any exist. This attribute is omitted from the collection if ws.start is 0."]
    #[serde(default, skip_serializing)]
    pub prev_collection_link: Option<String>,
    #[doc = "A link that identifies the type of resource that this collection represents"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<String>,
    #[doc = "The starting offset for the page of entries to retrieve"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    #[doc = "The total number of entries"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_size: Option<u64>,
}
#[doc = "`Activity`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct Activity {
    #[doc = "The timestamp of when the event occurred"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_time: Option<String>,
    #[doc = "Identifier for this subscriber activity.  This is set to the type if no reasonable identifier exists."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "The link to the event type"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<String>,
    #[doc = "The link to the event resource (may be null)"]
    #[serde(default, skip_serializing)]
    pub self_link: Option<String>,
    #[doc = "The link to the [Subscriber](#tag/Subscribers/paths/~1accounts~1{accountId}~1lists~1{listId}~1subscribers~1{subscriberId}/get) resource"]
    #[serde(default, skip_serializing)]
    pub subscriber_link: Option<String>,
    #[doc = "The type of activity"]
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_: Option<ActivityType>,
}
string_enum! { pub enum ActivityType { Click => "click", Link => "link", Open => "open", SentMessage => "sent_message", Subscribed => "subscribed", TrackedEvent => "tracked_event", Verified => "verified" } }
#[doc = "`AddSubscriberRequestBody`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AddSubscriberRequestBody {
    #[doc = "The customer ad tracking field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ad_tracking: Option<AddSubscriberRequestBodyAdTracking>,
    #[doc = "The custom fields specified on the subscriber.  Note that the custom fields are required to already exist for the list. See [Custom Fields](#tag/Custom-Fields) for details."]
    #[serde(
        default,
        skip_serializing_if = "HashMap::is_empty"
    )]
    pub custom_fields: HashMap<
        String,
        AddSubscriberRequestBodyCustomFieldsValue,
    >,
    #[doc = "The subscriber's email address"]
    pub email: AddSubscriberRequestBodyEmail,
    #[doc = "The subscriber's IP address. This field is used to determine the following Geo Location fields: area_code, city, country, dma_code, latitude, longitude, postal_code, and region. IP address can only be specified when Subscribers are initially created. Internal, private, or reserved IP addresses are not acceptable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<AddSubscriberRequestBodyIpAddress>,
    #[doc = "The sequence number of the last followup message sent to the subscriber.  This field determines the next followup message to be sent to the Subscriber.  When set to 0 (default), the Subscriber should receive the 1st (autoresponse) Followup message.  Set the value of this field to 1001 if you do not want any Followups to be sent to this Subscriber."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_followup_message_number_sent: Option<i64>,
    #[doc = "Miscellaneous notes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub misc_notes: Option<AddSubscriberRequestBodyMiscNotes>,
    #[doc = "The subscriber's name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<AddSubscriberRequestBodyName>,
    #[doc = "If this parameter is present and set to `true`, then custom field names are matched case sensitively.  Enabling this option also causes the operation to fail if a custom field is included that is not defined for the list."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strict_custom_fields: Option<AddSubscriberRequestBodyStrictCustomFields>,
    #[doc = "A list of tags added to the subscriber.  This field is used to apply a list of tags to a Subscriber. With Campaigns, you can trigger a series of messages based on what Tags have been applied to your subscribers."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[doc = "If this parameter is present and set to `true`, then if a subscriber is already present on the list, the subscriber will be updated. <br><br> **Note:** <br>\n- Only the fields defined in the <a href='#tag/Subscribers/paths/~1accounts~1{accountId}~1lists~1{listId}~1subscribers~1{subscriberId}/patch'>patch</a> endpoint will be updated.\n- Any tags in the request will be **appended** to the existing Subscriber."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_existing: Option<AddSubscriberRequestBodyUpdateExisting>,
}
validated_string!(AddSubscriberRequestBodyAdTracking, min=1, max=20);
validated_string!(AddSubscriberRequestBodyCustomFieldsValue, min=1);
validated_string!(AddSubscriberRequestBodyEmail, min=1, max=50);
validated_string!(AddSubscriberRequestBodyIpAddress, min=1, max=60);
validated_string!(AddSubscriberRequestBodyMiscNotes, min=1, max=60);
validated_string!(AddSubscriberRequestBodyName, min=1, max=60);
string_enum! { pub enum AddSubscriberRequestBodyStrictCustomFields { True => "true", False => "false" } }
string_enum! { pub enum AddSubscriberRequestBodyUpdateExisting { True => "true", False => "false" } }
#[doc = "This will create an access token using an `authorization_code`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AuthCode {
    #[doc = "The client ID of the application. Available on the <a href=\"https://labs.aweber.com/apps\" target=\"_blank\">My Apps Page</a>. <br>_Not required if `Authorization` header is set._"]
    pub client_id: String,
    #[doc = "The client secret of the application. Available on the <a href=\"https://labs.aweber.com/apps\" target=\"_blank\">My Apps Page</a>. <br>_Not required if `Authorization` header is set._"]
    pub client_secret: String,
    #[doc = "The authorization code received from the authorization call."]
    pub code: String,
    #[doc = "The grant type of the token."]
    pub grant_type: AuthCodeGrantType,
}
string_enum! { pub enum AuthCodeGrantType { AuthorizationCode => "authorization_code" } }
#[doc = "`AuthError`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct AuthError {
    #[doc = "The type of authentication error returned.\nThe following error may be received:\n\n| Error Type       | Explanation                              |\n|------------------|------------------------------------------|\n| invalid_token    | The access token is invalid or has expired |\n"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "A human friendly description of the error\n"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
}
#[doc = "`Broadcast`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct Broadcast {
    #[doc = "The link to the broadcast archive page, only set if the broadcast was sent."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive_url: Option<String>,
    #[doc = "<b>[Please read <a href=\"https://help.aweber.com/hc/en-us/articles/360025741194\" target=\"_blank\">our KB article</a> before using this field.]</b><br>The content of the message in AMP format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_amp: Option<String>,
    #[doc = "The content of the message in html format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[doc = "The content of the message in plain text, displayed when HTML is not supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_text: Option<String>,
    #[doc = "The broadcast ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub broadcast_id: Option<i64>,
    #[doc = "Enables links in the email message to be tracked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub click_tracking_enabled: Option<bool>,
    #[doc = "The link to this broadcasts clicks collection."]
    #[serde(default, skip_serializing)]
    pub clicks_collection_link: Option<String>,
    #[doc = "When the message was created."]
    #[serde(default, skip_serializing_if = "Option::is_none", with = "flexible_datetime::option")]
    pub created_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "List of [Lists](#tag/Lists) URLs that are excluded in the delivery of this broadcast.<br> This is the `self_link` of the list here - e.g. `https://api.aweber.com/1.0/accounts/<account_id>/lists/<list_id>`"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude_lists: Vec<String>,
    #[doc = "URL to the [Facebook broadcast integration](#tag/Integrations) to use for this broadcast. When the broadcast is sent, the subject of the broadcast will be posted to this Facebook integration  - e.g., `https://api.aweber.com/1.0/accounts/<account_id>/integrations/<integration_id>`"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub facebook_integration: Option<String>,
    #[doc = "If true, the plain text field will be shown when HTML is not supported. If false, it will auto-generate plain text based on the HTML given."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_customized_body_text: Option<bool>,
    #[doc = "List of [Lists](#tag/Lists) URLs that are included in the delivery of this broadcast.<br> This is the `self_link` of the list here - e.g. `https://api.aweber.com/1.0/accounts/<account_id>/lists/<list_id>`"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub include_lists: Vec<String>,
    #[doc = "Whether the broadcast enabled sharing via an archive url."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    #[doc = "A list of links relevant to the broadcast such as RSS feeds and broadcast archives."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<BroadcastLinksItem>,
    #[doc = "If true, notify when stats are available on a sent broadcast message, defaults to true."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notify_on_send: Option<bool>,
    #[doc = "The link to this broadcasts opens collection."]
    #[serde(default, skip_serializing)]
    pub opens_collection_link: Option<String>,
    #[doc = "When broadcast is scheduled to send."]
    #[serde(default, skip_serializing_if = "Option::is_none", with = "flexible_datetime::option")]
    pub scheduled_for: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "URL to the [Segment](#tag/Segments) to send this broadcast to."]
    #[serde(default, skip_serializing)]
    pub segment_link: Option<::serde_json::Value>,
    #[doc = "The name of the segment the broadcast will be sent to, or Unknown if the segment no longer exists."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segment_name: Option<String>,
    #[doc = "The link to this resource."]
    #[serde(default, skip_serializing)]
    pub self_link: Option<String>,
    #[doc = "Date/Time broadcast was sent."]
    #[serde(default, skip_serializing_if = "Option::is_none", with = "flexible_datetime::option")]
    pub sent_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "A list of statistics for a broadcast, only set for sent broadcasts."]
    #[serde(default, skip_serializing_if = "::serde_json::Map::is_empty")]
    pub stats: ::serde_json::Map<String, ::serde_json::Value>,
    #[doc = "The status of the broadcast."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<BroadcastStatus>,
    #[doc = "The broadcast subject line."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[doc = "URL to the [Twitter broadcast integration](#tag/Integrations) to use for this broadcast. When the broadcast is sent, the subject of the broadcast will be tweeted - e.g., `https://api.aweber.com/1.0/accounts/<account_id>/integrations/<integration_id>`"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub twitter_integration: Option<String>,
    #[doc = "The broadcast UUID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}
#[doc = "`BroadcastClicks`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct BroadcastClicks {
    #[doc = "A list of aggregated click entries, showing unique clicks per subscriber with click statistics"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<BroadcastClicksEntriesItem>,
    #[doc = "A link to the next page of entries if more entries exist. This attribute is omitted from the collection if there are no more entries."]
    #[serde(default, skip_serializing)]
    pub next_collection_link: Option<String>,
    #[doc = "A link to the previous page of entries if any exist."]
    #[serde(default, skip_serializing)]
    pub prev_collection_link: Option<String>,
    #[doc = "A link that identifies the type of resource that this collection represents"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<String>,
    #[doc = "The total number of unique subscribers who clicked the broadcast"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_size: Option<u64>,
}
#[doc = "`BroadcastClicksDetailed`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct BroadcastClicksDetailed {
    #[doc = "A list of individual click event entries, showing each click occurrence with the specific URL"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<BroadcastClicksDetailedEntriesItem>,
    #[doc = "A link to the next page of entries if more entries exist. This attribute is omitted from the collection if there are no more entries."]
    #[serde(default, skip_serializing)]
    pub next_collection_link: Option<String>,
    #[doc = "A link to the previous page of entries if any exist."]
    #[serde(default, skip_serializing)]
    pub prev_collection_link: Option<String>,
    #[doc = "A link that identifies the type of resource that this collection represents"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<String>,
    #[doc = "The total number of individual click events (can be higher than unique subscribers if some clicked multiple times)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_size: Option<u64>,
}
#[doc = "`BroadcastClicksDetailedEntriesItem`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct BroadcastClicksDetailedEntriesItem {
    #[doc = "Timestamp of when this specific click occurred"]
    #[serde(default, skip_serializing_if = "Option::is_none", with = "flexible_datetime::option")]
    pub clicked_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Email address of the subscriber who clicked the broadcast. *Note:* This is the current email address of the subscriber, not necessarily the email address the broadcast was sent to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Time when this individual click event occurred (same value as clicked_at)"]
    #[serde(default, skip_serializing_if = "Option::is_none", with = "flexible_datetime::option")]
    pub event_time: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "A link to the click type"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<String>,
    #[doc = "The link to the subscriber entry"]
    #[serde(default, skip_serializing)]
    pub subscriber_link: Option<String>,
    #[doc = "Type of subscriber activity"]
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_: Option<BroadcastClicksDetailedEntriesItemType>,
    #[doc = "The URL that was clicked"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
string_enum! { pub enum BroadcastClicksDetailedEntriesItemType { Click => "click" } }
#[doc = "`BroadcastClicksEntriesItem`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct BroadcastClicksEntriesItem {
    #[doc = "Email address of the subscriber who clicked the broadcast. *Note:* This is the current email address of the subscriber, not necessarily the email address the broadcast was sent to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Time when the first click occurred for this subscriber"]
    #[serde(default, skip_serializing_if = "Option::is_none", with = "flexible_datetime::option")]
    pub event_time: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Time of the first click by this subscriber"]
    #[serde(default, skip_serializing_if = "Option::is_none", with = "flexible_datetime::option")]
    pub first_click_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "Time of the last click by this subscriber"]
    #[serde(default, skip_serializing_if = "Option::is_none", with = "flexible_datetime::option")]
    pub last_click_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "A link to the click type"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<String>,
    #[doc = "The link to the subscriber entry"]
    #[serde(default, skip_serializing)]
    pub subscriber_link: Option<String>,
    #[doc = "Total number of clicks by this subscriber"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_clicks: Option<::std::num::NonZeroU64>,
    #[doc = "Type of subscriber activity"]
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_: Option<BroadcastClicksEntriesItemType>,
}
string_enum! { pub enum BroadcastClicksEntriesItemType { Click => "click" } }
#[doc = "`BroadcastLinksItem`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct BroadcastLinksItem {
    #[doc = "Link target"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[doc = "Link relation (`\"list_archive_rss_url\"`, `\"list_archive_url\"`, etc.)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rel: Option<String>,
}
#[doc = "`BroadcastOpens`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct BroadcastOpens {
    #[doc = "A list of entries"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<BroadcastOpensEntriesItem>,
    #[doc = "A link to the next page of entries if more entries exist. This attribute is omitted from the collection if there are no more entries."]
    #[serde(default, skip_serializing)]
    pub next_collection_link: Option<String>,
    #[doc = "A link to the previous page of entries if any exist."]
    #[serde(default, skip_serializing)]
    pub prev_collection_link: Option<String>,
    #[doc = "A link that identifies the type of resource that this collection represents"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<String>,
    #[doc = "The total number of entries"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_size: Option<u64>,
}
#[doc = "`BroadcastOpensEntriesItem`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct BroadcastOpensEntriesItem {
    #[doc = "Email address of the subscriber who opened the broadcast. *Note:* This is the current email address of the subscriber, not necessarily the email address the broadcast was sent to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "Time that the open occurred"]
    #[serde(default, skip_serializing_if = "Option::is_none", with = "flexible_datetime::option")]
    pub event_time: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "A link to the open type"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<String>,
    #[doc = "The link to the subscriber entry"]
    #[serde(default, skip_serializing)]
    pub subscriber_link: Option<String>,
    #[doc = "Type of subscriber activity"]
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_: Option<BroadcastOpensEntriesItemType>,
}
string_enum! { pub enum BroadcastOpensEntriesItemType { Open => "open" } }
string_enum! { pub enum BroadcastStatus { AdminHold => "admin_hold", Draft => "draft", Scheduled => "scheduled", Sending => "sending", Sent => "sent", Unknown => "unknown" } }
#[doc = "`Broadcasts`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct Broadcasts {
    #[doc = "The list of broadcast entries."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<BroadcastsEntriesItem>,
    #[doc = "A link to the next page of entries if more entries exist. This attribute is omitted from the collection if there are no more entries."]
    #[serde(default, skip_serializing)]
    pub next_collection_link: Option<String>,
    #[doc = "A link to the previous page of entries if previous entries exist. This attribute is omitted if this is the first page."]
    #[serde(default, skip_serializing)]
    pub prev_collection_link: Option<String>,
    #[doc = "A link that identifies the type of resource that this collection represents."]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<String>,
    #[doc = "A link to the current page of entries."]
    #[serde(default, skip_serializing)]
    pub self_link: Option<String>,
    #[doc = "The starting offset for the page of entries to retrieve."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    #[doc = "A link to a resource that provides the total number of broadcasts in the collection."]
    #[serde(default, skip_serializing)]
    pub total_size_link: Option<String>,
}
#[doc = "`BroadcastsEntriesItem`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct BroadcastsEntriesItem {
    #[doc = "The legacy broadcast ID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub broadcast_id: Option<i64>,
    #[doc = "When broadcast is scheduled to send."]
    #[serde(default, skip_serializing_if = "Option::is_none", with = "flexible_datetime::option")]
    pub scheduled_for: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "The link to this resource."]
    #[serde(default, skip_serializing)]
    pub self_link: Option<String>,
    #[doc = "Date/Time broadcast was sent."]
    #[serde(default, skip_serializing_if = "Option::is_none", with = "flexible_datetime::option")]
    pub sent_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "The status of the broadcast."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<BroadcastsEntriesItemStatus>,
    #[doc = "The broadcast subject line."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[doc = "The broadcast UUID."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
}
string_enum! { pub enum BroadcastsEntriesItemStatus { Draft => "draft", Scheduled => "scheduled", Sent => "sent" } }
#[doc = "`Campaign`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct Campaign {
    #[doc = "Type of campaign. (f: followup, b: broadcast)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub campaign_type: Option<CampaignCampaignType>,
    #[doc = "Whether click tracking is enabled"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub click_tracking_enabled: Option<bool>,
    #[doc = "Type of message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<CampaignContentType>,
    #[doc = "Unique campaign ID for followup or broadcast"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "Is the campaign in the campaign archive (broadcast only)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    #[doc = "A URL to retrieve the [Links](#tag/Campaigns/paths/~1accounts~1{accountId}~1lists~1{listId}~1campaigns~1{campaignType}{campaignId}~1links/get) for this campaign"]
    #[serde(default, skip_serializing)]
    pub links_collection_link: Option<String>,
    #[doc = "Message interval (followup only)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_interval: Option<::std::num::NonZeroU64>,
    #[doc = "Followup sequence number (followup only)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_number: Option<::std::num::NonZeroU64>,
    #[doc = "A link that identifies the type of resource that this collection represents"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<String>,
    #[doc = "When broadcast is scheduled to send (broadcast only)"]
    #[serde(default, skip_serializing_if = "Option::is_none", with = "flexible_datetime::option")]
    pub scheduled_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "The link to this resource"]
    #[serde(default, skip_serializing)]
    pub self_link: Option<String>,
    #[doc = "Date/Time campaign was sent (broadcast only)"]
    #[serde(default, skip_serializing_if = "Option::is_none", with = "flexible_datetime::option")]
    pub sent_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spam_assassin_score: Option<f32>,
    #[doc = "A link to the [statistics collection](#tag/Campaigns/paths/~1accounts~1{accountId}~1lists~1{listId}~1campaigns~1b{campaignId}~1stats/get) for this campaign"]
    #[serde(default, skip_serializing)]
    pub stats_collection_link: Option<String>,
    #[doc = "Subject of message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[doc = "Total clicks of this message"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_clicks: Option<i64>,
    #[doc = "Total number of messages opened"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_opens: Option<i64>,
    #[doc = "Total number of messages sent"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_sent: Option<i64>,
    #[doc = "Total number of spam complaints received"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_spam_complaints: Option<i64>,
    #[doc = "Total number of undeliverable messages"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_undelivered: Option<i64>,
    #[doc = "Total number of unsubscribes from this campaign"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_unsubscribes: Option<i64>,
    #[doc = "Twitter account where broadcast was tweeted (broadcasts only). Links to [Integration](#tag/Integrations/paths/~1accounts~1{accountId}~1integrations~1{integrationId}/get)."]
    #[serde(default, skip_serializing)]
    pub twitter_account_link: Option<String>,
}
string_enum! { pub enum CampaignCampaignType { B => "b", F => "f" } }
string_enum! { pub enum CampaignContentType { Text => "Text", Html => "HTML", TextHtml => "Text/HTML" } }
#[doc = "`Confidential`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Confidential {
    #[doc = "The client ID of the application. Available on the <a href=\"https://labs.aweber.com/apps\" target=\"_blank\">My Apps Page</a>. <br>_Not required if `Authorization` header is set._"]
    pub client_id: String,
    #[doc = "The client secret of the application. Available on the <a href=\"https://labs.aweber.com/apps\" target=\"_blank\">My Apps Page</a>. <br>_Not required if `Authorization` header is set._"]
    pub client_secret: String,
    #[doc = "The access token or refresh token to revoke"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[doc = "Type of token given in the token parameter. Valid values are <ul><li>access_token<li>refresh_token</ul><br>If not specified, search for both kinds of tokens."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_type_hint: Option<ConfidentialTokenTypeHint>,
}
string_enum! { pub enum ConfidentialTokenTypeHint { AccessToken => "access_token", RefreshToken => "refresh_token" } }
#[doc = "`CreateBroadcast`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct CreateBroadcast {
    #[doc = "<b>[Please read <a href=\"https://help.aweber.com/hc/en-us/articles/360025741194\" target=\"_blank\">our KB article</a> before using this field.]</b><br>The content of the message in AMP format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_amp: Option<String>,
    #[doc = "The content of the message in html format. If body_text is not provided, it will be auto-generated. If body_text is not provided, body_html must be provided."]
    pub body_html: String,
    #[doc = "The content of the message in plain text, used when HTML is not supported. If body_html is not provided, the broadcast will be sent using only the body_text. If body_text is not provided, body_html must be provided."]
    pub body_text: String,
    #[doc = "Enables links in the email message to be tracked"]
    #[serde(default = "defaults::default_bool::<true>")]
    pub click_tracking_enabled: bool,
    #[doc = "JSON encoded list of [Lists](#tag/Lists) URLs to exclude in the delivery of this broadcast.<br> Use the `self_link` of the list here - e.g. `https://api.aweber.com/1.0/accounts/<account_id>/lists/<list_id>`. If updated, this value will replace the existing excluded_lists."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude_lists: Option<String>,
    #[doc = "URL to the [Facebook broadcast integration](#tag/Integrations) to use for this broadcast. When the broadcast is sent, the subject of the broadcast will be posted to this Facebook integration  - e.g., `https://api.aweber.com/1.0/accounts/<account_id>/integrations/<integration_id>`"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub facebook_integration: Option<String>,
    #[doc = "JSON encoded list of [Lists](#tag/Lists) URLs to include in the delivery of this broadcast.<br> Use the `self_link` of the list here - e.g. `https://api.aweber.com/1.0/accounts/<account_id>/lists/<list_id>`. If updated, this value will replace the existing included_lists."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_lists: Option<String>,
    #[doc = "Whether the broadcast enabled sharing via an archive url"]
    #[serde(default = "defaults::default_bool::<true>")]
    pub is_archived: bool,
    #[doc = "If true, notify when stats are available on a sent broadcast message"]
    #[serde(default = "defaults::default_bool::<true>")]
    pub notify_on_send: bool,
    #[doc = "URL to the [Segment](#tag/Segments) to send this broadcast to.  Use the `self_link` of the\nsegment here - e.g.\n`https://api.aweber.com/1.0/accounts/<account_id>/lists/<list_id>/segments/<segment_id>`.\nIf not specified, the broadcast will be sent to the “Active Subscribers” segment.\n"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segment_link: Option<::serde_json::Value>,
    #[doc = "The broadcast subject line. Subject must not be empty nor contain only whitespace."]
    pub subject: String,
    #[doc = "URL to the [Twitter broadcast integration](#tag/Integrations) to use for this broadcast. When the broadcast is sent, the subject of the broadcast will be tweeted - e.g., `https://api.aweber.com/1.0/accounts/<account_id>/integrations/<integration_id>`"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub twitter_integration: Option<String>,
}
#[doc = "`CustomField`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct CustomField {
    #[doc = "The ETag HTTP header"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_etag: Option<String>,
    #[doc = "The unique ID for the custom field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "Whether the subscriber is allowed to update the custom field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_subscriber_updateable: Option<bool>,
    #[doc = "The name of the custom field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The link to the custom field type"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<String>,
    #[doc = "The link to the event resource"]
    #[serde(default, skip_serializing)]
    pub self_link: Option<String>,
}
#[doc = "`CustomFields`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct CustomFields {
    #[doc = "A list of custom field entries"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<CustomField>,
    #[doc = "A link to the next page of entries if more entries exist. This attribute is omitted from the collection if there are no more entries."]
    #[serde(default, skip_serializing)]
    pub next_collection_link: Option<String>,
    #[doc = "A link to the previous page of entries if any exist. This attribute is omitted from the collection if ws.start is 0."]
    #[serde(default, skip_serializing)]
    pub prev_collection_link: Option<String>,
    #[doc = "A link that identifies the type of resource that this collection represents"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<String>,
    #[doc = "The starting offset for the page of entries to retrieve"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    #[doc = "The total number of entries"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_size: Option<u64>,
}
validated_string!(DeleteAccountsListsSubscribersSubscriberEmail, min=1, max=50);
#[doc = "`EndpointError`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct EndpointError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<EndpointErrorError>,
}
#[doc = "An error object.\nThe following error may be received:\n\n| Error Type           | Explanation                              |\n|----------------------|------------------------------------------|\n| UnauthorizedError    | There is a problem with your authorization credentials |\n\nPlease see the message body for more details.\n"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct EndpointErrorError {
    #[doc = "A link to the documentation that describes the error"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documentation_url: Option<String>,
    #[doc = "A human friendly description of the error"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[doc = "The HTTP status code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<EndpointErrorErrorStatus>,
    #[doc = "The API error type"]
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_: Option<EndpointErrorErrorType>,
}
#[doc = "The HTTP status code"]
#[derive(serde::Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct EndpointErrorErrorStatus(i64);
impl std::ops::Deref for EndpointErrorErrorStatus {
    type Target = i64;
    fn deref(&self) -> &i64 {
        &self.0
    }
}
impl From<EndpointErrorErrorStatus> for i64 {
    fn from(value: EndpointErrorErrorStatus) -> Self {
        value.0
    }
}
impl TryFrom<i64> for EndpointErrorErrorStatus {
    type Error = self::error::ConversionError;
    fn try_from(value: i64) -> Result<Self, self::error::ConversionError> {
        if ![401_i64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for EndpointErrorErrorStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<i64>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
string_enum! { pub enum EndpointErrorErrorType { UnauthorizedError => "UnauthorizedError" } }
#[doc = "`FindCampaigns`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct FindCampaigns {
    #[doc = "A list of campaign entries"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<Campaign>,
    #[doc = "A link to the next page of entries if more entries exist. This attribute is omitted from the collection if there are no more entries."]
    #[serde(default, skip_serializing)]
    pub next_collection_link: Option<String>,
    #[doc = "A link to the previous page of entries if any exist. This attribute is omitted from the collection if ws.start is 0."]
    #[serde(default, skip_serializing)]
    pub prev_collection_link: Option<String>,
    #[doc = "The starting offset for the page of entries to retrieve"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    #[doc = "Link to the total number of entries"]
    #[serde(default, skip_serializing)]
    pub total_size_link: Option<String>,
}
#[doc = "`FindLists`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct FindLists {
    #[doc = "The list of list entries"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<List>,
    #[doc = "The starting offset for the page of entries to retrieve"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    #[doc = "A link to check the total number of entries found for this resource"]
    #[serde(default, skip_serializing)]
    pub total_size_link: Option<String>,
}
#[doc = "`FindSubscribers`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct FindSubscribers {
    #[doc = "The list of subscriber entries plus list information"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<SubscriberFind>,
    #[doc = "A link to the next page of entries if more entries exist. This attribute is omitted from the collection if there are no more entries."]
    #[serde(default, skip_serializing)]
    pub next_collection_link: Option<String>,
    #[doc = "A link to the previous page of entries if previous entries exist. This attribute is omitted if this is the first page."]
    #[serde(default, skip_serializing)]
    pub prev_collection_link: Option<String>,
    #[doc = "The starting offset for the page of entries to retrieve"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    #[doc = "Link to the total number of entries"]
    #[serde(default, skip_serializing)]
    pub total_size_link: Option<String>,
}
#[doc = "`FindSubscribersAccount`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct FindSubscribersAccount {
    #[doc = "The list of subscriber entries plus list information"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<SubscriberFind>,
    #[doc = "A link to the next page of entries if more entries exist. This attribute is omitted from the collection if there are no more entries."]
    #[serde(default, skip_serializing)]
    pub next_collection_link: Option<String>,
    #[doc = "A link to the previous page of entries if previous entries exist. This attribute is omitted if this is the first page."]
    #[serde(default, skip_serializing)]
    pub prev_collection_link: Option<String>,
    #[doc = "A link that identifies the type of resource that this collection represents"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<String>,
    #[doc = "A link to the current page of entries."]
    #[serde(default, skip_serializing)]
    pub self_link: Option<String>,
    #[doc = "The starting offset for the page of entries to retrieve"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    #[doc = "Link to the total number of entries"]
    #[serde(default, skip_serializing)]
    pub total_size_link: Option<String>,
}
#[doc = "`Form`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Form {
    #[doc = "The client ID of the application. Available on the <a href=\"https://labs.aweber.com/apps\" target=\"_blank\">My Apps Page</a>. <br>_Not required if `Authorization` header is set._"]
    pub client_id: String,
    #[doc = "The client secret of the application. Available on the <a href=\"https://labs.aweber.com/apps\" target=\"_blank\">My Apps Page</a>. <br>_Not required if `Authorization` header is set._"]
    pub client_secret: String,
    #[doc = "The access token or refresh token to revoke"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[doc = "Type of token given in the token parameter. Valid values are <ul><li>access_token<li>refresh_token</ul><br>If not specified, search for both kinds of tokens."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_type_hint: Option<FormTokenTypeHint>,
}
#[doc = "`FormPkce`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct FormPkce {
    #[doc = "The client ID of the application. Available on the <a href=\"https://labs.aweber.com/apps\" target=\"_blank\">My Apps Page</a>. <br>_Not required if `Authorization` header is set._"]
    pub client_id: String,
    #[doc = "The access token or refresh token to revoke"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[doc = "Type of token given in the token parameter. Valid values are <ul><li>access_token<li>refresh_token</ul><br>If not specified, search for both kinds of tokens."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_type_hint: Option<FormPkceTokenTypeHint>,
}
string_enum! { pub enum FormPkceTokenTypeHint { AccessToken => "access_token", RefreshToken => "refresh_token" } }
string_enum! { pub enum FormTokenTypeHint { AccessToken => "access_token", RefreshToken => "refresh_token" } }
validated_string!(GetAccountsFindsubscribersAdTracking, min=1, max=20);
validated_string!(GetAccountsFindsubscribersCity, min=1, max=100);
validated_string!(GetAccountsFindsubscribersCountry, min=1, max=100);
validated_string!(GetAccountsFindsubscribersEmail, min=1, max=50);
validated_string!(GetAccountsFindsubscribersMiscNotes, max=60);
validated_string!(GetAccountsFindsubscribersName, min=1, max=60);
validated_string!(GetAccountsFindsubscribersPostalCode, min=1, max=100);
validated_string!(GetAccountsFindsubscribersRegion, min=1, max=100);
string_enum! { pub enum GetAccountsFindsubscribersStatus { Subscribed => "subscribed", Unsubscribed => "unsubscribed", Unconfirmed => "unconfirmed" } }
string_enum! { pub enum GetAccountsFindsubscribersSubscriptionMethod { Api => "api", Email => "email", Import => "import", Webform => "webform" } }
string_enum! { pub enum GetAccountsFindsubscribersUnsubscribeMethod { UnsubscribeLink => "unsubscribe link", CustomerCp => "customer cp", Undeliverable => "undeliverable", ApiUnsubscribe => "api: unsubscribe", ApiMove => "api: move" } }
string_enum! { pub enum GetAccountsFindsubscribersWsOp { FindSubscribers => "findSubscribers" } }
string_enum! { pub enum GetAccountsFindsubscribersWsShow { TotalSize => "total_size" } }
string_enum! { pub enum GetAccountsGetwebformsWsOp { GetWebForms => "getWebForms" } }
string_enum! { pub enum GetAccountsGetwebformsplittestsWsOp { GetWebFormSplitTests => "getWebFormSplitTests" } }
string_enum! { pub enum GetAccountsListsBroadcastsStatus { Draft => "draft", Scheduled => "scheduled", Sent => "sent" } }
#[doc = "`GetAccountsListsBroadcastsTotalResponse`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct GetAccountsListsBroadcastsTotalResponse {
    #[doc = "The total broadcasts with the status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_size: Option<i64>,
}
string_enum! { pub enum GetAccountsListsBroadcastsTotalStatus { Draft => "draft", Scheduled => "scheduled", Sent => "sent" } }
string_enum! { pub enum GetAccountsListsCampaignsBcampaignidStats2StatsId { TotalClicks => "total_clicks", UniqueClicks => "unique_clicks", TotalOpens => "total_opens", UniqueOpens => "unique_opens", TotalSales => "total_sales", TotalSalesDollars => "total_sales_dollars", TotalUnsubscribed => "total_unsubscribed", HourlyOpens => "hourly_opens", HourlyClicks => "hourly_clicks", HourlyWebhits => "hourly_webhits", HourlySales => "hourly_sales", HourlyUnsubscribed => "hourly_unsubscribed", DailyOpens => "daily_opens", DailyClicks => "daily_clicks", DailyWebhits => "daily_webhits", DailySales => "daily_sales", DailyUnsubscribed => "daily_unsubscribed", ClicksByLink => "clicks_by_link", WebhitsByLink => "webhits_by_link", OpensBySubscriber => "opens_by_subscriber", SalesBySubscriber => "sales_by_subscriber" } }
string_enum! { pub enum GetAccountsListsCampaignsCampaigntypecampaignidCampaignType { B => "b", F => "f" } }
string_enum! { pub enum GetAccountsListsCampaignsFindCampaignType { B => "b", F => "f" } }
string_enum! { pub enum GetAccountsListsCampaignsFindWsOp { Find => "find" } }
string_enum! { pub enum GetAccountsListsCampaignsFindWsShow { TotalSize => "total_size" } }
validated_string!(GetAccountsListsFindName, min=1, max=100);
string_enum! { pub enum GetAccountsListsFindWsOp { Find => "find" } }
string_enum! { pub enum GetAccountsListsFindWsShow { TotalSize => "total_size" } }
#[doc = "`GetAccountsListsSubscribers2Response`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct GetAccountsListsSubscribers2Response {
    #[doc = "The customer ad tracking field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ad_tracking: Option<String>,
    #[doc = "The subscriber's area code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area_code: Option<i64>,
    #[doc = "The subscriber's address city"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "The subscriber's address country"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "The custom fields specified on the subscriber"]
    #[serde(
        default,
        skip_serializing_if = "HashMap::is_empty"
    )]
    pub custom_fields:
        HashMap<String, Option<String>>,
    #[doc = "The subscriber's designated market area code (USA and canada only)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dma_code: Option<i64>,
    #[doc = "The subscriber's email address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "The lead ID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    #[doc = "The subscriber's IP address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "A value indication if the subscriber's email address confirmed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
    #[doc = "The sequence number of the last followup message sent to the subscriber.  If this value is set to 1001, then the subscriber will not receive followup messages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_followup_message_number_sent: Option<i64>,
    #[doc = "The last followup message sent to the subscriber"]
    #[serde(default, skip_serializing_if = "Option::is_none", with = "flexible_datetime::option")]
    pub last_followup_sent_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "A link to the [Last Follow Up Message](#tag/Campaigns/paths/~1accounts~1{accountId}~1lists~1{listId}~1campaigns~1{campaignType}{campaignId}/get) the subscriber was sent"]
    #[serde(default, skip_serializing)]
    pub last_followup_sent_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[doc = "Miscellaneous notes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub misc_notes: Option<String>,
    #[doc = "The subscriber's name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The subscriber's address postal code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[doc = "The subscriber's address stage or region abbreviation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[doc = "A link that identifies the type of resource that this collection represents"]
    #[serde(default, skip_serializing)]
    pub resource_type_link:
        Option<GetAccountsListsSubscribers2ResponseResourceTypeLink>,
    #[doc = "The link to this resource"]
    #[serde(default, skip_serializing)]
    pub self_link: Option<String>,
    #[doc = "The subscriber's status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<GetAccountsListsSubscribers2ResponseStatus>,
    #[doc = "The timestamp for when the subscriber subscribed"]
    #[serde(default, skip_serializing_if = "Option::is_none", with = "flexible_datetime::option")]
    pub subscribed_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "The method by which the subscriber was subscribed.\n\n| Method    | Description                              |\n| ------    | -----------                              |\n| api       | subscribed via an API integration        |\n| email     | subscriber emailed to list to opt-in     |\n| import    | subscriber was imported by the customer  |\n| webform   | subscriber subscribed via a web form     |\n"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_method:
        Option<GetAccountsListsSubscribers2ResponseSubscriptionMethod>,
    #[doc = "The webform url from which the subscriber subscribed from"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_url: Option<String>,
    #[doc = "A list of tags added to the subscriber"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[doc = "The method that describes how the subscriber unsubscribed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unsubscribe_method:
        Option<GetAccountsListsSubscribers2ResponseUnsubscribeMethod>,
    #[doc = "The timestamp for when the subscriber unsubscribed"]
    #[serde(default, skip_serializing_if = "Option::is_none", with = "flexible_datetime::option")]
    pub unsubscribed_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "The UUID (universally unique identifier) for the subscriber"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "The timestamp for when the subscriber confirmed their email address"]
    #[serde(default, skip_serializing_if = "Option::is_none", with = "flexible_datetime::option")]
    pub verified_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
}
string_enum! { pub enum GetAccountsListsSubscribers2ResponseResourceTypeLink { HttpsApiAweberCom10Subscriber => "https://api.aweber.com/1.0/#subscriber" } }
string_enum! { pub enum GetAccountsListsSubscribers2ResponseStatus { Subscribed => "subscribed", Unsubscribed => "unsubscribed", Unconfirmed => "unconfirmed" } }
string_enum! { pub enum GetAccountsListsSubscribers2ResponseSubscriptionMethod { Api => "api", Email => "email", Import => "import", Webform => "webform" } }
string_enum! { pub enum GetAccountsListsSubscribers2ResponseUnsubscribeMethod { UnsubscribeLink => "unsubscribe link", CustomerCp => "customer cp", Undeliverable => "undeliverable", ApiUnsubscribe => "api: unsubscribe", ApiMove => "api: move" } }
validated_string!(GetAccountsListsSubscribersFindAdTracking, min=1, max=20);
validated_string!(GetAccountsListsSubscribersFindCity, min=1, max=100);
validated_string!(GetAccountsListsSubscribersFindCountry, min=1, max=100);
validated_string!(GetAccountsListsSubscribersFindEmail, min=1, max=50);
validated_string!(GetAccountsListsSubscribersFindMiscNotes, max=60);
validated_string!(GetAccountsListsSubscribersFindName, min=1, max=60);
validated_string!(GetAccountsListsSubscribersFindPostalCode, min=1, max=100);
validated_string!(GetAccountsListsSubscribersFindRegion, min=1, max=100);
string_enum! { pub enum GetAccountsListsSubscribersFindSortKey { SubscribedAt => "subscribed_at", UnsubscribedAt => "unsubscribed_at" } }
string_enum! { pub enum GetAccountsListsSubscribersFindSortOrder { Asc => "asc", Desc => "desc" } }
string_enum! { pub enum GetAccountsListsSubscribersFindStatus { Subscribed => "subscribed", Unsubscribed => "unsubscribed", Unconfirmed => "unconfirmed" } }
string_enum! { pub enum GetAccountsListsSubscribersFindSubscriptionMethod { Api => "api", Email => "email", Import => "import", Webform => "webform" } }
string_enum! { pub enum GetAccountsListsSubscribersFindUnsubscribeMethod { UnsubscribeLink => "unsubscribe link", CustomerCp => "customer cp", Undeliverable => "undeliverable", ApiUnsubscribe => "api: unsubscribe", ApiMove => "api: move" } }
string_enum! { pub enum GetAccountsListsSubscribersFindWsOp { Find => "find" } }
string_enum! { pub enum GetAccountsListsSubscribersFindWsShow { TotalSize => "total_size" } }
string_enum! { pub enum GetAccountsListsSubscribersGetactivityWsOp { GetActivity => "getActivity" } }
string_enum! { pub enum GetAccountsListsSubscribersSortOrder { Asc => "asc", Desc => "desc" } }
validated_string!(GetBroadcastLinksAnalyticsAccountId, uuid);
validated_string!(GetBroadcastLinksAnalyticsBroadcastId, uuid);
string_enum! { pub enum GetBroadcastLinksAnalyticsFilter { Clicks => "clicks", Pageviews => "pageviews" } }
#[doc = "`GetBroadcastLinksAnalyticsResponseItem`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct GetBroadcastLinksAnalyticsResponseItem {
    #[doc = "Total number of interactions with the link"]
    pub total: i64,
    #[doc = "Type of interaction (click or pageview)"]
    #[serde(rename = "type")]
    pub type_: GetBroadcastLinksAnalyticsResponseItemType,
    #[doc = "Number of unique interactions with the link"]
    pub unique: i64,
    #[doc = "The URL of the link"]
    pub url: String,
}
string_enum! { pub enum GetBroadcastLinksAnalyticsResponseItemType { Click => "click", Pageview => "pageview" } }
string_enum! { pub enum GetBroadcastLinksAnalyticsSortBy { Unique => "unique", Total => "total" } }
#[doc = "`Integration`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct Integration {
    #[doc = "The ETag HTTP header"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_etag: Option<String>,
    #[doc = "The unique ID for the integration"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "The integration user name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    #[doc = "The link to the integration type"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<String>,
    #[doc = "The link to this resource"]
    #[serde(default, skip_serializing)]
    pub self_link: Option<String>,
    #[doc = "The integration name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service_name: Option<String>,
}
#[doc = "`Integrations`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct Integrations {
    #[doc = "A list of integration entries"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<Integration>,
    #[doc = "A link to the next page of entries if more entries exist. This attribute is omitted from the collection if there are no more entries."]
    #[serde(default, skip_serializing)]
    pub next_collection_link: Option<String>,
    #[doc = "A link to the previous page of entries if any exist. This attribute is omitted from the collection if ws.start is 0."]
    #[serde(default, skip_serializing)]
    pub prev_collection_link: Option<String>,
    #[doc = "A link that identifies the type of resource that this collection represents"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<String>,
    #[doc = "The starting offset for the page of entries to retrieve"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    #[doc = "The total number of entries"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_size: Option<u64>,
}
#[doc = "`LandingPage`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct LandingPage {
    #[doc = "The draft HTML of the landing page"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_html: Option<String>,
    #[doc = "When the landing page was created"]
    #[serde(default, skip_serializing_if = "Option::is_none", with = "flexible_datetime::option")]
    pub created_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "The unique ID for the landing page"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<::uuid::Uuid>,
    #[doc = "When the landing page was last modified"]
    #[serde(default, skip_serializing_if = "Option::is_none", with = "flexible_datetime::option")]
    pub modified_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "The landing page name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "When the landing page was published"]
    #[serde(default, skip_serializing_if = "Option::is_none", with = "flexible_datetime::option")]
    pub published_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "The published HTML of the landing page"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub published_html: Option<String>,
    #[doc = "The URL for the landing page"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub published_url: Option<String>,
    #[doc = "The link to the landing page type"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<String>,
    #[doc = "The link to this resource"]
    #[serde(default, skip_serializing)]
    pub self_link: Option<String>,
    #[doc = "The status of the landing page"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<LandingPageStatus>,
}
#[doc = "`LandingPageNoContent`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct LandingPageNoContent {
    #[doc = "When the landing page was created"]
    #[serde(default, skip_serializing_if = "Option::is_none", with = "flexible_datetime::option")]
    pub created_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "The unique ID for the landing page"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<::uuid::Uuid>,
    #[doc = "When the landing page was last modified"]
    #[serde(default, skip_serializing_if = "Option::is_none", with = "flexible_datetime::option")]
    pub modified_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "The landing page name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "When the landing page was published"]
    #[serde(default, skip_serializing_if = "Option::is_none", with = "flexible_datetime::option")]
    pub published_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "The URL for the landing page"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub published_url: Option<String>,
    #[doc = "The link to the landing page type"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<String>,
    #[doc = "The link to this resource"]
    #[serde(default, skip_serializing)]
    pub self_link: Option<String>,
    #[doc = "The status of the landing page"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<LandingPageNoContentStatus>,
}
string_enum! { pub enum LandingPageNoContentStatus { Published => "published", Unpublished => "unpublished" } }
string_enum! { pub enum LandingPageStatus { Published => "published", Unpublished => "unpublished" } }
#[doc = "`LandingPages`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct LandingPages {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<LandingPageNoContent>,
    #[doc = "A link to the next page of entries if more entries exist. This attribute is omitted from the collection if there are no more entries."]
    #[serde(default, skip_serializing)]
    pub next_collection_link: Option<String>,
    #[doc = "A link to the previous page of entries if any exist. This attribute is omitted from the collection if ws.start is 0."]
    #[serde(default, skip_serializing)]
    pub prev_collection_link: Option<String>,
    #[doc = "The link to the landing page page type"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<String>,
    #[doc = "The starting offset for the page of entries to retrieve"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    #[doc = "The total number of entries"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_size: Option<u64>,
}
#[doc = "`List`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct List {
    #[doc = "[E-Mail Campaigns](#tag/Campaigns/paths/~1accounts~1{accountId}~1lists~1{listId}~1campaigns/get) used by this list"]
    #[serde(default, skip_serializing)]
    pub campaigns_collection_link: Option<String>,
    #[doc = "Subscriber [Custom Fields](#tag/Custom-Fields/paths/~1accounts~1{accountId}~1lists~1{listId}~1custom_fields/get) defined for this list"]
    #[serde(default, skip_serializing)]
    pub custom_fields_collection_link: Option<String>,
    #[doc = "Draft [Broadcasts](#tag/Broadcasts/paths/~1accounts~1{accountId}~1lists~1{listId}~1broadcasts/get) belonging to this list"]
    #[serde(default, skip_serializing)]
    pub draft_broadcasts_link: Option<String>,
    #[doc = "The ETag HTTP header"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_etag: Option<String>,
    #[doc = "The unique ID for the list"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "Link to the [Landing Pages](#tag/Landing-Pages/paths/~1accounts~1{accountId}~1lists~1{listId}~1landing_pages/get) used by this list"]
    #[serde(default, skip_serializing)]
    pub landing_pages_collection_link: Option<String>,
    #[doc = "The name of the list.  This name may be changed by the customer at any time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ListName>,
    #[doc = "The link to the list type"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<String>,
    #[doc = "Scheduled [Broadcasts](#tag/Broadcasts/paths/~1accounts~1{accountId}~1lists~1{listId}~1broadcasts/get) belonging to this list"]
    #[serde(default, skip_serializing)]
    pub scheduled_broadcasts_link: Option<String>,
    #[doc = "Link to the Subscriber [Segments](#tag/Segments/paths/~1accounts~1{accountId}~1lists~1{listId}~1segments/get) used by this list"]
    #[serde(default, skip_serializing)]
    pub segments_collection_link: Option<String>,
    #[doc = "The link to this resource"]
    #[serde(default, skip_serializing)]
    pub self_link: Option<String>,
    #[doc = "Sent [Broadcasts](#tag/Broadcasts/paths/~1accounts~1{accountId}~1lists~1{listId}~1broadcasts/get) belonging to this list"]
    #[serde(default, skip_serializing)]
    pub sent_broadcasts_link: Option<String>,
    #[doc = "Link to the [Subscribers](#tag/Subscribers/paths/~1accounts~1{accountId}~1lists~1{listId}~1subscribers/get) belonging to this list"]
    #[serde(default, skip_serializing)]
    pub subscribers_collection_link: Option<String>,
    #[doc = "Number of subscribers where status = subscribed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_subscribed_subscribers: Option<u64>,
    #[doc = "Number of subscribers"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_subscribers: Option<u64>,
    #[doc = "Number of subscribers that were subscribed today"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_subscribers_subscribed_today: Option<u64>,
    #[doc = "Number of subscribers that were subscribed yesterday"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_subscribers_subscribed_yesterday: Option<u64>,
    #[doc = "Number of subscribers where status = unconfirmed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_unconfirmed_subscribers: Option<u64>,
    #[doc = "Number of subscribers where status = unsubscribed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_unsubscribed_subscribers: Option<u64>,
    #[doc = "Unique list ID of list.\n\nThe unique list ID is the automatically generated name for your list.\n\nIf you create a custom sign up form, you'll need to specify the unique list ID in the form's HTML code, ensuring that subscribers will be added to the list.\n\nThe unique list ID is also the first part of your autoresponder's e-mail address used by subscribers to sign up to your list.\n\nA subscriber can sign up to your list by sending an email to this address. (Though this method is rarely used anymore.)\n"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique_list_id: Option<ListUniqueListId>,
    #[doc = "The UUID (universally unique identifier) for the list"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "Public key that is used to validate web push notifications"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vapid_public_key: Option<String>,
    #[doc = "[Webform Split Tests](#tag/Webforms/paths/~1accounts~1{accountId}~1lists~1{listId}~1web_form_split_tests/get) defined for this list"]
    #[serde(default, skip_serializing)]
    pub web_form_split_tests_collection_link: Option<String>,
    #[doc = "[Webforms](#tag/Webforms/paths/~1accounts~1{accountId}~1lists~1{listId}~1web_forms/get) defined for this list."]
    #[serde(default, skip_serializing)]
    pub web_forms_collection_link: Option<String>,
}
#[doc = "`ListCampaigns`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct ListCampaigns {
    #[doc = "A list of campaign entries"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<Campaign>,
    #[doc = "A link to the next page of entries if more entries exist. This attribute is omitted from the collection if there are no more entries."]
    #[serde(default, skip_serializing)]
    pub next_collection_link: Option<String>,
    #[doc = "A link to the previous page of entries if any exist. This attribute is omitted from the collection if ws.start is 0."]
    #[serde(default, skip_serializing)]
    pub prev_collection_link: Option<String>,
    #[doc = "The starting offset for the page of entries to retrieve"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    #[doc = "The total number of entries"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_size: Option<i64>,
}
validated_string!(ListName, min=1, max=32);
validated_string!(ListUniqueListId, min=1, max=30);
#[doc = "`Lists`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct Lists {
    #[doc = "A list of list entries"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<List>,
    #[doc = "A link to the next page of entries if more entries exist. This attribute is omitted from the collection if there are no more entries."]
    #[serde(default, skip_serializing)]
    pub next_collection_link: Option<String>,
    #[doc = "A link to the previous page of entries if any exist. This attribute is omitted from the collection if ws.start is 0."]
    #[serde(default, skip_serializing)]
    pub prev_collection_link: Option<String>,
    #[doc = "A link that identifies the type of resource that this collection represents"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<String>,
    #[doc = "The starting offset for the page of entries to retrieve"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    #[doc = "The total number of entries"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_size: Option<u64>,
}
#[doc = "`MoveSubscriberRequestBody`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct MoveSubscriberRequestBody {
    #[doc = "If set to true, this will cause the move of a subscriber to fail if the custom fields from the origin list do not match (case insensitively) to the target list"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enforce_custom_field_mapping: Option<bool>,
    #[doc = "The sequence number of the last followup message sent to the subscriber.  This field determines the next followup message to be sent to the Subscriber.  When set to 0, the Subscriber will receive the 1st (autoresponse) Followup message.  Set the value of this field to 1001 if you do not want any Followups to be sent to this Subscriber."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_followup_message_number_sent: Option<i64>,
    #[doc = "The link to the destination [List](#tag/Lists/paths/~1accounts~1{accountId}~1lists~1{listId}/get)"]
    pub list_link: String,
    #[doc = "The method name - expecting \"move\""]
    #[serde(rename = "ws.op")]
    pub ws_op: MoveSubscriberRequestBodyWsOp,
}
string_enum! { pub enum MoveSubscriberRequestBodyWsOp { Move => "move" } }
validated_string!(OauthCallback);
validated_string!(OauthConsumerKey);
validated_string!(OauthNonce);
validated_string!(OauthSignature);
validated_string!(OauthSignatureMethod);
validated_string!(OauthTimestamp);
validated_string!(OauthToken);
validated_string!(OauthVersion);
#[doc = "`PatchAccountsListsCustomFieldsBody`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct PatchAccountsListsCustomFieldsBody {
    #[doc = "Whether the subscriber is allowed to update the custom field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_subscriber_updateable: Option<bool>,
    #[doc = "The name of the custom field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
validated_string!(PatchAccountsListsSubscribersSubscriberEmail, min=1, max=50);
#[doc = "`Pkce`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Pkce {
    #[doc = "The client ID of the application. Available on the <a href=\"https://labs.aweber.com/apps\" target=\"_blank\">My Apps Page</a>. <br>_Not required if `Authorization` header is set._"]
    pub client_id: String,
    #[doc = "The authorization code received from the authorization call."]
    pub code: String,
    #[doc = "As described in the <a href=\"https://tools.ietf.org/html/rfc7636#section-4.1\" target=\"_blank\">PKCE RFC</a>, this is a high-entropy cryptographic random string with a minimum length of 43 characters and a maximum length of 128 characters."]
    pub code_verifier: String,
    #[doc = "The grant type of the token."]
    pub grant_type: PkceGrantType,
}
string_enum! { pub enum PkceGrantType { AuthorizationCode => "authorization_code" } }
#[doc = "`PostAccountsListsBroadcastsCancelResponse`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct PostAccountsListsBroadcastsCancelResponse {
    #[doc = "The link to this resource."]
    #[serde(default, skip_serializing)]
    pub self_link: Option<String>,
}
#[doc = "`PostAccountsListsBroadcastsScheduleResponse`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct PostAccountsListsBroadcastsScheduleResponse {
    #[doc = "The link to the broadcast resource."]
    #[serde(default, skip_serializing)]
    pub self_link: Option<String>,
}
#[doc = "`PostAccountsListsCustomFieldsBody`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct PostAccountsListsCustomFieldsBody {
    #[doc = "The name of the custom field"]
    pub name: String,
    #[doc = "The method name - expecting \"create\""]
    #[serde(rename = "ws.op")]
    pub ws_op: PostAccountsListsCustomFieldsBodyWsOp,
}
string_enum! { pub enum PostAccountsListsCustomFieldsBodyWsOp { Create => "create" } }
#[doc = "`PostOauth2RevokeBody`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum PostOauth2RevokeBody {
    Confidential(Confidential),
    RevokePkce(RevokePkce),
}
impl From<Confidential> for PostOauth2RevokeBody {
    fn from(value: Confidential) -> Self {
        Self::Confidential(value)
    }
}
impl From<RevokePkce> for PostOauth2RevokeBody {
    fn from(value: RevokePkce) -> Self {
        Self::RevokePkce(value)
    }
}
#[doc = "`PostOauth2TokenBody`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum PostOauth2TokenBody {
    AuthCode(AuthCode),
    RefreshTokenConfidential(RefreshTokenConfidential),
    RefreshTokenPublic(RefreshTokenPublic),
    Pkce(Pkce),
}
impl From<AuthCode> for PostOauth2TokenBody {
    fn from(value: AuthCode) -> Self {
        Self::AuthCode(value)
    }
}
impl From<RefreshTokenConfidential> for PostOauth2TokenBody {
    fn from(value: RefreshTokenConfidential) -> Self {
        Self::RefreshTokenConfidential(value)
    }
}
impl From<RefreshTokenPublic> for PostOauth2TokenBody {
    fn from(value: RefreshTokenPublic) -> Self {
        Self::RefreshTokenPublic(value)
    }
}
impl From<Pkce> for PostOauth2TokenBody {
    fn from(value: Pkce) -> Self {
        Self::Pkce(value)
    }
}
#[doc = "`PostOauth2TokenResponse`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct PostOauth2TokenResponse {
    #[doc = "The access token, which will be used to represent this specific AWeber user's account and can be used to gain access to their data. It should be stored for later use, consider doing so in a database or some other local storage in your application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[doc = "The time in seconds in which the access token will expire."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<i64>,
    #[doc = "The refresh token, which will be used to refresh a user's access token. It should be stored for later use, consider doing so in a database or some other local storage in your application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
    #[doc = "The type of token the access token will be used as."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_type: Option<String>,
}
#[doc = "`PostOauthAccessTokenBody`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct PostOauthAccessTokenBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth_callback: Option<OauthCallback>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth_consumer_key: Option<OauthConsumerKey>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth_nonce: Option<OauthNonce>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth_signature: Option<OauthSignature>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth_signature_method: Option<OauthSignatureMethod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth_timestamp: Option<OauthTimestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth_token: Option<OauthToken>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth_version: Option<OauthVersion>,
}
#[doc = "`PostOauthAccessTokenResponse`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct PostOauthAccessTokenResponse {
    #[doc = "The access token, which will be used to represent this specific AWeber user's account and can be used to gain access to their data. It should be stored for later use, consider doing so in a database or some other local storage in your application."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth_token: Option<String>,
    #[doc = "This is a newly generated secret which will be used in the creation of the request's oauth_signature. This token secret is paired exclusively with the access token and does not expire. It should be stored for later use. Consider savng it in a database or some other local storage in your application. At this point, the oauth_token and oauth_token_secret that were generated in Step 1 as the request token have expired, and can not be used again."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth_token_secret: Option<bool>,
}
#[doc = "`PostOauthRequestTokenBody`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct PostOauthRequestTokenBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth_callback: Option<OauthCallback>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth_consumer_key: Option<OauthConsumerKey>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth_nonce: Option<OauthNonce>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth_signature: Option<OauthSignature>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth_signature_method: Option<OauthSignatureMethod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth_timestamp: Option<OauthTimestamp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth_token: Option<OauthToken>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth_version: Option<OauthVersion>,
}
#[doc = "`PostOauthRequestTokenResponse`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct PostOauthRequestTokenResponse {
    #[doc = "Whether the callback was accepted. If true, the result of the authorization step will be sent to this url."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth_callback_confirmed: Option<bool>,
    #[doc = "This is a newly generated request token that temporarily represents the user of the application. This will expire when an access token is created for this user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth_token: Option<String>,
}
#[doc = "`Purchase`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Purchase {
    #[doc = "The customer ad tracking field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ad_tracking: Option<PurchaseAdTracking>,
    #[doc = "Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html)."]
    pub currency: String,
    #[doc = "The custom fields specified on the subscriber"]
    #[serde(
        default,
        skip_serializing_if = "HashMap::is_empty"
    )]
    pub custom_fields:
        HashMap<String, Option<String>>,
    #[doc = "The subscriber's email address"]
    pub email: PurchaseEmail,
    #[doc = "A custom note associated with this specific tracked event"]
    pub event_note: String,
    #[doc = "The timestamp of when the event occurred"]
    pub event_time: String,
    #[doc = "The subscriber's IP address. This must be a public IP address."]
    pub ip_address: PurchaseIpAddress,
    #[doc = "Miscellaneous notes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub misc_notes: Option<PurchaseMiscNotes>,
    #[doc = "The subscriber's name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<PurchaseName>,
    #[doc = "A custom description for the page or event"]
    pub product_name: String,
    #[doc = "A list of tags added to the subscriber"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<PurchaseTagsItem>,
    #[doc = "The URL for the tracked event"]
    pub url: String,
    pub value: f64,
    #[doc = "The sales tracking url profile for the web page"]
    pub vendor: String,
}
validated_string!(PurchaseAdTracking, min=1, max=20);
validated_string!(PurchaseEmail, min=1, max=50);
validated_string!(PurchaseIpAddress, min=1, max=60);
validated_string!(PurchaseMiscNotes, max=60);
validated_string!(PurchaseName, min=1, max=60);
validated_string!(PurchaseTagsItem, min=1);
#[doc = "This will create an access token using a `refresh_token`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct RefreshTokenConfidential {
    #[doc = "The client ID of the application. Available on the <a href=\"https://labs.aweber.com/apps\" target=\"_blank\">My Apps Page</a>. <br>_Not required if `Authorization` header is set._"]
    pub client_id: String,
    #[doc = "The client secret of the application. Available on the <a href=\"https://labs.aweber.com/apps\" target=\"_blank\">My Apps Page</a>. <br>_Not required if `Authorization` header is set._"]
    pub client_secret: String,
    #[doc = "The grant type of the token."]
    pub grant_type: RefreshTokenConfidentialGrantType,
    #[doc = "The refresh token associated with the expired access token."]
    pub refresh_token: String,
}
string_enum! { pub enum RefreshTokenConfidentialGrantType { RefreshToken => "refresh_token" } }
#[doc = "`RefreshTokenPublic`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct RefreshTokenPublic {
    #[doc = "The client ID of the application. Available on the <a href=\"https://labs.aweber.com/apps\" target=\"_blank\">My Apps Page</a>. <br>_Not required if `Authorization` header is set._"]
    pub client_id: String,
    #[doc = "The grant type of the token."]
    pub grant_type: RefreshTokenPublicGrantType,
    #[doc = "The refresh token associated with the expired access token."]
    pub refresh_token: String,
}
string_enum! { pub enum RefreshTokenPublicGrantType { RefreshToken => "refresh_token" } }
#[doc = "`RevokePkce`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct RevokePkce {
    #[doc = "The client ID of the application. Available on the <a href=\"https://labs.aweber.com/apps\" target=\"_blank\">My Apps Page</a>. <br>_Not required if `Authorization` header is set._"]
    pub client_id: String,
    #[doc = "The access token or refresh token to revoke"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    #[doc = "Type of token given in the token parameter. Valid values are <ul><li>access_token<li>refresh_token</ul><br>If not specified, search for both kinds of tokens."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_type_hint: Option<RevokePkceTokenTypeHint>,
}
string_enum! { pub enum RevokePkceTokenTypeHint { AccessToken => "access_token", RefreshToken => "refresh_token" } }
#[doc = "`ScheduleBroadcast`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ScheduleBroadcast {
    #[doc = "Scheduled time for sending broadcast message, ISO-8601 formatted."]
    #[serde(with = "flexible_datetime")]
    pub scheduled_for: ::chrono::DateTime<::chrono::offset::Utc>,
}
#[doc = "`Segment`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct Segment {
    #[doc = "The unique ID for the segment"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "Whether the segment is part of a split test.\n\nMust be false to use for scheduling a broadcast.\n\nSegments that were created automatically for a split test in the control panel are set to true.\n"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_split_test: Option<bool>,
    #[doc = "The segment name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The link to the segment type"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<String>,
    #[doc = "The link to this resource"]
    #[serde(default, skip_serializing)]
    pub self_link: Option<String>,
}
#[doc = "`Segments`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct Segments {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<Segment>,
    #[doc = "A link to the next page of entries if more entries exist. This attribute is omitted from the collection if there are no more entries."]
    #[serde(default, skip_serializing)]
    pub next_collection_link: Option<String>,
    #[doc = "A link to the previous page of entries if any exist. This attribute is omitted from the collection if ws.start is 0."]
    #[serde(default, skip_serializing)]
    pub prev_collection_link: Option<String>,
    #[doc = "The link to the segment page type"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<String>,
    #[doc = "The starting offset for the page of entries to retrieve"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    #[doc = "The total number of entries"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_size: Option<u64>,
}
#[doc = "`Stat`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct Stat {
    #[doc = "A description of the statistic"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Statistic's ID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "A link that identifies the type of resource that this represents"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<StatResourceTypeLink>,
    #[doc = "The link to this resource"]
    #[serde(default, skip_serializing)]
    pub self_link: Option<String>,
    #[doc = "The statistic's value\n\nThe datatype of the value attribute may be different for each Stat.\nStats can be integer_stat, decimal_stat, or list_stat. Evaluate the\nresource_type_link attribute if you must programmatically handle\ndifferent datatypes in a special way.\n\nBelow is a list of the statistics ID and return type\n\n>\n> __Aggregate Statistics__\n>\n> * total_clicks - integer_stat\n> * unique_clicks - integer_stat\n> * total_opens - integer_stat\n> * unique_opens - integer_stat\n> * total_sales - integer_stat\n> * total_sales_dollars - decimal_stat\n> * total_unsubscribed - integer_stat\n\n> __Time Related Statistics__\n>\n> * hourly_opens - list_stat\n> * hourly_clicks - list_stat\n> * hourly_webhits - list_stat\n> * hourly_sales - list_stat\n> * hourly_unsubscribes - list_stat\n> * daily_opens - list_stat\n> * daily_clicks - list_stat\n> * daily_webhits - list_stat\n> * daily_sales - list_stat\n> * daily_unsubscribes - list_stat\n\n> __Top 10 URL Statistics__\n>\n> * clicks_by_link - list_stat\n> * webhits_by_link - list_stat\n\n>  __Top 10 Subscriber Statistics__ (Requires access to subscriber data)\n>\n> * opens_by_subscriber - list_stat\n> * sales_by_subscriber - list_stat\n"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<StatValue>,
}
string_enum! { pub enum StatResourceTypeLink { IntegerStat => "integer_stat", DecimalStat => "decimal_stat", ListStat => "list_stat" } }
#[doc = "The statistic's value\n\nThe datatype of the value attribute may be different for each Stat.\nStats can be integer_stat, decimal_stat, or list_stat. Evaluate the\nresource_type_link attribute if you must programmatically handle\ndifferent datatypes in a special way.\n\nBelow is a list of the statistics ID and return type\n\n>\n> __Aggregate Statistics__\n>\n> * total_clicks - integer_stat\n> * unique_clicks - integer_stat\n> * total_opens - integer_stat\n> * unique_opens - integer_stat\n> * total_sales - integer_stat\n> * total_sales_dollars - decimal_stat\n> * total_unsubscribed - integer_stat\n\n> __Time Related Statistics__\n>\n> * hourly_opens - list_stat\n> * hourly_clicks - list_stat\n> * hourly_webhits - list_stat\n> * hourly_sales - list_stat\n> * hourly_unsubscribes - list_stat\n> * daily_opens - list_stat\n> * daily_clicks - list_stat\n> * daily_webhits - list_stat\n> * daily_sales - list_stat\n> * daily_unsubscribes - list_stat\n\n> __Top 10 URL Statistics__\n>\n> * clicks_by_link - list_stat\n> * webhits_by_link - list_stat\n\n>  __Top 10 Subscriber Statistics__ (Requires access to subscriber data)\n>\n> * opens_by_subscriber - list_stat\n> * sales_by_subscriber - list_stat\n"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum StatValue {
    Integer(i64),
    Number(f64),
    String(String),
}
impl std::fmt::Display for StatValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Integer(x) => x.fmt(f),
            Self::Number(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl From<i64> for StatValue {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
impl From<f64> for StatValue {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[doc = "`Stats`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct Stats {
    #[doc = "A list of stats entries"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<Stat>,
    #[doc = "A link to the next page of entries if more entries exist. This attribute is omitted from the collection if there are no more entries."]
    #[serde(default, skip_serializing)]
    pub next_collection_link: Option<String>,
    #[doc = "A link to the previous page of entries if any exist. This attribute is omitted from the collection if ws.start is 0."]
    #[serde(default, skip_serializing)]
    pub prev_collection_link: Option<String>,
    #[doc = "A link that identifies the type of resource that this represents"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<String>,
    #[doc = "The starting offset for the page of entries to retrieve"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    #[doc = "The total number of entries"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_size: Option<i64>,
}
#[doc = "`Subscriber`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct Subscriber {
    #[doc = "The customer ad tracking field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ad_tracking: Option<String>,
    #[doc = "The subscriber's area code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area_code: Option<i64>,
    #[doc = "The subscriber's address city"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[doc = "The subscriber's address country"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[doc = "The custom fields specified on the subscriber"]
    #[serde(
        default,
        skip_serializing_if = "HashMap::is_empty"
    )]
    pub custom_fields:
        HashMap<String, Option<String>>,
    #[doc = "The subscriber's designated market area code (USA and canada only)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dma_code: Option<i64>,
    #[doc = "The subscriber's email address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "The lead ID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    #[doc = "The subscriber's IP address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[doc = "A value indication if the subscriber's email address confirmed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
    #[doc = "The sequence number of the last followup message sent to the subscriber.  If this value is set to 1001, then the subscriber will not receive followup messages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_followup_message_number_sent: Option<i64>,
    #[doc = "The last followup message sent to the subscriber"]
    #[serde(default, skip_serializing_if = "Option::is_none", with = "flexible_datetime::option")]
    pub last_followup_sent_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "A link to the [Last Follow Up Message](#tag/Campaigns/paths/~1accounts~1{accountId}~1lists~1{listId}~1campaigns~1{campaignType}{campaignId}/get) the subscriber was sent"]
    #[serde(default, skip_serializing)]
    pub last_followup_sent_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[doc = "Miscellaneous notes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub misc_notes: Option<String>,
    #[doc = "The subscriber's name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The subscriber's address postal code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[doc = "The subscriber's address stage or region abbreviation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[doc = "A link that identifies the type of resource that this collection represents"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<SubscriberResourceTypeLink>,
    #[doc = "The link to this resource"]
    #[serde(default, skip_serializing)]
    pub self_link: Option<String>,
    #[doc = "The subscriber's status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SubscriberStatus>,
    #[doc = "The timestamp for when the subscriber subscribed"]
    #[serde(default, skip_serializing_if = "Option::is_none", with = "flexible_datetime::option")]
    pub subscribed_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "The method by which the subscriber was subscribed.\n\n| Method    | Description                              |\n| ------    | -----------                              |\n| api       | subscribed via an API integration        |\n| email     | subscriber emailed to list to opt-in     |\n| import    | subscriber was imported by the customer  |\n| webform   | subscriber subscribed via a web form     |\n"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_method: Option<SubscriberSubscriptionMethod>,
    #[doc = "The webform url from which the subscriber subscribed from"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_url: Option<String>,
    #[doc = "A list of tags added to the subscriber"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[doc = "The method that describes how the subscriber unsubscribed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unsubscribe_method: Option<SubscriberUnsubscribeMethod>,
    #[doc = "The timestamp for when the subscriber unsubscribed"]
    #[serde(default, skip_serializing_if = "Option::is_none", with = "flexible_datetime::option")]
    pub unsubscribed_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "The timestamp for when the subscriber confirmed their email address"]
    #[serde(default, skip_serializing_if = "Option::is_none", with = "flexible_datetime::option")]
    pub verified_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
}
#[doc = "`SubscriberFind`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct SubscriberFind {
    #[doc = "The customer ad tracking field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ad_tracking: Option<SubscriberFindAdTracking>,
    #[doc = "The subscriber's area code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub area_code: Option<i64>,
    #[doc = "The subscriber's address city"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<SubscriberFindCity>,
    #[doc = "The subscriber's address country"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<SubscriberFindCountry>,
    #[doc = "The custom fields specified on the subscriber"]
    #[serde(
        default,
        skip_serializing_if = "HashMap::is_empty"
    )]
    pub custom_fields:
        HashMap<String, Option<String>>,
    #[doc = "The subscriber's designated market area code (usa and canada only)"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dma_code: Option<i64>,
    #[doc = "The subscriber's email address"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<SubscriberFindEmail>,
    #[doc = "The lead ID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,
    #[doc = "The subscriber's IP address"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<SubscriberFindIpAddress>,
    #[doc = "A value indication if the subscriber's email address confirmed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
    #[doc = "The sequence number of the last followup message sent to the subscriber"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_followup_message_number_sent: Option<i64>,
    #[doc = "The last followup message sent to the subscriber"]
    #[serde(default, skip_serializing_if = "Option::is_none", with = "flexible_datetime::option")]
    pub last_followup_sent_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "A link to the [Last Follow Up Message](#tag/Campaigns/paths/~1accounts~1{accountId}~1lists~1{listId}~1campaigns~1{campaignType}{campaignId}/get) the subscriber was sent"]
    #[serde(default, skip_serializing)]
    pub last_followup_sent_link: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[doc = "A link to the [List](#tag/Lists/paths/~1accounts~1{accountId}~1lists~1{listId}/get) the subscriber is on"]
    #[serde(default, skip_serializing)]
    pub list_link: Option<String>,
    #[doc = "List name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list_name: Option<SubscriberFindListName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[doc = "Miscellaneous notes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub misc_notes: Option<SubscriberFindMiscNotes>,
    #[doc = "The subscriber's name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<SubscriberFindName>,
    #[doc = "The subscriber's address postal code"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<SubscriberFindPostalCode>,
    #[doc = "The subscriber's address stage or region abbreviation"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<SubscriberFindRegion>,
    #[doc = "A link that identifies the type of resource that this collection represents"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<String>,
    #[doc = "The link to this resource"]
    #[serde(default, skip_serializing)]
    pub self_link: Option<String>,
    #[doc = "The subscriber's status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<SubscriberFindStatus>,
    #[doc = "The timestamp for when the subscriber subscribed"]
    #[serde(default, skip_serializing_if = "Option::is_none", with = "flexible_datetime::option")]
    pub subscribed_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "The method by which the subscriber was subscribed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_method: Option<SubscriberFindSubscriptionMethod>,
    #[doc = "The webform url from which the subscriber subscribed from"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subscription_url: Option<String>,
    #[doc = "A list of tags added to the subscriber"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<SubscriberFindTagsItem>,
    #[doc = "The method that describes how the subscriber unsubscribed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unsubscribe_method: Option<SubscriberFindUnsubscribeMethod>,
    #[doc = "The timestamp for when the subscriber unsubscribed"]
    #[serde(default, skip_serializing_if = "Option::is_none", with = "flexible_datetime::option")]
    pub unsubscribed_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
    #[doc = "The timestamp for when the subscriber confirmed their email address"]
    #[serde(default, skip_serializing_if = "Option::is_none", with = "flexible_datetime::option")]
    pub verified_at: Option<::chrono::DateTime<::chrono::offset::Utc>>,
}
validated_string!(SubscriberFindAdTracking, min=1, max=20);
validated_string!(SubscriberFindCity, min=1, max=100);
validated_string!(SubscriberFindCountry, min=1, max=100);
validated_string!(SubscriberFindEmail, min=1, max=50);
validated_string!(SubscriberFindIpAddress, min=1, max=60);
validated_string!(SubscriberFindListName, min=1, max=32);
validated_string!(SubscriberFindMiscNotes, max=60);
validated_string!(SubscriberFindName, min=1, max=60);
validated_string!(SubscriberFindPostalCode, min=1, max=100);
validated_string!(SubscriberFindRegion, min=1, max=100);
string_enum! { pub enum SubscriberFindStatus { Subscribed => "subscribed", Unsubscribed => "unsubscribed", Unconfirmed => "unconfirmed" } }
string_enum! { pub enum SubscriberFindSubscriptionMethod { Api => "api", Email => "email", Import => "import", Webform => "webform" } }
validated_string!(SubscriberFindTagsItem, min=1);
string_enum! { pub enum SubscriberFindUnsubscribeMethod { UnsubscribeLink => "unsubscribe link", CustomerCp => "customer cp", Undeliverable => "undeliverable", ApiUnsubscribe => "api: unsubscribe", ApiMove => "api: move" } }
#[doc = "`SubscriberGetActivity`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct SubscriberGetActivity {
    #[doc = "The list of activity entries.  The contents of the entry varies, but will always have the following fields listed"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<Activity>,
    #[doc = "A link to the next page of entries if more entries exist. This attribute is omitted from the collection if there are no more entries."]
    #[serde(default, skip_serializing)]
    pub next_collection_link: Option<String>,
    #[doc = "A link to the previous page of entries if previous entries exist. This attribute is omitted if this is the first page."]
    #[serde(default, skip_serializing)]
    pub prev_collection_link: Option<String>,
    #[doc = "The starting offset for the page of entries to retrieve"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    #[doc = "A link to check the total number of entries found for this resource"]
    #[serde(default, skip_serializing)]
    pub total_size_link: Option<String>,
}
string_enum! { pub enum SubscriberResourceTypeLink { HttpsApiAweberCom10Subscriber => "https://api.aweber.com/1.0/#subscriber" } }
string_enum! { pub enum SubscriberStatus { Subscribed => "subscribed", Unsubscribed => "unsubscribed", Unconfirmed => "unconfirmed" } }
string_enum! { pub enum SubscriberSubscriptionMethod { Api => "api", Email => "email", Import => "import", Webform => "webform" } }
string_enum! { pub enum SubscriberUnsubscribeMethod { UnsubscribeLink => "unsubscribe link", CustomerCp => "customer cp", Undeliverable => "undeliverable", ApiUnsubscribe => "api: unsubscribe", ApiMove => "api: move" } }
#[doc = "`Subscribers`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct Subscribers {
    #[doc = "The list of subscriber entries"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<Subscriber>,
    #[doc = "A link to the next page of entries if more entries exist. This attribute is omitted from the collection if there are no more entries."]
    #[serde(default, skip_serializing)]
    pub next_collection_link: Option<String>,
    #[doc = "A link to the previous page of entries if previous entries exist. This attribute is omitted if this is the first page."]
    #[serde(default, skip_serializing)]
    pub prev_collection_link: Option<String>,
    #[doc = "A link that identifies the type of resource that this collection represents"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<String>,
    #[doc = "A link to the current page of entries."]
    #[serde(default, skip_serializing)]
    pub self_link: Option<String>,
    #[doc = "The starting offset for the page of entries to retrieve"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    #[doc = "The total number of entries"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_size: Option<u64>,
}
#[doc = "`Token`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Token {
    #[doc = "The client ID of the application. Available on the <a href=\"https://labs.aweber.com/apps\" target=\"_blank\">My Apps Page</a>. <br>_Not required if `Authorization` header is set._"]
    pub client_id: String,
    #[doc = "The client secret of the application. Available on the <a href=\"https://labs.aweber.com/apps\" target=\"_blank\">My Apps Page</a>. <br>_Not required if `Authorization` header is set._"]
    pub client_secret: String,
}
#[doc = "`UpdateBroadcast`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct UpdateBroadcast {
    #[doc = "<b>[Please read <a href=\"https://help.aweber.com/hc/en-us/articles/360025741194\" target=\"_blank\">our KB article</a> before using this field.]</b><br>The content of the message in AMP format."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_amp: Option<String>,
    #[doc = "The content of the message in html format. If body_text is not provided, it will be auto-generated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_html: Option<String>,
    #[doc = "The content of the message in plain text, used when HTML is not supported. If body_html is not provided, the broadcast will be sent using only the body_text."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_text: Option<String>,
    #[doc = "Enables links in the email message to be tracked."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub click_tracking_enabled: Option<bool>,
    #[doc = "JSON encoded list of [Lists](#tag/Lists) URLs to exclude in the delivery of this broadcast.<br> Use the `self_link` of the list here - e.g. `https://api.aweber.com/1.0/accounts/<account_id>/lists/<list_id>`. If updated, this value will replace the existing excluded_lists."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude_lists: Option<String>,
    #[doc = "URL to the [Facebook broadcast integration](#tag/Integrations) to use for this broadcast. When the broadcast is sent, the subject of the broadcast will be posted to this Facebook integration  - e.g., `https://api.aweber.com/1.0/accounts/<account_id>/integrations/<integration_id>`"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub facebook_integration: Option<String>,
    #[doc = "JSON encoded list of [Lists](#tag/Lists) URLs to include in the delivery of this broadcast.<br> Use the `self_link` of the list here - e.g. `https://api.aweber.com/1.0/accounts/<account_id>/lists/<list_id>`. If updated, this value will replace the existing included_lists."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_lists: Option<String>,
    #[doc = "Whether the broadcast enabled sharing via an archive url."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_archived: Option<bool>,
    #[doc = "If true, notify when stats are available on a sent broadcast message, defaults to true."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notify_on_send: Option<bool>,
    #[doc = "URL to the [Segment](#tag/Segments) to send this broadcast to.  Use the `self_link` of the segment here - e.g. `https://api.aweber.com/1.0/accounts/<account_id>/lists/<list_id>/segments/<segment_id>`. If not specified, the broadcast will be sent to the “Active Subscribers” segment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segment_link: Option<String>,
    #[doc = "The broadcast subject line. Subject must not be empty nor contain only whitespace."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[doc = "URL to the [Twitter broadcast integration](#tag/Integrations) to use for this broadcast. When the broadcast is sent, the subject of the broadcast will be tweeted - e.g., `https://api.aweber.com/1.0/accounts/<account_id>/integrations/<integration_id>`"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub twitter_integration: Option<String>,
}
#[doc = "`UpdateSubscriberRequestBody`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct UpdateSubscriberRequestBody {
    #[doc = "The customer ad tracking field"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ad_tracking: Option<UpdateSubscriberRequestBodyAdTracking>,
    #[doc = "The custom fields specified on the subscriber.  Custom fields are represented as a sub-object where only the **values** can be modified. <br><br> **Note:** If updating custom field values, **all** fields must be included in the request. If all fields are not included, the omitted fields will be set to *null*. <br><br> In order to modify custom field values:\n- Retrieve the custom fields and values for the subscriber.\n- Add all fields and their values to the PATCH request.\n- Modify the value **only** for the fields you wish to update.\n- Submit the request containing all the custom fields and their respective values."]
    #[serde(
        default,
        skip_serializing_if = "HashMap::is_empty"
    )]
    pub custom_fields: HashMap<
        String,
        Option<UpdateSubscriberRequestBodyCustomFieldsValue>,
    >,
    #[doc = "The subscriber's email address"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[doc = "The sequence number of the last followup message sent to the subscriber.  This field determines the next followup message to be sent to the Subscriber.  When set to 0, the Subscriber will receive the 1st (autoresponse) Followup message.  Set the value of this field to 1001 if you do not want any Followups to be sent to this Subscriber."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_followup_message_number_sent: Option<i64>,
    #[doc = "Miscellaneous notes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub misc_notes: Option<String>,
    #[doc = "The subscriber's name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<UpdateSubscriberRequestBodyName>,
    #[doc = "The subscriber's status.<br> **Note** you cannot set a subscriber's status to \"unconfirmed\"."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<UpdateSubscriberRequestBodyStatus>,
    #[doc = "If this parameter is present and set to `true`, then custom field names are matched case sensitively.  Enabling this option also causes the operation to fail if a custom field is included that is not defined for the list."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strict_custom_fields:
        Option<UpdateSubscriberRequestBodyStrictCustomFields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<UpdateSubscriberRequestBodyTags>,
}
validated_string!(UpdateSubscriberRequestBodyAdTracking, min=1, max=20);
validated_string!(UpdateSubscriberRequestBodyCustomFieldsValue, min=1);
validated_string!(UpdateSubscriberRequestBodyName, min=1, max=60);
string_enum! { pub enum UpdateSubscriberRequestBodyStatus { Subscribed => "subscribed", Unsubscribed => "unsubscribed" } }
string_enum! { pub enum UpdateSubscriberRequestBodyStrictCustomFields { True => "true", False => "false" } }
#[doc = "An object with the keys \"add\" and/or \"remove\" and values of lists of tags"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct UpdateSubscriberRequestBodyTags {
    #[doc = "List of tags to add to the subscriber"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub add: Vec<String>,
    #[doc = "List of tags to remove from the subscriber"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remove: Vec<String>,
}
#[doc = "`WebFormSplitTests`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct WebFormSplitTests {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<WebformSplitTest>,
    #[doc = "A link to the next page of entries if more entries exist. This attribute is omitted from the collection if there are no more entries."]
    #[serde(default, skip_serializing)]
    pub next_collection_link: Option<String>,
    #[doc = "A link to the previous page of entries if any exist. This attribute is omitted from the collection if ws.start is 0."]
    #[serde(default, skip_serializing)]
    pub prev_collection_link: Option<String>,
    #[doc = "The link to the event type"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<String>,
    #[doc = "The starting offset for the page of entries to retrieve"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    #[doc = "The total number of entries"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_size: Option<u64>,
}
#[doc = "`Webform`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct Webform {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversion_percentage: Option<f32>,
    #[doc = "The webform HTML source URL"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html_source_link: Option<String>,
    #[doc = "The ETag HTTP header"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_etag: Option<String>,
    #[doc = "The unique ID for the webform activity"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "Whether the webform is active or not"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[doc = "The webform javascript source URL"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub javascript_source_link: Option<String>,
    #[doc = "The webform name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The link to the webform type"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<String>,
    #[doc = "The link to this resource"]
    #[serde(default, skip_serializing)]
    pub self_link: Option<String>,
    #[doc = "A list of tags added to the webform"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<WebformTagsItem>,
    #[doc = "The total number of times the webform is displayed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_displays: Option<i64>,
    #[doc = "The total number of submissions received by the webform"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_submissions: Option<i64>,
    #[doc = "The total number of unique times the webform is displayed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_unique_displays: Option<i64>,
    #[doc = "The webform display type"]
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_: Option<WebformType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique_conversion_percentage: Option<f32>,
}
#[doc = "`WebformSplitTest`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct WebformSplitTest {
    #[doc = "The webform [Split Test Components](#tag/Webforms/paths/~1accounts~1{accountId}~1lists~1{listId}~1web_form_split_tests~1{splitTestId}~1components/get) URL"]
    #[serde(default, skip_serializing)]
    pub components_collection_link: Option<String>,
    #[doc = "The ETag HTTP header"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_etag: Option<String>,
    #[doc = "The unique ID for the webform split test"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[doc = "Whether the split test is active or not"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[doc = "The webform javascript source URL"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub javascript_source_link: Option<String>,
    #[doc = "The split test name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The link to the split test type"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<String>,
    #[doc = "The link to this resource"]
    #[serde(default, skip_serializing)]
    pub self_link: Option<String>,
}
#[doc = "`WebformSplitTestComponent`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct WebformSplitTestComponent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conversion_percentage: Option<f32>,
    #[doc = "The webform HTML source URL"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub html_source_link: Option<String>,
    #[doc = "The ETag HTTP header"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_etag: Option<String>,
    #[doc = "The unique ID for the webform component"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Whether the webform is active or not"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[doc = "The webform javascript source URL"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub javascript_source_link: Option<String>,
    #[doc = "The webform name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The link to the webform type"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<String>,
    #[doc = "The link to this resource"]
    #[serde(default, skip_serializing)]
    pub self_link: Option<String>,
    #[doc = "A list of tags added to the webform component"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<WebformSplitTestComponentTagsItem>,
    #[doc = "The total number of times the webform is displayed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_displays: Option<u64>,
    #[doc = "The total number of submissions received by the webform"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_submissions: Option<u64>,
    #[doc = "The total number of unique times the webform is displayed"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_unique_displays: Option<u64>,
    #[doc = "The webform display type"]
    #[serde(
        rename = "type",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub type_: Option<WebformSplitTestComponentType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique_conversion_percentage: Option<f32>,
    #[doc = "The link to [Webform](#tag/Webforms/paths/~1accounts~1{accountId}~1lists~1{listId}~1web_forms~1{webformId}/get)"]
    #[serde(default, skip_serializing)]
    pub web_form_link: Option<String>,
    #[doc = "The relative display weight"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<u64>,
}
validated_string!(WebformSplitTestComponentTagsItem, min=1);
string_enum! { pub enum WebformSplitTestComponentType { Exitpopup => "exitpopup", Inline => "inline", Lightbox => "lightbox", Popover => "popover", Popunder => "popunder", Popup => "popup", Styled => "styled" } }
#[doc = "`WebformSplitTestComponents`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct WebformSplitTestComponents {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<WebformSplitTestComponent>,
    #[doc = "A link to the next page of entries if more entries exist. This attribute is omitted from the collection if there are no more entries."]
    #[serde(default, skip_serializing)]
    pub next_collection_link: Option<String>,
    #[doc = "A link to the previous page of entries if any exist. This attribute is omitted from the collection if ws.start is 0."]
    #[serde(default, skip_serializing)]
    pub prev_collection_link: Option<String>,
    #[doc = "The link to the Component type"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<String>,
    #[doc = "The starting offset for the page of entries to retrieve"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    #[doc = "The total number of entries"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_size: Option<u64>,
}
validated_string!(WebformTagsItem, min=1);
string_enum! { pub enum WebformType { Exitpopup => "exitpopup", Inline => "inline", Lightbox => "lightbox", Popover => "popover", Popunder => "popunder", Popup => "popup", Styled => "styled" } }
#[doc = "`Webforms`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct Webforms {
    #[doc = "A list of webform entries"]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<Webform>,
    #[doc = "A link to the next page of entries if more entries exist. This attribute is omitted from the collection if there are no more entries."]
    #[serde(default, skip_serializing)]
    pub next_collection_link: Option<String>,
    #[doc = "A link to the previous page of entries if any exist. This attribute is omitted from the collection if ws.start is 0."]
    #[serde(default, skip_serializing)]
    pub prev_collection_link: Option<String>,
    #[doc = "A link that identifies the type of resource that this collection represents"]
    #[serde(default, skip_serializing)]
    pub resource_type_link: Option<String>,
    #[doc = "The starting offset for the page of entries to retrieve"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    #[doc = "The total number of entries"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_size: Option<u64>,
}
#[doc = r" Generation of default values for serde."]
pub mod defaults {
    pub(super) fn default_bool<const V: bool>() -> bool {
        V
    }
}

/// Flexible datetime deserialization that handles both timezone-aware
/// ("2025-11-21T10:31:53.202963-05:00") and naive ("2025-11-21T10:56:38") formats.
mod flexible_datetime {
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&dt.to_rfc3339())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        // Try full RFC 3339 / ISO 8601 with timezone first
        if let Ok(dt) = DateTime::parse_from_rfc3339(&s) {
            return Ok(dt.with_timezone(&Utc));
        }
        // Try with timezone offset but without T separator or other formats
        if let Ok(dt) = s.parse::<DateTime<Utc>>() {
            return Ok(dt);
        }
        // Fall back to naive datetime (no timezone), assume UTC
        NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S")
            .or_else(|_| NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S%.f"))
            .map(|ndt| ndt.and_utc())
            .map_err(serde::de::Error::custom)
    }

    pub mod option {
        use chrono::{DateTime, NaiveDateTime, Utc};
        use serde::{self, Deserialize, Deserializer, Serializer};

        pub fn serialize<S>(opt: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match opt {
                Some(dt) => super::serialize(dt, serializer),
                None => serializer.serialize_none(),
            }
        }

        pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
        where
            D: Deserializer<'de>,
        {
            let opt: Option<String> = Option::deserialize(deserializer)?;
            match opt {
                Some(ref s) if s.is_empty() => Ok(None),
                Some(s) => {
                    if let Ok(dt) = DateTime::parse_from_rfc3339(&s) {
                        return Ok(Some(dt.with_timezone(&Utc)));
                    }
                    if let Ok(dt) = s.parse::<DateTime<Utc>>() {
                        return Ok(Some(dt));
                    }
                    NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S")
                        .or_else(|_| NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S%.f"))
                        .map(|ndt| Some(ndt.and_utc()))
                        .map_err(serde::de::Error::custom)
                }
                None => Ok(None),
            }
        }
    }
}

pub trait PaginatedCollection {
    type Item: serde::Serialize;
    fn take_entries(&mut self) -> Vec<Self::Item>;
    fn next_collection_link(&self) -> Option<&str>;
}

macro_rules! impl_paginated_collection {
    ($collection:ty, $item:ty) => {
        impl PaginatedCollection for $collection {
            type Item = $item;
            fn take_entries(&mut self) -> Vec<$item> { std::mem::take(&mut self.entries) }
            fn next_collection_link(&self) -> Option<&str> { self.next_collection_link.as_deref() }
        }
    };
}

impl_paginated_collection!(Accounts, Account);
impl_paginated_collection!(BroadcastClicks, BroadcastClicksEntriesItem);
impl_paginated_collection!(BroadcastClicksDetailed, BroadcastClicksDetailedEntriesItem);
impl_paginated_collection!(BroadcastOpens, BroadcastOpensEntriesItem);
impl_paginated_collection!(Broadcasts, BroadcastsEntriesItem);
impl_paginated_collection!(CustomFields, CustomField);
impl_paginated_collection!(FindCampaigns, Campaign);
impl_paginated_collection!(FindSubscribers, SubscriberFind);
impl_paginated_collection!(FindSubscribersAccount, SubscriberFind);
impl_paginated_collection!(Integrations, Integration);
impl_paginated_collection!(LandingPages, LandingPageNoContent);
impl_paginated_collection!(ListCampaigns, Campaign);
impl_paginated_collection!(Lists, List);
impl_paginated_collection!(Segments, Segment);
impl_paginated_collection!(Stats, Stat);
impl_paginated_collection!(SubscriberGetActivity, Activity);
impl_paginated_collection!(Subscribers, Subscriber);
impl_paginated_collection!(WebFormSplitTests, WebformSplitTest);
impl_paginated_collection!(WebformSplitTestComponents, WebformSplitTestComponent);
impl_paginated_collection!(Webforms, Webform);
