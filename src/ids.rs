const MESSAGE_ID_DIGITS: usize = 24;

macro_rules! uuid_id {
    ($name:ident, $error:ident, $noun:expr, $doc:expr, $error_doc:expr) => {
        #[doc = $doc]
        #[derive(serde::Serialize, Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        #[serde(transparent)]
        pub struct $name(uuid::Uuid);

        impl std::str::FromStr for $name {
            type Err = $error;

            fn from_str(text: &str) -> Result<$name, $error> {
                match uuid::Uuid::parse_str(text) {
                    Ok(parsed) => Ok($name(parsed)),
                    Err(_) => Err($error {
                        rejected: text.to_string(),
                    }),
                }
            }
        }

        impl std::convert::TryFrom<&str> for $name {
            type Error = $error;

            fn try_from(text: &str) -> Result<$name, $error> {
                text.parse()
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D: serde::Deserializer<'de>>(
                deserializer: D,
            ) -> Result<$name, D::Error> {
                let text = <std::string::String as serde::Deserialize>::deserialize(deserializer)?;
                text.parse().map_err(serde::de::Error::custom)
            }
        }

        #[doc = $error_doc]
        #[derive(Clone, Debug)]
        pub struct $error {
            rejected: std::string::String,
        }

        impl std::fmt::Display for $error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "'{}' is not {}", self.rejected, $noun)
            }
        }

        impl std::error::Error for $error {}
    };
}

uuid_id!(
    WorkflowId,
    WorkflowIdError,
    "a workflow id",
    "A workflow's id. The sharing code is this same id.",
    "Text that is not the id it was read as."
);
uuid_id!(
    ListUid,
    ListUidError,
    "a list uid",
    "A list's uid, as campaign service names a workflow's parent.",
    "Text that is not the id it was read as."
);
uuid_id!(
    AccountUid,
    AccountUidError,
    "an account uid",
    "An account's uid, as the internal services name an owner.",
    "Text that is not the id it was read as."
);
uuid_id!(
    RuleId,
    RuleIdError,
    "a rule id",
    "One ruleset rule's id. A step is its anchor rule's id.",
    "Text that is not the id it was read as."
);
uuid_id!(
    BranchId,
    BranchIdError,
    "a branch id",
    "A set-branch branch's id. Its shape on the wire is at\n`research/campaign-ruleset-wire-format.md § Split path (set-branch)`.",
    "Text that is not the id it was read as."
);

#[cfg(feature = "workflows")]
impl RuleId {
    pub(crate) fn new() -> RuleId {
        RuleId(uuid::Uuid::new_v4())
    }
}

#[cfg(feature = "workflows")]
impl BranchId {
    pub(crate) fn new() -> BranchId {
        BranchId(uuid::Uuid::new_v4())
    }
}

#[derive(serde::Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct MessageId(String);

impl std::str::FromStr for MessageId {
    type Err = MessageIdError;

    fn from_str(text: &str) -> Result<MessageId, MessageIdError> {
        let shaped =
            text.len() == MESSAGE_ID_DIGITS && text.bytes().all(|byte| byte.is_ascii_hexdigit());
        if shaped {
            Ok(MessageId(text.to_string()))
        } else {
            Err(MessageIdError {
                rejected: text.to_string(),
            })
        }
    }
}

impl std::convert::TryFrom<&str> for MessageId {
    type Error = MessageIdError;

    fn try_from(text: &str) -> Result<MessageId, MessageIdError> {
        text.parse()
    }
}

impl std::fmt::Display for MessageId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl<'de> serde::Deserialize<'de> for MessageId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<MessageId, D::Error> {
        let text = <String as serde::Deserialize>::deserialize(deserializer)?;
        text.parse().map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug)]
pub struct MessageIdError {
    rejected: String,
}

impl std::fmt::Display for MessageIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}' is not a 24 hex digit message id", self.rejected)
    }
}

impl std::error::Error for MessageIdError {}
