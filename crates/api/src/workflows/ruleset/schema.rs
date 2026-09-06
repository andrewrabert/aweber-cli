#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
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
#[doc = "Defines an action to take when an event is being processed"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Action\","]
#[doc = "  \"description\": \"Defines an action to take when an event is being processed\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"definition\","]
#[doc = "    \"id\","]
#[doc = "    \"metadata\","]
#[doc = "    \"parents\","]
#[doc = "    \"title\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"branch\": {"]
#[doc = "      \"title\": \"Branch ID\","]
#[doc = "      \"description\": \"The branch that this action belongs to\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"pattern\": \"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$\""]
#[doc = "    },"]
#[doc = "    \"definition\": {"]
#[doc = "      \"title\": \"Definition\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/actions_campaign_change_rule_state_v1\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/actions_email_send_v1\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/actions_followup_enable_v1\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/actions_followup_send_autoresponse_v1\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/actions_log_log_v1\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/actions_schedule_change_event_states_v1\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/actions_schedule_wait_v1\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/actions_tag_modify_tags_v1\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/actions_stop_stop\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/actions_webfeed_create_webfeed_message_v1\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/actions_branch_set_branch\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/actions_branch_pause_branch\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/actions_schedule_set_branch_v1\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"title\": \"Action ID\","]
#[doc = "      \"description\": \"Generated UUIDv4 representing this action for execution state.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$\""]
#[doc = "    },"]
#[doc = "    \"is_deleted\": {"]
#[doc = "      \"title\": \"Is Deleted\","]
#[doc = "      \"description\": \"Indicate whether the action is deleted\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"title\": \"Metadata\","]
#[doc = "      \"description\": \"Metadata used by front-end applications\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"parents\": {"]
#[doc = "      \"title\": \"Parent Event IDs\","]
#[doc = "      \"description\": \"A list of IDs for the events that this action would be processed for.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\","]
#[doc = "        \"pattern\": \"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$\""]
#[doc = "      },"]
#[doc = "      \"minItems\": 1,"]
#[doc = "      \"uniqueItems\": true"]
#[doc = "    },"]
#[doc = "    \"recurring\": {"]
#[doc = "      \"title\": \"Recurring\","]
#[doc = "      \"description\": \"Indicate whether an action supports recurrence\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"title\": \"Optional Title\","]
#[doc = "      \"description\": \"A title that can be used for the UI to indicate a child campaign or logic branch\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Action {
    #[doc = "The branch that this action belongs to"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub branch: ::std::option::Option<BranchId>,
    pub definition: Definition,
    #[doc = "Generated UUIDv4 representing this action for execution state."]
    pub id: ActionId,
    #[doc = "Indicate whether the action is deleted"]
    #[serde(default)]
    pub is_deleted: bool,
    #[doc = "Metadata used by front-end applications"]
    pub metadata: ::std::option::Option<::std::string::String>,
    #[doc = "A list of IDs for the events that this action would be processed for."]
    pub parents: Vec<ParentEventIDsItem>,
    #[doc = "Indicate whether an action supports recurrence"]
    #[serde(default)]
    pub recurring: bool,
    #[doc = "A title that can be used for the UI to indicate a child campaign or logic branch"]
    pub title: ::std::option::Option<::std::string::String>,
}
impl Action {
    pub fn builder() -> builder::Action {
        Default::default()
    }
}
#[doc = "Generated UUIDv4 representing this action for execution state."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Action ID\","]
#[doc = "  \"description\": \"Generated UUIDv4 representing this action for execution state.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ActionId(::std::string::String);
impl ::std::ops::Deref for ActionId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ActionId> for ::std::string::String {
    fn from(value: ActionId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ActionId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new(
                    "^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$",
                )
                .unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err ("doesn't match pattern \"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$\"" . into ()) ;
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ActionId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ActionId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ActionId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ActionId {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Pauses processing of a ruleset for a specific branch to be resumed later"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Pause Branch\","]
#[doc = "  \"description\": \"Pauses processing of a ruleset for a specific branch to be resumed later\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"rulesengine.action.pause_branch\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"rulesengine.action.pause_branch\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"ruleset\","]
#[doc = "        \"subscriber\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"ruleset\": {"]
#[doc = "          \"title\": \"Ruleset ID\","]
#[doc = "          \"description\": \"The ruleset ID\","]
#[doc = "          \"default\": \"<ruleset>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<ruleset>\""]
#[doc = "        },"]
#[doc = "        \"subscriber\": {"]
#[doc = "          \"title\": \"Subscriber ID\","]
#[doc = "          \"description\": \"The subscriber ID\","]
#[doc = "          \"default\": \"<event:subscriber>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:subscriber>\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionsBranchPauseBranch {
    pub function: ::std::string::String,
    pub kwargs: ActionsBranchPauseBranchKwargs,
}
impl ActionsBranchPauseBranch {
    pub fn builder() -> builder::ActionsBranchPauseBranch {
        Default::default()
    }
}
#[doc = "`ActionsBranchPauseBranchKwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"ruleset\","]
#[doc = "    \"subscriber\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ruleset\": {"]
#[doc = "      \"title\": \"Ruleset ID\","]
#[doc = "      \"description\": \"The ruleset ID\","]
#[doc = "      \"default\": \"<ruleset>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<ruleset>\""]
#[doc = "    },"]
#[doc = "    \"subscriber\": {"]
#[doc = "      \"title\": \"Subscriber ID\","]
#[doc = "      \"description\": \"The subscriber ID\","]
#[doc = "      \"default\": \"<event:subscriber>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:subscriber>\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionsBranchPauseBranchKwargs {
    #[doc = "The ruleset ID"]
    pub ruleset: ::std::string::String,
    #[doc = "The subscriber ID"]
    pub subscriber: ::std::string::String,
}
impl ActionsBranchPauseBranchKwargs {
    pub fn builder() -> builder::ActionsBranchPauseBranchKwargs {
        Default::default()
    }
}
#[doc = "Sets the active branch for processing a subscriber"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Set Branch\","]
#[doc = "  \"description\": \"Sets the active branch for processing a subscriber\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"rulesengine.action.set_branch\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"rulesengine.action.set_branch\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"branches\","]
#[doc = "        \"ruleset\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"branches\": {"]
#[doc = "          \"title\": \"Branches\","]
#[doc = "          \"description\": \"The list of branches to choose from\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"oneOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/filter\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"type\": \"object\","]
#[doc = "                \"required\": ["]
#[doc = "                  \"id\""]
#[doc = "                ],"]
#[doc = "                \"properties\": {"]
#[doc = "                  \"id\": {"]
#[doc = "                    \"title\": \"ID\","]
#[doc = "                    \"description\": \"The filter or branch ID\","]
#[doc = "                    \"type\": ["]
#[doc = "                      \"string\""]
#[doc = "                    ]"]
#[doc = "                  }"]
#[doc = "                },"]
#[doc = "                \"additionalProperties\": false"]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"minItems\": 1"]
#[doc = "        },"]
#[doc = "        \"ruleset\": {"]
#[doc = "          \"title\": \"Ruleset\","]
#[doc = "          \"description\": \"The ID of the ruleset to go to\","]
#[doc = "          \"default\": \"<ruleset>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<ruleset>\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionsBranchSetBranch {
    pub function: ::std::string::String,
    pub kwargs: ActionsBranchSetBranchKwargs,
}
impl ActionsBranchSetBranch {
    pub fn builder() -> builder::ActionsBranchSetBranch {
        Default::default()
    }
}
#[doc = "`ActionsBranchSetBranchKwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"branches\","]
#[doc = "    \"ruleset\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"branches\": {"]
#[doc = "      \"title\": \"Branches\","]
#[doc = "      \"description\": \"The list of branches to choose from\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"oneOf\": ["]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/$defs/filter\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"id\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"id\": {"]
#[doc = "                \"title\": \"ID\","]
#[doc = "                \"description\": \"The filter or branch ID\","]
#[doc = "                \"type\": ["]
#[doc = "                  \"string\""]
#[doc = "                ]"]
#[doc = "              }"]
#[doc = "            },"]
#[doc = "            \"additionalProperties\": false"]
#[doc = "          }"]
#[doc = "        ]"]
#[doc = "      },"]
#[doc = "      \"minItems\": 1"]
#[doc = "    },"]
#[doc = "    \"ruleset\": {"]
#[doc = "      \"title\": \"Ruleset\","]
#[doc = "      \"description\": \"The ID of the ruleset to go to\","]
#[doc = "      \"default\": \"<ruleset>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<ruleset>\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionsBranchSetBranchKwargs {
    #[doc = "The list of branches to choose from"]
    pub branches: ::std::vec::Vec<BranchesItem>,
    #[doc = "The ID of the ruleset to go to"]
    pub ruleset: ::std::string::String,
}
impl ActionsBranchSetBranchKwargs {
    pub fn builder() -> builder::ActionsBranchSetBranchKwargs {
        Default::default()
    }
}
#[doc = "Campaign Action function definitions"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Change Rule State\","]
#[doc = "  \"description\": \"Campaign Action function definitions\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"ruleset.campaign.action.change_rule_state_v1\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ruleset.campaign.action.change_rule_state_v1\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"ruleset\","]
#[doc = "        \"state\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"ruleset\": {"]
#[doc = "          \"title\": \"Rule ID\","]
#[doc = "          \"description\": \"The ID of the ruleset in the Rule service\","]
#[doc = "          \"default\": \"<event:ruleset>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^((<event:\\\\w+>)|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}))$\""]
#[doc = "        },"]
#[doc = "        \"state\": {"]
#[doc = "          \"title\": \"Rule State\","]
#[doc = "          \"description\": \"The state to update the ruleset to\","]
#[doc = "          \"default\": \"<event:state>\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionsCampaignChangeRuleStateV1 {
    pub function: ::std::string::String,
    pub kwargs: ActionsCampaignChangeRuleStateV1Kwargs,
}
impl ActionsCampaignChangeRuleStateV1 {
    pub fn builder() -> builder::ActionsCampaignChangeRuleStateV1 {
        Default::default()
    }
}
#[doc = "`ActionsCampaignChangeRuleStateV1Kwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"ruleset\","]
#[doc = "    \"state\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ruleset\": {"]
#[doc = "      \"title\": \"Rule ID\","]
#[doc = "      \"description\": \"The ID of the ruleset in the Rule service\","]
#[doc = "      \"default\": \"<event:ruleset>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^((<event:\\\\w+>)|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}))$\""]
#[doc = "    },"]
#[doc = "    \"state\": {"]
#[doc = "      \"title\": \"Rule State\","]
#[doc = "      \"description\": \"The state to update the ruleset to\","]
#[doc = "      \"default\": \"<event:state>\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionsCampaignChangeRuleStateV1Kwargs {
    #[doc = "The ID of the ruleset in the Rule service"]
    pub ruleset: RuleId,
    #[doc = "The state to update the ruleset to"]
    pub state: ::std::string::String,
}
impl ActionsCampaignChangeRuleStateV1Kwargs {
    pub fn builder() -> builder::ActionsCampaignChangeRuleStateV1Kwargs {
        Default::default()
    }
}
#[doc = "Send an email message to the recipient specified in the triggering event"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Send Email\","]
#[doc = "  \"description\": \"Send an email message to the recipient specified in the triggering event\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"ruleset.email.action.compose_v1\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ruleset.email.action.compose_v1\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"account\","]
#[doc = "        \"list\","]
#[doc = "        \"meapi_id\","]
#[doc = "        \"message\","]
#[doc = "        \"recipient\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"account\": {"]
#[doc = "          \"title\": \"Account\","]
#[doc = "          \"description\": \"The Account to send the email for\","]
#[doc = "          \"default\": \"<event:account>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:account>\""]
#[doc = "        },"]
#[doc = "        \"list\": {"]
#[doc = "          \"title\": \"List\","]
#[doc = "          \"description\": \"The List to send the email for\","]
#[doc = "          \"default\": \"<event:list>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:list>\""]
#[doc = "        },"]
#[doc = "        \"meapi_id\": {"]
#[doc = "          \"title\": \"Message Editor ID\","]
#[doc = "          \"description\": \"The message api ID OID for the email template.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^[0-9a-fA-F]{24}$\""]
#[doc = "        },"]
#[doc = "        \"message\": {"]
#[doc = "          \"title\": \"Message\","]
#[doc = "          \"description\": \"The message ID for the email to send. When using the <message:new> syntax, the template value should be the message editor ID.\","]
#[doc = "          \"default\": \"<message:new message=123>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^((<message:new message=[0-9a-fA-F]{24}>)|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}))$\""]
#[doc = "        },"]
#[doc = "        \"recipient\": {"]
#[doc = "          \"title\": \"Recipient\","]
#[doc = "          \"description\": \"The recipient to send the email to\","]
#[doc = "          \"default\": \"<event:recipient>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:recipient>\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionsEmailSendV1 {
    pub function: ::std::string::String,
    pub kwargs: ActionsEmailSendV1Kwargs,
}
impl ActionsEmailSendV1 {
    pub fn builder() -> builder::ActionsEmailSendV1 {
        Default::default()
    }
}
#[doc = "`ActionsEmailSendV1Kwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"account\","]
#[doc = "    \"list\","]
#[doc = "    \"meapi_id\","]
#[doc = "    \"message\","]
#[doc = "    \"recipient\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"account\": {"]
#[doc = "      \"title\": \"Account\","]
#[doc = "      \"description\": \"The Account to send the email for\","]
#[doc = "      \"default\": \"<event:account>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:account>\""]
#[doc = "    },"]
#[doc = "    \"list\": {"]
#[doc = "      \"title\": \"List\","]
#[doc = "      \"description\": \"The List to send the email for\","]
#[doc = "      \"default\": \"<event:list>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:list>\""]
#[doc = "    },"]
#[doc = "    \"meapi_id\": {"]
#[doc = "      \"title\": \"Message Editor ID\","]
#[doc = "      \"description\": \"The message api ID OID for the email template.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^[0-9a-fA-F]{24}$\""]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"title\": \"Message\","]
#[doc = "      \"description\": \"The message ID for the email to send. When using the <message:new> syntax, the template value should be the message editor ID.\","]
#[doc = "      \"default\": \"<message:new message=123>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^((<message:new message=[0-9a-fA-F]{24}>)|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}))$\""]
#[doc = "    },"]
#[doc = "    \"recipient\": {"]
#[doc = "      \"title\": \"Recipient\","]
#[doc = "      \"description\": \"The recipient to send the email to\","]
#[doc = "      \"default\": \"<event:recipient>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:recipient>\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionsEmailSendV1Kwargs {
    #[doc = "The Account to send the email for"]
    pub account: ::std::string::String,
    #[doc = "The List to send the email for"]
    pub list: ::std::string::String,
    #[doc = "The message api ID OID for the email template."]
    pub meapi_id: MessageEditorId,
    #[doc = "The message ID for the email to send. When using the <message:new> syntax, the template value should be the message editor ID."]
    pub message: Message,
    #[doc = "The recipient to send the email to"]
    pub recipient: ::std::string::String,
}
impl ActionsEmailSendV1Kwargs {
    pub fn builder() -> builder::ActionsEmailSendV1Kwargs {
        Default::default()
    }
}
#[doc = "Enable sending of the followup series to the recipient"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Enable Followup Series\","]
#[doc = "  \"description\": \"Enable sending of the followup series to the recipient\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"ruleset.followup.action.enable_v1\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ruleset.followup.action.enable_v1\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"recipient\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"recipient\": {"]
#[doc = "          \"title\": \"Recipient\","]
#[doc = "          \"description\": \"The recipient who should receive followups\","]
#[doc = "          \"default\": \"<event:recipient>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:recipient>\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionsFollowupEnableV1 {
    pub function: ::std::string::String,
    pub kwargs: ActionsFollowupEnableV1Kwargs,
}
impl ActionsFollowupEnableV1 {
    pub fn builder() -> builder::ActionsFollowupEnableV1 {
        Default::default()
    }
}
#[doc = "`ActionsFollowupEnableV1Kwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"recipient\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"recipient\": {"]
#[doc = "      \"title\": \"Recipient\","]
#[doc = "      \"description\": \"The recipient who should receive followups\","]
#[doc = "      \"default\": \"<event:recipient>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:recipient>\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionsFollowupEnableV1Kwargs {
    #[doc = "The recipient who should receive followups"]
    pub recipient: ::std::string::String,
}
impl ActionsFollowupEnableV1Kwargs {
    pub fn builder() -> builder::ActionsFollowupEnableV1Kwargs {
        Default::default()
    }
}
#[doc = "Send the first message in the followups series"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Send Autoresponse\","]
#[doc = "  \"description\": \"Send the first message in the followups series\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"ruleset.followup.action.send_autoresponse_v1\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ruleset.followup.action.send_autoresponse_v1\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"recipient\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"recipient\": {"]
#[doc = "          \"title\": \"Recipient\","]
#[doc = "          \"description\": \"The recipient to send the autoresponse to\","]
#[doc = "          \"default\": \"<event:recipient>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:recipient>\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionsFollowupSendAutoresponseV1 {
    pub function: ::std::string::String,
    pub kwargs: ActionsFollowupSendAutoresponseV1Kwargs,
}
impl ActionsFollowupSendAutoresponseV1 {
    pub fn builder() -> builder::ActionsFollowupSendAutoresponseV1 {
        Default::default()
    }
}
#[doc = "`ActionsFollowupSendAutoresponseV1Kwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"recipient\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"recipient\": {"]
#[doc = "      \"title\": \"Recipient\","]
#[doc = "      \"description\": \"The recipient to send the autoresponse to\","]
#[doc = "      \"default\": \"<event:recipient>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:recipient>\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionsFollowupSendAutoresponseV1Kwargs {
    #[doc = "The recipient to send the autoresponse to"]
    pub recipient: ::std::string::String,
}
impl ActionsFollowupSendAutoresponseV1Kwargs {
    pub fn builder() -> builder::ActionsFollowupSendAutoresponseV1Kwargs {
        Default::default()
    }
}
#[doc = "Logs a message"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Log\","]
#[doc = "  \"description\": \"Logs a message\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"rulesengine.action.log_v1\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"rulesengine.action.log_v1\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"event\","]
#[doc = "        \"level\","]
#[doc = "        \"message\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"event\": {"]
#[doc = "          \"title\": \"Message\","]
#[doc = "          \"description\": \"Used for macro replacement in the log message\","]
#[doc = "          \"default\": \"<event>\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"level\": {"]
#[doc = "          \"title\": \"Level\","]
#[doc = "          \"description\": \"The log level for the message\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"debug\","]
#[doc = "            \"info\","]
#[doc = "            \"warning\","]
#[doc = "            \"error\","]
#[doc = "            \"critical\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"message\": {"]
#[doc = "          \"title\": \"Message\","]
#[doc = "          \"description\": \"The message to log\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionsLogLogV1 {
    pub function: ::std::string::String,
    pub kwargs: ActionsLogLogV1Kwargs,
}
impl ActionsLogLogV1 {
    pub fn builder() -> builder::ActionsLogLogV1 {
        Default::default()
    }
}
#[doc = "`ActionsLogLogV1Kwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"event\","]
#[doc = "    \"level\","]
#[doc = "    \"message\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"event\": {"]
#[doc = "      \"title\": \"Message\","]
#[doc = "      \"description\": \"Used for macro replacement in the log message\","]
#[doc = "      \"default\": \"<event>\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"level\": {"]
#[doc = "      \"title\": \"Level\","]
#[doc = "      \"description\": \"The log level for the message\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"debug\","]
#[doc = "        \"info\","]
#[doc = "        \"warning\","]
#[doc = "        \"error\","]
#[doc = "        \"critical\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"message\": {"]
#[doc = "      \"title\": \"Message\","]
#[doc = "      \"description\": \"The message to log\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionsLogLogV1Kwargs {
    #[doc = "Used for macro replacement in the log message"]
    pub event: ::std::string::String,
    #[doc = "The log level for the message"]
    pub level: Level,
    #[doc = "The message to log"]
    pub message: ::std::string::String,
}
impl ActionsLogLogV1Kwargs {
    pub fn builder() -> builder::ActionsLogLogV1Kwargs {
        Default::default()
    }
}
#[doc = "Update state of events associated with a rule set"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Change Event States\","]
#[doc = "  \"description\": \"Update state of events associated with a rule set\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"ruleset.schedule.action.change_event_states_v1\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ruleset.schedule.action.change_event_states_v1\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"ruleset\","]
#[doc = "        \"state\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"ruleset\": {"]
#[doc = "          \"title\": \"Ruleset ID\","]
#[doc = "          \"description\": \"The ruleset whose state has changed.\","]
#[doc = "          \"default\": \"<event:ruleset>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^((<event:\\\\w+>)|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}))$\""]
#[doc = "        },"]
#[doc = "        \"state\": {"]
#[doc = "          \"title\": \"Ruleset state\","]
#[doc = "          \"description\": \"The state that the Ruleset transitioned to\","]
#[doc = "          \"default\": \"<event:value>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^<event:value>\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionsScheduleChangeEventStatesV1 {
    pub function: ::std::string::String,
    pub kwargs: ActionsScheduleChangeEventStatesV1Kwargs,
}
impl ActionsScheduleChangeEventStatesV1 {
    pub fn builder() -> builder::ActionsScheduleChangeEventStatesV1 {
        Default::default()
    }
}
#[doc = "`ActionsScheduleChangeEventStatesV1Kwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"ruleset\","]
#[doc = "    \"state\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ruleset\": {"]
#[doc = "      \"title\": \"Ruleset ID\","]
#[doc = "      \"description\": \"The ruleset whose state has changed.\","]
#[doc = "      \"default\": \"<event:ruleset>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^((<event:\\\\w+>)|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}))$\""]
#[doc = "    },"]
#[doc = "    \"state\": {"]
#[doc = "      \"title\": \"Ruleset state\","]
#[doc = "      \"description\": \"The state that the Ruleset transitioned to\","]
#[doc = "      \"default\": \"<event:value>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^<event:value>\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionsScheduleChangeEventStatesV1Kwargs {
    #[doc = "The ruleset whose state has changed."]
    pub ruleset: RulesetId,
    #[doc = "The state that the Ruleset transitioned to"]
    pub state: RulesetState,
}
impl ActionsScheduleChangeEventStatesV1Kwargs {
    pub fn builder() -> builder::ActionsScheduleChangeEventStatesV1Kwargs {
        Default::default()
    }
}
#[doc = "Sets the active branch for processing a subscriber. Schedules a wait_complete.v1 event if configured to allow extra time."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Set Branch\","]
#[doc = "  \"description\": \"Sets the active branch for processing a subscriber. Schedules a wait_complete.v1 event if configured to allow extra time.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"ruleset.schedule.action.set_branch_v1\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ruleset.schedule.action.set_branch_v1\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"account\","]
#[doc = "        \"action\","]
#[doc = "        \"branches\","]
#[doc = "        \"delay\","]
#[doc = "        \"id\","]
#[doc = "        \"list\","]
#[doc = "        \"recipient\","]
#[doc = "        \"ruleset\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"account\": {"]
#[doc = "          \"title\": \"Account\","]
#[doc = "          \"description\": \"The account associated with the branch wait\","]
#[doc = "          \"default\": \"<event:account>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:account>\""]
#[doc = "        },"]
#[doc = "        \"action\": {"]
#[doc = "          \"title\": \"Action ID\","]
#[doc = "          \"description\": \"The ID of the action in the ruleset\","]
#[doc = "          \"default\": \"<action>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^((<action>)|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}))$\""]
#[doc = "        },"]
#[doc = "        \"branches\": {"]
#[doc = "          \"title\": \"Branches\","]
#[doc = "          \"description\": \"The list of branches to choose from\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"oneOf\": ["]
#[doc = "              {"]
#[doc = "                \"$ref\": \"#/$defs/filter\""]
#[doc = "              },"]
#[doc = "              {"]
#[doc = "                \"type\": \"object\","]
#[doc = "                \"required\": ["]
#[doc = "                  \"id\""]
#[doc = "                ],"]
#[doc = "                \"properties\": {"]
#[doc = "                  \"id\": {"]
#[doc = "                    \"title\": \"ID\","]
#[doc = "                    \"description\": \"The filter or branch ID\","]
#[doc = "                    \"type\": ["]
#[doc = "                      \"string\""]
#[doc = "                    ]"]
#[doc = "                  }"]
#[doc = "                },"]
#[doc = "                \"additionalProperties\": false"]
#[doc = "              }"]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"minItems\": 1"]
#[doc = "        },"]
#[doc = "        \"delay\": {"]
#[doc = "          \"title\": \"Delay\","]
#[doc = "          \"description\": \"A ISO-8601 formatted duration that specifies the delay from the execution time until the wait-complete event is published\","]
#[doc = "          \"default\": \"PT0S\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"title\": \"Wait ID\","]
#[doc = "          \"description\": \"Generated ID that uniquely identifies the branch wait event\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$\""]
#[doc = "        },"]
#[doc = "        \"list\": {"]
#[doc = "          \"title\": \"List\","]
#[doc = "          \"description\": \"The list associated with the branch wait\","]
#[doc = "          \"default\": \"<event:list>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:list>\""]
#[doc = "        },"]
#[doc = "        \"recipient\": {"]
#[doc = "          \"title\": \"Recipient\","]
#[doc = "          \"description\": \"The recipient associated with the branch wait\","]
#[doc = "          \"default\": \"<event:recipient>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:recipient>\""]
#[doc = "        },"]
#[doc = "        \"rrules\": {"]
#[doc = "          \"title\": \"Recurrence Rules\","]
#[doc = "          \"description\": \"An array of RFC-5545 Recurrence Rule for defining when the wait-complete event can be send\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"minItems\": 0,"]
#[doc = "          \"uniqueItems\": true"]
#[doc = "        },"]
#[doc = "        \"ruleset\": {"]
#[doc = "          \"title\": \"Ruleset ID\","]
#[doc = "          \"description\": \"The ID of the ruleset to go to\","]
#[doc = "          \"default\": \"<ruleset>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<ruleset>\""]
#[doc = "        },"]
#[doc = "        \"timezone\": {"]
#[doc = "          \"title\": \"Timezone\","]
#[doc = "          \"description\": \"The timezone of the message in Olson format\","]
#[doc = "          \"default\": \"UTC\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"use_subscriber_timezone\": {"]
#[doc = "          \"title\": \"Use Subscriber Timezone\","]
#[doc = "          \"description\": \"Indicate whether to use the Subscriber timezone of the default\","]
#[doc = "          \"default\": false,"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionsScheduleSetBranchV1 {
    pub function: ::std::string::String,
    pub kwargs: ActionsScheduleSetBranchV1Kwargs,
}
impl ActionsScheduleSetBranchV1 {
    pub fn builder() -> builder::ActionsScheduleSetBranchV1 {
        Default::default()
    }
}
#[doc = "`ActionsScheduleSetBranchV1Kwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"account\","]
#[doc = "    \"action\","]
#[doc = "    \"branches\","]
#[doc = "    \"delay\","]
#[doc = "    \"id\","]
#[doc = "    \"list\","]
#[doc = "    \"recipient\","]
#[doc = "    \"ruleset\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"account\": {"]
#[doc = "      \"title\": \"Account\","]
#[doc = "      \"description\": \"The account associated with the branch wait\","]
#[doc = "      \"default\": \"<event:account>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:account>\""]
#[doc = "    },"]
#[doc = "    \"action\": {"]
#[doc = "      \"title\": \"Action ID\","]
#[doc = "      \"description\": \"The ID of the action in the ruleset\","]
#[doc = "      \"default\": \"<action>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^((<action>)|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}))$\""]
#[doc = "    },"]
#[doc = "    \"branches\": {"]
#[doc = "      \"title\": \"Branches\","]
#[doc = "      \"description\": \"The list of branches to choose from\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"oneOf\": ["]
#[doc = "          {"]
#[doc = "            \"$ref\": \"#/$defs/filter\""]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"type\": \"object\","]
#[doc = "            \"required\": ["]
#[doc = "              \"id\""]
#[doc = "            ],"]
#[doc = "            \"properties\": {"]
#[doc = "              \"id\": {"]
#[doc = "                \"title\": \"ID\","]
#[doc = "                \"description\": \"The filter or branch ID\","]
#[doc = "                \"type\": ["]
#[doc = "                  \"string\""]
#[doc = "                ]"]
#[doc = "              }"]
#[doc = "            },"]
#[doc = "            \"additionalProperties\": false"]
#[doc = "          }"]
#[doc = "        ]"]
#[doc = "      },"]
#[doc = "      \"minItems\": 1"]
#[doc = "    },"]
#[doc = "    \"delay\": {"]
#[doc = "      \"title\": \"Delay\","]
#[doc = "      \"description\": \"A ISO-8601 formatted duration that specifies the delay from the execution time until the wait-complete event is published\","]
#[doc = "      \"default\": \"PT0S\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"title\": \"Wait ID\","]
#[doc = "      \"description\": \"Generated ID that uniquely identifies the branch wait event\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$\""]
#[doc = "    },"]
#[doc = "    \"list\": {"]
#[doc = "      \"title\": \"List\","]
#[doc = "      \"description\": \"The list associated with the branch wait\","]
#[doc = "      \"default\": \"<event:list>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:list>\""]
#[doc = "    },"]
#[doc = "    \"recipient\": {"]
#[doc = "      \"title\": \"Recipient\","]
#[doc = "      \"description\": \"The recipient associated with the branch wait\","]
#[doc = "      \"default\": \"<event:recipient>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:recipient>\""]
#[doc = "    },"]
#[doc = "    \"rrules\": {"]
#[doc = "      \"title\": \"Recurrence Rules\","]
#[doc = "      \"description\": \"An array of RFC-5545 Recurrence Rule for defining when the wait-complete event can be send\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"minItems\": 0,"]
#[doc = "      \"uniqueItems\": true"]
#[doc = "    },"]
#[doc = "    \"ruleset\": {"]
#[doc = "      \"title\": \"Ruleset ID\","]
#[doc = "      \"description\": \"The ID of the ruleset to go to\","]
#[doc = "      \"default\": \"<ruleset>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<ruleset>\""]
#[doc = "    },"]
#[doc = "    \"timezone\": {"]
#[doc = "      \"title\": \"Timezone\","]
#[doc = "      \"description\": \"The timezone of the message in Olson format\","]
#[doc = "      \"default\": \"UTC\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"use_subscriber_timezone\": {"]
#[doc = "      \"title\": \"Use Subscriber Timezone\","]
#[doc = "      \"description\": \"Indicate whether to use the Subscriber timezone of the default\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionsScheduleSetBranchV1Kwargs {
    #[doc = "The account associated with the branch wait"]
    pub account: ::std::string::String,
    #[doc = "The ID of the action in the ruleset"]
    pub action: ActionId,
    #[doc = "The list of branches to choose from"]
    pub branches: ::std::vec::Vec<BranchesItem>,
    #[doc = "A ISO-8601 formatted duration that specifies the delay from the execution time until the wait-complete event is published"]
    pub delay: ::std::string::String,
    #[doc = "Generated ID that uniquely identifies the branch wait event"]
    pub id: WaitId,
    #[doc = "The list associated with the branch wait"]
    pub list: ::std::string::String,
    #[doc = "The recipient associated with the branch wait"]
    pub recipient: ::std::string::String,
    #[doc = "An array of RFC-5545 Recurrence Rule for defining when the wait-complete event can be send"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub rrules: ::std::option::Option<Vec<::std::string::String>>,
    #[doc = "The ID of the ruleset to go to"]
    pub ruleset: ::std::string::String,
    #[doc = "The timezone of the message in Olson format"]
    #[serde(default = "defaults::actions_schedule_set_branch_v1_kwargs_timezone")]
    pub timezone: ::std::string::String,
    #[doc = "Indicate whether to use the Subscriber timezone of the default"]
    #[serde(default)]
    pub use_subscriber_timezone: bool,
}
impl ActionsScheduleSetBranchV1Kwargs {
    pub fn builder() -> builder::ActionsScheduleSetBranchV1Kwargs {
        Default::default()
    }
}
#[doc = "Schedules a wait_complete.v1 event to be published by the scheduling service."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Wait\","]
#[doc = "  \"description\": \"Schedules a wait_complete.v1 event to be published by the scheduling service.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"ruleset.schedule.action.wait_v1\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ruleset.schedule.action.wait_v1\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"account\","]
#[doc = "        \"delay\","]
#[doc = "        \"id\","]
#[doc = "        \"list\","]
#[doc = "        \"recipient\","]
#[doc = "        \"rrules\","]
#[doc = "        \"ruleset\","]
#[doc = "        \"timezone\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"account\": {"]
#[doc = "          \"title\": \"Account\","]
#[doc = "          \"description\": \"The account associated with the wait\","]
#[doc = "          \"default\": \"<event:account>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:account>\""]
#[doc = "        },"]
#[doc = "        \"action\": {"]
#[doc = "          \"title\": \"Action ID\","]
#[doc = "          \"description\": \"The ID of the action in the ruleset\","]
#[doc = "          \"default\": \"<action>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^((<action>)|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}))$\""]
#[doc = "        },"]
#[doc = "        \"delay\": {"]
#[doc = "          \"title\": \"Delay\","]
#[doc = "          \"description\": \"A ISO-8601 formatted duration that specifies the delay from the execution time until the wait-complete event is published\","]
#[doc = "          \"default\": \"PT0S\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"title\": \"Wait ID\","]
#[doc = "          \"description\": \"Generated ID that uniquely identifies the wait event\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$\""]
#[doc = "        },"]
#[doc = "        \"list\": {"]
#[doc = "          \"title\": \"List\","]
#[doc = "          \"description\": \"The list associated with the wait\","]
#[doc = "          \"default\": \"<event:list>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:list>\""]
#[doc = "        },"]
#[doc = "        \"recipient\": {"]
#[doc = "          \"title\": \"Recipient\","]
#[doc = "          \"description\": \"The recipient associated with the wait\","]
#[doc = "          \"default\": \"<event:recipient>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:recipient>\""]
#[doc = "        },"]
#[doc = "        \"rrules\": {"]
#[doc = "          \"title\": \"Recurrence Rules\","]
#[doc = "          \"description\": \"An array of RFC-5545 Recurrence Rule for defining when the wait-complete event can be send\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"minItems\": 0,"]
#[doc = "          \"uniqueItems\": true"]
#[doc = "        },"]
#[doc = "        \"ruleset\": {"]
#[doc = "          \"title\": \"Ruleset ID\","]
#[doc = "          \"description\": \"The ID of the ruleset in the Rule service\","]
#[doc = "          \"default\": \"<ruleset>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^((<ruleset>)|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}))$\""]
#[doc = "        },"]
#[doc = "        \"timezone\": {"]
#[doc = "          \"title\": \"Timezone\","]
#[doc = "          \"description\": \"The timezone of the message in Olson format\","]
#[doc = "          \"default\": \"UTC\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"use_subscriber_timezone\": {"]
#[doc = "          \"title\": \"Use Subscriber Timezone\","]
#[doc = "          \"description\": \"Indicate whether to use the Subscriber timezone of the default\","]
#[doc = "          \"default\": false,"]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionsScheduleWaitV1 {
    pub function: ::std::string::String,
    pub kwargs: ActionsScheduleWaitV1Kwargs,
}
impl ActionsScheduleWaitV1 {
    pub fn builder() -> builder::ActionsScheduleWaitV1 {
        Default::default()
    }
}
#[doc = "`ActionsScheduleWaitV1Kwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"account\","]
#[doc = "    \"delay\","]
#[doc = "    \"id\","]
#[doc = "    \"list\","]
#[doc = "    \"recipient\","]
#[doc = "    \"rrules\","]
#[doc = "    \"ruleset\","]
#[doc = "    \"timezone\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"account\": {"]
#[doc = "      \"title\": \"Account\","]
#[doc = "      \"description\": \"The account associated with the wait\","]
#[doc = "      \"default\": \"<event:account>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:account>\""]
#[doc = "    },"]
#[doc = "    \"action\": {"]
#[doc = "      \"title\": \"Action ID\","]
#[doc = "      \"description\": \"The ID of the action in the ruleset\","]
#[doc = "      \"default\": \"<action>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^((<action>)|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}))$\""]
#[doc = "    },"]
#[doc = "    \"delay\": {"]
#[doc = "      \"title\": \"Delay\","]
#[doc = "      \"description\": \"A ISO-8601 formatted duration that specifies the delay from the execution time until the wait-complete event is published\","]
#[doc = "      \"default\": \"PT0S\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"title\": \"Wait ID\","]
#[doc = "      \"description\": \"Generated ID that uniquely identifies the wait event\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$\""]
#[doc = "    },"]
#[doc = "    \"list\": {"]
#[doc = "      \"title\": \"List\","]
#[doc = "      \"description\": \"The list associated with the wait\","]
#[doc = "      \"default\": \"<event:list>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:list>\""]
#[doc = "    },"]
#[doc = "    \"recipient\": {"]
#[doc = "      \"title\": \"Recipient\","]
#[doc = "      \"description\": \"The recipient associated with the wait\","]
#[doc = "      \"default\": \"<event:recipient>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:recipient>\""]
#[doc = "    },"]
#[doc = "    \"rrules\": {"]
#[doc = "      \"title\": \"Recurrence Rules\","]
#[doc = "      \"description\": \"An array of RFC-5545 Recurrence Rule for defining when the wait-complete event can be send\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"minItems\": 0,"]
#[doc = "      \"uniqueItems\": true"]
#[doc = "    },"]
#[doc = "    \"ruleset\": {"]
#[doc = "      \"title\": \"Ruleset ID\","]
#[doc = "      \"description\": \"The ID of the ruleset in the Rule service\","]
#[doc = "      \"default\": \"<ruleset>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^((<ruleset>)|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}))$\""]
#[doc = "    },"]
#[doc = "    \"timezone\": {"]
#[doc = "      \"title\": \"Timezone\","]
#[doc = "      \"description\": \"The timezone of the message in Olson format\","]
#[doc = "      \"default\": \"UTC\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"use_subscriber_timezone\": {"]
#[doc = "      \"title\": \"Use Subscriber Timezone\","]
#[doc = "      \"description\": \"Indicate whether to use the Subscriber timezone of the default\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionsScheduleWaitV1Kwargs {
    #[doc = "The account associated with the wait"]
    pub account: ::std::string::String,
    #[doc = "The ID of the action in the ruleset"]
    #[serde(default = "defaults::actions_schedule_wait_v1_kwargs_action")]
    pub action: ActionId,
    #[doc = "A ISO-8601 formatted duration that specifies the delay from the execution time until the wait-complete event is published"]
    pub delay: ::std::string::String,
    #[doc = "Generated ID that uniquely identifies the wait event"]
    pub id: WaitId,
    #[doc = "The list associated with the wait"]
    pub list: ::std::string::String,
    #[doc = "The recipient associated with the wait"]
    pub recipient: ::std::string::String,
    #[doc = "An array of RFC-5545 Recurrence Rule for defining when the wait-complete event can be send"]
    pub rrules: Vec<::std::string::String>,
    #[doc = "The ID of the ruleset in the Rule service"]
    pub ruleset: RulesetId,
    #[doc = "The timezone of the message in Olson format"]
    pub timezone: ::std::string::String,
    #[doc = "Indicate whether to use the Subscriber timezone of the default"]
    #[serde(default)]
    pub use_subscriber_timezone: bool,
}
impl ActionsScheduleWaitV1Kwargs {
    pub fn builder() -> builder::ActionsScheduleWaitV1Kwargs {
        Default::default()
    }
}
#[doc = "Stops furthering processing of a ruleset"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Stop\","]
#[doc = "  \"description\": \"Stops furthering processing of a ruleset\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"rulesengine.action.stop\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"rulesengine.action.stop\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"ruleset\","]
#[doc = "        \"subscriber\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"ruleset\": {"]
#[doc = "          \"title\": \"Ruleset\","]
#[doc = "          \"description\": \"The ID of the ruleset to stop\","]
#[doc = "          \"default\": \"<ruleset>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<ruleset>\""]
#[doc = "        },"]
#[doc = "        \"subscriber\": {"]
#[doc = "          \"title\": \"Subscriber\","]
#[doc = "          \"description\": \"The subscriber for which the ruleset is stopped\","]
#[doc = "          \"default\": \"<subscriber>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<subscriber>\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionsStopStop {
    pub function: ::std::string::String,
    pub kwargs: ActionsStopStopKwargs,
}
impl ActionsStopStop {
    pub fn builder() -> builder::ActionsStopStop {
        Default::default()
    }
}
#[doc = "`ActionsStopStopKwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"ruleset\","]
#[doc = "    \"subscriber\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ruleset\": {"]
#[doc = "      \"title\": \"Ruleset\","]
#[doc = "      \"description\": \"The ID of the ruleset to stop\","]
#[doc = "      \"default\": \"<ruleset>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<ruleset>\""]
#[doc = "    },"]
#[doc = "    \"subscriber\": {"]
#[doc = "      \"title\": \"Subscriber\","]
#[doc = "      \"description\": \"The subscriber for which the ruleset is stopped\","]
#[doc = "      \"default\": \"<subscriber>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<subscriber>\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionsStopStopKwargs {
    #[doc = "The ID of the ruleset to stop"]
    pub ruleset: ::std::string::String,
    #[doc = "The subscriber for which the ruleset is stopped"]
    pub subscriber: ::std::string::String,
}
impl ActionsStopStopKwargs {
    pub fn builder() -> builder::ActionsStopStopKwargs {
        Default::default()
    }
}
#[doc = "Tag Action function definitions"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Modify tags\","]
#[doc = "  \"description\": \"Tag Action function definitions\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"ruleset.tag.action.modify_tags_v1\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ruleset.tag.action.modify_tags_v1\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"account\","]
#[doc = "        \"add_labels\","]
#[doc = "        \"list\","]
#[doc = "        \"recipient\","]
#[doc = "        \"remove_labels\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"account\": {"]
#[doc = "          \"title\": \"Account\","]
#[doc = "          \"description\": \"The account to modify tags\","]
#[doc = "          \"default\": \"<event:account>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:account>\""]
#[doc = "        },"]
#[doc = "        \"add_labels\": {"]
#[doc = "          \"title\": \"Tags to apply to the recipient\","]
#[doc = "          \"description\": \"The list of tags to be applied to the recipient\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"uniqueItems\": false"]
#[doc = "        },"]
#[doc = "        \"list\": {"]
#[doc = "          \"title\": \"List\","]
#[doc = "          \"description\": \"The list to modify tags\","]
#[doc = "          \"default\": \"<event:list>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:list>\""]
#[doc = "        },"]
#[doc = "        \"recipient\": {"]
#[doc = "          \"title\": \"Recipient\","]
#[doc = "          \"description\": \"The subscriber to modify tags on\","]
#[doc = "          \"default\": \"<event:recipient>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^<(event:recipient|subscriber)>$\""]
#[doc = "        },"]
#[doc = "        \"remove_labels\": {"]
#[doc = "          \"title\": \"Tags to remove from the recipient\","]
#[doc = "          \"description\": \"The list of tags to be removed from the recipient\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"uniqueItems\": false"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionsTagModifyTagsV1 {
    pub function: ::std::string::String,
    pub kwargs: ActionsTagModifyTagsV1Kwargs,
}
impl ActionsTagModifyTagsV1 {
    pub fn builder() -> builder::ActionsTagModifyTagsV1 {
        Default::default()
    }
}
#[doc = "`ActionsTagModifyTagsV1Kwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"account\","]
#[doc = "    \"add_labels\","]
#[doc = "    \"list\","]
#[doc = "    \"recipient\","]
#[doc = "    \"remove_labels\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"account\": {"]
#[doc = "      \"title\": \"Account\","]
#[doc = "      \"description\": \"The account to modify tags\","]
#[doc = "      \"default\": \"<event:account>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:account>\""]
#[doc = "    },"]
#[doc = "    \"add_labels\": {"]
#[doc = "      \"title\": \"Tags to apply to the recipient\","]
#[doc = "      \"description\": \"The list of tags to be applied to the recipient\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"uniqueItems\": false"]
#[doc = "    },"]
#[doc = "    \"list\": {"]
#[doc = "      \"title\": \"List\","]
#[doc = "      \"description\": \"The list to modify tags\","]
#[doc = "      \"default\": \"<event:list>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:list>\""]
#[doc = "    },"]
#[doc = "    \"recipient\": {"]
#[doc = "      \"title\": \"Recipient\","]
#[doc = "      \"description\": \"The subscriber to modify tags on\","]
#[doc = "      \"default\": \"<event:recipient>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^<(event:recipient|subscriber)>$\""]
#[doc = "    },"]
#[doc = "    \"remove_labels\": {"]
#[doc = "      \"title\": \"Tags to remove from the recipient\","]
#[doc = "      \"description\": \"The list of tags to be removed from the recipient\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"uniqueItems\": false"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionsTagModifyTagsV1Kwargs {
    #[doc = "The account to modify tags"]
    pub account: ::std::string::String,
    #[doc = "The list of tags to be applied to the recipient"]
    pub add_labels: ::std::vec::Vec<::std::string::String>,
    #[doc = "The list to modify tags"]
    pub list: ::std::string::String,
    #[doc = "The subscriber to modify tags on"]
    pub recipient: Recipient,
    #[doc = "The list of tags to be removed from the recipient"]
    pub remove_labels: ::std::vec::Vec<::std::string::String>,
}
impl ActionsTagModifyTagsV1Kwargs {
    pub fn builder() -> builder::ActionsTagModifyTagsV1Kwargs {
        Default::default()
    }
}
#[doc = "Create a draft message and possibly a broadcast from webfeed items."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Created Webfeed message\","]
#[doc = "  \"description\": \"Create a draft message and possibly a broadcast from webfeed items.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"webfeed_campaign.action.create_webfeed_message_v1\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"webfeed_campaign.action.create_webfeed_message_v1\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"account\","]
#[doc = "        \"list\","]
#[doc = "        \"segment\","]
#[doc = "        \"send_broadcast\","]
#[doc = "        \"template\","]
#[doc = "        \"webfeed\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"account\": {"]
#[doc = "          \"title\": \"Account\","]
#[doc = "          \"description\": \"The account to send the email for\","]
#[doc = "          \"default\": \"<event:account>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:account>\""]
#[doc = "        },"]
#[doc = "        \"list\": {"]
#[doc = "          \"title\": \"List\","]
#[doc = "          \"description\": \"The list to send the email for\","]
#[doc = "          \"default\": \"<event:list>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:list>\""]
#[doc = "        },"]
#[doc = "        \"segment\": {"]
#[doc = "          \"title\": \"Segment\","]
#[doc = "          \"description\": \"ID of segment to whom broadcast should be sent.\","]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"minimum\": 1.0"]
#[doc = "        },"]
#[doc = "        \"send_broadcast\": {"]
#[doc = "          \"title\": \"Send broadcast flag..\","]
#[doc = "          \"description\": \"Flag indicating that a broadcast should be sent.\","]
#[doc = "          \"type\": \"boolean\""]
#[doc = "        },"]
#[doc = "        \"template\": {"]
#[doc = "          \"title\": \"Template  message id\","]
#[doc = "          \"description\": \"The message api ObjectID for the message template.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^[0-9a-fA-F]{24}$\""]
#[doc = "        },"]
#[doc = "        \"webfeed\": {"]
#[doc = "          \"title\": \"Webfeed\","]
#[doc = "          \"description\": \"The webfeed to create message from.\","]
#[doc = "          \"default\": \"<event:webfeed>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"oneOf\": ["]
#[doc = "            {"]
#[doc = "              \"type\": \"integer\""]
#[doc = "            },"]
#[doc = "            {"]
#[doc = "              \"const\": \"<event:webfeed>\""]
#[doc = "            }"]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionsWebfeedCreateWebfeedMessageV1 {
    pub function: ::std::string::String,
    pub kwargs: ActionsWebfeedCreateWebfeedMessageV1Kwargs,
}
impl ActionsWebfeedCreateWebfeedMessageV1 {
    pub fn builder() -> builder::ActionsWebfeedCreateWebfeedMessageV1 {
        Default::default()
    }
}
#[doc = "`ActionsWebfeedCreateWebfeedMessageV1Kwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"account\","]
#[doc = "    \"list\","]
#[doc = "    \"segment\","]
#[doc = "    \"send_broadcast\","]
#[doc = "    \"template\","]
#[doc = "    \"webfeed\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"account\": {"]
#[doc = "      \"title\": \"Account\","]
#[doc = "      \"description\": \"The account to send the email for\","]
#[doc = "      \"default\": \"<event:account>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:account>\""]
#[doc = "    },"]
#[doc = "    \"list\": {"]
#[doc = "      \"title\": \"List\","]
#[doc = "      \"description\": \"The list to send the email for\","]
#[doc = "      \"default\": \"<event:list>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:list>\""]
#[doc = "    },"]
#[doc = "    \"segment\": {"]
#[doc = "      \"title\": \"Segment\","]
#[doc = "      \"description\": \"ID of segment to whom broadcast should be sent.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 1.0"]
#[doc = "    },"]
#[doc = "    \"send_broadcast\": {"]
#[doc = "      \"title\": \"Send broadcast flag..\","]
#[doc = "      \"description\": \"Flag indicating that a broadcast should be sent.\","]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"template\": {"]
#[doc = "      \"title\": \"Template  message id\","]
#[doc = "      \"description\": \"The message api ObjectID for the message template.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^[0-9a-fA-F]{24}$\""]
#[doc = "    },"]
#[doc = "    \"webfeed\": {"]
#[doc = "      \"title\": \"Webfeed\","]
#[doc = "      \"description\": \"The webfeed to create message from.\","]
#[doc = "      \"default\": \"<event:webfeed>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"integer\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"const\": \"<event:webfeed>\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ActionsWebfeedCreateWebfeedMessageV1Kwargs {
    #[doc = "The account to send the email for"]
    pub account: ::std::string::String,
    #[doc = "The list to send the email for"]
    pub list: ::std::string::String,
    #[doc = "ID of segment to whom broadcast should be sent."]
    pub segment: ::std::num::NonZeroU64,
    #[doc = "Flag indicating that a broadcast should be sent."]
    pub send_broadcast: bool,
    #[doc = "The message api ObjectID for the message template."]
    pub template: TemplateMessageId,
    #[doc = "The webfeed to create message from."]
    pub webfeed: Webfeed,
}
impl ActionsWebfeedCreateWebfeedMessageV1Kwargs {
    pub fn builder() -> builder::ActionsWebfeedCreateWebfeedMessageV1Kwargs {
        Default::default()
    }
}
#[doc = "The branch that this action belongs to"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Branch ID\","]
#[doc = "  \"description\": \"The branch that this action belongs to\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct BranchId(::std::string::String);
impl ::std::ops::Deref for BranchId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<BranchId> for ::std::string::String {
    fn from(value: BranchId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for BranchId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new(
                    "^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$",
                )
                .unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err ("doesn't match pattern \"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$\"" . into ()) ;
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for BranchId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for BranchId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BranchId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for BranchId {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`BranchesItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/filter\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"id\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"id\": {"]
#[doc = "          \"title\": \"ID\","]
#[doc = "          \"description\": \"The filter or branch ID\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged, deny_unknown_fields)]
pub enum BranchesItem {
    Filter(Filter),
    Object {
        #[doc = "The filter or branch ID"]
        id: ::std::string::String,
    },
}
impl ::std::convert::From<Filter> for BranchesItem {
    fn from(value: Filter) -> Self {
        Self::Filter(value)
    }
}
#[doc = "The campaign that has been changed"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Campaign ID\","]
#[doc = "  \"description\": \"The campaign that has been changed\","]
#[doc = "  \"default\": \"<event:campaign>\","]
#[doc = "  \"readOnly\": true,"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^((<event:campaign>)|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}))$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct CampaignId(::std::string::String);
impl ::std::ops::Deref for CampaignId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<CampaignId> for ::std::string::String {
    fn from(value: CampaignId) -> Self {
        value.0
    }
}
impl ::std::default::Default for CampaignId {
    fn default() -> Self {
        CampaignId("<event:campaign>".to_string())
    }
}
impl ::std::str::FromStr for CampaignId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(
            || {
                :: regress :: Regex :: new ("^((<event:campaign>)|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}))$") . unwrap ()
            },
        );
        if PATTERN.find(value).is_none() {
            return Err ("doesn't match pattern \"^((<event:campaign>)|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}))$\"" . into ()) ;
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for CampaignId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for CampaignId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for CampaignId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for CampaignId {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`ClickUrLsItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"format\": \"uri\","]
#[doc = "  \"pattern\": \"^https?://(.*)$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ClickUrLsItem(::std::string::String);
impl ::std::ops::Deref for ClickUrLsItem {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ClickUrLsItem> for ::std::string::String {
    fn from(value: ClickUrLsItem) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ClickUrLsItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^https?://(.*)$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^https?://(.*)$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ClickUrLsItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ClickUrLsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ClickUrLsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ClickUrLsItem {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Valid ISO 3166-1 alpha-2 country codes"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Country Code\","]
#[doc = "  \"description\": \"Valid ISO 3166-1 alpha-2 country codes\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"AD\","]
#[doc = "    \"AE\","]
#[doc = "    \"AF\","]
#[doc = "    \"AG\","]
#[doc = "    \"AI\","]
#[doc = "    \"AL\","]
#[doc = "    \"AM\","]
#[doc = "    \"AO\","]
#[doc = "    \"AQ\","]
#[doc = "    \"AR\","]
#[doc = "    \"AS\","]
#[doc = "    \"AT\","]
#[doc = "    \"AU\","]
#[doc = "    \"AW\","]
#[doc = "    \"AX\","]
#[doc = "    \"AZ\","]
#[doc = "    \"BA\","]
#[doc = "    \"BB\","]
#[doc = "    \"BD\","]
#[doc = "    \"BE\","]
#[doc = "    \"BF\","]
#[doc = "    \"BG\","]
#[doc = "    \"BH\","]
#[doc = "    \"BI\","]
#[doc = "    \"BJ\","]
#[doc = "    \"BL\","]
#[doc = "    \"BM\","]
#[doc = "    \"BN\","]
#[doc = "    \"BO\","]
#[doc = "    \"BQ\","]
#[doc = "    \"BR\","]
#[doc = "    \"BS\","]
#[doc = "    \"BT\","]
#[doc = "    \"BV\","]
#[doc = "    \"BW\","]
#[doc = "    \"BY\","]
#[doc = "    \"BZ\","]
#[doc = "    \"CA\","]
#[doc = "    \"CC\","]
#[doc = "    \"CD\","]
#[doc = "    \"CF\","]
#[doc = "    \"CG\","]
#[doc = "    \"CH\","]
#[doc = "    \"CI\","]
#[doc = "    \"CK\","]
#[doc = "    \"CL\","]
#[doc = "    \"CM\","]
#[doc = "    \"CN\","]
#[doc = "    \"CO\","]
#[doc = "    \"CR\","]
#[doc = "    \"CU\","]
#[doc = "    \"CV\","]
#[doc = "    \"CW\","]
#[doc = "    \"CX\","]
#[doc = "    \"CY\","]
#[doc = "    \"CZ\","]
#[doc = "    \"DE\","]
#[doc = "    \"DJ\","]
#[doc = "    \"DK\","]
#[doc = "    \"DM\","]
#[doc = "    \"DO\","]
#[doc = "    \"DZ\","]
#[doc = "    \"EC\","]
#[doc = "    \"EE\","]
#[doc = "    \"EG\","]
#[doc = "    \"EH\","]
#[doc = "    \"ER\","]
#[doc = "    \"ES\","]
#[doc = "    \"ET\","]
#[doc = "    \"FI\","]
#[doc = "    \"FJ\","]
#[doc = "    \"FK\","]
#[doc = "    \"FM\","]
#[doc = "    \"FO\","]
#[doc = "    \"FR\","]
#[doc = "    \"GA\","]
#[doc = "    \"GB\","]
#[doc = "    \"GD\","]
#[doc = "    \"GE\","]
#[doc = "    \"GF\","]
#[doc = "    \"GG\","]
#[doc = "    \"GH\","]
#[doc = "    \"GI\","]
#[doc = "    \"GL\","]
#[doc = "    \"GM\","]
#[doc = "    \"GN\","]
#[doc = "    \"GP\","]
#[doc = "    \"GQ\","]
#[doc = "    \"GR\","]
#[doc = "    \"GS\","]
#[doc = "    \"GT\","]
#[doc = "    \"GU\","]
#[doc = "    \"GW\","]
#[doc = "    \"GY\","]
#[doc = "    \"HK\","]
#[doc = "    \"HM\","]
#[doc = "    \"HN\","]
#[doc = "    \"HR\","]
#[doc = "    \"HT\","]
#[doc = "    \"HU\","]
#[doc = "    \"ID\","]
#[doc = "    \"IE\","]
#[doc = "    \"IL\","]
#[doc = "    \"IM\","]
#[doc = "    \"IN\","]
#[doc = "    \"IO\","]
#[doc = "    \"IQ\","]
#[doc = "    \"IR\","]
#[doc = "    \"IS\","]
#[doc = "    \"IT\","]
#[doc = "    \"JE\","]
#[doc = "    \"JM\","]
#[doc = "    \"JO\","]
#[doc = "    \"JP\","]
#[doc = "    \"KE\","]
#[doc = "    \"KG\","]
#[doc = "    \"KH\","]
#[doc = "    \"KI\","]
#[doc = "    \"KM\","]
#[doc = "    \"KN\","]
#[doc = "    \"KP\","]
#[doc = "    \"KR\","]
#[doc = "    \"KW\","]
#[doc = "    \"KY\","]
#[doc = "    \"KZ\","]
#[doc = "    \"LA\","]
#[doc = "    \"LB\","]
#[doc = "    \"LC\","]
#[doc = "    \"LI\","]
#[doc = "    \"LK\","]
#[doc = "    \"LR\","]
#[doc = "    \"LS\","]
#[doc = "    \"LT\","]
#[doc = "    \"LU\","]
#[doc = "    \"LV\","]
#[doc = "    \"LY\","]
#[doc = "    \"MA\","]
#[doc = "    \"MC\","]
#[doc = "    \"MD\","]
#[doc = "    \"ME\","]
#[doc = "    \"MF\","]
#[doc = "    \"MG\","]
#[doc = "    \"MH\","]
#[doc = "    \"MK\","]
#[doc = "    \"ML\","]
#[doc = "    \"MM\","]
#[doc = "    \"MN\","]
#[doc = "    \"MO\","]
#[doc = "    \"MP\","]
#[doc = "    \"MQ\","]
#[doc = "    \"MR\","]
#[doc = "    \"MS\","]
#[doc = "    \"MT\","]
#[doc = "    \"MU\","]
#[doc = "    \"MV\","]
#[doc = "    \"MW\","]
#[doc = "    \"MX\","]
#[doc = "    \"MY\","]
#[doc = "    \"MZ\","]
#[doc = "    \"NA\","]
#[doc = "    \"NC\","]
#[doc = "    \"NE\","]
#[doc = "    \"NF\","]
#[doc = "    \"NG\","]
#[doc = "    \"NI\","]
#[doc = "    \"NL\","]
#[doc = "    \"NO\","]
#[doc = "    \"NP\","]
#[doc = "    \"NR\","]
#[doc = "    \"NU\","]
#[doc = "    \"NZ\","]
#[doc = "    \"OM\","]
#[doc = "    \"PA\","]
#[doc = "    \"PE\","]
#[doc = "    \"PF\","]
#[doc = "    \"PG\","]
#[doc = "    \"PH\","]
#[doc = "    \"PK\","]
#[doc = "    \"PL\","]
#[doc = "    \"PM\","]
#[doc = "    \"PN\","]
#[doc = "    \"PR\","]
#[doc = "    \"PS\","]
#[doc = "    \"PT\","]
#[doc = "    \"PW\","]
#[doc = "    \"PY\","]
#[doc = "    \"QA\","]
#[doc = "    \"RE\","]
#[doc = "    \"RO\","]
#[doc = "    \"RS\","]
#[doc = "    \"RU\","]
#[doc = "    \"RW\","]
#[doc = "    \"SA\","]
#[doc = "    \"SB\","]
#[doc = "    \"SC\","]
#[doc = "    \"SD\","]
#[doc = "    \"SE\","]
#[doc = "    \"SG\","]
#[doc = "    \"SH\","]
#[doc = "    \"SI\","]
#[doc = "    \"SJ\","]
#[doc = "    \"SK\","]
#[doc = "    \"SL\","]
#[doc = "    \"SM\","]
#[doc = "    \"SN\","]
#[doc = "    \"SO\","]
#[doc = "    \"SR\","]
#[doc = "    \"SS\","]
#[doc = "    \"ST\","]
#[doc = "    \"SV\","]
#[doc = "    \"SX\","]
#[doc = "    \"SY\","]
#[doc = "    \"SZ\","]
#[doc = "    \"TC\","]
#[doc = "    \"TD\","]
#[doc = "    \"TF\","]
#[doc = "    \"TG\","]
#[doc = "    \"TH\","]
#[doc = "    \"TJ\","]
#[doc = "    \"TK\","]
#[doc = "    \"TL\","]
#[doc = "    \"TM\","]
#[doc = "    \"TN\","]
#[doc = "    \"TO\","]
#[doc = "    \"TR\","]
#[doc = "    \"TT\","]
#[doc = "    \"TV\","]
#[doc = "    \"TW\","]
#[doc = "    \"TZ\","]
#[doc = "    \"UA\","]
#[doc = "    \"UG\","]
#[doc = "    \"UM\","]
#[doc = "    \"US\","]
#[doc = "    \"UY\","]
#[doc = "    \"UZ\","]
#[doc = "    \"VA\","]
#[doc = "    \"VC\","]
#[doc = "    \"VE\","]
#[doc = "    \"VG\","]
#[doc = "    \"VI\","]
#[doc = "    \"VN\","]
#[doc = "    \"VU\","]
#[doc = "    \"WF\","]
#[doc = "    \"WS\","]
#[doc = "    \"YE\","]
#[doc = "    \"YT\","]
#[doc = "    \"ZA\","]
#[doc = "    \"ZM\","]
#[doc = "    \"ZW\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum Countries {
    #[serde(rename = "AD")]
    Ad,
    #[serde(rename = "AE")]
    Ae,
    #[serde(rename = "AF")]
    Af,
    #[serde(rename = "AG")]
    Ag,
    #[serde(rename = "AI")]
    Ai,
    #[serde(rename = "AL")]
    Al,
    #[serde(rename = "AM")]
    Am,
    #[serde(rename = "AO")]
    Ao,
    #[serde(rename = "AQ")]
    Aq,
    #[serde(rename = "AR")]
    Ar,
    #[serde(rename = "AS")]
    As,
    #[serde(rename = "AT")]
    At,
    #[serde(rename = "AU")]
    Au,
    #[serde(rename = "AW")]
    Aw,
    #[serde(rename = "AX")]
    Ax,
    #[serde(rename = "AZ")]
    Az,
    #[serde(rename = "BA")]
    Ba,
    #[serde(rename = "BB")]
    Bb,
    #[serde(rename = "BD")]
    Bd,
    #[serde(rename = "BE")]
    Be,
    #[serde(rename = "BF")]
    Bf,
    #[serde(rename = "BG")]
    Bg,
    #[serde(rename = "BH")]
    Bh,
    #[serde(rename = "BI")]
    Bi,
    #[serde(rename = "BJ")]
    Bj,
    #[serde(rename = "BL")]
    Bl,
    #[serde(rename = "BM")]
    Bm,
    #[serde(rename = "BN")]
    Bn,
    #[serde(rename = "BO")]
    Bo,
    #[serde(rename = "BQ")]
    Bq,
    #[serde(rename = "BR")]
    Br,
    #[serde(rename = "BS")]
    Bs,
    #[serde(rename = "BT")]
    Bt,
    #[serde(rename = "BV")]
    Bv,
    #[serde(rename = "BW")]
    Bw,
    #[serde(rename = "BY")]
    By,
    #[serde(rename = "BZ")]
    Bz,
    #[serde(rename = "CA")]
    Ca,
    #[serde(rename = "CC")]
    Cc,
    #[serde(rename = "CD")]
    Cd,
    #[serde(rename = "CF")]
    Cf,
    #[serde(rename = "CG")]
    Cg,
    #[serde(rename = "CH")]
    Ch,
    #[serde(rename = "CI")]
    Ci,
    #[serde(rename = "CK")]
    Ck,
    #[serde(rename = "CL")]
    Cl,
    #[serde(rename = "CM")]
    Cm,
    #[serde(rename = "CN")]
    Cn,
    #[serde(rename = "CO")]
    Co,
    #[serde(rename = "CR")]
    Cr,
    #[serde(rename = "CU")]
    Cu,
    #[serde(rename = "CV")]
    Cv,
    #[serde(rename = "CW")]
    Cw,
    #[serde(rename = "CX")]
    Cx,
    #[serde(rename = "CY")]
    Cy,
    #[serde(rename = "CZ")]
    Cz,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "DJ")]
    Dj,
    #[serde(rename = "DK")]
    Dk,
    #[serde(rename = "DM")]
    Dm,
    #[serde(rename = "DO")]
    Do,
    #[serde(rename = "DZ")]
    Dz,
    #[serde(rename = "EC")]
    Ec,
    #[serde(rename = "EE")]
    Ee,
    #[serde(rename = "EG")]
    Eg,
    #[serde(rename = "EH")]
    Eh,
    #[serde(rename = "ER")]
    Er,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "ET")]
    Et,
    #[serde(rename = "FI")]
    Fi,
    #[serde(rename = "FJ")]
    Fj,
    #[serde(rename = "FK")]
    Fk,
    #[serde(rename = "FM")]
    Fm,
    #[serde(rename = "FO")]
    Fo,
    #[serde(rename = "FR")]
    Fr,
    #[serde(rename = "GA")]
    Ga,
    #[serde(rename = "GB")]
    Gb,
    #[serde(rename = "GD")]
    Gd,
    #[serde(rename = "GE")]
    Ge,
    #[serde(rename = "GF")]
    Gf,
    #[serde(rename = "GG")]
    Gg,
    #[serde(rename = "GH")]
    Gh,
    #[serde(rename = "GI")]
    Gi,
    #[serde(rename = "GL")]
    Gl,
    #[serde(rename = "GM")]
    Gm,
    #[serde(rename = "GN")]
    Gn,
    #[serde(rename = "GP")]
    Gp,
    #[serde(rename = "GQ")]
    Gq,
    #[serde(rename = "GR")]
    Gr,
    #[serde(rename = "GS")]
    Gs,
    #[serde(rename = "GT")]
    Gt,
    #[serde(rename = "GU")]
    Gu,
    #[serde(rename = "GW")]
    Gw,
    #[serde(rename = "GY")]
    Gy,
    #[serde(rename = "HK")]
    Hk,
    #[serde(rename = "HM")]
    Hm,
    #[serde(rename = "HN")]
    Hn,
    #[serde(rename = "HR")]
    Hr,
    #[serde(rename = "HT")]
    Ht,
    #[serde(rename = "HU")]
    Hu,
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "IE")]
    Ie,
    #[serde(rename = "IL")]
    Il,
    #[serde(rename = "IM")]
    Im,
    #[serde(rename = "IN")]
    In,
    #[serde(rename = "IO")]
    Io,
    #[serde(rename = "IQ")]
    Iq,
    #[serde(rename = "IR")]
    Ir,
    #[serde(rename = "IS")]
    Is,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "JE")]
    Je,
    #[serde(rename = "JM")]
    Jm,
    #[serde(rename = "JO")]
    Jo,
    #[serde(rename = "JP")]
    Jp,
    #[serde(rename = "KE")]
    Ke,
    #[serde(rename = "KG")]
    Kg,
    #[serde(rename = "KH")]
    Kh,
    #[serde(rename = "KI")]
    Ki,
    #[serde(rename = "KM")]
    Km,
    #[serde(rename = "KN")]
    Kn,
    #[serde(rename = "KP")]
    Kp,
    #[serde(rename = "KR")]
    Kr,
    #[serde(rename = "KW")]
    Kw,
    #[serde(rename = "KY")]
    Ky,
    #[serde(rename = "KZ")]
    Kz,
    #[serde(rename = "LA")]
    La,
    #[serde(rename = "LB")]
    Lb,
    #[serde(rename = "LC")]
    Lc,
    #[serde(rename = "LI")]
    Li,
    #[serde(rename = "LK")]
    Lk,
    #[serde(rename = "LR")]
    Lr,
    #[serde(rename = "LS")]
    Ls,
    #[serde(rename = "LT")]
    Lt,
    #[serde(rename = "LU")]
    Lu,
    #[serde(rename = "LV")]
    Lv,
    #[serde(rename = "LY")]
    Ly,
    #[serde(rename = "MA")]
    Ma,
    #[serde(rename = "MC")]
    Mc,
    #[serde(rename = "MD")]
    Md,
    #[serde(rename = "ME")]
    Me,
    #[serde(rename = "MF")]
    Mf,
    #[serde(rename = "MG")]
    Mg,
    #[serde(rename = "MH")]
    Mh,
    #[serde(rename = "MK")]
    Mk,
    #[serde(rename = "ML")]
    Ml,
    #[serde(rename = "MM")]
    Mm,
    #[serde(rename = "MN")]
    Mn,
    #[serde(rename = "MO")]
    Mo,
    #[serde(rename = "MP")]
    Mp,
    #[serde(rename = "MQ")]
    Mq,
    #[serde(rename = "MR")]
    Mr,
    #[serde(rename = "MS")]
    Ms,
    #[serde(rename = "MT")]
    Mt,
    #[serde(rename = "MU")]
    Mu,
    #[serde(rename = "MV")]
    Mv,
    #[serde(rename = "MW")]
    Mw,
    #[serde(rename = "MX")]
    Mx,
    #[serde(rename = "MY")]
    My,
    #[serde(rename = "MZ")]
    Mz,
    #[serde(rename = "NA")]
    Na,
    #[serde(rename = "NC")]
    Nc,
    #[serde(rename = "NE")]
    Ne,
    #[serde(rename = "NF")]
    Nf,
    #[serde(rename = "NG")]
    Ng,
    #[serde(rename = "NI")]
    Ni,
    #[serde(rename = "NL")]
    Nl,
    #[serde(rename = "NO")]
    No,
    #[serde(rename = "NP")]
    Np,
    #[serde(rename = "NR")]
    Nr,
    #[serde(rename = "NU")]
    Nu,
    #[serde(rename = "NZ")]
    Nz,
    #[serde(rename = "OM")]
    Om,
    #[serde(rename = "PA")]
    Pa,
    #[serde(rename = "PE")]
    Pe,
    #[serde(rename = "PF")]
    Pf,
    #[serde(rename = "PG")]
    Pg,
    #[serde(rename = "PH")]
    Ph,
    #[serde(rename = "PK")]
    Pk,
    #[serde(rename = "PL")]
    Pl,
    #[serde(rename = "PM")]
    Pm,
    #[serde(rename = "PN")]
    Pn,
    #[serde(rename = "PR")]
    Pr,
    #[serde(rename = "PS")]
    Ps,
    #[serde(rename = "PT")]
    Pt,
    #[serde(rename = "PW")]
    Pw,
    #[serde(rename = "PY")]
    Py,
    #[serde(rename = "QA")]
    Qa,
    #[serde(rename = "RE")]
    Re,
    #[serde(rename = "RO")]
    Ro,
    #[serde(rename = "RS")]
    Rs,
    #[serde(rename = "RU")]
    Ru,
    #[serde(rename = "RW")]
    Rw,
    #[serde(rename = "SA")]
    Sa,
    #[serde(rename = "SB")]
    Sb,
    #[serde(rename = "SC")]
    Sc,
    #[serde(rename = "SD")]
    Sd,
    #[serde(rename = "SE")]
    Se,
    #[serde(rename = "SG")]
    Sg,
    #[serde(rename = "SH")]
    Sh,
    #[serde(rename = "SI")]
    Si,
    #[serde(rename = "SJ")]
    Sj,
    #[serde(rename = "SK")]
    Sk,
    #[serde(rename = "SL")]
    Sl,
    #[serde(rename = "SM")]
    Sm,
    #[serde(rename = "SN")]
    Sn,
    #[serde(rename = "SO")]
    So,
    #[serde(rename = "SR")]
    Sr,
    #[serde(rename = "SS")]
    Ss,
    #[serde(rename = "ST")]
    St,
    #[serde(rename = "SV")]
    Sv,
    #[serde(rename = "SX")]
    Sx,
    #[serde(rename = "SY")]
    Sy,
    #[serde(rename = "SZ")]
    Sz,
    #[serde(rename = "TC")]
    Tc,
    #[serde(rename = "TD")]
    Td,
    #[serde(rename = "TF")]
    Tf,
    #[serde(rename = "TG")]
    Tg,
    #[serde(rename = "TH")]
    Th,
    #[serde(rename = "TJ")]
    Tj,
    #[serde(rename = "TK")]
    Tk,
    #[serde(rename = "TL")]
    Tl,
    #[serde(rename = "TM")]
    Tm,
    #[serde(rename = "TN")]
    Tn,
    #[serde(rename = "TO")]
    To,
    #[serde(rename = "TR")]
    Tr,
    #[serde(rename = "TT")]
    Tt,
    #[serde(rename = "TV")]
    Tv,
    #[serde(rename = "TW")]
    Tw,
    #[serde(rename = "TZ")]
    Tz,
    #[serde(rename = "UA")]
    Ua,
    #[serde(rename = "UG")]
    Ug,
    #[serde(rename = "UM")]
    Um,
    #[serde(rename = "US")]
    Us,
    #[serde(rename = "UY")]
    Uy,
    #[serde(rename = "UZ")]
    Uz,
    #[serde(rename = "VA")]
    Va,
    #[serde(rename = "VC")]
    Vc,
    #[serde(rename = "VE")]
    Ve,
    #[serde(rename = "VG")]
    Vg,
    #[serde(rename = "VI")]
    Vi,
    #[serde(rename = "VN")]
    Vn,
    #[serde(rename = "VU")]
    Vu,
    #[serde(rename = "WF")]
    Wf,
    #[serde(rename = "WS")]
    Ws,
    #[serde(rename = "YE")]
    Ye,
    #[serde(rename = "YT")]
    Yt,
    #[serde(rename = "ZA")]
    Za,
    #[serde(rename = "ZM")]
    Zm,
    #[serde(rename = "ZW")]
    Zw,
}
impl ::std::fmt::Display for Countries {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Ad => f.write_str("AD"),
            Self::Ae => f.write_str("AE"),
            Self::Af => f.write_str("AF"),
            Self::Ag => f.write_str("AG"),
            Self::Ai => f.write_str("AI"),
            Self::Al => f.write_str("AL"),
            Self::Am => f.write_str("AM"),
            Self::Ao => f.write_str("AO"),
            Self::Aq => f.write_str("AQ"),
            Self::Ar => f.write_str("AR"),
            Self::As => f.write_str("AS"),
            Self::At => f.write_str("AT"),
            Self::Au => f.write_str("AU"),
            Self::Aw => f.write_str("AW"),
            Self::Ax => f.write_str("AX"),
            Self::Az => f.write_str("AZ"),
            Self::Ba => f.write_str("BA"),
            Self::Bb => f.write_str("BB"),
            Self::Bd => f.write_str("BD"),
            Self::Be => f.write_str("BE"),
            Self::Bf => f.write_str("BF"),
            Self::Bg => f.write_str("BG"),
            Self::Bh => f.write_str("BH"),
            Self::Bi => f.write_str("BI"),
            Self::Bj => f.write_str("BJ"),
            Self::Bl => f.write_str("BL"),
            Self::Bm => f.write_str("BM"),
            Self::Bn => f.write_str("BN"),
            Self::Bo => f.write_str("BO"),
            Self::Bq => f.write_str("BQ"),
            Self::Br => f.write_str("BR"),
            Self::Bs => f.write_str("BS"),
            Self::Bt => f.write_str("BT"),
            Self::Bv => f.write_str("BV"),
            Self::Bw => f.write_str("BW"),
            Self::By => f.write_str("BY"),
            Self::Bz => f.write_str("BZ"),
            Self::Ca => f.write_str("CA"),
            Self::Cc => f.write_str("CC"),
            Self::Cd => f.write_str("CD"),
            Self::Cf => f.write_str("CF"),
            Self::Cg => f.write_str("CG"),
            Self::Ch => f.write_str("CH"),
            Self::Ci => f.write_str("CI"),
            Self::Ck => f.write_str("CK"),
            Self::Cl => f.write_str("CL"),
            Self::Cm => f.write_str("CM"),
            Self::Cn => f.write_str("CN"),
            Self::Co => f.write_str("CO"),
            Self::Cr => f.write_str("CR"),
            Self::Cu => f.write_str("CU"),
            Self::Cv => f.write_str("CV"),
            Self::Cw => f.write_str("CW"),
            Self::Cx => f.write_str("CX"),
            Self::Cy => f.write_str("CY"),
            Self::Cz => f.write_str("CZ"),
            Self::De => f.write_str("DE"),
            Self::Dj => f.write_str("DJ"),
            Self::Dk => f.write_str("DK"),
            Self::Dm => f.write_str("DM"),
            Self::Do => f.write_str("DO"),
            Self::Dz => f.write_str("DZ"),
            Self::Ec => f.write_str("EC"),
            Self::Ee => f.write_str("EE"),
            Self::Eg => f.write_str("EG"),
            Self::Eh => f.write_str("EH"),
            Self::Er => f.write_str("ER"),
            Self::Es => f.write_str("ES"),
            Self::Et => f.write_str("ET"),
            Self::Fi => f.write_str("FI"),
            Self::Fj => f.write_str("FJ"),
            Self::Fk => f.write_str("FK"),
            Self::Fm => f.write_str("FM"),
            Self::Fo => f.write_str("FO"),
            Self::Fr => f.write_str("FR"),
            Self::Ga => f.write_str("GA"),
            Self::Gb => f.write_str("GB"),
            Self::Gd => f.write_str("GD"),
            Self::Ge => f.write_str("GE"),
            Self::Gf => f.write_str("GF"),
            Self::Gg => f.write_str("GG"),
            Self::Gh => f.write_str("GH"),
            Self::Gi => f.write_str("GI"),
            Self::Gl => f.write_str("GL"),
            Self::Gm => f.write_str("GM"),
            Self::Gn => f.write_str("GN"),
            Self::Gp => f.write_str("GP"),
            Self::Gq => f.write_str("GQ"),
            Self::Gr => f.write_str("GR"),
            Self::Gs => f.write_str("GS"),
            Self::Gt => f.write_str("GT"),
            Self::Gu => f.write_str("GU"),
            Self::Gw => f.write_str("GW"),
            Self::Gy => f.write_str("GY"),
            Self::Hk => f.write_str("HK"),
            Self::Hm => f.write_str("HM"),
            Self::Hn => f.write_str("HN"),
            Self::Hr => f.write_str("HR"),
            Self::Ht => f.write_str("HT"),
            Self::Hu => f.write_str("HU"),
            Self::Id => f.write_str("ID"),
            Self::Ie => f.write_str("IE"),
            Self::Il => f.write_str("IL"),
            Self::Im => f.write_str("IM"),
            Self::In => f.write_str("IN"),
            Self::Io => f.write_str("IO"),
            Self::Iq => f.write_str("IQ"),
            Self::Ir => f.write_str("IR"),
            Self::Is => f.write_str("IS"),
            Self::It => f.write_str("IT"),
            Self::Je => f.write_str("JE"),
            Self::Jm => f.write_str("JM"),
            Self::Jo => f.write_str("JO"),
            Self::Jp => f.write_str("JP"),
            Self::Ke => f.write_str("KE"),
            Self::Kg => f.write_str("KG"),
            Self::Kh => f.write_str("KH"),
            Self::Ki => f.write_str("KI"),
            Self::Km => f.write_str("KM"),
            Self::Kn => f.write_str("KN"),
            Self::Kp => f.write_str("KP"),
            Self::Kr => f.write_str("KR"),
            Self::Kw => f.write_str("KW"),
            Self::Ky => f.write_str("KY"),
            Self::Kz => f.write_str("KZ"),
            Self::La => f.write_str("LA"),
            Self::Lb => f.write_str("LB"),
            Self::Lc => f.write_str("LC"),
            Self::Li => f.write_str("LI"),
            Self::Lk => f.write_str("LK"),
            Self::Lr => f.write_str("LR"),
            Self::Ls => f.write_str("LS"),
            Self::Lt => f.write_str("LT"),
            Self::Lu => f.write_str("LU"),
            Self::Lv => f.write_str("LV"),
            Self::Ly => f.write_str("LY"),
            Self::Ma => f.write_str("MA"),
            Self::Mc => f.write_str("MC"),
            Self::Md => f.write_str("MD"),
            Self::Me => f.write_str("ME"),
            Self::Mf => f.write_str("MF"),
            Self::Mg => f.write_str("MG"),
            Self::Mh => f.write_str("MH"),
            Self::Mk => f.write_str("MK"),
            Self::Ml => f.write_str("ML"),
            Self::Mm => f.write_str("MM"),
            Self::Mn => f.write_str("MN"),
            Self::Mo => f.write_str("MO"),
            Self::Mp => f.write_str("MP"),
            Self::Mq => f.write_str("MQ"),
            Self::Mr => f.write_str("MR"),
            Self::Ms => f.write_str("MS"),
            Self::Mt => f.write_str("MT"),
            Self::Mu => f.write_str("MU"),
            Self::Mv => f.write_str("MV"),
            Self::Mw => f.write_str("MW"),
            Self::Mx => f.write_str("MX"),
            Self::My => f.write_str("MY"),
            Self::Mz => f.write_str("MZ"),
            Self::Na => f.write_str("NA"),
            Self::Nc => f.write_str("NC"),
            Self::Ne => f.write_str("NE"),
            Self::Nf => f.write_str("NF"),
            Self::Ng => f.write_str("NG"),
            Self::Ni => f.write_str("NI"),
            Self::Nl => f.write_str("NL"),
            Self::No => f.write_str("NO"),
            Self::Np => f.write_str("NP"),
            Self::Nr => f.write_str("NR"),
            Self::Nu => f.write_str("NU"),
            Self::Nz => f.write_str("NZ"),
            Self::Om => f.write_str("OM"),
            Self::Pa => f.write_str("PA"),
            Self::Pe => f.write_str("PE"),
            Self::Pf => f.write_str("PF"),
            Self::Pg => f.write_str("PG"),
            Self::Ph => f.write_str("PH"),
            Self::Pk => f.write_str("PK"),
            Self::Pl => f.write_str("PL"),
            Self::Pm => f.write_str("PM"),
            Self::Pn => f.write_str("PN"),
            Self::Pr => f.write_str("PR"),
            Self::Ps => f.write_str("PS"),
            Self::Pt => f.write_str("PT"),
            Self::Pw => f.write_str("PW"),
            Self::Py => f.write_str("PY"),
            Self::Qa => f.write_str("QA"),
            Self::Re => f.write_str("RE"),
            Self::Ro => f.write_str("RO"),
            Self::Rs => f.write_str("RS"),
            Self::Ru => f.write_str("RU"),
            Self::Rw => f.write_str("RW"),
            Self::Sa => f.write_str("SA"),
            Self::Sb => f.write_str("SB"),
            Self::Sc => f.write_str("SC"),
            Self::Sd => f.write_str("SD"),
            Self::Se => f.write_str("SE"),
            Self::Sg => f.write_str("SG"),
            Self::Sh => f.write_str("SH"),
            Self::Si => f.write_str("SI"),
            Self::Sj => f.write_str("SJ"),
            Self::Sk => f.write_str("SK"),
            Self::Sl => f.write_str("SL"),
            Self::Sm => f.write_str("SM"),
            Self::Sn => f.write_str("SN"),
            Self::So => f.write_str("SO"),
            Self::Sr => f.write_str("SR"),
            Self::Ss => f.write_str("SS"),
            Self::St => f.write_str("ST"),
            Self::Sv => f.write_str("SV"),
            Self::Sx => f.write_str("SX"),
            Self::Sy => f.write_str("SY"),
            Self::Sz => f.write_str("SZ"),
            Self::Tc => f.write_str("TC"),
            Self::Td => f.write_str("TD"),
            Self::Tf => f.write_str("TF"),
            Self::Tg => f.write_str("TG"),
            Self::Th => f.write_str("TH"),
            Self::Tj => f.write_str("TJ"),
            Self::Tk => f.write_str("TK"),
            Self::Tl => f.write_str("TL"),
            Self::Tm => f.write_str("TM"),
            Self::Tn => f.write_str("TN"),
            Self::To => f.write_str("TO"),
            Self::Tr => f.write_str("TR"),
            Self::Tt => f.write_str("TT"),
            Self::Tv => f.write_str("TV"),
            Self::Tw => f.write_str("TW"),
            Self::Tz => f.write_str("TZ"),
            Self::Ua => f.write_str("UA"),
            Self::Ug => f.write_str("UG"),
            Self::Um => f.write_str("UM"),
            Self::Us => f.write_str("US"),
            Self::Uy => f.write_str("UY"),
            Self::Uz => f.write_str("UZ"),
            Self::Va => f.write_str("VA"),
            Self::Vc => f.write_str("VC"),
            Self::Ve => f.write_str("VE"),
            Self::Vg => f.write_str("VG"),
            Self::Vi => f.write_str("VI"),
            Self::Vn => f.write_str("VN"),
            Self::Vu => f.write_str("VU"),
            Self::Wf => f.write_str("WF"),
            Self::Ws => f.write_str("WS"),
            Self::Ye => f.write_str("YE"),
            Self::Yt => f.write_str("YT"),
            Self::Za => f.write_str("ZA"),
            Self::Zm => f.write_str("ZM"),
            Self::Zw => f.write_str("ZW"),
        }
    }
}
impl ::std::str::FromStr for Countries {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "AD" => Ok(Self::Ad),
            "AE" => Ok(Self::Ae),
            "AF" => Ok(Self::Af),
            "AG" => Ok(Self::Ag),
            "AI" => Ok(Self::Ai),
            "AL" => Ok(Self::Al),
            "AM" => Ok(Self::Am),
            "AO" => Ok(Self::Ao),
            "AQ" => Ok(Self::Aq),
            "AR" => Ok(Self::Ar),
            "AS" => Ok(Self::As),
            "AT" => Ok(Self::At),
            "AU" => Ok(Self::Au),
            "AW" => Ok(Self::Aw),
            "AX" => Ok(Self::Ax),
            "AZ" => Ok(Self::Az),
            "BA" => Ok(Self::Ba),
            "BB" => Ok(Self::Bb),
            "BD" => Ok(Self::Bd),
            "BE" => Ok(Self::Be),
            "BF" => Ok(Self::Bf),
            "BG" => Ok(Self::Bg),
            "BH" => Ok(Self::Bh),
            "BI" => Ok(Self::Bi),
            "BJ" => Ok(Self::Bj),
            "BL" => Ok(Self::Bl),
            "BM" => Ok(Self::Bm),
            "BN" => Ok(Self::Bn),
            "BO" => Ok(Self::Bo),
            "BQ" => Ok(Self::Bq),
            "BR" => Ok(Self::Br),
            "BS" => Ok(Self::Bs),
            "BT" => Ok(Self::Bt),
            "BV" => Ok(Self::Bv),
            "BW" => Ok(Self::Bw),
            "BY" => Ok(Self::By),
            "BZ" => Ok(Self::Bz),
            "CA" => Ok(Self::Ca),
            "CC" => Ok(Self::Cc),
            "CD" => Ok(Self::Cd),
            "CF" => Ok(Self::Cf),
            "CG" => Ok(Self::Cg),
            "CH" => Ok(Self::Ch),
            "CI" => Ok(Self::Ci),
            "CK" => Ok(Self::Ck),
            "CL" => Ok(Self::Cl),
            "CM" => Ok(Self::Cm),
            "CN" => Ok(Self::Cn),
            "CO" => Ok(Self::Co),
            "CR" => Ok(Self::Cr),
            "CU" => Ok(Self::Cu),
            "CV" => Ok(Self::Cv),
            "CW" => Ok(Self::Cw),
            "CX" => Ok(Self::Cx),
            "CY" => Ok(Self::Cy),
            "CZ" => Ok(Self::Cz),
            "DE" => Ok(Self::De),
            "DJ" => Ok(Self::Dj),
            "DK" => Ok(Self::Dk),
            "DM" => Ok(Self::Dm),
            "DO" => Ok(Self::Do),
            "DZ" => Ok(Self::Dz),
            "EC" => Ok(Self::Ec),
            "EE" => Ok(Self::Ee),
            "EG" => Ok(Self::Eg),
            "EH" => Ok(Self::Eh),
            "ER" => Ok(Self::Er),
            "ES" => Ok(Self::Es),
            "ET" => Ok(Self::Et),
            "FI" => Ok(Self::Fi),
            "FJ" => Ok(Self::Fj),
            "FK" => Ok(Self::Fk),
            "FM" => Ok(Self::Fm),
            "FO" => Ok(Self::Fo),
            "FR" => Ok(Self::Fr),
            "GA" => Ok(Self::Ga),
            "GB" => Ok(Self::Gb),
            "GD" => Ok(Self::Gd),
            "GE" => Ok(Self::Ge),
            "GF" => Ok(Self::Gf),
            "GG" => Ok(Self::Gg),
            "GH" => Ok(Self::Gh),
            "GI" => Ok(Self::Gi),
            "GL" => Ok(Self::Gl),
            "GM" => Ok(Self::Gm),
            "GN" => Ok(Self::Gn),
            "GP" => Ok(Self::Gp),
            "GQ" => Ok(Self::Gq),
            "GR" => Ok(Self::Gr),
            "GS" => Ok(Self::Gs),
            "GT" => Ok(Self::Gt),
            "GU" => Ok(Self::Gu),
            "GW" => Ok(Self::Gw),
            "GY" => Ok(Self::Gy),
            "HK" => Ok(Self::Hk),
            "HM" => Ok(Self::Hm),
            "HN" => Ok(Self::Hn),
            "HR" => Ok(Self::Hr),
            "HT" => Ok(Self::Ht),
            "HU" => Ok(Self::Hu),
            "ID" => Ok(Self::Id),
            "IE" => Ok(Self::Ie),
            "IL" => Ok(Self::Il),
            "IM" => Ok(Self::Im),
            "IN" => Ok(Self::In),
            "IO" => Ok(Self::Io),
            "IQ" => Ok(Self::Iq),
            "IR" => Ok(Self::Ir),
            "IS" => Ok(Self::Is),
            "IT" => Ok(Self::It),
            "JE" => Ok(Self::Je),
            "JM" => Ok(Self::Jm),
            "JO" => Ok(Self::Jo),
            "JP" => Ok(Self::Jp),
            "KE" => Ok(Self::Ke),
            "KG" => Ok(Self::Kg),
            "KH" => Ok(Self::Kh),
            "KI" => Ok(Self::Ki),
            "KM" => Ok(Self::Km),
            "KN" => Ok(Self::Kn),
            "KP" => Ok(Self::Kp),
            "KR" => Ok(Self::Kr),
            "KW" => Ok(Self::Kw),
            "KY" => Ok(Self::Ky),
            "KZ" => Ok(Self::Kz),
            "LA" => Ok(Self::La),
            "LB" => Ok(Self::Lb),
            "LC" => Ok(Self::Lc),
            "LI" => Ok(Self::Li),
            "LK" => Ok(Self::Lk),
            "LR" => Ok(Self::Lr),
            "LS" => Ok(Self::Ls),
            "LT" => Ok(Self::Lt),
            "LU" => Ok(Self::Lu),
            "LV" => Ok(Self::Lv),
            "LY" => Ok(Self::Ly),
            "MA" => Ok(Self::Ma),
            "MC" => Ok(Self::Mc),
            "MD" => Ok(Self::Md),
            "ME" => Ok(Self::Me),
            "MF" => Ok(Self::Mf),
            "MG" => Ok(Self::Mg),
            "MH" => Ok(Self::Mh),
            "MK" => Ok(Self::Mk),
            "ML" => Ok(Self::Ml),
            "MM" => Ok(Self::Mm),
            "MN" => Ok(Self::Mn),
            "MO" => Ok(Self::Mo),
            "MP" => Ok(Self::Mp),
            "MQ" => Ok(Self::Mq),
            "MR" => Ok(Self::Mr),
            "MS" => Ok(Self::Ms),
            "MT" => Ok(Self::Mt),
            "MU" => Ok(Self::Mu),
            "MV" => Ok(Self::Mv),
            "MW" => Ok(Self::Mw),
            "MX" => Ok(Self::Mx),
            "MY" => Ok(Self::My),
            "MZ" => Ok(Self::Mz),
            "NA" => Ok(Self::Na),
            "NC" => Ok(Self::Nc),
            "NE" => Ok(Self::Ne),
            "NF" => Ok(Self::Nf),
            "NG" => Ok(Self::Ng),
            "NI" => Ok(Self::Ni),
            "NL" => Ok(Self::Nl),
            "NO" => Ok(Self::No),
            "NP" => Ok(Self::Np),
            "NR" => Ok(Self::Nr),
            "NU" => Ok(Self::Nu),
            "NZ" => Ok(Self::Nz),
            "OM" => Ok(Self::Om),
            "PA" => Ok(Self::Pa),
            "PE" => Ok(Self::Pe),
            "PF" => Ok(Self::Pf),
            "PG" => Ok(Self::Pg),
            "PH" => Ok(Self::Ph),
            "PK" => Ok(Self::Pk),
            "PL" => Ok(Self::Pl),
            "PM" => Ok(Self::Pm),
            "PN" => Ok(Self::Pn),
            "PR" => Ok(Self::Pr),
            "PS" => Ok(Self::Ps),
            "PT" => Ok(Self::Pt),
            "PW" => Ok(Self::Pw),
            "PY" => Ok(Self::Py),
            "QA" => Ok(Self::Qa),
            "RE" => Ok(Self::Re),
            "RO" => Ok(Self::Ro),
            "RS" => Ok(Self::Rs),
            "RU" => Ok(Self::Ru),
            "RW" => Ok(Self::Rw),
            "SA" => Ok(Self::Sa),
            "SB" => Ok(Self::Sb),
            "SC" => Ok(Self::Sc),
            "SD" => Ok(Self::Sd),
            "SE" => Ok(Self::Se),
            "SG" => Ok(Self::Sg),
            "SH" => Ok(Self::Sh),
            "SI" => Ok(Self::Si),
            "SJ" => Ok(Self::Sj),
            "SK" => Ok(Self::Sk),
            "SL" => Ok(Self::Sl),
            "SM" => Ok(Self::Sm),
            "SN" => Ok(Self::Sn),
            "SO" => Ok(Self::So),
            "SR" => Ok(Self::Sr),
            "SS" => Ok(Self::Ss),
            "ST" => Ok(Self::St),
            "SV" => Ok(Self::Sv),
            "SX" => Ok(Self::Sx),
            "SY" => Ok(Self::Sy),
            "SZ" => Ok(Self::Sz),
            "TC" => Ok(Self::Tc),
            "TD" => Ok(Self::Td),
            "TF" => Ok(Self::Tf),
            "TG" => Ok(Self::Tg),
            "TH" => Ok(Self::Th),
            "TJ" => Ok(Self::Tj),
            "TK" => Ok(Self::Tk),
            "TL" => Ok(Self::Tl),
            "TM" => Ok(Self::Tm),
            "TN" => Ok(Self::Tn),
            "TO" => Ok(Self::To),
            "TR" => Ok(Self::Tr),
            "TT" => Ok(Self::Tt),
            "TV" => Ok(Self::Tv),
            "TW" => Ok(Self::Tw),
            "TZ" => Ok(Self::Tz),
            "UA" => Ok(Self::Ua),
            "UG" => Ok(Self::Ug),
            "UM" => Ok(Self::Um),
            "US" => Ok(Self::Us),
            "UY" => Ok(Self::Uy),
            "UZ" => Ok(Self::Uz),
            "VA" => Ok(Self::Va),
            "VC" => Ok(Self::Vc),
            "VE" => Ok(Self::Ve),
            "VG" => Ok(Self::Vg),
            "VI" => Ok(Self::Vi),
            "VN" => Ok(Self::Vn),
            "VU" => Ok(Self::Vu),
            "WF" => Ok(Self::Wf),
            "WS" => Ok(Self::Ws),
            "YE" => Ok(Self::Ye),
            "YT" => Ok(Self::Yt),
            "ZA" => Ok(Self::Za),
            "ZM" => Ok(Self::Zm),
            "ZW" => Ok(Self::Zw),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Countries {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Countries {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Countries {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Returns true if any of the URLs in a list of messages contains a fragment"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Message Click URL Contains\","]
#[doc = "  \"description\": \"Returns true if any of the URLs in a list of messages contains a fragment\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"expect\","]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\","]
#[doc = "    \"operator\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"expect\": {"]
#[doc = "      \"title\": \"Return Expectation\","]
#[doc = "      \"description\": \"The expected return value from the function\","]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"ruleset.analytics.filter.any_message_click_url_contains_v1\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ruleset.analytics.filter.any_message_click_url_contains_v1\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"account\","]
#[doc = "        \"fragments\","]
#[doc = "        \"messages\","]
#[doc = "        \"subscriber\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"account\": {"]
#[doc = "          \"title\": \"Account\","]
#[doc = "          \"description\": \"The account ID\","]
#[doc = "          \"default\": \"<event:account>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:account>\""]
#[doc = "        },"]
#[doc = "        \"fragments\": {"]
#[doc = "          \"title\": \"URL Fragments\","]
#[doc = "          \"description\": \"The list of URL fragments to check for in click URLs by the subscriber\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"minLength\": 1"]
#[doc = "          },"]
#[doc = "          \"minItems\": 1"]
#[doc = "        },"]
#[doc = "        \"messages\": {"]
#[doc = "          \"title\": \"Messages\","]
#[doc = "          \"description\": \"The list of messages previously sent to the subscriber in the ruleset\","]
#[doc = "          \"default\": \"<messages:sent>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<messages:sent>\""]
#[doc = "        },"]
#[doc = "        \"subscriber\": {"]
#[doc = "          \"title\": \"Subscriber\","]
#[doc = "          \"description\": \"The subscriber ID\","]
#[doc = "          \"default\": \"<event:subscriber>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:subscriber>\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"memoize_id\": {"]
#[doc = "      \"title\": \"Memoize ID\","]
#[doc = "      \"description\": \"A unique ID to use to memoize the result of the function\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12})$\""]
#[doc = "    },"]
#[doc = "    \"operator\": {"]
#[doc = "      \"$ref\": \"#/$defs/operators\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CriteriaAnalyticsAnyMessageClickUrlContainsV1 {
    #[doc = "The expected return value from the function"]
    pub expect: bool,
    pub function: ::std::string::String,
    pub kwargs: CriteriaAnalyticsAnyMessageClickUrlContainsV1Kwargs,
    #[doc = "A unique ID to use to memoize the result of the function"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub memoize_id: ::std::option::Option<MemoizeId>,
    pub operator: Operators,
}
impl CriteriaAnalyticsAnyMessageClickUrlContainsV1 {
    pub fn builder() -> builder::CriteriaAnalyticsAnyMessageClickUrlContainsV1 {
        Default::default()
    }
}
#[doc = "`CriteriaAnalyticsAnyMessageClickUrlContainsV1Kwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"account\","]
#[doc = "    \"fragments\","]
#[doc = "    \"messages\","]
#[doc = "    \"subscriber\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"account\": {"]
#[doc = "      \"title\": \"Account\","]
#[doc = "      \"description\": \"The account ID\","]
#[doc = "      \"default\": \"<event:account>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:account>\""]
#[doc = "    },"]
#[doc = "    \"fragments\": {"]
#[doc = "      \"title\": \"URL Fragments\","]
#[doc = "      \"description\": \"The list of URL fragments to check for in click URLs by the subscriber\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\","]
#[doc = "        \"minLength\": 1"]
#[doc = "      },"]
#[doc = "      \"minItems\": 1"]
#[doc = "    },"]
#[doc = "    \"messages\": {"]
#[doc = "      \"title\": \"Messages\","]
#[doc = "      \"description\": \"The list of messages previously sent to the subscriber in the ruleset\","]
#[doc = "      \"default\": \"<messages:sent>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<messages:sent>\""]
#[doc = "    },"]
#[doc = "    \"subscriber\": {"]
#[doc = "      \"title\": \"Subscriber\","]
#[doc = "      \"description\": \"The subscriber ID\","]
#[doc = "      \"default\": \"<event:subscriber>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:subscriber>\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CriteriaAnalyticsAnyMessageClickUrlContainsV1Kwargs {
    #[doc = "The account ID"]
    pub account: ::std::string::String,
    #[doc = "The list of URL fragments to check for in click URLs by the subscriber"]
    pub fragments: ::std::vec::Vec<UrlFragmentsItem>,
    #[doc = "The list of messages previously sent to the subscriber in the ruleset"]
    pub messages: ::std::string::String,
    #[doc = "The subscriber ID"]
    pub subscriber: ::std::string::String,
}
impl CriteriaAnalyticsAnyMessageClickUrlContainsV1Kwargs {
    pub fn builder() -> builder::CriteriaAnalyticsAnyMessageClickUrlContainsV1Kwargs {
        Default::default()
    }
}
#[doc = "Returns true if any of the URLs in a list of messages have been clicked by the subscriber"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Message URLs Clicked\","]
#[doc = "  \"description\": \"Returns true if any of the URLs in a list of messages have been clicked by the subscriber\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"expect\","]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\","]
#[doc = "    \"operator\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"expect\": {"]
#[doc = "      \"title\": \"Return Expectation\","]
#[doc = "      \"description\": \"The expected return value from the function\","]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"ruleset.analytics.filter.any_message_click_urls_v1\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ruleset.analytics.filter.any_message_click_urls_v1\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"account\","]
#[doc = "        \"messages\","]
#[doc = "        \"subscriber\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"account\": {"]
#[doc = "          \"title\": \"Account\","]
#[doc = "          \"description\": \"The account ID\","]
#[doc = "          \"default\": \"<event:account>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:account>\""]
#[doc = "        },"]
#[doc = "        \"click_urls\": {"]
#[doc = "          \"title\": \"Click URLs\","]
#[doc = "          \"description\": \"The list of URLs to check for clicks\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\","]
#[doc = "            \"format\": \"uri\","]
#[doc = "            \"pattern\": \"^https?://(.*)$\""]
#[doc = "          }"]
#[doc = "        },"]
#[doc = "        \"messages\": {"]
#[doc = "          \"title\": \"Messages\","]
#[doc = "          \"description\": \"The list of messages previously sent to the subscriber in the ruleset\","]
#[doc = "          \"default\": \"<messages:sent>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<messages:sent>\""]
#[doc = "        },"]
#[doc = "        \"subscriber\": {"]
#[doc = "          \"title\": \"Subscriber\","]
#[doc = "          \"description\": \"The subscriber ID\","]
#[doc = "          \"default\": \"<event:subscriber>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:subscriber>\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"memoize_id\": {"]
#[doc = "      \"title\": \"Memoize ID\","]
#[doc = "      \"description\": \"A unique ID to use to memoize the result of the function\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12})$\""]
#[doc = "    },"]
#[doc = "    \"operator\": {"]
#[doc = "      \"$ref\": \"#/$defs/operators\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CriteriaAnalyticsAnyMessageClickUrlsV1 {
    #[doc = "The expected return value from the function"]
    pub expect: bool,
    pub function: ::std::string::String,
    pub kwargs: CriteriaAnalyticsAnyMessageClickUrlsV1Kwargs,
    #[doc = "A unique ID to use to memoize the result of the function"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub memoize_id: ::std::option::Option<MemoizeId>,
    pub operator: Operators,
}
impl CriteriaAnalyticsAnyMessageClickUrlsV1 {
    pub fn builder() -> builder::CriteriaAnalyticsAnyMessageClickUrlsV1 {
        Default::default()
    }
}
#[doc = "`CriteriaAnalyticsAnyMessageClickUrlsV1Kwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"account\","]
#[doc = "    \"messages\","]
#[doc = "    \"subscriber\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"account\": {"]
#[doc = "      \"title\": \"Account\","]
#[doc = "      \"description\": \"The account ID\","]
#[doc = "      \"default\": \"<event:account>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:account>\""]
#[doc = "    },"]
#[doc = "    \"click_urls\": {"]
#[doc = "      \"title\": \"Click URLs\","]
#[doc = "      \"description\": \"The list of URLs to check for clicks\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\","]
#[doc = "        \"format\": \"uri\","]
#[doc = "        \"pattern\": \"^https?://(.*)$\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"messages\": {"]
#[doc = "      \"title\": \"Messages\","]
#[doc = "      \"description\": \"The list of messages previously sent to the subscriber in the ruleset\","]
#[doc = "      \"default\": \"<messages:sent>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<messages:sent>\""]
#[doc = "    },"]
#[doc = "    \"subscriber\": {"]
#[doc = "      \"title\": \"Subscriber\","]
#[doc = "      \"description\": \"The subscriber ID\","]
#[doc = "      \"default\": \"<event:subscriber>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:subscriber>\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CriteriaAnalyticsAnyMessageClickUrlsV1Kwargs {
    #[doc = "The account ID"]
    pub account: ::std::string::String,
    #[doc = "The list of URLs to check for clicks"]
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub click_urls: ::std::vec::Vec<ClickUrLsItem>,
    #[doc = "The list of messages previously sent to the subscriber in the ruleset"]
    pub messages: ::std::string::String,
    #[doc = "The subscriber ID"]
    pub subscriber: ::std::string::String,
}
impl CriteriaAnalyticsAnyMessageClickUrlsV1Kwargs {
    pub fn builder() -> builder::CriteriaAnalyticsAnyMessageClickUrlsV1Kwargs {
        Default::default()
    }
}
#[doc = "Returns true if any of the messages in the list have been clicked by the subscriber"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Message Clicked\","]
#[doc = "  \"description\": \"Returns true if any of the messages in the list have been clicked by the subscriber\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"expect\","]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\","]
#[doc = "    \"operator\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"expect\": {"]
#[doc = "      \"title\": \"Return Expectation\","]
#[doc = "      \"description\": \"The expected return value from the function\","]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"ruleset.analytics.filter.any_message_clicks_v1\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ruleset.analytics.filter.any_message_clicks_v1\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"account\","]
#[doc = "        \"messages\","]
#[doc = "        \"subscriber\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"account\": {"]
#[doc = "          \"title\": \"Account\","]
#[doc = "          \"description\": \"The account ID\","]
#[doc = "          \"default\": \"<event:account>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:account>\""]
#[doc = "        },"]
#[doc = "        \"messages\": {"]
#[doc = "          \"title\": \"Messages\","]
#[doc = "          \"description\": \"The list of messages previously sent to the subscriber in the ruleset\","]
#[doc = "          \"default\": \"<messages:sent>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<messages:sent>\""]
#[doc = "        },"]
#[doc = "        \"subscriber\": {"]
#[doc = "          \"title\": \"Subscriber\","]
#[doc = "          \"description\": \"The subscriber ID\","]
#[doc = "          \"default\": \"<event:subscriber>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:subscriber>\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"memoize_id\": {"]
#[doc = "      \"title\": \"Memoize ID\","]
#[doc = "      \"description\": \"A unique ID to use to memoize the result of the function\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12})$\""]
#[doc = "    },"]
#[doc = "    \"operator\": {"]
#[doc = "      \"$ref\": \"#/$defs/operators\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CriteriaAnalyticsAnyMessageClicksV1 {
    #[doc = "The expected return value from the function"]
    pub expect: bool,
    pub function: ::std::string::String,
    pub kwargs: CriteriaAnalyticsAnyMessageClicksV1Kwargs,
    #[doc = "A unique ID to use to memoize the result of the function"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub memoize_id: ::std::option::Option<MemoizeId>,
    pub operator: Operators,
}
impl CriteriaAnalyticsAnyMessageClicksV1 {
    pub fn builder() -> builder::CriteriaAnalyticsAnyMessageClicksV1 {
        Default::default()
    }
}
#[doc = "`CriteriaAnalyticsAnyMessageClicksV1Kwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"account\","]
#[doc = "    \"messages\","]
#[doc = "    \"subscriber\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"account\": {"]
#[doc = "      \"title\": \"Account\","]
#[doc = "      \"description\": \"The account ID\","]
#[doc = "      \"default\": \"<event:account>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:account>\""]
#[doc = "    },"]
#[doc = "    \"messages\": {"]
#[doc = "      \"title\": \"Messages\","]
#[doc = "      \"description\": \"The list of messages previously sent to the subscriber in the ruleset\","]
#[doc = "      \"default\": \"<messages:sent>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<messages:sent>\""]
#[doc = "    },"]
#[doc = "    \"subscriber\": {"]
#[doc = "      \"title\": \"Subscriber\","]
#[doc = "      \"description\": \"The subscriber ID\","]
#[doc = "      \"default\": \"<event:subscriber>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:subscriber>\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CriteriaAnalyticsAnyMessageClicksV1Kwargs {
    #[doc = "The account ID"]
    pub account: ::std::string::String,
    #[doc = "The list of messages previously sent to the subscriber in the ruleset"]
    pub messages: ::std::string::String,
    #[doc = "The subscriber ID"]
    pub subscriber: ::std::string::String,
}
impl CriteriaAnalyticsAnyMessageClicksV1Kwargs {
    pub fn builder() -> builder::CriteriaAnalyticsAnyMessageClicksV1Kwargs {
        Default::default()
    }
}
#[doc = "Returns true if any of the messages in the list have been opened by the subscriber"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Message Opened\","]
#[doc = "  \"description\": \"Returns true if any of the messages in the list have been opened by the subscriber\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"expect\","]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\","]
#[doc = "    \"operator\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"expect\": {"]
#[doc = "      \"title\": \"Return Expectation\","]
#[doc = "      \"description\": \"The expected return value from the function\","]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"ruleset.analytics.filter.any_message_opens_v1\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ruleset.analytics.filter.any_message_opens_v1\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"account\","]
#[doc = "        \"messages\","]
#[doc = "        \"subscriber\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"account\": {"]
#[doc = "          \"title\": \"Account\","]
#[doc = "          \"description\": \"The account ID\","]
#[doc = "          \"default\": \"<event:account>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:account>\""]
#[doc = "        },"]
#[doc = "        \"messages\": {"]
#[doc = "          \"title\": \"Messages\","]
#[doc = "          \"description\": \"The list of messages previously sent to the subscriber in the ruleset\","]
#[doc = "          \"default\": \"<messages:sent>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<messages:sent>\""]
#[doc = "        },"]
#[doc = "        \"subscriber\": {"]
#[doc = "          \"title\": \"Subscriber\","]
#[doc = "          \"description\": \"The subscriber ID\","]
#[doc = "          \"default\": \"<event:subscriber>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:subscriber>\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"memoize_id\": {"]
#[doc = "      \"title\": \"Memoize ID\","]
#[doc = "      \"description\": \"A unique ID to use to memoize the result of the function\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12})$\""]
#[doc = "    },"]
#[doc = "    \"operator\": {"]
#[doc = "      \"$ref\": \"#/$defs/operators\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CriteriaAnalyticsAnyMessageOpensV1 {
    #[doc = "The expected return value from the function"]
    pub expect: bool,
    pub function: ::std::string::String,
    pub kwargs: CriteriaAnalyticsAnyMessageOpensV1Kwargs,
    #[doc = "A unique ID to use to memoize the result of the function"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub memoize_id: ::std::option::Option<MemoizeId>,
    pub operator: Operators,
}
impl CriteriaAnalyticsAnyMessageOpensV1 {
    pub fn builder() -> builder::CriteriaAnalyticsAnyMessageOpensV1 {
        Default::default()
    }
}
#[doc = "`CriteriaAnalyticsAnyMessageOpensV1Kwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"account\","]
#[doc = "    \"messages\","]
#[doc = "    \"subscriber\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"account\": {"]
#[doc = "      \"title\": \"Account\","]
#[doc = "      \"description\": \"The account ID\","]
#[doc = "      \"default\": \"<event:account>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:account>\""]
#[doc = "    },"]
#[doc = "    \"messages\": {"]
#[doc = "      \"title\": \"Messages\","]
#[doc = "      \"description\": \"The list of messages previously sent to the subscriber in the ruleset\","]
#[doc = "      \"default\": \"<messages:sent>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<messages:sent>\""]
#[doc = "    },"]
#[doc = "    \"subscriber\": {"]
#[doc = "      \"title\": \"Subscriber\","]
#[doc = "      \"description\": \"The subscriber ID\","]
#[doc = "      \"default\": \"<event:subscriber>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:subscriber>\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CriteriaAnalyticsAnyMessageOpensV1Kwargs {
    #[doc = "The account ID"]
    pub account: ::std::string::String,
    #[doc = "The list of messages previously sent to the subscriber in the ruleset"]
    pub messages: ::std::string::String,
    #[doc = "The subscriber ID"]
    pub subscriber: ::std::string::String,
}
impl CriteriaAnalyticsAnyMessageOpensV1Kwargs {
    pub fn builder() -> builder::CriteriaAnalyticsAnyMessageOpensV1Kwargs {
        Default::default()
    }
}
#[doc = "Performs an equality check for the campaign serial number in the event against the Campaign service."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Campaign Serial Number\","]
#[doc = "  \"description\": \"Performs an equality check for the campaign serial number in the event against the Campaign service.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"expect\","]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\","]
#[doc = "    \"operator\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"expect\": {"]
#[doc = "      \"title\": \"Return Exepctation\","]
#[doc = "      \"description\": \"The expected return value from the function\","]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"ruleset.campaign.filter.validate_serial_v1\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^ruleset.campaign.filter.validate_serial_v1$\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"account\","]
#[doc = "        \"campaign\","]
#[doc = "        \"list\","]
#[doc = "        \"serial\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"account\": {"]
#[doc = "          \"title\": \"Account\","]
#[doc = "          \"description\": \"The Account the campaign is associated with\","]
#[doc = "          \"default\": \"<event:account>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:account>\""]
#[doc = "        },"]
#[doc = "        \"campaign\": {"]
#[doc = "          \"title\": \"Campaign ID\","]
#[doc = "          \"description\": \"The campaign that has been changed\","]
#[doc = "          \"default\": \"<event:campaign>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^((<event:campaign>)|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}))$\""]
#[doc = "        },"]
#[doc = "        \"list\": {"]
#[doc = "          \"title\": \"Mailing List\","]
#[doc = "          \"description\": \"The mailing list the campaign is associated with\","]
#[doc = "          \"default\": \"<event:list>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:list>\""]
#[doc = "        },"]
#[doc = "        \"serial\": {"]
#[doc = "          \"title\": \"Change Serial #\","]
#[doc = "          \"description\": \"The serial number to validate for the campaign\","]
#[doc = "          \"default\": \"<event:serial>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:serial>\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"operator\": {"]
#[doc = "      \"$ref\": \"#/$defs/operators\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaCampaignSerialV1 {
    #[doc = "The expected return value from the function"]
    pub expect: bool,
    pub function: FunctionPath,
    pub kwargs: CriteriaCampaignSerialV1Kwargs,
    pub operator: Operators,
}
impl CriteriaCampaignSerialV1 {
    pub fn builder() -> builder::CriteriaCampaignSerialV1 {
        Default::default()
    }
}
#[doc = "`CriteriaCampaignSerialV1Kwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"account\","]
#[doc = "    \"campaign\","]
#[doc = "    \"list\","]
#[doc = "    \"serial\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"account\": {"]
#[doc = "      \"title\": \"Account\","]
#[doc = "      \"description\": \"The Account the campaign is associated with\","]
#[doc = "      \"default\": \"<event:account>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:account>\""]
#[doc = "    },"]
#[doc = "    \"campaign\": {"]
#[doc = "      \"title\": \"Campaign ID\","]
#[doc = "      \"description\": \"The campaign that has been changed\","]
#[doc = "      \"default\": \"<event:campaign>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^((<event:campaign>)|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}))$\""]
#[doc = "    },"]
#[doc = "    \"list\": {"]
#[doc = "      \"title\": \"Mailing List\","]
#[doc = "      \"description\": \"The mailing list the campaign is associated with\","]
#[doc = "      \"default\": \"<event:list>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:list>\""]
#[doc = "    },"]
#[doc = "    \"serial\": {"]
#[doc = "      \"title\": \"Change Serial #\","]
#[doc = "      \"description\": \"The serial number to validate for the campaign\","]
#[doc = "      \"default\": \"<event:serial>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:serial>\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaCampaignSerialV1Kwargs {
    #[doc = "The Account the campaign is associated with"]
    pub account: ::std::string::String,
    #[doc = "The campaign that has been changed"]
    pub campaign: CampaignId,
    #[doc = "The mailing list the campaign is associated with"]
    pub list: ::std::string::String,
    #[doc = "The serial number to validate for the campaign"]
    pub serial: ::std::string::String,
}
impl CriteriaCampaignSerialV1Kwargs {
    pub fn builder() -> builder::CriteriaCampaignSerialV1Kwargs {
        Default::default()
    }
}
#[doc = "Performs an equality check for the specified value against the specified field in the event"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Event Field Value\","]
#[doc = "  \"description\": \"Performs an equality check for the specified value against the specified field in the event\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"expect\","]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\","]
#[doc = "    \"operator\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"expect\": {"]
#[doc = "      \"title\": \"Return Exepctation\","]
#[doc = "      \"description\": \"The expected return value from the function\","]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": ["]
#[doc = "        \"boolean\","]
#[doc = "        \"string\","]
#[doc = "        \"integer\","]
#[doc = "        \"null\","]
#[doc = "        \"number\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"rulesengine.filter.event_value\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"rulesengine.filter.event_value\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"key\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"key\": {"]
#[doc = "          \"title\": \"Key Name\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"operator\": {"]
#[doc = "      \"$ref\": \"#/$defs/operators\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaEventValue {
    #[doc = "The expected return value from the function"]
    pub expect: ReturnExepctation,
    pub function: ::std::string::String,
    pub kwargs: CriteriaEventValueKwargs,
    pub operator: Operators,
}
impl CriteriaEventValue {
    pub fn builder() -> builder::CriteriaEventValue {
        Default::default()
    }
}
#[doc = "Performs a check to see if a value from the event is in an array"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Event Field Value In Array\","]
#[doc = "  \"description\": \"Performs a check to see if a value from the event is in an array\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"expect\","]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\","]
#[doc = "    \"operator\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"expect\": {"]
#[doc = "      \"title\": \"Return Exepctation\","]
#[doc = "      \"description\": \"The expected return value from the function\","]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"rulesengine.filter.event_value_in\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"rulesengine.filter.event_value_in\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"key\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"key\": {"]
#[doc = "          \"title\": \"Key Name\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"values\": {"]
#[doc = "          \"title\": \"Values\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": ["]
#[doc = "              \"integer\","]
#[doc = "              \"string\","]
#[doc = "              \"boolean\","]
#[doc = "              \"number\","]
#[doc = "              \"null\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"minItems\": 1,"]
#[doc = "          \"uniqueItems\": true"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"operator\": {"]
#[doc = "      \"$ref\": \"#/$defs/operators\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaEventValueIn {
    #[doc = "The expected return value from the function"]
    pub expect: bool,
    pub function: ::std::string::String,
    pub kwargs: CriteriaEventValueInKwargs,
    pub operator: Operators,
}
impl CriteriaEventValueIn {
    pub fn builder() -> builder::CriteriaEventValueIn {
        Default::default()
    }
}
#[doc = "`CriteriaEventValueInKwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"key\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"key\": {"]
#[doc = "      \"title\": \"Key Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"values\": {"]
#[doc = "      \"title\": \"Values\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": ["]
#[doc = "          \"integer\","]
#[doc = "          \"string\","]
#[doc = "          \"boolean\","]
#[doc = "          \"number\","]
#[doc = "          \"null\""]
#[doc = "        ]"]
#[doc = "      },"]
#[doc = "      \"minItems\": 1,"]
#[doc = "      \"uniqueItems\": true"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaEventValueInKwargs {
    pub key: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub values: ::std::option::Option<Vec<ValuesItem>>,
}
impl CriteriaEventValueInKwargs {
    pub fn builder() -> builder::CriteriaEventValueInKwargs {
        Default::default()
    }
}
#[doc = "Check if a value in the event is comparable to any of the URLs in an array"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Event Field Value is Comparable to URL In Array\","]
#[doc = "  \"description\": \"Check if a value in the event is comparable to any of the URLs in an array\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"expect\","]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\","]
#[doc = "    \"operator\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"expect\": {"]
#[doc = "      \"title\": \"Return Exepctation\","]
#[doc = "      \"description\": \"The expected return value from the function\","]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"rulesengine.filter.event_value_in_urls\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"rulesengine.filter.event_value_in_urls\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"key\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"key\": {"]
#[doc = "          \"title\": \"Key Name\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"urls\": {"]
#[doc = "          \"title\": \"URLs\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": ["]
#[doc = "              \"string\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          \"minItems\": 1,"]
#[doc = "          \"uniqueItems\": true"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"operator\": {"]
#[doc = "      \"$ref\": \"#/$defs/operators\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaEventValueInUrls {
    #[doc = "The expected return value from the function"]
    pub expect: bool,
    pub function: ::std::string::String,
    pub kwargs: CriteriaEventValueInUrlsKwargs,
    pub operator: Operators,
}
impl CriteriaEventValueInUrls {
    pub fn builder() -> builder::CriteriaEventValueInUrls {
        Default::default()
    }
}
#[doc = "`CriteriaEventValueInUrlsKwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"key\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"key\": {"]
#[doc = "      \"title\": \"Key Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"urls\": {"]
#[doc = "      \"title\": \"URLs\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": ["]
#[doc = "          \"string\""]
#[doc = "        ]"]
#[doc = "      },"]
#[doc = "      \"minItems\": 1,"]
#[doc = "      \"uniqueItems\": true"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaEventValueInUrlsKwargs {
    pub key: ::std::string::String,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub urls: ::std::option::Option<Vec<::std::string::String>>,
}
impl CriteriaEventValueInUrlsKwargs {
    pub fn builder() -> builder::CriteriaEventValueInUrlsKwargs {
        Default::default()
    }
}
#[doc = "`CriteriaEventValueKwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"key\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"key\": {"]
#[doc = "      \"title\": \"Key Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaEventValueKwargs {
    pub key: ::std::string::String,
}
impl CriteriaEventValueKwargs {
    pub fn builder() -> builder::CriteriaEventValueKwargs {
        Default::default()
    }
}
#[doc = "Returns true if followups are disabled for a given recipient"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Is Followups Disabled\","]
#[doc = "  \"description\": \"Returns true if followups are disabled for a given recipient\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"expect\","]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\","]
#[doc = "    \"operator\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"expect\": {"]
#[doc = "      \"title\": \"Return Expectation\","]
#[doc = "      \"description\": \"The expected return value from the function\","]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"ruleset.followup.filter.is_disabled_v1\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ruleset.followup.filter.is_disabled_v1\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"recipient\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"recipient\": {"]
#[doc = "          \"title\": \"Recipient\","]
#[doc = "          \"description\": \"The recipient that should receive followups\","]
#[doc = "          \"default\": \"<event:recipient>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:recipient>\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"operator\": {"]
#[doc = "      \"$ref\": \"#/$defs/operators\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaFollowupIsDisabledV1 {
    #[doc = "The expected return value from the function"]
    pub expect: bool,
    pub function: ::std::string::String,
    pub kwargs: CriteriaFollowupIsDisabledV1Kwargs,
    pub operator: Operators,
}
impl CriteriaFollowupIsDisabledV1 {
    pub fn builder() -> builder::CriteriaFollowupIsDisabledV1 {
        Default::default()
    }
}
#[doc = "`CriteriaFollowupIsDisabledV1Kwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"recipient\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"recipient\": {"]
#[doc = "      \"title\": \"Recipient\","]
#[doc = "      \"description\": \"The recipient that should receive followups\","]
#[doc = "      \"default\": \"<event:recipient>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:recipient>\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaFollowupIsDisabledV1Kwargs {
    #[doc = "The recipient that should receive followups"]
    pub recipient: ::std::string::String,
}
impl CriteriaFollowupIsDisabledV1Kwargs {
    pub fn builder() -> builder::CriteriaFollowupIsDisabledV1Kwargs {
        Default::default()
    }
}
#[doc = "Performs an equality check for the specified Country value against the GeoIP resolved country for the remote_ip value in the event"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"GeoIP Country\","]
#[doc = "  \"description\": \"Performs an equality check for the specified Country value against the GeoIP resolved country for the remote_ip value in the event\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"expect\","]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\","]
#[doc = "    \"operator\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"expect\": {"]
#[doc = "      \"title\": \"Country Code Expectation\","]
#[doc = "      \"$ref\": \"#/$defs/countries\""]
#[doc = "    },"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"ruleset.geoip.filter.geoip_country_v1\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ruleset.geoip.filter.geoip_country_v1\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"remote_ip\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"remote_ip\": {"]
#[doc = "          \"title\": \"IP Address\","]
#[doc = "          \"default\": \"<event:remote_ip>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:remote_ip>\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"operator\": {"]
#[doc = "      \"$ref\": \"#/$defs/operators\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaGeoipCountryV1 {
    pub expect: Countries,
    pub function: ::std::string::String,
    pub kwargs: CriteriaGeoipCountryV1Kwargs,
    pub operator: Operators,
}
impl CriteriaGeoipCountryV1 {
    pub fn builder() -> builder::CriteriaGeoipCountryV1 {
        Default::default()
    }
}
#[doc = "`CriteriaGeoipCountryV1Kwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"remote_ip\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"remote_ip\": {"]
#[doc = "      \"title\": \"IP Address\","]
#[doc = "      \"default\": \"<event:remote_ip>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:remote_ip>\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaGeoipCountryV1Kwargs {
    pub remote_ip: ::std::string::String,
}
impl CriteriaGeoipCountryV1Kwargs {
    pub fn builder() -> builder::CriteriaGeoipCountryV1Kwargs {
        Default::default()
    }
}
#[doc = "Select the criterion to filter the event by"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Criterion\","]
#[doc = "  \"description\": \"Select the criterion to filter the event by\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/criteria_campaign_serial_v1\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/criteria_event_value\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/criteria_event_value_in\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/criteria_event_value_in_urls\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/criteria_geoip_country_v1\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/criteria_followup_is_disabled_v1\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/criteria_recipient_adtracking_v1\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/criteria_recipient_custom_field_v1\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/criteria_ruleset_reentry_allowed\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/criteria_tag_all_tags_v1\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/criteria_tag_any_tags_v1\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/criteria_url_url_contains_v1\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/criteria_url_url_equals_v1\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/criteria_rss_state_changed_v1\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/criteria_messagemap_messageapi_id_v1\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/criteria_math_randbelow_v1\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/criteria_analytics_any_message_opens_v1\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/criteria_analytics_any_message_clicks_v1\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/criteria_analytics_any_message_click_urls_v1\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/criteria_analytics_any_message_click_url_contains_v1\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/criteria_ruleset_is_checkpointed\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum CriteriaItems {
    CampaignSerialV1(CriteriaCampaignSerialV1),
    EventValue(CriteriaEventValue),
    EventValueIn(CriteriaEventValueIn),
    EventValueInUrls(CriteriaEventValueInUrls),
    GeoipCountryV1(CriteriaGeoipCountryV1),
    FollowupIsDisabledV1(CriteriaFollowupIsDisabledV1),
    RecipientAdtrackingV1(CriteriaRecipientAdtrackingV1),
    RecipientCustomFieldV1(CriteriaRecipientCustomFieldV1),
    RulesetReentryAllowed(CriteriaRulesetReentryAllowed),
    TagAllTagsV1(CriteriaTagAllTagsV1),
    TagAnyTagsV1(CriteriaTagAnyTagsV1),
    UrlUrlContainsV1(CriteriaUrlUrlContainsV1),
    UrlUrlEqualsV1(CriteriaUrlUrlEqualsV1),
    RssStateChangedV1(CriteriaRssStateChangedV1),
    MessagemapMessageapiIdV1(CriteriaMessagemapMessageapiIdV1),
    MathRandbelowV1(CriteriaMathRandbelowV1),
    AnalyticsAnyMessageOpensV1(CriteriaAnalyticsAnyMessageOpensV1),
    AnalyticsAnyMessageClicksV1(CriteriaAnalyticsAnyMessageClicksV1),
    AnalyticsAnyMessageClickUrlsV1(CriteriaAnalyticsAnyMessageClickUrlsV1),
    AnalyticsAnyMessageClickUrlContainsV1(CriteriaAnalyticsAnyMessageClickUrlContainsV1),
    RulesetIsCheckpointed(CriteriaRulesetIsCheckpointed),
}
impl ::std::convert::From<CriteriaCampaignSerialV1> for CriteriaItems {
    fn from(value: CriteriaCampaignSerialV1) -> Self {
        Self::CampaignSerialV1(value)
    }
}
impl ::std::convert::From<CriteriaEventValue> for CriteriaItems {
    fn from(value: CriteriaEventValue) -> Self {
        Self::EventValue(value)
    }
}
impl ::std::convert::From<CriteriaEventValueIn> for CriteriaItems {
    fn from(value: CriteriaEventValueIn) -> Self {
        Self::EventValueIn(value)
    }
}
impl ::std::convert::From<CriteriaEventValueInUrls> for CriteriaItems {
    fn from(value: CriteriaEventValueInUrls) -> Self {
        Self::EventValueInUrls(value)
    }
}
impl ::std::convert::From<CriteriaGeoipCountryV1> for CriteriaItems {
    fn from(value: CriteriaGeoipCountryV1) -> Self {
        Self::GeoipCountryV1(value)
    }
}
impl ::std::convert::From<CriteriaFollowupIsDisabledV1> for CriteriaItems {
    fn from(value: CriteriaFollowupIsDisabledV1) -> Self {
        Self::FollowupIsDisabledV1(value)
    }
}
impl ::std::convert::From<CriteriaRecipientAdtrackingV1> for CriteriaItems {
    fn from(value: CriteriaRecipientAdtrackingV1) -> Self {
        Self::RecipientAdtrackingV1(value)
    }
}
impl ::std::convert::From<CriteriaRecipientCustomFieldV1> for CriteriaItems {
    fn from(value: CriteriaRecipientCustomFieldV1) -> Self {
        Self::RecipientCustomFieldV1(value)
    }
}
impl ::std::convert::From<CriteriaRulesetReentryAllowed> for CriteriaItems {
    fn from(value: CriteriaRulesetReentryAllowed) -> Self {
        Self::RulesetReentryAllowed(value)
    }
}
impl ::std::convert::From<CriteriaTagAllTagsV1> for CriteriaItems {
    fn from(value: CriteriaTagAllTagsV1) -> Self {
        Self::TagAllTagsV1(value)
    }
}
impl ::std::convert::From<CriteriaTagAnyTagsV1> for CriteriaItems {
    fn from(value: CriteriaTagAnyTagsV1) -> Self {
        Self::TagAnyTagsV1(value)
    }
}
impl ::std::convert::From<CriteriaUrlUrlContainsV1> for CriteriaItems {
    fn from(value: CriteriaUrlUrlContainsV1) -> Self {
        Self::UrlUrlContainsV1(value)
    }
}
impl ::std::convert::From<CriteriaUrlUrlEqualsV1> for CriteriaItems {
    fn from(value: CriteriaUrlUrlEqualsV1) -> Self {
        Self::UrlUrlEqualsV1(value)
    }
}
impl ::std::convert::From<CriteriaRssStateChangedV1> for CriteriaItems {
    fn from(value: CriteriaRssStateChangedV1) -> Self {
        Self::RssStateChangedV1(value)
    }
}
impl ::std::convert::From<CriteriaMessagemapMessageapiIdV1> for CriteriaItems {
    fn from(value: CriteriaMessagemapMessageapiIdV1) -> Self {
        Self::MessagemapMessageapiIdV1(value)
    }
}
impl ::std::convert::From<CriteriaMathRandbelowV1> for CriteriaItems {
    fn from(value: CriteriaMathRandbelowV1) -> Self {
        Self::MathRandbelowV1(value)
    }
}
impl ::std::convert::From<CriteriaAnalyticsAnyMessageOpensV1> for CriteriaItems {
    fn from(value: CriteriaAnalyticsAnyMessageOpensV1) -> Self {
        Self::AnalyticsAnyMessageOpensV1(value)
    }
}
impl ::std::convert::From<CriteriaAnalyticsAnyMessageClicksV1> for CriteriaItems {
    fn from(value: CriteriaAnalyticsAnyMessageClicksV1) -> Self {
        Self::AnalyticsAnyMessageClicksV1(value)
    }
}
impl ::std::convert::From<CriteriaAnalyticsAnyMessageClickUrlsV1> for CriteriaItems {
    fn from(value: CriteriaAnalyticsAnyMessageClickUrlsV1) -> Self {
        Self::AnalyticsAnyMessageClickUrlsV1(value)
    }
}
impl ::std::convert::From<CriteriaAnalyticsAnyMessageClickUrlContainsV1> for CriteriaItems {
    fn from(value: CriteriaAnalyticsAnyMessageClickUrlContainsV1) -> Self {
        Self::AnalyticsAnyMessageClickUrlContainsV1(value)
    }
}
impl ::std::convert::From<CriteriaRulesetIsCheckpointed> for CriteriaItems {
    fn from(value: CriteriaRulesetIsCheckpointed) -> Self {
        Self::RulesetIsCheckpointed(value)
    }
}
#[doc = "Compares the result of a generated random number to an expected value"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Check if Random Number is Below Value\","]
#[doc = "  \"description\": \"Compares the result of a generated random number to an expected value\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"expect\","]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\","]
#[doc = "    \"operator\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"expect\": {"]
#[doc = "      \"title\": \"Return Expectation\","]
#[doc = "      \"description\": \"The expected return value from the function\","]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"ruleset.math.filter.randbelow_v1\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ruleset.math.filter.randbelow_v1\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"upperbound\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"upperbound\": {"]
#[doc = "          \"title\": \"Upper Bound\","]
#[doc = "          \"description\": \"The inclusive upper bound of the random number\","]
#[doc = "          \"default\": 100,"]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"minimum\": 0.0"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"memoize_id\": {"]
#[doc = "      \"title\": \"Memoize ID\","]
#[doc = "      \"description\": \"A unique ID to use to memoize the result of the function\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12})$\""]
#[doc = "    },"]
#[doc = "    \"operator\": {"]
#[doc = "      \"$ref\": \"#/$defs/operators\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CriteriaMathRandbelowV1 {
    #[doc = "The expected return value from the function"]
    pub expect: i64,
    pub function: ::std::string::String,
    pub kwargs: CriteriaMathRandbelowV1Kwargs,
    #[doc = "A unique ID to use to memoize the result of the function"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub memoize_id: ::std::option::Option<MemoizeId>,
    pub operator: Operators,
}
impl CriteriaMathRandbelowV1 {
    pub fn builder() -> builder::CriteriaMathRandbelowV1 {
        Default::default()
    }
}
#[doc = "`CriteriaMathRandbelowV1Kwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"upperbound\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"upperbound\": {"]
#[doc = "      \"title\": \"Upper Bound\","]
#[doc = "      \"description\": \"The inclusive upper bound of the random number\","]
#[doc = "      \"default\": 100,"]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 0.0"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CriteriaMathRandbelowV1Kwargs {
    #[doc = "The inclusive upper bound of the random number"]
    pub upperbound: u64,
}
impl CriteriaMathRandbelowV1Kwargs {
    pub fn builder() -> builder::CriteriaMathRandbelowV1Kwargs {
        Default::default()
    }
}
#[doc = "Performs an equality check for the message ID value against the messageapi ID from the messagemap service"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Associated Meapi Id\","]
#[doc = "  \"description\": \"Performs an equality check for the message ID value against the messageapi ID from the messagemap service\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"expect\","]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\","]
#[doc = "    \"operator\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"expect\": {"]
#[doc = "      \"title\": \"Return Expectation\","]
#[doc = "      \"description\": \"The expected return value from the function\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"ruleset.messagemap.filter.messageapi_id_v1\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ruleset.messagemap.filter.messageapi_id_v1\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"message\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"message\": {"]
#[doc = "          \"default\": \"<event:message>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:message>\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"operator\": {"]
#[doc = "      \"$ref\": \"#/$defs/operators\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaMessagemapMessageapiIdV1 {
    #[doc = "The expected return value from the function"]
    pub expect: ::std::option::Option<::std::string::String>,
    pub function: ::std::string::String,
    pub kwargs: CriteriaMessagemapMessageapiIdV1Kwargs,
    pub operator: Operators,
}
impl CriteriaMessagemapMessageapiIdV1 {
    pub fn builder() -> builder::CriteriaMessagemapMessageapiIdV1 {
        Default::default()
    }
}
#[doc = "`CriteriaMessagemapMessageapiIdV1Kwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"message\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"message\": {"]
#[doc = "      \"default\": \"<event:message>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:message>\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaMessagemapMessageapiIdV1Kwargs {
    pub message: ::std::string::String,
}
impl CriteriaMessagemapMessageapiIdV1Kwargs {
    pub fn builder() -> builder::CriteriaMessagemapMessageapiIdV1Kwargs {
        Default::default()
    }
}
#[doc = "Performs an equality check for the specified value against the adtracking field for the recipient"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Recipient Ad Tracking Value\","]
#[doc = "  \"description\": \"Performs an equality check for the specified value against the adtracking field for the recipient\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"expect\","]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\","]
#[doc = "    \"operator\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"expect\": {"]
#[doc = "      \"title\": \"Return Exepctation\","]
#[doc = "      \"description\": \"The expected return value from the function\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"ruleset.recipient.filter.adtracking_v1\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^ruleset.recipient.filter.adtracking_v1$\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"account\","]
#[doc = "        \"list\","]
#[doc = "        \"recipient\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"account\": {"]
#[doc = "          \"default\": \"<event:account>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:account>\""]
#[doc = "        },"]
#[doc = "        \"list\": {"]
#[doc = "          \"default\": \"<event:list>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:list>\""]
#[doc = "        },"]
#[doc = "        \"recipient\": {"]
#[doc = "          \"default\": \"<event:recipient>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:recipient>\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"operator\": {"]
#[doc = "      \"$ref\": \"#/$defs/operators\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaRecipientAdtrackingV1 {
    #[doc = "The expected return value from the function"]
    pub expect: ::std::option::Option<::std::string::String>,
    pub function: FunctionPath,
    pub kwargs: CriteriaRecipientAdtrackingV1Kwargs,
    pub operator: Operators,
}
impl CriteriaRecipientAdtrackingV1 {
    pub fn builder() -> builder::CriteriaRecipientAdtrackingV1 {
        Default::default()
    }
}
#[doc = "`CriteriaRecipientAdtrackingV1Kwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"account\","]
#[doc = "    \"list\","]
#[doc = "    \"recipient\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"account\": {"]
#[doc = "      \"default\": \"<event:account>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:account>\""]
#[doc = "    },"]
#[doc = "    \"list\": {"]
#[doc = "      \"default\": \"<event:list>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:list>\""]
#[doc = "    },"]
#[doc = "    \"recipient\": {"]
#[doc = "      \"default\": \"<event:recipient>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:recipient>\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaRecipientAdtrackingV1Kwargs {
    pub account: ::std::string::String,
    pub list: ::std::string::String,
    pub recipient: ::std::string::String,
}
impl CriteriaRecipientAdtrackingV1Kwargs {
    pub fn builder() -> builder::CriteriaRecipientAdtrackingV1Kwargs {
        Default::default()
    }
}
#[doc = "Performs an equality check for the specified custom-field and value against the recipient's data in the recipient service"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Recipient Custom-Field Value\","]
#[doc = "  \"description\": \"Performs an equality check for the specified custom-field and value against the recipient's data in the recipient service\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"expect\","]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\","]
#[doc = "    \"operator\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"expect\": {"]
#[doc = "      \"title\": \"Return Exepctation\","]
#[doc = "      \"description\": \"The expected return value from the function\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"integer\","]
#[doc = "        \"number\","]
#[doc = "        \"boolean\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"ruleset.recipient.filter.custom_field_v1\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ruleset.recipient.filter.custom_field_v1\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"account\","]
#[doc = "        \"field\","]
#[doc = "        \"list\","]
#[doc = "        \"recipient\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"account\": {"]
#[doc = "          \"default\": \"<event:account>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:account>\""]
#[doc = "        },"]
#[doc = "        \"field\": {"]
#[doc = "          \"title\": \"Custom-Field Name\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"list\": {"]
#[doc = "          \"default\": \"<event:list>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:list>\""]
#[doc = "        },"]
#[doc = "        \"recipient\": {"]
#[doc = "          \"default\": \"<event:recipient>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:recipient>\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"operator\": {"]
#[doc = "      \"$ref\": \"#/$defs/operators\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaRecipientCustomFieldV1 {
    #[doc = "The expected return value from the function"]
    pub expect: ReturnExepctation,
    pub function: ::std::string::String,
    pub kwargs: CriteriaRecipientCustomFieldV1Kwargs,
    pub operator: Operators,
}
impl CriteriaRecipientCustomFieldV1 {
    pub fn builder() -> builder::CriteriaRecipientCustomFieldV1 {
        Default::default()
    }
}
#[doc = "`CriteriaRecipientCustomFieldV1Kwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"account\","]
#[doc = "    \"field\","]
#[doc = "    \"list\","]
#[doc = "    \"recipient\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"account\": {"]
#[doc = "      \"default\": \"<event:account>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:account>\""]
#[doc = "    },"]
#[doc = "    \"field\": {"]
#[doc = "      \"title\": \"Custom-Field Name\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"list\": {"]
#[doc = "      \"default\": \"<event:list>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:list>\""]
#[doc = "    },"]
#[doc = "    \"recipient\": {"]
#[doc = "      \"default\": \"<event:recipient>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:recipient>\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaRecipientCustomFieldV1Kwargs {
    pub account: ::std::string::String,
    pub field: ::std::string::String,
    pub list: ::std::string::String,
    pub recipient: ::std::string::String,
}
impl CriteriaRecipientCustomFieldV1Kwargs {
    pub fn builder() -> builder::CriteriaRecipientCustomFieldV1Kwargs {
        Default::default()
    }
}
#[doc = "Returns true if a RSS has new posts since last checked"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"RSS State Changed\","]
#[doc = "  \"description\": \"Returns true if a RSS has new posts since last checked\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"expect\","]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\","]
#[doc = "    \"operator\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"expect\": {"]
#[doc = "      \"title\": \"Return Expectation\","]
#[doc = "      \"description\": \"The expected return value from the function\","]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"ruleset.rss.filter.state_changed_v1\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ruleset.rss.filter.state_changed_v1\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"account\","]
#[doc = "        \"list\","]
#[doc = "        \"min_new_items\","]
#[doc = "        \"ruleset\","]
#[doc = "        \"subscriber\","]
#[doc = "        \"url\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"account\": {"]
#[doc = "          \"description\": \"The associated account ID\","]
#[doc = "          \"default\": \"<event:account>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:account>\""]
#[doc = "        },"]
#[doc = "        \"categories\": {"]
#[doc = "          \"title\": \"The RSS categories\","]
#[doc = "          \"description\": \"A list of categories for filtering the RSS posts\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"uniqueItems\": true"]
#[doc = "        },"]
#[doc = "        \"list\": {"]
#[doc = "          \"description\": \"The associated list ID\","]
#[doc = "          \"default\": \"<event:list>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:list>\""]
#[doc = "        },"]
#[doc = "        \"min_new_items\": {"]
#[doc = "          \"title\": \"The minimum number of RSS posts to check for\","]
#[doc = "          \"description\": \"The minimum number of new RSS posts that will trigger the actions.\","]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"minimum\": 1.0"]
#[doc = "        },"]
#[doc = "        \"ruleset\": {"]
#[doc = "          \"title\": \"Ruleset ID\","]
#[doc = "          \"description\": \"The ID of the ruleset in the Rule service\","]
#[doc = "          \"default\": \"<ruleset>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^((<ruleset>)|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}))$\""]
#[doc = "        },"]
#[doc = "        \"subscriber\": {"]
#[doc = "          \"description\": \"The associated subscriber ID\","]
#[doc = "          \"default\": \"<event:subscriber>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:subscriber>\""]
#[doc = "        },"]
#[doc = "        \"url\": {"]
#[doc = "          \"title\": \"The blog RSS URL\","]
#[doc = "          \"description\": \"The RSS URL to check for new posts\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"format\": \"uri\","]
#[doc = "          \"pattern\": \"^https?://*\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"operator\": {"]
#[doc = "      \"$ref\": \"#/$defs/operators\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaRssStateChangedV1 {
    #[doc = "The expected return value from the function"]
    pub expect: bool,
    pub function: ::std::string::String,
    pub kwargs: CriteriaRssStateChangedV1Kwargs,
    pub operator: Operators,
}
impl CriteriaRssStateChangedV1 {
    pub fn builder() -> builder::CriteriaRssStateChangedV1 {
        Default::default()
    }
}
#[doc = "`CriteriaRssStateChangedV1Kwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"account\","]
#[doc = "    \"list\","]
#[doc = "    \"min_new_items\","]
#[doc = "    \"ruleset\","]
#[doc = "    \"subscriber\","]
#[doc = "    \"url\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"account\": {"]
#[doc = "      \"description\": \"The associated account ID\","]
#[doc = "      \"default\": \"<event:account>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:account>\""]
#[doc = "    },"]
#[doc = "    \"categories\": {"]
#[doc = "      \"title\": \"The RSS categories\","]
#[doc = "      \"description\": \"A list of categories for filtering the RSS posts\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"uniqueItems\": true"]
#[doc = "    },"]
#[doc = "    \"list\": {"]
#[doc = "      \"description\": \"The associated list ID\","]
#[doc = "      \"default\": \"<event:list>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:list>\""]
#[doc = "    },"]
#[doc = "    \"min_new_items\": {"]
#[doc = "      \"title\": \"The minimum number of RSS posts to check for\","]
#[doc = "      \"description\": \"The minimum number of new RSS posts that will trigger the actions.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 1.0"]
#[doc = "    },"]
#[doc = "    \"ruleset\": {"]
#[doc = "      \"title\": \"Ruleset ID\","]
#[doc = "      \"description\": \"The ID of the ruleset in the Rule service\","]
#[doc = "      \"default\": \"<ruleset>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^((<ruleset>)|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}))$\""]
#[doc = "    },"]
#[doc = "    \"subscriber\": {"]
#[doc = "      \"description\": \"The associated subscriber ID\","]
#[doc = "      \"default\": \"<event:subscriber>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:subscriber>\""]
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"title\": \"The blog RSS URL\","]
#[doc = "      \"description\": \"The RSS URL to check for new posts\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"format\": \"uri\","]
#[doc = "      \"pattern\": \"^https?://*\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaRssStateChangedV1Kwargs {
    #[doc = "The associated account ID"]
    pub account: ::std::string::String,
    #[doc = "A list of categories for filtering the RSS posts"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub categories: ::std::option::Option<Vec<::std::string::String>>,
    #[doc = "The associated list ID"]
    pub list: ::std::string::String,
    #[doc = "The minimum number of new RSS posts that will trigger the actions."]
    pub min_new_items: ::std::num::NonZeroU64,
    #[doc = "The ID of the ruleset in the Rule service"]
    pub ruleset: RulesetId,
    #[doc = "The associated subscriber ID"]
    pub subscriber: ::std::string::String,
    #[doc = "The RSS URL to check for new posts"]
    pub url: TheBlogRssUrl,
}
impl CriteriaRssStateChangedV1Kwargs {
    pub fn builder() -> builder::CriteriaRssStateChangedV1Kwargs {
        Default::default()
    }
}
#[doc = "Checks if the subscriber is checkpointed in the ruleset"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Is Checkpointed\","]
#[doc = "  \"description\": \"Checks if the subscriber is checkpointed in the ruleset\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"expect\","]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\","]
#[doc = "    \"operator\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"expect\": {"]
#[doc = "      \"title\": \"Return Expectation\","]
#[doc = "      \"description\": \"The expected return value from the function\","]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"rulesengine.filter.is_checkpointed\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"rulesengine.filter.is_checkpointed\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"ruleset\","]
#[doc = "        \"subscriber\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"ruleset\": {"]
#[doc = "          \"title\": \"Ruleset ID\","]
#[doc = "          \"description\": \"The ruleset ID\","]
#[doc = "          \"default\": \"<ruleset>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<ruleset>\""]
#[doc = "        },"]
#[doc = "        \"subscriber\": {"]
#[doc = "          \"title\": \"Subscriber ID\","]
#[doc = "          \"description\": \"The subscriber ID\","]
#[doc = "          \"default\": \"<event:subscriber>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:subscriber>\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    \"operator\": {"]
#[doc = "      \"$ref\": \"#/$defs/operators\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CriteriaRulesetIsCheckpointed {
    #[doc = "The expected return value from the function"]
    pub expect: bool,
    pub function: ::std::string::String,
    pub kwargs: CriteriaRulesetIsCheckpointedKwargs,
    pub operator: Operators,
}
impl CriteriaRulesetIsCheckpointed {
    pub fn builder() -> builder::CriteriaRulesetIsCheckpointed {
        Default::default()
    }
}
#[doc = "`CriteriaRulesetIsCheckpointedKwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"ruleset\","]
#[doc = "    \"subscriber\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ruleset\": {"]
#[doc = "      \"title\": \"Ruleset ID\","]
#[doc = "      \"description\": \"The ruleset ID\","]
#[doc = "      \"default\": \"<ruleset>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<ruleset>\""]
#[doc = "    },"]
#[doc = "    \"subscriber\": {"]
#[doc = "      \"title\": \"Subscriber ID\","]
#[doc = "      \"description\": \"The subscriber ID\","]
#[doc = "      \"default\": \"<event:subscriber>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:subscriber>\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct CriteriaRulesetIsCheckpointedKwargs {
    #[doc = "The ruleset ID"]
    pub ruleset: ::std::string::String,
    #[doc = "The subscriber ID"]
    pub subscriber: ::std::string::String,
}
impl CriteriaRulesetIsCheckpointedKwargs {
    pub fn builder() -> builder::CriteriaRulesetIsCheckpointedKwargs {
        Default::default()
    }
}
#[doc = "Returns true if the ruleset isn't processed yet or can be restarted."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Reentry into ruleset allowed.\","]
#[doc = "  \"description\": \"Returns true if the ruleset isn't processed yet or can be restarted.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"expect\","]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\","]
#[doc = "    \"operator\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"expect\": {"]
#[doc = "      \"title\": \"Return Expectation\","]
#[doc = "      \"description\": \"The expected return value from the function\","]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"rulesengine.filter.reentry_allowed\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"rulesengine.filter.reentry_allowed\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"ruleset\","]
#[doc = "        \"subscriber\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"ruleset\": {"]
#[doc = "          \"default\": \"<ruleset>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<ruleset>\""]
#[doc = "        },"]
#[doc = "        \"subscriber\": {"]
#[doc = "          \"default\": \"<subscriber>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<subscriber>\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"operator\": {"]
#[doc = "      \"$ref\": \"#/$defs/operators\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaRulesetReentryAllowed {
    #[doc = "The expected return value from the function"]
    pub expect: bool,
    pub function: ::std::string::String,
    pub kwargs: CriteriaRulesetReentryAllowedKwargs,
    pub operator: Operators,
}
impl CriteriaRulesetReentryAllowed {
    pub fn builder() -> builder::CriteriaRulesetReentryAllowed {
        Default::default()
    }
}
#[doc = "`CriteriaRulesetReentryAllowedKwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"ruleset\","]
#[doc = "    \"subscriber\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"ruleset\": {"]
#[doc = "      \"default\": \"<ruleset>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<ruleset>\""]
#[doc = "    },"]
#[doc = "    \"subscriber\": {"]
#[doc = "      \"default\": \"<subscriber>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<subscriber>\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaRulesetReentryAllowedKwargs {
    pub ruleset: ::std::string::String,
    pub subscriber: ::std::string::String,
}
impl CriteriaRulesetReentryAllowedKwargs {
    pub fn builder() -> builder::CriteriaRulesetReentryAllowedKwargs {
        Default::default()
    }
}
#[doc = "Returns true if all of the specified tags exists in the tags for the specified subscriber"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Do All Tags Match\","]
#[doc = "  \"description\": \"Returns true if all of the specified tags exists in the tags for the specified subscriber\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"expect\","]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\","]
#[doc = "    \"operator\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"expect\": {"]
#[doc = "      \"title\": \"Return Exepctation\","]
#[doc = "      \"description\": \"The expected return value from the function\","]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"ruleset.tag.filter.all_tags_v1\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ruleset.tag.filter.all_tags_v1\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"account\","]
#[doc = "        \"labels\","]
#[doc = "        \"list\","]
#[doc = "        \"recipient\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"account\": {"]
#[doc = "          \"default\": \"<event:account>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:account>\""]
#[doc = "        },"]
#[doc = "        \"labels\": {"]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"uniqueItems\": false"]
#[doc = "        },"]
#[doc = "        \"list\": {"]
#[doc = "          \"default\": \"<event:list>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:list>\""]
#[doc = "        },"]
#[doc = "        \"recipient\": {"]
#[doc = "          \"default\": \"<event:recipient>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:recipient>\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"operator\": {"]
#[doc = "      \"$ref\": \"#/$defs/operators\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaTagAllTagsV1 {
    #[doc = "The expected return value from the function"]
    pub expect: bool,
    pub function: ::std::string::String,
    pub kwargs: CriteriaTagAllTagsV1Kwargs,
    pub operator: Operators,
}
impl CriteriaTagAllTagsV1 {
    pub fn builder() -> builder::CriteriaTagAllTagsV1 {
        Default::default()
    }
}
#[doc = "`CriteriaTagAllTagsV1Kwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"account\","]
#[doc = "    \"labels\","]
#[doc = "    \"list\","]
#[doc = "    \"recipient\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"account\": {"]
#[doc = "      \"default\": \"<event:account>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:account>\""]
#[doc = "    },"]
#[doc = "    \"labels\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"uniqueItems\": false"]
#[doc = "    },"]
#[doc = "    \"list\": {"]
#[doc = "      \"default\": \"<event:list>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:list>\""]
#[doc = "    },"]
#[doc = "    \"recipient\": {"]
#[doc = "      \"default\": \"<event:recipient>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:recipient>\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaTagAllTagsV1Kwargs {
    pub account: ::std::string::String,
    pub labels: ::std::vec::Vec<::std::string::String>,
    pub list: ::std::string::String,
    pub recipient: ::std::string::String,
}
impl CriteriaTagAllTagsV1Kwargs {
    pub fn builder() -> builder::CriteriaTagAllTagsV1Kwargs {
        Default::default()
    }
}
#[doc = "Returns true if any of the specified tags exists in the tags for the specified subscriber"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Do Any Tags Match\","]
#[doc = "  \"description\": \"Returns true if any of the specified tags exists in the tags for the specified subscriber\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"expect\","]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\","]
#[doc = "    \"operator\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"expect\": {"]
#[doc = "      \"title\": \"Return Exepctation\","]
#[doc = "      \"description\": \"The expected return value from the function\","]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"ruleset.tag.filter.any_tags_v1\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ruleset.tag.filter.any_tags_v1\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"account\","]
#[doc = "        \"labels\","]
#[doc = "        \"list\","]
#[doc = "        \"recipient\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"account\": {"]
#[doc = "          \"default\": \"<event:account>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:account>\""]
#[doc = "        },"]
#[doc = "        \"labels\": {"]
#[doc = "          \"title\": \"Tags to be evaluated\","]
#[doc = "          \"description\": \"The list of tags to be evaulated\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"uniqueItems\": false"]
#[doc = "        },"]
#[doc = "        \"list\": {"]
#[doc = "          \"default\": \"<event:list>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:list>\""]
#[doc = "        },"]
#[doc = "        \"recipient\": {"]
#[doc = "          \"default\": \"<event:recipient>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:recipient>\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"memoize_id\": {"]
#[doc = "      \"title\": \"Memoize ID\","]
#[doc = "      \"description\": \"A unique ID to use to memoize the result of the function\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12})$\""]
#[doc = "    },"]
#[doc = "    \"operator\": {"]
#[doc = "      \"$ref\": \"#/$defs/operators\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaTagAnyTagsV1 {
    #[doc = "The expected return value from the function"]
    pub expect: bool,
    pub function: ::std::string::String,
    pub kwargs: CriteriaTagAnyTagsV1Kwargs,
    #[doc = "A unique ID to use to memoize the result of the function"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub memoize_id: ::std::option::Option<MemoizeId>,
    pub operator: Operators,
}
impl CriteriaTagAnyTagsV1 {
    pub fn builder() -> builder::CriteriaTagAnyTagsV1 {
        Default::default()
    }
}
#[doc = "`CriteriaTagAnyTagsV1Kwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"account\","]
#[doc = "    \"labels\","]
#[doc = "    \"list\","]
#[doc = "    \"recipient\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"account\": {"]
#[doc = "      \"default\": \"<event:account>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:account>\""]
#[doc = "    },"]
#[doc = "    \"labels\": {"]
#[doc = "      \"title\": \"Tags to be evaluated\","]
#[doc = "      \"description\": \"The list of tags to be evaulated\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"uniqueItems\": false"]
#[doc = "    },"]
#[doc = "    \"list\": {"]
#[doc = "      \"default\": \"<event:list>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:list>\""]
#[doc = "    },"]
#[doc = "    \"recipient\": {"]
#[doc = "      \"default\": \"<event:recipient>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:recipient>\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaTagAnyTagsV1Kwargs {
    pub account: ::std::string::String,
    #[doc = "The list of tags to be evaulated"]
    pub labels: ::std::vec::Vec<::std::string::String>,
    pub list: ::std::string::String,
    pub recipient: ::std::string::String,
}
impl CriteriaTagAnyTagsV1Kwargs {
    pub fn builder() -> builder::CriteriaTagAnyTagsV1Kwargs {
        Default::default()
    }
}
#[doc = "Performs a check to see if a url from the event contains a string"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Url matches event url\","]
#[doc = "  \"description\": \"Performs a check to see if a url from the event contains a string\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"expect\","]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\","]
#[doc = "    \"operator\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"expect\": {"]
#[doc = "      \"title\": \"Return Exepctation\","]
#[doc = "      \"description\": \"The expected return value from the function\","]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"ruleset.url.filter.url_contains_v1\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ruleset.url.filter.url_contains_v1\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"contains\","]
#[doc = "        \"url\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"contains\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"url\": {"]
#[doc = "          \"default\": \"<event:url>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:url>\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"operator\": {"]
#[doc = "      \"$ref\": \"#/$defs/operators\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaUrlUrlContainsV1 {
    #[doc = "The expected return value from the function"]
    pub expect: bool,
    pub function: ::std::string::String,
    pub kwargs: CriteriaUrlUrlContainsV1Kwargs,
    pub operator: Operators,
}
impl CriteriaUrlUrlContainsV1 {
    pub fn builder() -> builder::CriteriaUrlUrlContainsV1 {
        Default::default()
    }
}
#[doc = "`CriteriaUrlUrlContainsV1Kwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"contains\","]
#[doc = "    \"url\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"contains\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"default\": \"<event:url>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:url>\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaUrlUrlContainsV1Kwargs {
    pub contains: ::std::string::String,
    pub url: ::std::string::String,
}
impl CriteriaUrlUrlContainsV1Kwargs {
    pub fn builder() -> builder::CriteriaUrlUrlContainsV1Kwargs {
        Default::default()
    }
}
#[doc = "Performs a check to see if a url from the event equals a provided url"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Url equals event url\","]
#[doc = "  \"description\": \"Performs a check to see if a url from the event equals a provided url\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"expect\","]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\","]
#[doc = "    \"operator\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"expect\": {"]
#[doc = "      \"title\": \"Return Exepctation\","]
#[doc = "      \"description\": \"The expected return value from the function\","]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"ruleset.url.filter.url_equals_v1\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ruleset.url.filter.url_equals_v1\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"match\","]
#[doc = "        \"url\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"match\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"url\": {"]
#[doc = "          \"default\": \"<event:url>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:url>\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"operator\": {"]
#[doc = "      \"$ref\": \"#/$defs/operators\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaUrlUrlEqualsV1 {
    #[doc = "The expected return value from the function"]
    pub expect: bool,
    pub function: ::std::string::String,
    pub kwargs: CriteriaUrlUrlEqualsV1Kwargs,
    pub operator: Operators,
}
impl CriteriaUrlUrlEqualsV1 {
    pub fn builder() -> builder::CriteriaUrlUrlEqualsV1 {
        Default::default()
    }
}
#[doc = "`CriteriaUrlUrlEqualsV1Kwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"match\","]
#[doc = "    \"url\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"match\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"url\": {"]
#[doc = "      \"default\": \"<event:url>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:url>\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaUrlUrlEqualsV1Kwargs {
    #[serde(rename = "match")]
    pub match_: ::std::string::String,
    pub url: ::std::string::String,
}
impl CriteriaUrlUrlEqualsV1Kwargs {
    pub fn builder() -> builder::CriteriaUrlUrlEqualsV1Kwargs {
        Default::default()
    }
}
#[doc = "Checks if a webfeed has new, unprocessed feed items."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Webfeed new feed items filter\","]
#[doc = "  \"description\": \"Checks if a webfeed has new, unprocessed feed items.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"expect\","]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\","]
#[doc = "    \"operator\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"expect\": {"]
#[doc = "      \"title\": \"Return Expectation\","]
#[doc = "      \"description\": \"The expected return value from the function\","]
#[doc = "      \"default\": true,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"webfeed_campaign.filter.new_feed_items_v1\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"webfeed_campaign.filter.new_feed_items_v1\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"account\","]
#[doc = "        \"list\","]
#[doc = "        \"num_items\","]
#[doc = "        \"ruleset\","]
#[doc = "        \"webfeed\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"account\": {"]
#[doc = "          \"default\": \"<event:account>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:account>\""]
#[doc = "        },"]
#[doc = "        \"list\": {"]
#[doc = "          \"default\": \"<event:list>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:list>\""]
#[doc = "        },"]
#[doc = "        \"num_items\": {"]
#[doc = "          \"title\": \"Number of new feed items.\","]
#[doc = "          \"description\": \"The minimum number of new feed items that will trigger the actions.\","]
#[doc = "          \"type\": \"integer\","]
#[doc = "          \"minimum\": 1.0"]
#[doc = "        },"]
#[doc = "        \"ruleset\": {"]
#[doc = "          \"title\": \"Ruleset ID\","]
#[doc = "          \"description\": \"The ID of the ruleset in the Rule service\","]
#[doc = "          \"default\": \"<ruleset>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^((<ruleset>)|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}))$\""]
#[doc = "        },"]
#[doc = "        \"webfeed\": {"]
#[doc = "          \"title\": \"Webfeed\","]
#[doc = "          \"description\": \"The webfeed to create the message from.\","]
#[doc = "          \"default\": \"<event:webfeed>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:webfeed>\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"operator\": {"]
#[doc = "      \"$ref\": \"#/$defs/operators\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaWebfeedNewFeedItemsV1 {
    #[doc = "The expected return value from the function"]
    pub expect: bool,
    pub function: ::std::string::String,
    pub kwargs: CriteriaWebfeedNewFeedItemsV1Kwargs,
    pub operator: Operators,
}
impl CriteriaWebfeedNewFeedItemsV1 {
    pub fn builder() -> builder::CriteriaWebfeedNewFeedItemsV1 {
        Default::default()
    }
}
#[doc = "`CriteriaWebfeedNewFeedItemsV1Kwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"account\","]
#[doc = "    \"list\","]
#[doc = "    \"num_items\","]
#[doc = "    \"ruleset\","]
#[doc = "    \"webfeed\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"account\": {"]
#[doc = "      \"default\": \"<event:account>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:account>\""]
#[doc = "    },"]
#[doc = "    \"list\": {"]
#[doc = "      \"default\": \"<event:list>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:list>\""]
#[doc = "    },"]
#[doc = "    \"num_items\": {"]
#[doc = "      \"title\": \"Number of new feed items.\","]
#[doc = "      \"description\": \"The minimum number of new feed items that will trigger the actions.\","]
#[doc = "      \"type\": \"integer\","]
#[doc = "      \"minimum\": 1.0"]
#[doc = "    },"]
#[doc = "    \"ruleset\": {"]
#[doc = "      \"title\": \"Ruleset ID\","]
#[doc = "      \"description\": \"The ID of the ruleset in the Rule service\","]
#[doc = "      \"default\": \"<ruleset>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^((<ruleset>)|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}))$\""]
#[doc = "    },"]
#[doc = "    \"webfeed\": {"]
#[doc = "      \"title\": \"Webfeed\","]
#[doc = "      \"description\": \"The webfeed to create the message from.\","]
#[doc = "      \"default\": \"<event:webfeed>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:webfeed>\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaWebfeedNewFeedItemsV1Kwargs {
    pub account: ::std::string::String,
    pub list: ::std::string::String,
    #[doc = "The minimum number of new feed items that will trigger the actions."]
    pub num_items: ::std::num::NonZeroU64,
    #[doc = "The ID of the ruleset in the Rule service"]
    pub ruleset: RulesetId,
    #[doc = "The webfeed to create the message from."]
    pub webfeed: ::std::string::String,
}
impl CriteriaWebfeedNewFeedItemsV1Kwargs {
    pub fn builder() -> builder::CriteriaWebfeedNewFeedItemsV1Kwargs {
        Default::default()
    }
}
#[doc = "Fake filter that actually creates the triggering event."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Schedule next recurring wait action\","]
#[doc = "  \"description\": \"Fake filter that actually creates the triggering event.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"expect\","]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\","]
#[doc = "    \"operator\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"expect\": {"]
#[doc = "      \"title\": \"Return Expectation\","]
#[doc = "      \"description\": \"The expected return value from the function\","]
#[doc = "      \"default\": true,"]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"webfeed_campaign.filter.recurring_wait_v1\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"webfeed_campaign.filter.recurring_wait_v1\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"account\","]
#[doc = "        \"list\","]
#[doc = "        \"rrules\","]
#[doc = "        \"ruleset\","]
#[doc = "        \"timezone\","]
#[doc = "        \"webfeed\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"account\": {"]
#[doc = "          \"default\": \"<event:account>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:account>\""]
#[doc = "        },"]
#[doc = "        \"list\": {"]
#[doc = "          \"default\": \"<event:list>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:list>\""]
#[doc = "        },"]
#[doc = "        \"rrules\": {"]
#[doc = "          \"title\": \"Recurrence Rules\","]
#[doc = "          \"description\": \"An array of RFC-5545 Recurrence Rule for defining when the wait-complete event can be sent\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"type\": \"string\""]
#[doc = "          },"]
#[doc = "          \"minItems\": 1,"]
#[doc = "          \"uniqueItems\": true"]
#[doc = "        },"]
#[doc = "        \"ruleset\": {"]
#[doc = "          \"title\": \"Ruleset ID\","]
#[doc = "          \"description\": \"The ID of the ruleset in the Rule service\","]
#[doc = "          \"default\": \"<ruleset>\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^((<ruleset>)|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}))$\""]
#[doc = "        },"]
#[doc = "        \"timezone\": {"]
#[doc = "          \"title\": \"Timezone\","]
#[doc = "          \"description\": \"The timezone of the message in Olson format\","]
#[doc = "          \"default\": \"UTC\","]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"webfeed\": {"]
#[doc = "          \"title\": \"Webfeed\","]
#[doc = "          \"description\": \"The webfeed to create the message from.\","]
#[doc = "          \"default\": \"<event:webfeed>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:webfeed>\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"operator\": {"]
#[doc = "      \"$ref\": \"#/$defs/operators\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaWebfeedRecurringWaitV1 {
    #[doc = "The expected return value from the function"]
    pub expect: bool,
    pub function: ::std::string::String,
    pub kwargs: CriteriaWebfeedRecurringWaitV1Kwargs,
    pub operator: Operators,
}
impl CriteriaWebfeedRecurringWaitV1 {
    pub fn builder() -> builder::CriteriaWebfeedRecurringWaitV1 {
        Default::default()
    }
}
#[doc = "`CriteriaWebfeedRecurringWaitV1Kwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"account\","]
#[doc = "    \"list\","]
#[doc = "    \"rrules\","]
#[doc = "    \"ruleset\","]
#[doc = "    \"timezone\","]
#[doc = "    \"webfeed\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"account\": {"]
#[doc = "      \"default\": \"<event:account>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:account>\""]
#[doc = "    },"]
#[doc = "    \"list\": {"]
#[doc = "      \"default\": \"<event:list>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:list>\""]
#[doc = "    },"]
#[doc = "    \"rrules\": {"]
#[doc = "      \"title\": \"Recurrence Rules\","]
#[doc = "      \"description\": \"An array of RFC-5545 Recurrence Rule for defining when the wait-complete event can be sent\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\""]
#[doc = "      },"]
#[doc = "      \"minItems\": 1,"]
#[doc = "      \"uniqueItems\": true"]
#[doc = "    },"]
#[doc = "    \"ruleset\": {"]
#[doc = "      \"title\": \"Ruleset ID\","]
#[doc = "      \"description\": \"The ID of the ruleset in the Rule service\","]
#[doc = "      \"default\": \"<ruleset>\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^((<ruleset>)|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}))$\""]
#[doc = "    },"]
#[doc = "    \"timezone\": {"]
#[doc = "      \"title\": \"Timezone\","]
#[doc = "      \"description\": \"The timezone of the message in Olson format\","]
#[doc = "      \"default\": \"UTC\","]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"webfeed\": {"]
#[doc = "      \"title\": \"Webfeed\","]
#[doc = "      \"description\": \"The webfeed to create the message from.\","]
#[doc = "      \"default\": \"<event:webfeed>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:webfeed>\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct CriteriaWebfeedRecurringWaitV1Kwargs {
    pub account: ::std::string::String,
    pub list: ::std::string::String,
    #[doc = "An array of RFC-5545 Recurrence Rule for defining when the wait-complete event can be sent"]
    pub rrules: Vec<::std::string::String>,
    #[doc = "The ID of the ruleset in the Rule service"]
    pub ruleset: RulesetId,
    #[doc = "The timezone of the message in Olson format"]
    pub timezone: ::std::string::String,
    #[doc = "The webfeed to create the message from."]
    pub webfeed: ::std::string::String,
}
impl CriteriaWebfeedRecurringWaitV1Kwargs {
    pub fn builder() -> builder::CriteriaWebfeedRecurringWaitV1Kwargs {
        Default::default()
    }
}
#[doc = "`Definition`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Definition\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/actions_campaign_change_rule_state_v1\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/actions_email_send_v1\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/actions_followup_enable_v1\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/actions_followup_send_autoresponse_v1\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/actions_log_log_v1\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/actions_schedule_change_event_states_v1\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/actions_schedule_wait_v1\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/actions_tag_modify_tags_v1\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/actions_stop_stop\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/actions_webfeed_create_webfeed_message_v1\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/actions_branch_set_branch\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/actions_branch_pause_branch\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/actions_schedule_set_branch_v1\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Definition {
    CampaignChangeRuleStateV1(ActionsCampaignChangeRuleStateV1),
    EmailSendV1(ActionsEmailSendV1),
    FollowupEnableV1(ActionsFollowupEnableV1),
    FollowupSendAutoresponseV1(ActionsFollowupSendAutoresponseV1),
    LogLogV1(ActionsLogLogV1),
    ScheduleChangeEventStatesV1(ActionsScheduleChangeEventStatesV1),
    ScheduleWaitV1(ActionsScheduleWaitV1),
    TagModifyTagsV1(ActionsTagModifyTagsV1),
    StopStop(ActionsStopStop),
    WebfeedCreateWebfeedMessageV1(ActionsWebfeedCreateWebfeedMessageV1),
    BranchSetBranch(ActionsBranchSetBranch),
    BranchPauseBranch(ActionsBranchPauseBranch),
    ScheduleSetBranchV1(ActionsScheduleSetBranchV1),
}
impl ::std::convert::From<ActionsCampaignChangeRuleStateV1> for Definition {
    fn from(value: ActionsCampaignChangeRuleStateV1) -> Self {
        Self::CampaignChangeRuleStateV1(value)
    }
}
impl ::std::convert::From<ActionsEmailSendV1> for Definition {
    fn from(value: ActionsEmailSendV1) -> Self {
        Self::EmailSendV1(value)
    }
}
impl ::std::convert::From<ActionsFollowupEnableV1> for Definition {
    fn from(value: ActionsFollowupEnableV1) -> Self {
        Self::FollowupEnableV1(value)
    }
}
impl ::std::convert::From<ActionsFollowupSendAutoresponseV1> for Definition {
    fn from(value: ActionsFollowupSendAutoresponseV1) -> Self {
        Self::FollowupSendAutoresponseV1(value)
    }
}
impl ::std::convert::From<ActionsLogLogV1> for Definition {
    fn from(value: ActionsLogLogV1) -> Self {
        Self::LogLogV1(value)
    }
}
impl ::std::convert::From<ActionsScheduleChangeEventStatesV1> for Definition {
    fn from(value: ActionsScheduleChangeEventStatesV1) -> Self {
        Self::ScheduleChangeEventStatesV1(value)
    }
}
impl ::std::convert::From<ActionsScheduleWaitV1> for Definition {
    fn from(value: ActionsScheduleWaitV1) -> Self {
        Self::ScheduleWaitV1(value)
    }
}
impl ::std::convert::From<ActionsTagModifyTagsV1> for Definition {
    fn from(value: ActionsTagModifyTagsV1) -> Self {
        Self::TagModifyTagsV1(value)
    }
}
impl ::std::convert::From<ActionsStopStop> for Definition {
    fn from(value: ActionsStopStop) -> Self {
        Self::StopStop(value)
    }
}
impl ::std::convert::From<ActionsWebfeedCreateWebfeedMessageV1> for Definition {
    fn from(value: ActionsWebfeedCreateWebfeedMessageV1) -> Self {
        Self::WebfeedCreateWebfeedMessageV1(value)
    }
}
impl ::std::convert::From<ActionsBranchSetBranch> for Definition {
    fn from(value: ActionsBranchSetBranch) -> Self {
        Self::BranchSetBranch(value)
    }
}
impl ::std::convert::From<ActionsBranchPauseBranch> for Definition {
    fn from(value: ActionsBranchPauseBranch) -> Self {
        Self::BranchPauseBranch(value)
    }
}
impl ::std::convert::From<ActionsScheduleSetBranchV1> for Definition {
    fn from(value: ActionsScheduleSetBranchV1) -> Self {
        Self::ScheduleSetBranchV1(value)
    }
}
#[doc = "Defines an event that should be processed by the ruleset, including filter criteria"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Event\","]
#[doc = "  \"description\": \"Defines an event that should be processed by the ruleset, including filter criteria\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"filter\","]
#[doc = "    \"id\","]
#[doc = "    \"metadata\","]
#[doc = "    \"parents\","]
#[doc = "    \"title\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"branch\": {"]
#[doc = "      \"title\": \"Branch ID\","]
#[doc = "      \"description\": \"The branch that this action belongs to\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"pattern\": \"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$\""]
#[doc = "    },"]
#[doc = "    \"filter\": {"]
#[doc = "      \"$ref\": \"#/$defs/filter\""]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"title\": \"Event ID\","]
#[doc = "      \"description\": \"Generated UUIDv4 representing this event for execution state.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$\""]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"title\": \"Metadata\","]
#[doc = "      \"description\": \"Metadata used by front-end applications\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"parents\": {"]
#[doc = "      \"title\": \"Parent Action IDs\","]
#[doc = "      \"description\": \"As list of action IDs that this type of event is processed for. When empty it is a top-level event for a ruleset.\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"type\": \"string\","]
#[doc = "        \"pattern\": \"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$\""]
#[doc = "      },"]
#[doc = "      \"minItems\": 0,"]
#[doc = "      \"uniqueItems\": true"]
#[doc = "    },"]
#[doc = "    \"recurring\": {"]
#[doc = "      \"title\": \"Recurring\","]
#[doc = "      \"description\": \"Indicate whether an event supports recurrence\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"title\": \"Optional Title\","]
#[doc = "      \"description\": \"A title that can be used for the UI to indicate a child campaign or logic branch\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"$ref\": \"#/$defs/event_type\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Event {
    #[doc = "The branch that this action belongs to"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub branch: ::std::option::Option<BranchId>,
    pub filter: Filter,
    #[doc = "Generated UUIDv4 representing this event for execution state."]
    pub id: EventId,
    #[doc = "Metadata used by front-end applications"]
    pub metadata: ::std::option::Option<::std::string::String>,
    #[doc = "As list of action IDs that this type of event is processed for. When empty it is a top-level event for a ruleset."]
    pub parents: Vec<ParentActionIDsItem>,
    #[doc = "Indicate whether an event supports recurrence"]
    #[serde(default)]
    pub recurring: bool,
    #[doc = "A title that can be used for the UI to indicate a child campaign or logic branch"]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "type")]
    pub type_: EventType,
}
impl Event {
    pub fn builder() -> builder::Event {
        Default::default()
    }
}
#[doc = "Generated UUIDv4 representing this event for execution state."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Event ID\","]
#[doc = "  \"description\": \"Generated UUIDv4 representing this event for execution state.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct EventId(::std::string::String);
impl ::std::ops::Deref for EventId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<EventId> for ::std::string::String {
    fn from(value: EventId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for EventId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new(
                    "^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$",
                )
                .unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err ("doesn't match pattern \"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$\"" . into ()) ;
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for EventId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EventId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for EventId {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Valid event types supported by the rules engine."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Event Type\","]
#[doc = "  \"description\": \"Valid event types supported by the rules engine.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"bounce.v1\","]
#[doc = "    \"campaign_state.v1\","]
#[doc = "    \"campaign_state.v2\","]
#[doc = "    \"click.v1\","]
#[doc = "    \"click.v2\","]
#[doc = "    \"deferral.v1\","]
#[doc = "    \"delivery.v1\","]
#[doc = "    \"drop.v1\","]
#[doc = "    \"open.v1\","]
#[doc = "    \"open.v2\","]
#[doc = "    \"pageview.v1\","]
#[doc = "    \"pageview.v2\","]
#[doc = "    \"spool.v1\","]
#[doc = "    \"subscribe.v1\","]
#[doc = "    \"tag.v1\","]
#[doc = "    \"unsubscribe.v1\","]
#[doc = "    \"wait_complete.v1\","]
#[doc = "    \"webfeed_wait_complete.v1\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum EventType {
    #[serde(rename = "bounce.v1")]
    BounceV1,
    #[serde(rename = "campaign_state.v1")]
    CampaignStateV1,
    #[serde(rename = "campaign_state.v2")]
    CampaignStateV2,
    #[serde(rename = "click.v1")]
    ClickV1,
    #[serde(rename = "click.v2")]
    ClickV2,
    #[serde(rename = "deferral.v1")]
    DeferralV1,
    #[serde(rename = "delivery.v1")]
    DeliveryV1,
    #[serde(rename = "drop.v1")]
    DropV1,
    #[serde(rename = "open.v1")]
    OpenV1,
    #[serde(rename = "open.v2")]
    OpenV2,
    #[serde(rename = "pageview.v1")]
    PageviewV1,
    #[serde(rename = "pageview.v2")]
    PageviewV2,
    #[serde(rename = "spool.v1")]
    SpoolV1,
    #[serde(rename = "subscribe.v1")]
    SubscribeV1,
    #[serde(rename = "tag.v1")]
    TagV1,
    #[serde(rename = "unsubscribe.v1")]
    UnsubscribeV1,
    #[serde(rename = "wait_complete.v1")]
    WaitCompleteV1,
    #[serde(rename = "webfeed_wait_complete.v1")]
    WebfeedWaitCompleteV1,
}
impl ::std::fmt::Display for EventType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::BounceV1 => f.write_str("bounce.v1"),
            Self::CampaignStateV1 => f.write_str("campaign_state.v1"),
            Self::CampaignStateV2 => f.write_str("campaign_state.v2"),
            Self::ClickV1 => f.write_str("click.v1"),
            Self::ClickV2 => f.write_str("click.v2"),
            Self::DeferralV1 => f.write_str("deferral.v1"),
            Self::DeliveryV1 => f.write_str("delivery.v1"),
            Self::DropV1 => f.write_str("drop.v1"),
            Self::OpenV1 => f.write_str("open.v1"),
            Self::OpenV2 => f.write_str("open.v2"),
            Self::PageviewV1 => f.write_str("pageview.v1"),
            Self::PageviewV2 => f.write_str("pageview.v2"),
            Self::SpoolV1 => f.write_str("spool.v1"),
            Self::SubscribeV1 => f.write_str("subscribe.v1"),
            Self::TagV1 => f.write_str("tag.v1"),
            Self::UnsubscribeV1 => f.write_str("unsubscribe.v1"),
            Self::WaitCompleteV1 => f.write_str("wait_complete.v1"),
            Self::WebfeedWaitCompleteV1 => f.write_str("webfeed_wait_complete.v1"),
        }
    }
}
impl ::std::str::FromStr for EventType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "bounce.v1" => Ok(Self::BounceV1),
            "campaign_state.v1" => Ok(Self::CampaignStateV1),
            "campaign_state.v2" => Ok(Self::CampaignStateV2),
            "click.v1" => Ok(Self::ClickV1),
            "click.v2" => Ok(Self::ClickV2),
            "deferral.v1" => Ok(Self::DeferralV1),
            "delivery.v1" => Ok(Self::DeliveryV1),
            "drop.v1" => Ok(Self::DropV1),
            "open.v1" => Ok(Self::OpenV1),
            "open.v2" => Ok(Self::OpenV2),
            "pageview.v1" => Ok(Self::PageviewV1),
            "pageview.v2" => Ok(Self::PageviewV2),
            "spool.v1" => Ok(Self::SpoolV1),
            "subscribe.v1" => Ok(Self::SubscribeV1),
            "tag.v1" => Ok(Self::TagV1),
            "unsubscribe.v1" => Ok(Self::UnsubscribeV1),
            "wait_complete.v1" => Ok(Self::WaitCompleteV1),
            "webfeed_wait_complete.v1" => Ok(Self::WebfeedWaitCompleteV1),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for EventType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for EventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for EventType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A filter definition for an event. It contains a filter type (any/all) and a list of criteria to match against."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Filter\","]
#[doc = "  \"description\": \"A filter definition for an event. It contains a filter type (any/all) and a list of criteria to match against.\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"None\","]
#[doc = "      \"type\": \"null\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"title\": \"Enabled\","]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"criteria\","]
#[doc = "        \"type\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"criteria\": {"]
#[doc = "          \"title\": \"Criteria\","]
#[doc = "          \"type\": \"array\","]
#[doc = "          \"items\": {"]
#[doc = "            \"$ref\": \"#/$defs/criteria_items\""]
#[doc = "          },"]
#[doc = "          \"minItems\": 1"]
#[doc = "        },"]
#[doc = "        \"id\": {"]
#[doc = "          \"title\": \"ID\","]
#[doc = "          \"description\": \"The branch or filter ID\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$\""]
#[doc = "        },"]
#[doc = "        \"metadata\": {"]
#[doc = "          \"title\": \"Metadata\","]
#[doc = "          \"description\": \"Metadata used by front-end applications\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"title\": {"]
#[doc = "          \"title\": \"Optional Title\","]
#[doc = "          \"description\": \"A title that can be used for the UI to indicate a logic branch\","]
#[doc = "          \"type\": ["]
#[doc = "            \"string\","]
#[doc = "            \"null\""]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        \"type\": {"]
#[doc = "          \"title\": \"Match on\","]
#[doc = "          \"description\": \"When set to any, if any criteria pass, the filter will return true. When set to all, all criteria must pass for the filter to return true.\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"any\","]
#[doc = "            \"all\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct Filter(pub ::std::option::Option<Filter>);
impl ::std::ops::Deref for Filter {
    type Target = ::std::option::Option<Filter>;
    fn deref(&self) -> &::std::option::Option<Filter> {
        &self.0
    }
}
impl ::std::convert::From<Filter> for ::std::option::Option<Filter> {
    fn from(value: Filter) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::option::Option<Filter>> for Filter {
    fn from(value: ::std::option::Option<Filter>) -> Self {
        Self(value)
    }
}
#[doc = "`Filter`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Enabled\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"criteria\","]
#[doc = "    \"type\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"criteria\": {"]
#[doc = "      \"title\": \"Criteria\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/criteria_items\""]
#[doc = "      },"]
#[doc = "      \"minItems\": 1"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"title\": \"ID\","]
#[doc = "      \"description\": \"The branch or filter ID\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$\""]
#[doc = "    },"]
#[doc = "    \"metadata\": {"]
#[doc = "      \"title\": \"Metadata\","]
#[doc = "      \"description\": \"Metadata used by front-end applications\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"title\": {"]
#[doc = "      \"title\": \"Optional Title\","]
#[doc = "      \"description\": \"A title that can be used for the UI to indicate a logic branch\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"type\": {"]
#[doc = "      \"title\": \"Match on\","]
#[doc = "      \"description\": \"When set to any, if any criteria pass, the filter will return true. When set to all, all criteria must pass for the filter to return true.\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"any\","]
#[doc = "        \"all\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Filter {
    pub criteria: ::std::vec::Vec<CriteriaItems>,
    #[doc = "The branch or filter ID"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<Id>,
    #[doc = "Metadata used by front-end applications"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub metadata: ::std::option::Option<::std::string::String>,
    #[doc = "A title that can be used for the UI to indicate a logic branch"]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub title: ::std::option::Option<::std::string::String>,
    #[doc = "When set to any, if any criteria pass, the filter will return true. When set to all, all criteria must pass for the filter to return true."]
    #[serde(rename = "type")]
    pub type_: MatchOn,
}
impl Filter {
    pub fn builder() -> builder::Filter {
        Default::default()
    }
}
#[doc = "`FunctionPath`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Function Path\","]
#[doc = "  \"default\": \"ruleset.campaign.filter.validate_serial_v1\","]
#[doc = "  \"readOnly\": true,"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^ruleset.campaign.filter.validate_serial_v1$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct FunctionPath(::std::string::String);
impl ::std::ops::Deref for FunctionPath {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<FunctionPath> for ::std::string::String {
    fn from(value: FunctionPath) -> Self {
        value.0
    }
}
impl ::std::default::Default for FunctionPath {
    fn default() -> Self {
        FunctionPath("ruleset.campaign.filter.validate_serial_v1".to_string())
    }
}
impl ::std::str::FromStr for FunctionPath {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^ruleset.campaign.filter.validate_serial_v1$").unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err(
                "doesn't match pattern \"^ruleset.campaign.filter.validate_serial_v1$\"".into(),
            );
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for FunctionPath {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FunctionPath {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FunctionPath {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for FunctionPath {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "The branch or filter ID"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ID\","]
#[doc = "  \"description\": \"The branch or filter ID\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Id(::std::string::String);
impl ::std::ops::Deref for Id {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Id> for ::std::string::String {
    fn from(value: Id) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for Id {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new(
                    "^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$",
                )
                .unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err ("doesn't match pattern \"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$\"" . into ()) ;
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Id {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Id {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Id {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Provide a recipient iterator that is used to amplify the actions of a ruleset to all recipients returned by the iterator"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Iterator\","]
#[doc = "  \"description\": \"Provide a recipient iterator that is used to amplify the actions of a ruleset to all recipients returned by the iterator\","]
#[doc = "  \"default\": null,"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"title\": \"Disabled\","]
#[doc = "      \"type\": \"null\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/iterators_mailing_list_recipients_v1\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/$defs/iterators_mailing_list_segment_v1\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Iterator {
    Disabled,
    IteratorsMailingListRecipientsV1(IteratorsMailingListRecipientsV1),
    IteratorsMailingListSegmentV1(IteratorsMailingListSegmentV1),
}
impl ::std::default::Default for Iterator {
    fn default() -> Self {
        Iterator::Disabled
    }
}
impl ::std::convert::From<IteratorsMailingListRecipientsV1> for Iterator {
    fn from(value: IteratorsMailingListRecipientsV1) -> Self {
        Self::IteratorsMailingListRecipientsV1(value)
    }
}
impl ::std::convert::From<IteratorsMailingListSegmentV1> for Iterator {
    fn from(value: IteratorsMailingListSegmentV1) -> Self {
        Self::IteratorsMailingListSegmentV1(value)
    }
}
#[doc = "Iterates over the list of recipient IDs for a given mailing list"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Mailing List Recipients\","]
#[doc = "  \"description\": \"Iterates over the list of recipient IDs for a given mailing list\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"ruleset.mailing_list.iterator.recipients_v1\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ruleset.mailing_list.iterator.recipients_v1\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"account\","]
#[doc = "        \"list\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"account\": {"]
#[doc = "          \"title\": \"Account\","]
#[doc = "          \"description\": \"The account that owns the mailing list\","]
#[doc = "          \"default\": \"<event:account>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:account>\""]
#[doc = "        },"]
#[doc = "        \"list\": {"]
#[doc = "          \"title\": \"List\","]
#[doc = "          \"description\": \"The mailing list ID\","]
#[doc = "          \"default\": \"<event:list>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:list>\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct IteratorsMailingListRecipientsV1 {
    pub function: ::std::string::String,
    pub kwargs: IteratorsMailingListRecipientsV1Kwargs,
}
impl IteratorsMailingListRecipientsV1 {
    pub fn builder() -> builder::IteratorsMailingListRecipientsV1 {
        Default::default()
    }
}
#[doc = "`IteratorsMailingListRecipientsV1Kwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"account\","]
#[doc = "    \"list\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"account\": {"]
#[doc = "      \"title\": \"Account\","]
#[doc = "      \"description\": \"The account that owns the mailing list\","]
#[doc = "      \"default\": \"<event:account>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:account>\""]
#[doc = "    },"]
#[doc = "    \"list\": {"]
#[doc = "      \"title\": \"List\","]
#[doc = "      \"description\": \"The mailing list ID\","]
#[doc = "      \"default\": \"<event:list>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:list>\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct IteratorsMailingListRecipientsV1Kwargs {
    #[doc = "The account that owns the mailing list"]
    pub account: ::std::string::String,
    #[doc = "The mailing list ID"]
    pub list: ::std::string::String,
}
impl IteratorsMailingListRecipientsV1Kwargs {
    pub fn builder() -> builder::IteratorsMailingListRecipientsV1Kwargs {
        Default::default()
    }
}
#[doc = "Iterates over a segment of recipient IDs for a given mailing list"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Mailing List Segment\","]
#[doc = "  \"description\": \"Iterates over a segment of recipient IDs for a given mailing list\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"function\","]
#[doc = "    \"kwargs\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"function\": {"]
#[doc = "      \"title\": \"Function Path\","]
#[doc = "      \"default\": \"ruleset.mailing_list.iterator.segment_v1\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"ruleset.mailing_list.iterator.segment_v1\""]
#[doc = "    },"]
#[doc = "    \"kwargs\": {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"required\": ["]
#[doc = "        \"account\","]
#[doc = "        \"list\","]
#[doc = "        \"segment\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"account\": {"]
#[doc = "          \"title\": \"Account\","]
#[doc = "          \"description\": \"The account that owns the mailing list\","]
#[doc = "          \"default\": \"<event:account>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:account>\""]
#[doc = "        },"]
#[doc = "        \"list\": {"]
#[doc = "          \"title\": \"List\","]
#[doc = "          \"description\": \"The mailing list ID\","]
#[doc = "          \"default\": \"<event:list>\","]
#[doc = "          \"readOnly\": true,"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"const\": \"<event:list>\""]
#[doc = "        },"]
#[doc = "        \"segment\": {"]
#[doc = "          \"title\": \"Segment\","]
#[doc = "          \"description\": \"The mailing list segment ID\","]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$\""]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct IteratorsMailingListSegmentV1 {
    pub function: ::std::string::String,
    pub kwargs: IteratorsMailingListSegmentV1Kwargs,
}
impl IteratorsMailingListSegmentV1 {
    pub fn builder() -> builder::IteratorsMailingListSegmentV1 {
        Default::default()
    }
}
#[doc = "`IteratorsMailingListSegmentV1Kwargs`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"account\","]
#[doc = "    \"list\","]
#[doc = "    \"segment\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"account\": {"]
#[doc = "      \"title\": \"Account\","]
#[doc = "      \"description\": \"The account that owns the mailing list\","]
#[doc = "      \"default\": \"<event:account>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:account>\""]
#[doc = "    },"]
#[doc = "    \"list\": {"]
#[doc = "      \"title\": \"List\","]
#[doc = "      \"description\": \"The mailing list ID\","]
#[doc = "      \"default\": \"<event:list>\","]
#[doc = "      \"readOnly\": true,"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"const\": \"<event:list>\""]
#[doc = "    },"]
#[doc = "    \"segment\": {"]
#[doc = "      \"title\": \"Segment\","]
#[doc = "      \"description\": \"The mailing list segment ID\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct IteratorsMailingListSegmentV1Kwargs {
    #[doc = "The account that owns the mailing list"]
    pub account: ::std::string::String,
    #[doc = "The mailing list ID"]
    pub list: ::std::string::String,
    #[doc = "The mailing list segment ID"]
    pub segment: Segment,
}
impl IteratorsMailingListSegmentV1Kwargs {
    pub fn builder() -> builder::IteratorsMailingListSegmentV1Kwargs {
        Default::default()
    }
}
#[doc = "The log level for the message"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Level\","]
#[doc = "  \"description\": \"The log level for the message\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"debug\","]
#[doc = "    \"info\","]
#[doc = "    \"warning\","]
#[doc = "    \"error\","]
#[doc = "    \"critical\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum Level {
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "critical")]
    Critical,
}
impl ::std::fmt::Display for Level {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Debug => f.write_str("debug"),
            Self::Info => f.write_str("info"),
            Self::Warning => f.write_str("warning"),
            Self::Error => f.write_str("error"),
            Self::Critical => f.write_str("critical"),
        }
    }
}
impl ::std::str::FromStr for Level {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "debug" => Ok(Self::Debug),
            "info" => Ok(Self::Info),
            "warning" => Ok(Self::Warning),
            "error" => Ok(Self::Error),
            "critical" => Ok(Self::Critical),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Level {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Level {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Level {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "When set to any, if any criteria pass, the filter will return true. When set to all, all criteria must pass for the filter to return true."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Match on\","]
#[doc = "  \"description\": \"When set to any, if any criteria pass, the filter will return true. When set to all, all criteria must pass for the filter to return true.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"any\","]
#[doc = "    \"all\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum MatchOn {
    #[serde(rename = "any")]
    Any,
    #[serde(rename = "all")]
    All,
}
impl ::std::fmt::Display for MatchOn {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Any => f.write_str("any"),
            Self::All => f.write_str("all"),
        }
    }
}
impl ::std::str::FromStr for MatchOn {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "any" => Ok(Self::Any),
            "all" => Ok(Self::All),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for MatchOn {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for MatchOn {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MatchOn {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A unique ID to use to memoize the result of the function"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Memoize ID\","]
#[doc = "  \"description\": \"A unique ID to use to memoize the result of the function\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12})$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct MemoizeId(::std::string::String);
impl ::std::ops::Deref for MemoizeId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<MemoizeId> for ::std::string::String {
    fn from(value: MemoizeId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for MemoizeId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(
            || {
                :: regress :: Regex :: new ("^([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12})$") . unwrap ()
            },
        );
        if PATTERN.find(value).is_none() {
            return Err ("doesn't match pattern \"^([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12})$\"" . into ()) ;
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for MemoizeId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for MemoizeId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MemoizeId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for MemoizeId {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "The message ID for the email to send. When using the <message:new> syntax, the template value should be the message editor ID."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Message\","]
#[doc = "  \"description\": \"The message ID for the email to send. When using the <message:new> syntax, the template value should be the message editor ID.\","]
#[doc = "  \"default\": \"<message:new message=123>\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^((<message:new message=[0-9a-fA-F]{24}>)|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}))$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Message(::std::string::String);
impl ::std::ops::Deref for Message {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Message> for ::std::string::String {
    fn from(value: Message) -> Self {
        value.0
    }
}
impl ::std::default::Default for Message {
    fn default() -> Self {
        Message("<message:new message=123>".to_string())
    }
}
impl ::std::str::FromStr for Message {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(
            || {
                :: regress :: Regex :: new ("^((<message:new message=[0-9a-fA-F]{24}>)|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}))$") . unwrap ()
            },
        );
        if PATTERN.find(value).is_none() {
            return Err ("doesn't match pattern \"^((<message:new message=[0-9a-fA-F]{24}>)|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}))$\"" . into ()) ;
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Message {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Message {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Message {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Message {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "The message api ID OID for the email template."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Message Editor ID\","]
#[doc = "  \"description\": \"The message api ID OID for the email template.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[0-9a-fA-F]{24}$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct MessageEditorId(::std::string::String);
impl ::std::ops::Deref for MessageEditorId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<MessageEditorId> for ::std::string::String {
    fn from(value: MessageEditorId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for MessageEditorId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[0-9a-fA-F]{24}$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[0-9a-fA-F]{24}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for MessageEditorId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for MessageEditorId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for MessageEditorId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for MessageEditorId {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "Operator to apply in the evaluation of the function return value and the expectation."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Comparison Operator\","]
#[doc = "  \"description\": \"Operator to apply in the evaluation of the function return value and the expectation.\","]
#[doc = "  \"default\": \"Equal\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"Equal\","]
#[doc = "    \"NotEqual\","]
#[doc = "    \"AtLeast\","]
#[doc = "    \"AtMost\","]
#[doc = "    \"Greater\","]
#[doc = "    \"Less\","]
#[doc = "    \"In\","]
#[doc = "    \"NotIn\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum Operators {
    #[serde(rename = "==")]
    Equal,
    #[serde(rename = "!=")]
    NotEqual,
    #[serde(rename = ">=")]
    AtLeast,
    #[serde(rename = "<=")]
    AtMost,
    #[serde(rename = ">")]
    Greater,
    #[serde(rename = "<")]
    Less,
    #[serde(rename = "in")]
    In,
    #[serde(rename = "not in")]
    NotIn,
}
impl ::std::fmt::Display for Operators {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Equal => f.write_str("=="),
            Self::NotEqual => f.write_str("!="),
            Self::AtLeast => f.write_str(">="),
            Self::AtMost => f.write_str("<="),
            Self::Greater => f.write_str(">"),
            Self::Less => f.write_str("<"),
            Self::In => f.write_str("in"),
            Self::NotIn => f.write_str("not in"),
        }
    }
}
impl ::std::str::FromStr for Operators {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "==" => Ok(Self::Equal),
            "!=" => Ok(Self::NotEqual),
            ">=" => Ok(Self::AtLeast),
            "<=" => Ok(Self::AtMost),
            ">" => Ok(Self::Greater),
            "<" => Ok(Self::Less),
            "in" => Ok(Self::In),
            "not in" => Ok(Self::NotIn),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for Operators {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Operators {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Operators {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::default::Default for Operators {
    fn default() -> Self {
        Operators::Equal
    }
}
#[doc = "The owner of the ruleset. For campaign builder, this will always be the account UUID."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Owner\","]
#[doc = "  \"description\": \"The owner of the ruleset. For campaign builder, this will always be the account UUID.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$|)\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Owner(::std::string::String);
impl ::std::ops::Deref for Owner {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Owner> for ::std::string::String {
    fn from(value: Owner) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for Owner {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(
            || {
                :: regress :: Regex :: new ("^([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$|)") . unwrap ()
            },
        );
        if PATTERN.find(value).is_none() {
            return Err ("doesn't match pattern \"^([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$|)\"" . into ()) ;
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Owner {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Owner {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Owner {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Owner {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`ParentActionIDsItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ParentActionIDsItem(::std::string::String);
impl ::std::ops::Deref for ParentActionIDsItem {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ParentActionIDsItem> for ::std::string::String {
    fn from(value: ParentActionIDsItem) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ParentActionIDsItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new(
                    "^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$",
                )
                .unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err ("doesn't match pattern \"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$\"" . into ()) ;
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ParentActionIDsItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ParentActionIDsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ParentActionIDsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ParentActionIDsItem {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`ParentEventIDsItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ParentEventIDsItem(::std::string::String);
impl ::std::ops::Deref for ParentEventIDsItem {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ParentEventIDsItem> for ::std::string::String {
    fn from(value: ParentEventIDsItem) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ParentEventIDsItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new(
                    "^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$",
                )
                .unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err ("doesn't match pattern \"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$\"" . into ()) ;
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ParentEventIDsItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for ParentEventIDsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ParentEventIDsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ParentEventIDsItem {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "The subscriber to modify tags on"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Recipient\","]
#[doc = "  \"description\": \"The subscriber to modify tags on\","]
#[doc = "  \"default\": \"<event:recipient>\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^<(event:recipient|subscriber)>$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Recipient(::std::string::String);
impl ::std::ops::Deref for Recipient {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Recipient> for ::std::string::String {
    fn from(value: Recipient) -> Self {
        value.0
    }
}
impl ::std::default::Default for Recipient {
    fn default() -> Self {
        Recipient("<event:recipient>".to_string())
    }
}
impl ::std::str::FromStr for Recipient {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^<(event:recipient|subscriber)>$").unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^<(event:recipient|subscriber)>$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Recipient {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Recipient {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Recipient {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Recipient {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "The expected return value from the function"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Return Exepctation\","]
#[doc = "  \"description\": \"The expected return value from the function\","]
#[doc = "  \"default\": true,"]
#[doc = "  \"type\": ["]
#[doc = "    \"boolean\","]
#[doc = "    \"string\","]
#[doc = "    \"integer\","]
#[doc = "    \"null\","]
#[doc = "    \"number\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ReturnExepctation {
    Null,
    Boolean(bool),
    Integer(i64),
    Number(f64),
    String(::std::string::String),
}
impl ::std::default::Default for ReturnExepctation {
    fn default() -> Self {
        ReturnExepctation::Boolean(true)
    }
}
impl ::std::convert::From<bool> for ReturnExepctation {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
impl ::std::convert::From<i64> for ReturnExepctation {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
impl ::std::convert::From<f64> for ReturnExepctation {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[doc = "The ID of the ruleset in the Rule service"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Rule ID\","]
#[doc = "  \"description\": \"The ID of the ruleset in the Rule service\","]
#[doc = "  \"default\": \"<event:ruleset>\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^((<event:\\\\w+>)|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}))$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RuleId(::std::string::String);
impl ::std::ops::Deref for RuleId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RuleId> for ::std::string::String {
    fn from(value: RuleId) -> Self {
        value.0
    }
}
impl ::std::default::Default for RuleId {
    fn default() -> Self {
        RuleId("<event:ruleset>".to_string())
    }
}
impl ::std::str::FromStr for RuleId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(
            || {
                :: regress :: Regex :: new ("^((<event:\\w+>)|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}))$") . unwrap ()
            },
        );
        if PATTERN.find(value).is_none() {
            return Err ("doesn't match pattern \"^((<event:\\w+>)|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}))$\"" . into ()) ;
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RuleId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RuleId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RuleId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RuleId {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "A ruleset contains a nested list of events, their criteria, and actions to take when the criteria matches for an event"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Ruleset\","]
#[doc = "  \"description\": \"A ruleset contains a nested list of events, their criteria, and actions to take when the criteria matches for an event\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"actions\","]
#[doc = "    \"events\","]
#[doc = "    \"id\","]
#[doc = "    \"iterator\","]
#[doc = "    \"owner\","]
#[doc = "    \"parent\","]
#[doc = "    \"state\","]
#[doc = "    \"system\","]
#[doc = "    \"version\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"actions\": {"]
#[doc = "      \"title\": \"Actions\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/action\""]
#[doc = "      },"]
#[doc = "      \"minItems\": 1"]
#[doc = "    },"]
#[doc = "    \"events\": {"]
#[doc = "      \"title\": \"Events\","]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/$defs/event\""]
#[doc = "      },"]
#[doc = "      \"minItems\": 1"]
#[doc = "    },"]
#[doc = "    \"id\": {"]
#[doc = "      \"title\": \"Ruleset ID\","]
#[doc = "      \"description\": \"A unique ID for the ruleset\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"pattern\": \"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$\""]
#[doc = "    },"]
#[doc = "    \"iterator\": {"]
#[doc = "      \"title\": \"Iterator\","]
#[doc = "      \"description\": \"Provide a recipient iterator that is used to amplify the actions of a ruleset to all recipients returned by the iterator\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"title\": \"Disabled\","]
#[doc = "          \"type\": \"null\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/iterators_mailing_list_recipients_v1\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/$defs/iterators_mailing_list_segment_v1\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"owner\": {"]
#[doc = "      \"title\": \"Owner\","]
#[doc = "      \"description\": \"The owner of the ruleset. For campaign builder, this will always be the account UUID.\","]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ],"]
#[doc = "      \"pattern\": \"^([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$|)\""]
#[doc = "    },"]
#[doc = "    \"parent\": {"]
#[doc = "      \"title\": \"Parent\","]
#[doc = "      \"description\": \"The parent for the ruleset. For campaign builder, this will most likely be the list UUID.\","]
#[doc = "      \"oneOf\": ["]
#[doc = "        {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"pattern\": \"^([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$|)\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"state\": {"]
#[doc = "      \"title\": \"Ruleset State\","]
#[doc = "      \"default\": \"draft\","]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"draft\","]
#[doc = "        \"active\","]
#[doc = "        \"paused\","]
#[doc = "        \"draining\","]
#[doc = "        \"stopped\","]
#[doc = "        \"archived\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"system\": {"]
#[doc = "      \"title\": \"System\","]
#[doc = "      \"description\": \"Is this a AWeber Internal System ruleset or a customer-facing campaign ruleset.\","]
#[doc = "      \"default\": false,"]
#[doc = "      \"type\": \"boolean\""]
#[doc = "    },"]
#[doc = "    \"version\": {"]
#[doc = "      \"title\": \"Version\","]
#[doc = "      \"description\": \"The ruleset format version\","]
#[doc = "      \"default\": 2,"]
#[doc = "      \"type\": \"number\""]
#[doc = "    }"]
#[doc = "  },"]
#[doc = "  \"additionalProperties\": false"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct Ruleset {
    pub actions: ::std::vec::Vec<Action>,
    pub events: ::std::vec::Vec<Event>,
    #[doc = "A unique ID for the ruleset"]
    pub id: RulesetId,
    #[doc = "Provide a recipient iterator that is used to amplify the actions of a ruleset to all recipients returned by the iterator"]
    pub iterator: Iterator,
    #[doc = "The owner of the ruleset. For campaign builder, this will always be the account UUID."]
    pub owner: ::std::option::Option<Owner>,
    #[doc = "The parent for the ruleset. For campaign builder, this will most likely be the list UUID."]
    pub parent: ::std::option::Option<RulesetParent>,
    pub state: RulesetState,
    #[doc = "Is this a AWeber Internal System ruleset or a customer-facing campaign ruleset."]
    pub system: bool,
    #[doc = "The ruleset format version"]
    pub version: f64,
}
impl Ruleset {
    pub fn builder() -> builder::Ruleset {
        Default::default()
    }
}
#[doc = "The ruleset whose state has changed."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Ruleset ID\","]
#[doc = "  \"description\": \"The ruleset whose state has changed.\","]
#[doc = "  \"default\": \"<event:ruleset>\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^((<event:\\\\w+>)|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}))$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RulesetId(::std::string::String);
impl ::std::ops::Deref for RulesetId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RulesetId> for ::std::string::String {
    fn from(value: RulesetId) -> Self {
        value.0
    }
}
impl ::std::default::Default for RulesetId {
    fn default() -> Self {
        RulesetId("<event:ruleset>".to_string())
    }
}
impl ::std::str::FromStr for RulesetId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(
            || {
                :: regress :: Regex :: new ("^((<event:\\w+>)|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}))$") . unwrap ()
            },
        );
        if PATTERN.find(value).is_none() {
            return Err ("doesn't match pattern \"^((<event:\\w+>)|([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}))$\"" . into ()) ;
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RulesetId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RulesetId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RulesetId {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`RulesetParent`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$|)\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RulesetParent(::std::string::String);
impl ::std::ops::Deref for RulesetParent {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RulesetParent> for ::std::string::String {
    fn from(value: RulesetParent) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RulesetParent {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> = ::std::sync::LazyLock::new(
            || {
                :: regress :: Regex :: new ("^([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$|)") . unwrap ()
            },
        );
        if PATTERN.find(value).is_none() {
            return Err ("doesn't match pattern \"^([a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$|)\"" . into ()) ;
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RulesetParent {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RulesetParent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetParent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RulesetParent {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "The state that the Ruleset transitioned to"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Ruleset state\","]
#[doc = "  \"description\": \"The state that the Ruleset transitioned to\","]
#[doc = "  \"default\": \"<event:value>\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^<event:value>\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RulesetState(::std::string::String);
impl ::std::ops::Deref for RulesetState {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RulesetState> for ::std::string::String {
    fn from(value: RulesetState) -> Self {
        value.0
    }
}
impl ::std::default::Default for RulesetState {
    fn default() -> Self {
        RulesetState("<event:value>".to_string())
    }
}
impl ::std::str::FromStr for RulesetState {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^<event:value>").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^<event:value>\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RulesetState {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for RulesetState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RulesetState {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RulesetState {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "The mailing list segment ID"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Segment\","]
#[doc = "  \"description\": \"The mailing list segment ID\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Segment(::std::string::String);
impl ::std::ops::Deref for Segment {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Segment> for ::std::string::String {
    fn from(value: Segment) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for Segment {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new(
                    "^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$",
                )
                .unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err ("doesn't match pattern \"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$\"" . into ()) ;
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Segment {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for Segment {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Segment {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Segment {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "The message api ObjectID for the message template."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Template  message id\","]
#[doc = "  \"description\": \"The message api ObjectID for the message template.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[0-9a-fA-F]{24}$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TemplateMessageId(::std::string::String);
impl ::std::ops::Deref for TemplateMessageId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TemplateMessageId> for ::std::string::String {
    fn from(value: TemplateMessageId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for TemplateMessageId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[0-9a-fA-F]{24}$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[0-9a-fA-F]{24}$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TemplateMessageId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TemplateMessageId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TemplateMessageId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TemplateMessageId {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "The RSS URL to check for new posts"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"The blog RSS URL\","]
#[doc = "  \"description\": \"The RSS URL to check for new posts\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"format\": \"uri\","]
#[doc = "  \"pattern\": \"^https?://*\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct TheBlogRssUrl(::std::string::String);
impl ::std::ops::Deref for TheBlogRssUrl {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<TheBlogRssUrl> for ::std::string::String {
    fn from(value: TheBlogRssUrl) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for TheBlogRssUrl {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^https?://*").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^https?://*\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for TheBlogRssUrl {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for TheBlogRssUrl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TheBlogRssUrl {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for TheBlogRssUrl {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`UrlFragmentsItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"minLength\": 1"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct UrlFragmentsItem(::std::string::String);
impl ::std::ops::Deref for UrlFragmentsItem {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<UrlFragmentsItem> for ::std::string::String {
    fn from(value: UrlFragmentsItem) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for UrlFragmentsItem {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for UrlFragmentsItem {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for UrlFragmentsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UrlFragmentsItem {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for UrlFragmentsItem {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`ValuesItem`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": ["]
#[doc = "    \"integer\","]
#[doc = "    \"string\","]
#[doc = "    \"boolean\","]
#[doc = "    \"number\","]
#[doc = "    \"null\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum ValuesItem {
    Null,
    Boolean(bool),
    Integer(i64),
    Number(f64),
    String(::std::string::String),
}
impl ::std::convert::From<bool> for ValuesItem {
    fn from(value: bool) -> Self {
        Self::Boolean(value)
    }
}
impl ::std::convert::From<i64> for ValuesItem {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
impl ::std::convert::From<f64> for ValuesItem {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
#[doc = "Generated ID that uniquely identifies the branch wait event"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Wait ID\","]
#[doc = "  \"description\": \"Generated ID that uniquely identifies the branch wait event\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"pattern\": \"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$\""]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct WaitId(::std::string::String);
impl ::std::ops::Deref for WaitId {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<WaitId> for ::std::string::String {
    fn from(value: WaitId) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for WaitId {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new(
                    "^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$",
                )
                .unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err ("doesn't match pattern \"^[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}$\"" . into ()) ;
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for WaitId {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for WaitId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WaitId {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for WaitId {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "The webfeed to create message from."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"Webfeed\","]
#[doc = "  \"description\": \"The webfeed to create message from.\","]
#[doc = "  \"default\": \"<event:webfeed>\","]
#[doc = "  \"readOnly\": true,"]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"const\": \"<event:webfeed>\""]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(untagged)]
pub enum Webfeed {
    Variant0(i64),
    Variant1(::serde_json::Value),
}
impl ::std::default::Default for Webfeed {
    fn default() -> Self {
        Webfeed::Variant1(
            ::serde_json::from_str::<::serde_json::Value>("\"<event:webfeed>\"").unwrap(),
        )
    }
}
impl ::std::convert::From<i64> for Webfeed {
    fn from(value: i64) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<::serde_json::Value> for Webfeed {
    fn from(value: ::serde_json::Value) -> Self {
        Self::Variant1(value)
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct Action {
        branch:
            ::std::result::Result<::std::option::Option<super::BranchId>, ::std::string::String>,
        definition: ::std::result::Result<super::Definition, ::std::string::String>,
        id: ::std::result::Result<super::ActionId, ::std::string::String>,
        is_deleted: ::std::result::Result<bool, ::std::string::String>,
        metadata: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        parents: ::std::result::Result<Vec<super::ParentEventIDsItem>, ::std::string::String>,
        recurring: ::std::result::Result<bool, ::std::string::String>,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for Action {
        fn default() -> Self {
            Self {
                branch: Ok(Default::default()),
                definition: Err("no value supplied for definition".to_string()),
                id: Err("no value supplied for id".to_string()),
                is_deleted: Ok(Default::default()),
                metadata: Err("no value supplied for metadata".to_string()),
                parents: Err("no value supplied for parents".to_string()),
                recurring: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
            }
        }
    }
    impl Action {
        pub fn branch<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::BranchId>>,
            T::Error: ::std::fmt::Display,
        {
            self.branch = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for branch: {e}"));
            self
        }
        pub fn definition<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Definition>,
            T::Error: ::std::fmt::Display,
        {
            self.definition = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for definition: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActionId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn is_deleted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.is_deleted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for is_deleted: {e}"));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn parents<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<Vec<super::ParentEventIDsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.parents = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for parents: {e}"));
            self
        }
        pub fn recurring<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.recurring = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for recurring: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Action> for super::Action {
        type Error = super::error::ConversionError;
        fn try_from(value: Action) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                branch: value.branch?,
                definition: value.definition?,
                id: value.id?,
                is_deleted: value.is_deleted?,
                metadata: value.metadata?,
                parents: value.parents?,
                recurring: value.recurring?,
                title: value.title?,
            })
        }
    }
    impl ::std::convert::From<super::Action> for Action {
        fn from(value: super::Action) -> Self {
            Self {
                branch: Ok(value.branch),
                definition: Ok(value.definition),
                id: Ok(value.id),
                is_deleted: Ok(value.is_deleted),
                metadata: Ok(value.metadata),
                parents: Ok(value.parents),
                recurring: Ok(value.recurring),
                title: Ok(value.title),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActionsBranchPauseBranch {
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<super::ActionsBranchPauseBranchKwargs, ::std::string::String>,
    }
    impl ::std::default::Default for ActionsBranchPauseBranch {
        fn default() -> Self {
            Self {
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
            }
        }
    }
    impl ActionsBranchPauseBranch {
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActionsBranchPauseBranchKwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ActionsBranchPauseBranch> for super::ActionsBranchPauseBranch {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActionsBranchPauseBranch,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                function: value.function?,
                kwargs: value.kwargs?,
            })
        }
    }
    impl ::std::convert::From<super::ActionsBranchPauseBranch> for ActionsBranchPauseBranch {
        fn from(value: super::ActionsBranchPauseBranch) -> Self {
            Self {
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActionsBranchPauseBranchKwargs {
        ruleset: ::std::result::Result<::std::string::String, ::std::string::String>,
        subscriber: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ActionsBranchPauseBranchKwargs {
        fn default() -> Self {
            Self {
                ruleset: Err("no value supplied for ruleset".to_string()),
                subscriber: Err("no value supplied for subscriber".to_string()),
            }
        }
    }
    impl ActionsBranchPauseBranchKwargs {
        pub fn ruleset<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.ruleset = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ruleset: {e}"));
            self
        }
        pub fn subscriber<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.subscriber = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subscriber: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ActionsBranchPauseBranchKwargs>
        for super::ActionsBranchPauseBranchKwargs
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActionsBranchPauseBranchKwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ruleset: value.ruleset?,
                subscriber: value.subscriber?,
            })
        }
    }
    impl ::std::convert::From<super::ActionsBranchPauseBranchKwargs>
        for ActionsBranchPauseBranchKwargs
    {
        fn from(value: super::ActionsBranchPauseBranchKwargs) -> Self {
            Self {
                ruleset: Ok(value.ruleset),
                subscriber: Ok(value.subscriber),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActionsBranchSetBranch {
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<super::ActionsBranchSetBranchKwargs, ::std::string::String>,
    }
    impl ::std::default::Default for ActionsBranchSetBranch {
        fn default() -> Self {
            Self {
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
            }
        }
    }
    impl ActionsBranchSetBranch {
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActionsBranchSetBranchKwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ActionsBranchSetBranch> for super::ActionsBranchSetBranch {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActionsBranchSetBranch,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                function: value.function?,
                kwargs: value.kwargs?,
            })
        }
    }
    impl ::std::convert::From<super::ActionsBranchSetBranch> for ActionsBranchSetBranch {
        fn from(value: super::ActionsBranchSetBranch) -> Self {
            Self {
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActionsBranchSetBranchKwargs {
        branches:
            ::std::result::Result<::std::vec::Vec<super::BranchesItem>, ::std::string::String>,
        ruleset: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ActionsBranchSetBranchKwargs {
        fn default() -> Self {
            Self {
                branches: Err("no value supplied for branches".to_string()),
                ruleset: Err("no value supplied for ruleset".to_string()),
            }
        }
    }
    impl ActionsBranchSetBranchKwargs {
        pub fn branches<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::BranchesItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.branches = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for branches: {e}"));
            self
        }
        pub fn ruleset<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.ruleset = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ruleset: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ActionsBranchSetBranchKwargs> for super::ActionsBranchSetBranchKwargs {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActionsBranchSetBranchKwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                branches: value.branches?,
                ruleset: value.ruleset?,
            })
        }
    }
    impl ::std::convert::From<super::ActionsBranchSetBranchKwargs> for ActionsBranchSetBranchKwargs {
        fn from(value: super::ActionsBranchSetBranchKwargs) -> Self {
            Self {
                branches: Ok(value.branches),
                ruleset: Ok(value.ruleset),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActionsCampaignChangeRuleStateV1 {
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<
            super::ActionsCampaignChangeRuleStateV1Kwargs,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ActionsCampaignChangeRuleStateV1 {
        fn default() -> Self {
            Self {
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
            }
        }
    }
    impl ActionsCampaignChangeRuleStateV1 {
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActionsCampaignChangeRuleStateV1Kwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ActionsCampaignChangeRuleStateV1>
        for super::ActionsCampaignChangeRuleStateV1
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActionsCampaignChangeRuleStateV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                function: value.function?,
                kwargs: value.kwargs?,
            })
        }
    }
    impl ::std::convert::From<super::ActionsCampaignChangeRuleStateV1>
        for ActionsCampaignChangeRuleStateV1
    {
        fn from(value: super::ActionsCampaignChangeRuleStateV1) -> Self {
            Self {
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActionsCampaignChangeRuleStateV1Kwargs {
        ruleset: ::std::result::Result<super::RuleId, ::std::string::String>,
        state: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ActionsCampaignChangeRuleStateV1Kwargs {
        fn default() -> Self {
            Self {
                ruleset: Err("no value supplied for ruleset".to_string()),
                state: Err("no value supplied for state".to_string()),
            }
        }
    }
    impl ActionsCampaignChangeRuleStateV1Kwargs {
        pub fn ruleset<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RuleId>,
            T::Error: ::std::fmt::Display,
        {
            self.ruleset = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ruleset: {e}"));
            self
        }
        pub fn state<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.state = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for state: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ActionsCampaignChangeRuleStateV1Kwargs>
        for super::ActionsCampaignChangeRuleStateV1Kwargs
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActionsCampaignChangeRuleStateV1Kwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ruleset: value.ruleset?,
                state: value.state?,
            })
        }
    }
    impl ::std::convert::From<super::ActionsCampaignChangeRuleStateV1Kwargs>
        for ActionsCampaignChangeRuleStateV1Kwargs
    {
        fn from(value: super::ActionsCampaignChangeRuleStateV1Kwargs) -> Self {
            Self {
                ruleset: Ok(value.ruleset),
                state: Ok(value.state),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActionsEmailSendV1 {
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<super::ActionsEmailSendV1Kwargs, ::std::string::String>,
    }
    impl ::std::default::Default for ActionsEmailSendV1 {
        fn default() -> Self {
            Self {
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
            }
        }
    }
    impl ActionsEmailSendV1 {
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActionsEmailSendV1Kwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ActionsEmailSendV1> for super::ActionsEmailSendV1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActionsEmailSendV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                function: value.function?,
                kwargs: value.kwargs?,
            })
        }
    }
    impl ::std::convert::From<super::ActionsEmailSendV1> for ActionsEmailSendV1 {
        fn from(value: super::ActionsEmailSendV1) -> Self {
            Self {
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActionsEmailSendV1Kwargs {
        account: ::std::result::Result<::std::string::String, ::std::string::String>,
        list: ::std::result::Result<::std::string::String, ::std::string::String>,
        meapi_id: ::std::result::Result<super::MessageEditorId, ::std::string::String>,
        message: ::std::result::Result<super::Message, ::std::string::String>,
        recipient: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ActionsEmailSendV1Kwargs {
        fn default() -> Self {
            Self {
                account: Err("no value supplied for account".to_string()),
                list: Err("no value supplied for list".to_string()),
                meapi_id: Err("no value supplied for meapi_id".to_string()),
                message: Err("no value supplied for message".to_string()),
                recipient: Err("no value supplied for recipient".to_string()),
            }
        }
    }
    impl ActionsEmailSendV1Kwargs {
        pub fn account<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.account = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for account: {e}"));
            self
        }
        pub fn list<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.list = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list: {e}"));
            self
        }
        pub fn meapi_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MessageEditorId>,
            T::Error: ::std::fmt::Display,
        {
            self.meapi_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for meapi_id: {e}"));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Message>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
        pub fn recipient<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.recipient = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for recipient: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ActionsEmailSendV1Kwargs> for super::ActionsEmailSendV1Kwargs {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActionsEmailSendV1Kwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                account: value.account?,
                list: value.list?,
                meapi_id: value.meapi_id?,
                message: value.message?,
                recipient: value.recipient?,
            })
        }
    }
    impl ::std::convert::From<super::ActionsEmailSendV1Kwargs> for ActionsEmailSendV1Kwargs {
        fn from(value: super::ActionsEmailSendV1Kwargs) -> Self {
            Self {
                account: Ok(value.account),
                list: Ok(value.list),
                meapi_id: Ok(value.meapi_id),
                message: Ok(value.message),
                recipient: Ok(value.recipient),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActionsFollowupEnableV1 {
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<super::ActionsFollowupEnableV1Kwargs, ::std::string::String>,
    }
    impl ::std::default::Default for ActionsFollowupEnableV1 {
        fn default() -> Self {
            Self {
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
            }
        }
    }
    impl ActionsFollowupEnableV1 {
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActionsFollowupEnableV1Kwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ActionsFollowupEnableV1> for super::ActionsFollowupEnableV1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActionsFollowupEnableV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                function: value.function?,
                kwargs: value.kwargs?,
            })
        }
    }
    impl ::std::convert::From<super::ActionsFollowupEnableV1> for ActionsFollowupEnableV1 {
        fn from(value: super::ActionsFollowupEnableV1) -> Self {
            Self {
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActionsFollowupEnableV1Kwargs {
        recipient: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ActionsFollowupEnableV1Kwargs {
        fn default() -> Self {
            Self {
                recipient: Err("no value supplied for recipient".to_string()),
            }
        }
    }
    impl ActionsFollowupEnableV1Kwargs {
        pub fn recipient<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.recipient = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for recipient: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ActionsFollowupEnableV1Kwargs>
        for super::ActionsFollowupEnableV1Kwargs
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActionsFollowupEnableV1Kwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                recipient: value.recipient?,
            })
        }
    }
    impl ::std::convert::From<super::ActionsFollowupEnableV1Kwargs> for ActionsFollowupEnableV1Kwargs {
        fn from(value: super::ActionsFollowupEnableV1Kwargs) -> Self {
            Self {
                recipient: Ok(value.recipient),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActionsFollowupSendAutoresponseV1 {
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<
            super::ActionsFollowupSendAutoresponseV1Kwargs,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ActionsFollowupSendAutoresponseV1 {
        fn default() -> Self {
            Self {
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
            }
        }
    }
    impl ActionsFollowupSendAutoresponseV1 {
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActionsFollowupSendAutoresponseV1Kwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ActionsFollowupSendAutoresponseV1>
        for super::ActionsFollowupSendAutoresponseV1
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActionsFollowupSendAutoresponseV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                function: value.function?,
                kwargs: value.kwargs?,
            })
        }
    }
    impl ::std::convert::From<super::ActionsFollowupSendAutoresponseV1>
        for ActionsFollowupSendAutoresponseV1
    {
        fn from(value: super::ActionsFollowupSendAutoresponseV1) -> Self {
            Self {
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActionsFollowupSendAutoresponseV1Kwargs {
        recipient: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ActionsFollowupSendAutoresponseV1Kwargs {
        fn default() -> Self {
            Self {
                recipient: Err("no value supplied for recipient".to_string()),
            }
        }
    }
    impl ActionsFollowupSendAutoresponseV1Kwargs {
        pub fn recipient<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.recipient = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for recipient: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ActionsFollowupSendAutoresponseV1Kwargs>
        for super::ActionsFollowupSendAutoresponseV1Kwargs
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActionsFollowupSendAutoresponseV1Kwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                recipient: value.recipient?,
            })
        }
    }
    impl ::std::convert::From<super::ActionsFollowupSendAutoresponseV1Kwargs>
        for ActionsFollowupSendAutoresponseV1Kwargs
    {
        fn from(value: super::ActionsFollowupSendAutoresponseV1Kwargs) -> Self {
            Self {
                recipient: Ok(value.recipient),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActionsLogLogV1 {
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<super::ActionsLogLogV1Kwargs, ::std::string::String>,
    }
    impl ::std::default::Default for ActionsLogLogV1 {
        fn default() -> Self {
            Self {
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
            }
        }
    }
    impl ActionsLogLogV1 {
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActionsLogLogV1Kwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ActionsLogLogV1> for super::ActionsLogLogV1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActionsLogLogV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                function: value.function?,
                kwargs: value.kwargs?,
            })
        }
    }
    impl ::std::convert::From<super::ActionsLogLogV1> for ActionsLogLogV1 {
        fn from(value: super::ActionsLogLogV1) -> Self {
            Self {
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActionsLogLogV1Kwargs {
        event: ::std::result::Result<::std::string::String, ::std::string::String>,
        level: ::std::result::Result<super::Level, ::std::string::String>,
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ActionsLogLogV1Kwargs {
        fn default() -> Self {
            Self {
                event: Err("no value supplied for event".to_string()),
                level: Err("no value supplied for level".to_string()),
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl ActionsLogLogV1Kwargs {
        pub fn event<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.event = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for event: {e}"));
            self
        }
        pub fn level<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Level>,
            T::Error: ::std::fmt::Display,
        {
            self.level = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for level: {e}"));
            self
        }
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ActionsLogLogV1Kwargs> for super::ActionsLogLogV1Kwargs {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActionsLogLogV1Kwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                event: value.event?,
                level: value.level?,
                message: value.message?,
            })
        }
    }
    impl ::std::convert::From<super::ActionsLogLogV1Kwargs> for ActionsLogLogV1Kwargs {
        fn from(value: super::ActionsLogLogV1Kwargs) -> Self {
            Self {
                event: Ok(value.event),
                level: Ok(value.level),
                message: Ok(value.message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActionsScheduleChangeEventStatesV1 {
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<
            super::ActionsScheduleChangeEventStatesV1Kwargs,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ActionsScheduleChangeEventStatesV1 {
        fn default() -> Self {
            Self {
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
            }
        }
    }
    impl ActionsScheduleChangeEventStatesV1 {
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActionsScheduleChangeEventStatesV1Kwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ActionsScheduleChangeEventStatesV1>
        for super::ActionsScheduleChangeEventStatesV1
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActionsScheduleChangeEventStatesV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                function: value.function?,
                kwargs: value.kwargs?,
            })
        }
    }
    impl ::std::convert::From<super::ActionsScheduleChangeEventStatesV1>
        for ActionsScheduleChangeEventStatesV1
    {
        fn from(value: super::ActionsScheduleChangeEventStatesV1) -> Self {
            Self {
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActionsScheduleChangeEventStatesV1Kwargs {
        ruleset: ::std::result::Result<super::RulesetId, ::std::string::String>,
        state: ::std::result::Result<super::RulesetState, ::std::string::String>,
    }
    impl ::std::default::Default for ActionsScheduleChangeEventStatesV1Kwargs {
        fn default() -> Self {
            Self {
                ruleset: Err("no value supplied for ruleset".to_string()),
                state: Err("no value supplied for state".to_string()),
            }
        }
    }
    impl ActionsScheduleChangeEventStatesV1Kwargs {
        pub fn ruleset<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RulesetId>,
            T::Error: ::std::fmt::Display,
        {
            self.ruleset = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ruleset: {e}"));
            self
        }
        pub fn state<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RulesetState>,
            T::Error: ::std::fmt::Display,
        {
            self.state = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for state: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ActionsScheduleChangeEventStatesV1Kwargs>
        for super::ActionsScheduleChangeEventStatesV1Kwargs
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActionsScheduleChangeEventStatesV1Kwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ruleset: value.ruleset?,
                state: value.state?,
            })
        }
    }
    impl ::std::convert::From<super::ActionsScheduleChangeEventStatesV1Kwargs>
        for ActionsScheduleChangeEventStatesV1Kwargs
    {
        fn from(value: super::ActionsScheduleChangeEventStatesV1Kwargs) -> Self {
            Self {
                ruleset: Ok(value.ruleset),
                state: Ok(value.state),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActionsScheduleSetBranchV1 {
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs:
            ::std::result::Result<super::ActionsScheduleSetBranchV1Kwargs, ::std::string::String>,
    }
    impl ::std::default::Default for ActionsScheduleSetBranchV1 {
        fn default() -> Self {
            Self {
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
            }
        }
    }
    impl ActionsScheduleSetBranchV1 {
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActionsScheduleSetBranchV1Kwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ActionsScheduleSetBranchV1> for super::ActionsScheduleSetBranchV1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActionsScheduleSetBranchV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                function: value.function?,
                kwargs: value.kwargs?,
            })
        }
    }
    impl ::std::convert::From<super::ActionsScheduleSetBranchV1> for ActionsScheduleSetBranchV1 {
        fn from(value: super::ActionsScheduleSetBranchV1) -> Self {
            Self {
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActionsScheduleSetBranchV1Kwargs {
        account: ::std::result::Result<::std::string::String, ::std::string::String>,
        action: ::std::result::Result<super::ActionId, ::std::string::String>,
        branches:
            ::std::result::Result<::std::vec::Vec<super::BranchesItem>, ::std::string::String>,
        delay: ::std::result::Result<::std::string::String, ::std::string::String>,
        id: ::std::result::Result<super::WaitId, ::std::string::String>,
        list: ::std::result::Result<::std::string::String, ::std::string::String>,
        recipient: ::std::result::Result<::std::string::String, ::std::string::String>,
        rrules: ::std::result::Result<
            ::std::option::Option<Vec<::std::string::String>>,
            ::std::string::String,
        >,
        ruleset: ::std::result::Result<::std::string::String, ::std::string::String>,
        timezone: ::std::result::Result<::std::string::String, ::std::string::String>,
        use_subscriber_timezone: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for ActionsScheduleSetBranchV1Kwargs {
        fn default() -> Self {
            Self {
                account: Err("no value supplied for account".to_string()),
                action: Err("no value supplied for action".to_string()),
                branches: Err("no value supplied for branches".to_string()),
                delay: Err("no value supplied for delay".to_string()),
                id: Err("no value supplied for id".to_string()),
                list: Err("no value supplied for list".to_string()),
                recipient: Err("no value supplied for recipient".to_string()),
                rrules: Ok(Default::default()),
                ruleset: Err("no value supplied for ruleset".to_string()),
                timezone: Ok(super::defaults::actions_schedule_set_branch_v1_kwargs_timezone()),
                use_subscriber_timezone: Ok(Default::default()),
            }
        }
    }
    impl ActionsScheduleSetBranchV1Kwargs {
        pub fn account<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.account = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for account: {e}"));
            self
        }
        pub fn action<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActionId>,
            T::Error: ::std::fmt::Display,
        {
            self.action = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for action: {e}"));
            self
        }
        pub fn branches<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::BranchesItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.branches = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for branches: {e}"));
            self
        }
        pub fn delay<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.delay = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for delay: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WaitId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn list<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.list = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list: {e}"));
            self
        }
        pub fn recipient<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.recipient = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for recipient: {e}"));
            self
        }
        pub fn rrules<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<Vec<::std::string::String>>>,
            T::Error: ::std::fmt::Display,
        {
            self.rrules = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rrules: {e}"));
            self
        }
        pub fn ruleset<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.ruleset = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ruleset: {e}"));
            self
        }
        pub fn timezone<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.timezone = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for timezone: {e}"));
            self
        }
        pub fn use_subscriber_timezone<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.use_subscriber_timezone = value.try_into().map_err(|e| {
                format!("error converting supplied value for use_subscriber_timezone: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<ActionsScheduleSetBranchV1Kwargs>
        for super::ActionsScheduleSetBranchV1Kwargs
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActionsScheduleSetBranchV1Kwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                account: value.account?,
                action: value.action?,
                branches: value.branches?,
                delay: value.delay?,
                id: value.id?,
                list: value.list?,
                recipient: value.recipient?,
                rrules: value.rrules?,
                ruleset: value.ruleset?,
                timezone: value.timezone?,
                use_subscriber_timezone: value.use_subscriber_timezone?,
            })
        }
    }
    impl ::std::convert::From<super::ActionsScheduleSetBranchV1Kwargs>
        for ActionsScheduleSetBranchV1Kwargs
    {
        fn from(value: super::ActionsScheduleSetBranchV1Kwargs) -> Self {
            Self {
                account: Ok(value.account),
                action: Ok(value.action),
                branches: Ok(value.branches),
                delay: Ok(value.delay),
                id: Ok(value.id),
                list: Ok(value.list),
                recipient: Ok(value.recipient),
                rrules: Ok(value.rrules),
                ruleset: Ok(value.ruleset),
                timezone: Ok(value.timezone),
                use_subscriber_timezone: Ok(value.use_subscriber_timezone),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActionsScheduleWaitV1 {
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<super::ActionsScheduleWaitV1Kwargs, ::std::string::String>,
    }
    impl ::std::default::Default for ActionsScheduleWaitV1 {
        fn default() -> Self {
            Self {
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
            }
        }
    }
    impl ActionsScheduleWaitV1 {
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActionsScheduleWaitV1Kwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ActionsScheduleWaitV1> for super::ActionsScheduleWaitV1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActionsScheduleWaitV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                function: value.function?,
                kwargs: value.kwargs?,
            })
        }
    }
    impl ::std::convert::From<super::ActionsScheduleWaitV1> for ActionsScheduleWaitV1 {
        fn from(value: super::ActionsScheduleWaitV1) -> Self {
            Self {
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActionsScheduleWaitV1Kwargs {
        account: ::std::result::Result<::std::string::String, ::std::string::String>,
        action: ::std::result::Result<super::ActionId, ::std::string::String>,
        delay: ::std::result::Result<::std::string::String, ::std::string::String>,
        id: ::std::result::Result<super::WaitId, ::std::string::String>,
        list: ::std::result::Result<::std::string::String, ::std::string::String>,
        recipient: ::std::result::Result<::std::string::String, ::std::string::String>,
        rrules: ::std::result::Result<Vec<::std::string::String>, ::std::string::String>,
        ruleset: ::std::result::Result<super::RulesetId, ::std::string::String>,
        timezone: ::std::result::Result<::std::string::String, ::std::string::String>,
        use_subscriber_timezone: ::std::result::Result<bool, ::std::string::String>,
    }
    impl ::std::default::Default for ActionsScheduleWaitV1Kwargs {
        fn default() -> Self {
            Self {
                account: Err("no value supplied for account".to_string()),
                action: Ok(super::defaults::actions_schedule_wait_v1_kwargs_action()),
                delay: Err("no value supplied for delay".to_string()),
                id: Err("no value supplied for id".to_string()),
                list: Err("no value supplied for list".to_string()),
                recipient: Err("no value supplied for recipient".to_string()),
                rrules: Err("no value supplied for rrules".to_string()),
                ruleset: Err("no value supplied for ruleset".to_string()),
                timezone: Err("no value supplied for timezone".to_string()),
                use_subscriber_timezone: Ok(Default::default()),
            }
        }
    }
    impl ActionsScheduleWaitV1Kwargs {
        pub fn account<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.account = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for account: {e}"));
            self
        }
        pub fn action<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActionId>,
            T::Error: ::std::fmt::Display,
        {
            self.action = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for action: {e}"));
            self
        }
        pub fn delay<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.delay = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for delay: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::WaitId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn list<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.list = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list: {e}"));
            self
        }
        pub fn recipient<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.recipient = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for recipient: {e}"));
            self
        }
        pub fn rrules<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.rrules = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rrules: {e}"));
            self
        }
        pub fn ruleset<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RulesetId>,
            T::Error: ::std::fmt::Display,
        {
            self.ruleset = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ruleset: {e}"));
            self
        }
        pub fn timezone<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.timezone = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for timezone: {e}"));
            self
        }
        pub fn use_subscriber_timezone<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.use_subscriber_timezone = value.try_into().map_err(|e| {
                format!("error converting supplied value for use_subscriber_timezone: {e}")
            });
            self
        }
    }
    impl ::std::convert::TryFrom<ActionsScheduleWaitV1Kwargs> for super::ActionsScheduleWaitV1Kwargs {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActionsScheduleWaitV1Kwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                account: value.account?,
                action: value.action?,
                delay: value.delay?,
                id: value.id?,
                list: value.list?,
                recipient: value.recipient?,
                rrules: value.rrules?,
                ruleset: value.ruleset?,
                timezone: value.timezone?,
                use_subscriber_timezone: value.use_subscriber_timezone?,
            })
        }
    }
    impl ::std::convert::From<super::ActionsScheduleWaitV1Kwargs> for ActionsScheduleWaitV1Kwargs {
        fn from(value: super::ActionsScheduleWaitV1Kwargs) -> Self {
            Self {
                account: Ok(value.account),
                action: Ok(value.action),
                delay: Ok(value.delay),
                id: Ok(value.id),
                list: Ok(value.list),
                recipient: Ok(value.recipient),
                rrules: Ok(value.rrules),
                ruleset: Ok(value.ruleset),
                timezone: Ok(value.timezone),
                use_subscriber_timezone: Ok(value.use_subscriber_timezone),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActionsStopStop {
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<super::ActionsStopStopKwargs, ::std::string::String>,
    }
    impl ::std::default::Default for ActionsStopStop {
        fn default() -> Self {
            Self {
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
            }
        }
    }
    impl ActionsStopStop {
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActionsStopStopKwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ActionsStopStop> for super::ActionsStopStop {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActionsStopStop,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                function: value.function?,
                kwargs: value.kwargs?,
            })
        }
    }
    impl ::std::convert::From<super::ActionsStopStop> for ActionsStopStop {
        fn from(value: super::ActionsStopStop) -> Self {
            Self {
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActionsStopStopKwargs {
        ruleset: ::std::result::Result<::std::string::String, ::std::string::String>,
        subscriber: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for ActionsStopStopKwargs {
        fn default() -> Self {
            Self {
                ruleset: Err("no value supplied for ruleset".to_string()),
                subscriber: Err("no value supplied for subscriber".to_string()),
            }
        }
    }
    impl ActionsStopStopKwargs {
        pub fn ruleset<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.ruleset = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ruleset: {e}"));
            self
        }
        pub fn subscriber<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.subscriber = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subscriber: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ActionsStopStopKwargs> for super::ActionsStopStopKwargs {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActionsStopStopKwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ruleset: value.ruleset?,
                subscriber: value.subscriber?,
            })
        }
    }
    impl ::std::convert::From<super::ActionsStopStopKwargs> for ActionsStopStopKwargs {
        fn from(value: super::ActionsStopStopKwargs) -> Self {
            Self {
                ruleset: Ok(value.ruleset),
                subscriber: Ok(value.subscriber),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActionsTagModifyTagsV1 {
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<super::ActionsTagModifyTagsV1Kwargs, ::std::string::String>,
    }
    impl ::std::default::Default for ActionsTagModifyTagsV1 {
        fn default() -> Self {
            Self {
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
            }
        }
    }
    impl ActionsTagModifyTagsV1 {
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActionsTagModifyTagsV1Kwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ActionsTagModifyTagsV1> for super::ActionsTagModifyTagsV1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActionsTagModifyTagsV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                function: value.function?,
                kwargs: value.kwargs?,
            })
        }
    }
    impl ::std::convert::From<super::ActionsTagModifyTagsV1> for ActionsTagModifyTagsV1 {
        fn from(value: super::ActionsTagModifyTagsV1) -> Self {
            Self {
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActionsTagModifyTagsV1Kwargs {
        account: ::std::result::Result<::std::string::String, ::std::string::String>,
        add_labels:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        list: ::std::result::Result<::std::string::String, ::std::string::String>,
        recipient: ::std::result::Result<super::Recipient, ::std::string::String>,
        remove_labels:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
    }
    impl ::std::default::Default for ActionsTagModifyTagsV1Kwargs {
        fn default() -> Self {
            Self {
                account: Err("no value supplied for account".to_string()),
                add_labels: Err("no value supplied for add_labels".to_string()),
                list: Err("no value supplied for list".to_string()),
                recipient: Err("no value supplied for recipient".to_string()),
                remove_labels: Err("no value supplied for remove_labels".to_string()),
            }
        }
    }
    impl ActionsTagModifyTagsV1Kwargs {
        pub fn account<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.account = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for account: {e}"));
            self
        }
        pub fn add_labels<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.add_labels = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for add_labels: {e}"));
            self
        }
        pub fn list<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.list = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list: {e}"));
            self
        }
        pub fn recipient<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Recipient>,
            T::Error: ::std::fmt::Display,
        {
            self.recipient = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for recipient: {e}"));
            self
        }
        pub fn remove_labels<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.remove_labels = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for remove_labels: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ActionsTagModifyTagsV1Kwargs> for super::ActionsTagModifyTagsV1Kwargs {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActionsTagModifyTagsV1Kwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                account: value.account?,
                add_labels: value.add_labels?,
                list: value.list?,
                recipient: value.recipient?,
                remove_labels: value.remove_labels?,
            })
        }
    }
    impl ::std::convert::From<super::ActionsTagModifyTagsV1Kwargs> for ActionsTagModifyTagsV1Kwargs {
        fn from(value: super::ActionsTagModifyTagsV1Kwargs) -> Self {
            Self {
                account: Ok(value.account),
                add_labels: Ok(value.add_labels),
                list: Ok(value.list),
                recipient: Ok(value.recipient),
                remove_labels: Ok(value.remove_labels),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActionsWebfeedCreateWebfeedMessageV1 {
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<
            super::ActionsWebfeedCreateWebfeedMessageV1Kwargs,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ActionsWebfeedCreateWebfeedMessageV1 {
        fn default() -> Self {
            Self {
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
            }
        }
    }
    impl ActionsWebfeedCreateWebfeedMessageV1 {
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ActionsWebfeedCreateWebfeedMessageV1Kwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ActionsWebfeedCreateWebfeedMessageV1>
        for super::ActionsWebfeedCreateWebfeedMessageV1
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActionsWebfeedCreateWebfeedMessageV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                function: value.function?,
                kwargs: value.kwargs?,
            })
        }
    }
    impl ::std::convert::From<super::ActionsWebfeedCreateWebfeedMessageV1>
        for ActionsWebfeedCreateWebfeedMessageV1
    {
        fn from(value: super::ActionsWebfeedCreateWebfeedMessageV1) -> Self {
            Self {
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ActionsWebfeedCreateWebfeedMessageV1Kwargs {
        account: ::std::result::Result<::std::string::String, ::std::string::String>,
        list: ::std::result::Result<::std::string::String, ::std::string::String>,
        segment: ::std::result::Result<::std::num::NonZeroU64, ::std::string::String>,
        send_broadcast: ::std::result::Result<bool, ::std::string::String>,
        template: ::std::result::Result<super::TemplateMessageId, ::std::string::String>,
        webfeed: ::std::result::Result<super::Webfeed, ::std::string::String>,
    }
    impl ::std::default::Default for ActionsWebfeedCreateWebfeedMessageV1Kwargs {
        fn default() -> Self {
            Self {
                account: Err("no value supplied for account".to_string()),
                list: Err("no value supplied for list".to_string()),
                segment: Err("no value supplied for segment".to_string()),
                send_broadcast: Err("no value supplied for send_broadcast".to_string()),
                template: Err("no value supplied for template".to_string()),
                webfeed: Err("no value supplied for webfeed".to_string()),
            }
        }
    }
    impl ActionsWebfeedCreateWebfeedMessageV1Kwargs {
        pub fn account<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.account = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for account: {e}"));
            self
        }
        pub fn list<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.list = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list: {e}"));
            self
        }
        pub fn segment<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::num::NonZeroU64>,
            T::Error: ::std::fmt::Display,
        {
            self.segment = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for segment: {e}"));
            self
        }
        pub fn send_broadcast<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.send_broadcast = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for send_broadcast: {e}"));
            self
        }
        pub fn template<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TemplateMessageId>,
            T::Error: ::std::fmt::Display,
        {
            self.template = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for template: {e}"));
            self
        }
        pub fn webfeed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Webfeed>,
            T::Error: ::std::fmt::Display,
        {
            self.webfeed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for webfeed: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ActionsWebfeedCreateWebfeedMessageV1Kwargs>
        for super::ActionsWebfeedCreateWebfeedMessageV1Kwargs
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ActionsWebfeedCreateWebfeedMessageV1Kwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                account: value.account?,
                list: value.list?,
                segment: value.segment?,
                send_broadcast: value.send_broadcast?,
                template: value.template?,
                webfeed: value.webfeed?,
            })
        }
    }
    impl ::std::convert::From<super::ActionsWebfeedCreateWebfeedMessageV1Kwargs>
        for ActionsWebfeedCreateWebfeedMessageV1Kwargs
    {
        fn from(value: super::ActionsWebfeedCreateWebfeedMessageV1Kwargs) -> Self {
            Self {
                account: Ok(value.account),
                list: Ok(value.list),
                segment: Ok(value.segment),
                send_broadcast: Ok(value.send_broadcast),
                template: Ok(value.template),
                webfeed: Ok(value.webfeed),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaAnalyticsAnyMessageClickUrlContainsV1 {
        expect: ::std::result::Result<bool, ::std::string::String>,
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<
            super::CriteriaAnalyticsAnyMessageClickUrlContainsV1Kwargs,
            ::std::string::String,
        >,
        memoize_id:
            ::std::result::Result<::std::option::Option<super::MemoizeId>, ::std::string::String>,
        operator: ::std::result::Result<super::Operators, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaAnalyticsAnyMessageClickUrlContainsV1 {
        fn default() -> Self {
            Self {
                expect: Err("no value supplied for expect".to_string()),
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
                memoize_id: Ok(Default::default()),
                operator: Err("no value supplied for operator".to_string()),
            }
        }
    }
    impl CriteriaAnalyticsAnyMessageClickUrlContainsV1 {
        pub fn expect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.expect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expect: {e}"));
            self
        }
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CriteriaAnalyticsAnyMessageClickUrlContainsV1Kwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
        pub fn memoize_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MemoizeId>>,
            T::Error: ::std::fmt::Display,
        {
            self.memoize_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for memoize_id: {e}"));
            self
        }
        pub fn operator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Operators>,
            T::Error: ::std::fmt::Display,
        {
            self.operator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for operator: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaAnalyticsAnyMessageClickUrlContainsV1>
        for super::CriteriaAnalyticsAnyMessageClickUrlContainsV1
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaAnalyticsAnyMessageClickUrlContainsV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expect: value.expect?,
                function: value.function?,
                kwargs: value.kwargs?,
                memoize_id: value.memoize_id?,
                operator: value.operator?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaAnalyticsAnyMessageClickUrlContainsV1>
        for CriteriaAnalyticsAnyMessageClickUrlContainsV1
    {
        fn from(value: super::CriteriaAnalyticsAnyMessageClickUrlContainsV1) -> Self {
            Self {
                expect: Ok(value.expect),
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
                memoize_id: Ok(value.memoize_id),
                operator: Ok(value.operator),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaAnalyticsAnyMessageClickUrlContainsV1Kwargs {
        account: ::std::result::Result<::std::string::String, ::std::string::String>,
        fragments:
            ::std::result::Result<::std::vec::Vec<super::UrlFragmentsItem>, ::std::string::String>,
        messages: ::std::result::Result<::std::string::String, ::std::string::String>,
        subscriber: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaAnalyticsAnyMessageClickUrlContainsV1Kwargs {
        fn default() -> Self {
            Self {
                account: Err("no value supplied for account".to_string()),
                fragments: Err("no value supplied for fragments".to_string()),
                messages: Err("no value supplied for messages".to_string()),
                subscriber: Err("no value supplied for subscriber".to_string()),
            }
        }
    }
    impl CriteriaAnalyticsAnyMessageClickUrlContainsV1Kwargs {
        pub fn account<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.account = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for account: {e}"));
            self
        }
        pub fn fragments<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::UrlFragmentsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.fragments = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for fragments: {e}"));
            self
        }
        pub fn messages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.messages = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for messages: {e}"));
            self
        }
        pub fn subscriber<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.subscriber = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subscriber: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaAnalyticsAnyMessageClickUrlContainsV1Kwargs>
        for super::CriteriaAnalyticsAnyMessageClickUrlContainsV1Kwargs
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaAnalyticsAnyMessageClickUrlContainsV1Kwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                account: value.account?,
                fragments: value.fragments?,
                messages: value.messages?,
                subscriber: value.subscriber?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaAnalyticsAnyMessageClickUrlContainsV1Kwargs>
        for CriteriaAnalyticsAnyMessageClickUrlContainsV1Kwargs
    {
        fn from(value: super::CriteriaAnalyticsAnyMessageClickUrlContainsV1Kwargs) -> Self {
            Self {
                account: Ok(value.account),
                fragments: Ok(value.fragments),
                messages: Ok(value.messages),
                subscriber: Ok(value.subscriber),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaAnalyticsAnyMessageClickUrlsV1 {
        expect: ::std::result::Result<bool, ::std::string::String>,
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<
            super::CriteriaAnalyticsAnyMessageClickUrlsV1Kwargs,
            ::std::string::String,
        >,
        memoize_id:
            ::std::result::Result<::std::option::Option<super::MemoizeId>, ::std::string::String>,
        operator: ::std::result::Result<super::Operators, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaAnalyticsAnyMessageClickUrlsV1 {
        fn default() -> Self {
            Self {
                expect: Err("no value supplied for expect".to_string()),
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
                memoize_id: Ok(Default::default()),
                operator: Err("no value supplied for operator".to_string()),
            }
        }
    }
    impl CriteriaAnalyticsAnyMessageClickUrlsV1 {
        pub fn expect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.expect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expect: {e}"));
            self
        }
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CriteriaAnalyticsAnyMessageClickUrlsV1Kwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
        pub fn memoize_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MemoizeId>>,
            T::Error: ::std::fmt::Display,
        {
            self.memoize_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for memoize_id: {e}"));
            self
        }
        pub fn operator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Operators>,
            T::Error: ::std::fmt::Display,
        {
            self.operator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for operator: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaAnalyticsAnyMessageClickUrlsV1>
        for super::CriteriaAnalyticsAnyMessageClickUrlsV1
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaAnalyticsAnyMessageClickUrlsV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expect: value.expect?,
                function: value.function?,
                kwargs: value.kwargs?,
                memoize_id: value.memoize_id?,
                operator: value.operator?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaAnalyticsAnyMessageClickUrlsV1>
        for CriteriaAnalyticsAnyMessageClickUrlsV1
    {
        fn from(value: super::CriteriaAnalyticsAnyMessageClickUrlsV1) -> Self {
            Self {
                expect: Ok(value.expect),
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
                memoize_id: Ok(value.memoize_id),
                operator: Ok(value.operator),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaAnalyticsAnyMessageClickUrlsV1Kwargs {
        account: ::std::result::Result<::std::string::String, ::std::string::String>,
        click_urls:
            ::std::result::Result<::std::vec::Vec<super::ClickUrLsItem>, ::std::string::String>,
        messages: ::std::result::Result<::std::string::String, ::std::string::String>,
        subscriber: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaAnalyticsAnyMessageClickUrlsV1Kwargs {
        fn default() -> Self {
            Self {
                account: Err("no value supplied for account".to_string()),
                click_urls: Ok(Default::default()),
                messages: Err("no value supplied for messages".to_string()),
                subscriber: Err("no value supplied for subscriber".to_string()),
            }
        }
    }
    impl CriteriaAnalyticsAnyMessageClickUrlsV1Kwargs {
        pub fn account<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.account = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for account: {e}"));
            self
        }
        pub fn click_urls<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ClickUrLsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.click_urls = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for click_urls: {e}"));
            self
        }
        pub fn messages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.messages = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for messages: {e}"));
            self
        }
        pub fn subscriber<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.subscriber = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subscriber: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaAnalyticsAnyMessageClickUrlsV1Kwargs>
        for super::CriteriaAnalyticsAnyMessageClickUrlsV1Kwargs
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaAnalyticsAnyMessageClickUrlsV1Kwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                account: value.account?,
                click_urls: value.click_urls?,
                messages: value.messages?,
                subscriber: value.subscriber?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaAnalyticsAnyMessageClickUrlsV1Kwargs>
        for CriteriaAnalyticsAnyMessageClickUrlsV1Kwargs
    {
        fn from(value: super::CriteriaAnalyticsAnyMessageClickUrlsV1Kwargs) -> Self {
            Self {
                account: Ok(value.account),
                click_urls: Ok(value.click_urls),
                messages: Ok(value.messages),
                subscriber: Ok(value.subscriber),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaAnalyticsAnyMessageClicksV1 {
        expect: ::std::result::Result<bool, ::std::string::String>,
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<
            super::CriteriaAnalyticsAnyMessageClicksV1Kwargs,
            ::std::string::String,
        >,
        memoize_id:
            ::std::result::Result<::std::option::Option<super::MemoizeId>, ::std::string::String>,
        operator: ::std::result::Result<super::Operators, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaAnalyticsAnyMessageClicksV1 {
        fn default() -> Self {
            Self {
                expect: Err("no value supplied for expect".to_string()),
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
                memoize_id: Ok(Default::default()),
                operator: Err("no value supplied for operator".to_string()),
            }
        }
    }
    impl CriteriaAnalyticsAnyMessageClicksV1 {
        pub fn expect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.expect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expect: {e}"));
            self
        }
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CriteriaAnalyticsAnyMessageClicksV1Kwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
        pub fn memoize_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MemoizeId>>,
            T::Error: ::std::fmt::Display,
        {
            self.memoize_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for memoize_id: {e}"));
            self
        }
        pub fn operator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Operators>,
            T::Error: ::std::fmt::Display,
        {
            self.operator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for operator: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaAnalyticsAnyMessageClicksV1>
        for super::CriteriaAnalyticsAnyMessageClicksV1
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaAnalyticsAnyMessageClicksV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expect: value.expect?,
                function: value.function?,
                kwargs: value.kwargs?,
                memoize_id: value.memoize_id?,
                operator: value.operator?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaAnalyticsAnyMessageClicksV1>
        for CriteriaAnalyticsAnyMessageClicksV1
    {
        fn from(value: super::CriteriaAnalyticsAnyMessageClicksV1) -> Self {
            Self {
                expect: Ok(value.expect),
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
                memoize_id: Ok(value.memoize_id),
                operator: Ok(value.operator),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaAnalyticsAnyMessageClicksV1Kwargs {
        account: ::std::result::Result<::std::string::String, ::std::string::String>,
        messages: ::std::result::Result<::std::string::String, ::std::string::String>,
        subscriber: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaAnalyticsAnyMessageClicksV1Kwargs {
        fn default() -> Self {
            Self {
                account: Err("no value supplied for account".to_string()),
                messages: Err("no value supplied for messages".to_string()),
                subscriber: Err("no value supplied for subscriber".to_string()),
            }
        }
    }
    impl CriteriaAnalyticsAnyMessageClicksV1Kwargs {
        pub fn account<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.account = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for account: {e}"));
            self
        }
        pub fn messages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.messages = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for messages: {e}"));
            self
        }
        pub fn subscriber<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.subscriber = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subscriber: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaAnalyticsAnyMessageClicksV1Kwargs>
        for super::CriteriaAnalyticsAnyMessageClicksV1Kwargs
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaAnalyticsAnyMessageClicksV1Kwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                account: value.account?,
                messages: value.messages?,
                subscriber: value.subscriber?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaAnalyticsAnyMessageClicksV1Kwargs>
        for CriteriaAnalyticsAnyMessageClicksV1Kwargs
    {
        fn from(value: super::CriteriaAnalyticsAnyMessageClicksV1Kwargs) -> Self {
            Self {
                account: Ok(value.account),
                messages: Ok(value.messages),
                subscriber: Ok(value.subscriber),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaAnalyticsAnyMessageOpensV1 {
        expect: ::std::result::Result<bool, ::std::string::String>,
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<
            super::CriteriaAnalyticsAnyMessageOpensV1Kwargs,
            ::std::string::String,
        >,
        memoize_id:
            ::std::result::Result<::std::option::Option<super::MemoizeId>, ::std::string::String>,
        operator: ::std::result::Result<super::Operators, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaAnalyticsAnyMessageOpensV1 {
        fn default() -> Self {
            Self {
                expect: Err("no value supplied for expect".to_string()),
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
                memoize_id: Ok(Default::default()),
                operator: Err("no value supplied for operator".to_string()),
            }
        }
    }
    impl CriteriaAnalyticsAnyMessageOpensV1 {
        pub fn expect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.expect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expect: {e}"));
            self
        }
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CriteriaAnalyticsAnyMessageOpensV1Kwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
        pub fn memoize_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MemoizeId>>,
            T::Error: ::std::fmt::Display,
        {
            self.memoize_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for memoize_id: {e}"));
            self
        }
        pub fn operator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Operators>,
            T::Error: ::std::fmt::Display,
        {
            self.operator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for operator: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaAnalyticsAnyMessageOpensV1>
        for super::CriteriaAnalyticsAnyMessageOpensV1
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaAnalyticsAnyMessageOpensV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expect: value.expect?,
                function: value.function?,
                kwargs: value.kwargs?,
                memoize_id: value.memoize_id?,
                operator: value.operator?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaAnalyticsAnyMessageOpensV1>
        for CriteriaAnalyticsAnyMessageOpensV1
    {
        fn from(value: super::CriteriaAnalyticsAnyMessageOpensV1) -> Self {
            Self {
                expect: Ok(value.expect),
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
                memoize_id: Ok(value.memoize_id),
                operator: Ok(value.operator),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaAnalyticsAnyMessageOpensV1Kwargs {
        account: ::std::result::Result<::std::string::String, ::std::string::String>,
        messages: ::std::result::Result<::std::string::String, ::std::string::String>,
        subscriber: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaAnalyticsAnyMessageOpensV1Kwargs {
        fn default() -> Self {
            Self {
                account: Err("no value supplied for account".to_string()),
                messages: Err("no value supplied for messages".to_string()),
                subscriber: Err("no value supplied for subscriber".to_string()),
            }
        }
    }
    impl CriteriaAnalyticsAnyMessageOpensV1Kwargs {
        pub fn account<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.account = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for account: {e}"));
            self
        }
        pub fn messages<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.messages = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for messages: {e}"));
            self
        }
        pub fn subscriber<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.subscriber = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subscriber: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaAnalyticsAnyMessageOpensV1Kwargs>
        for super::CriteriaAnalyticsAnyMessageOpensV1Kwargs
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaAnalyticsAnyMessageOpensV1Kwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                account: value.account?,
                messages: value.messages?,
                subscriber: value.subscriber?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaAnalyticsAnyMessageOpensV1Kwargs>
        for CriteriaAnalyticsAnyMessageOpensV1Kwargs
    {
        fn from(value: super::CriteriaAnalyticsAnyMessageOpensV1Kwargs) -> Self {
            Self {
                account: Ok(value.account),
                messages: Ok(value.messages),
                subscriber: Ok(value.subscriber),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaCampaignSerialV1 {
        expect: ::std::result::Result<bool, ::std::string::String>,
        function: ::std::result::Result<super::FunctionPath, ::std::string::String>,
        kwargs: ::std::result::Result<super::CriteriaCampaignSerialV1Kwargs, ::std::string::String>,
        operator: ::std::result::Result<super::Operators, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaCampaignSerialV1 {
        fn default() -> Self {
            Self {
                expect: Err("no value supplied for expect".to_string()),
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
                operator: Err("no value supplied for operator".to_string()),
            }
        }
    }
    impl CriteriaCampaignSerialV1 {
        pub fn expect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.expect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expect: {e}"));
            self
        }
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FunctionPath>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CriteriaCampaignSerialV1Kwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
        pub fn operator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Operators>,
            T::Error: ::std::fmt::Display,
        {
            self.operator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for operator: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaCampaignSerialV1> for super::CriteriaCampaignSerialV1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaCampaignSerialV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expect: value.expect?,
                function: value.function?,
                kwargs: value.kwargs?,
                operator: value.operator?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaCampaignSerialV1> for CriteriaCampaignSerialV1 {
        fn from(value: super::CriteriaCampaignSerialV1) -> Self {
            Self {
                expect: Ok(value.expect),
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
                operator: Ok(value.operator),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaCampaignSerialV1Kwargs {
        account: ::std::result::Result<::std::string::String, ::std::string::String>,
        campaign: ::std::result::Result<super::CampaignId, ::std::string::String>,
        list: ::std::result::Result<::std::string::String, ::std::string::String>,
        serial: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaCampaignSerialV1Kwargs {
        fn default() -> Self {
            Self {
                account: Err("no value supplied for account".to_string()),
                campaign: Err("no value supplied for campaign".to_string()),
                list: Err("no value supplied for list".to_string()),
                serial: Err("no value supplied for serial".to_string()),
            }
        }
    }
    impl CriteriaCampaignSerialV1Kwargs {
        pub fn account<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.account = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for account: {e}"));
            self
        }
        pub fn campaign<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CampaignId>,
            T::Error: ::std::fmt::Display,
        {
            self.campaign = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for campaign: {e}"));
            self
        }
        pub fn list<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.list = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list: {e}"));
            self
        }
        pub fn serial<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.serial = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for serial: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaCampaignSerialV1Kwargs>
        for super::CriteriaCampaignSerialV1Kwargs
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaCampaignSerialV1Kwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                account: value.account?,
                campaign: value.campaign?,
                list: value.list?,
                serial: value.serial?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaCampaignSerialV1Kwargs>
        for CriteriaCampaignSerialV1Kwargs
    {
        fn from(value: super::CriteriaCampaignSerialV1Kwargs) -> Self {
            Self {
                account: Ok(value.account),
                campaign: Ok(value.campaign),
                list: Ok(value.list),
                serial: Ok(value.serial),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaEventValue {
        expect: ::std::result::Result<super::ReturnExepctation, ::std::string::String>,
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<super::CriteriaEventValueKwargs, ::std::string::String>,
        operator: ::std::result::Result<super::Operators, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaEventValue {
        fn default() -> Self {
            Self {
                expect: Err("no value supplied for expect".to_string()),
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
                operator: Err("no value supplied for operator".to_string()),
            }
        }
    }
    impl CriteriaEventValue {
        pub fn expect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ReturnExepctation>,
            T::Error: ::std::fmt::Display,
        {
            self.expect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expect: {e}"));
            self
        }
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CriteriaEventValueKwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
        pub fn operator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Operators>,
            T::Error: ::std::fmt::Display,
        {
            self.operator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for operator: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaEventValue> for super::CriteriaEventValue {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaEventValue,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expect: value.expect?,
                function: value.function?,
                kwargs: value.kwargs?,
                operator: value.operator?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaEventValue> for CriteriaEventValue {
        fn from(value: super::CriteriaEventValue) -> Self {
            Self {
                expect: Ok(value.expect),
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
                operator: Ok(value.operator),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaEventValueIn {
        expect: ::std::result::Result<bool, ::std::string::String>,
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<super::CriteriaEventValueInKwargs, ::std::string::String>,
        operator: ::std::result::Result<super::Operators, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaEventValueIn {
        fn default() -> Self {
            Self {
                expect: Err("no value supplied for expect".to_string()),
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
                operator: Err("no value supplied for operator".to_string()),
            }
        }
    }
    impl CriteriaEventValueIn {
        pub fn expect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.expect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expect: {e}"));
            self
        }
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CriteriaEventValueInKwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
        pub fn operator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Operators>,
            T::Error: ::std::fmt::Display,
        {
            self.operator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for operator: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaEventValueIn> for super::CriteriaEventValueIn {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaEventValueIn,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expect: value.expect?,
                function: value.function?,
                kwargs: value.kwargs?,
                operator: value.operator?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaEventValueIn> for CriteriaEventValueIn {
        fn from(value: super::CriteriaEventValueIn) -> Self {
            Self {
                expect: Ok(value.expect),
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
                operator: Ok(value.operator),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaEventValueInKwargs {
        key: ::std::result::Result<::std::string::String, ::std::string::String>,
        values: ::std::result::Result<
            ::std::option::Option<Vec<super::ValuesItem>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CriteriaEventValueInKwargs {
        fn default() -> Self {
            Self {
                key: Err("no value supplied for key".to_string()),
                values: Ok(Default::default()),
            }
        }
    }
    impl CriteriaEventValueInKwargs {
        pub fn key<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.key = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for key: {e}"));
            self
        }
        pub fn values<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<Vec<super::ValuesItem>>>,
            T::Error: ::std::fmt::Display,
        {
            self.values = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for values: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaEventValueInKwargs> for super::CriteriaEventValueInKwargs {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaEventValueInKwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                key: value.key?,
                values: value.values?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaEventValueInKwargs> for CriteriaEventValueInKwargs {
        fn from(value: super::CriteriaEventValueInKwargs) -> Self {
            Self {
                key: Ok(value.key),
                values: Ok(value.values),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaEventValueInUrls {
        expect: ::std::result::Result<bool, ::std::string::String>,
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<super::CriteriaEventValueInUrlsKwargs, ::std::string::String>,
        operator: ::std::result::Result<super::Operators, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaEventValueInUrls {
        fn default() -> Self {
            Self {
                expect: Err("no value supplied for expect".to_string()),
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
                operator: Err("no value supplied for operator".to_string()),
            }
        }
    }
    impl CriteriaEventValueInUrls {
        pub fn expect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.expect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expect: {e}"));
            self
        }
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CriteriaEventValueInUrlsKwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
        pub fn operator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Operators>,
            T::Error: ::std::fmt::Display,
        {
            self.operator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for operator: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaEventValueInUrls> for super::CriteriaEventValueInUrls {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaEventValueInUrls,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expect: value.expect?,
                function: value.function?,
                kwargs: value.kwargs?,
                operator: value.operator?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaEventValueInUrls> for CriteriaEventValueInUrls {
        fn from(value: super::CriteriaEventValueInUrls) -> Self {
            Self {
                expect: Ok(value.expect),
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
                operator: Ok(value.operator),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaEventValueInUrlsKwargs {
        key: ::std::result::Result<::std::string::String, ::std::string::String>,
        urls: ::std::result::Result<
            ::std::option::Option<Vec<::std::string::String>>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for CriteriaEventValueInUrlsKwargs {
        fn default() -> Self {
            Self {
                key: Err("no value supplied for key".to_string()),
                urls: Ok(Default::default()),
            }
        }
    }
    impl CriteriaEventValueInUrlsKwargs {
        pub fn key<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.key = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for key: {e}"));
            self
        }
        pub fn urls<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<Vec<::std::string::String>>>,
            T::Error: ::std::fmt::Display,
        {
            self.urls = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for urls: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaEventValueInUrlsKwargs>
        for super::CriteriaEventValueInUrlsKwargs
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaEventValueInUrlsKwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                key: value.key?,
                urls: value.urls?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaEventValueInUrlsKwargs>
        for CriteriaEventValueInUrlsKwargs
    {
        fn from(value: super::CriteriaEventValueInUrlsKwargs) -> Self {
            Self {
                key: Ok(value.key),
                urls: Ok(value.urls),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaEventValueKwargs {
        key: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaEventValueKwargs {
        fn default() -> Self {
            Self {
                key: Err("no value supplied for key".to_string()),
            }
        }
    }
    impl CriteriaEventValueKwargs {
        pub fn key<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.key = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for key: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaEventValueKwargs> for super::CriteriaEventValueKwargs {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaEventValueKwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self { key: value.key? })
        }
    }
    impl ::std::convert::From<super::CriteriaEventValueKwargs> for CriteriaEventValueKwargs {
        fn from(value: super::CriteriaEventValueKwargs) -> Self {
            Self { key: Ok(value.key) }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaFollowupIsDisabledV1 {
        expect: ::std::result::Result<bool, ::std::string::String>,
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs:
            ::std::result::Result<super::CriteriaFollowupIsDisabledV1Kwargs, ::std::string::String>,
        operator: ::std::result::Result<super::Operators, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaFollowupIsDisabledV1 {
        fn default() -> Self {
            Self {
                expect: Err("no value supplied for expect".to_string()),
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
                operator: Err("no value supplied for operator".to_string()),
            }
        }
    }
    impl CriteriaFollowupIsDisabledV1 {
        pub fn expect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.expect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expect: {e}"));
            self
        }
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CriteriaFollowupIsDisabledV1Kwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
        pub fn operator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Operators>,
            T::Error: ::std::fmt::Display,
        {
            self.operator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for operator: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaFollowupIsDisabledV1> for super::CriteriaFollowupIsDisabledV1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaFollowupIsDisabledV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expect: value.expect?,
                function: value.function?,
                kwargs: value.kwargs?,
                operator: value.operator?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaFollowupIsDisabledV1> for CriteriaFollowupIsDisabledV1 {
        fn from(value: super::CriteriaFollowupIsDisabledV1) -> Self {
            Self {
                expect: Ok(value.expect),
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
                operator: Ok(value.operator),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaFollowupIsDisabledV1Kwargs {
        recipient: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaFollowupIsDisabledV1Kwargs {
        fn default() -> Self {
            Self {
                recipient: Err("no value supplied for recipient".to_string()),
            }
        }
    }
    impl CriteriaFollowupIsDisabledV1Kwargs {
        pub fn recipient<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.recipient = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for recipient: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaFollowupIsDisabledV1Kwargs>
        for super::CriteriaFollowupIsDisabledV1Kwargs
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaFollowupIsDisabledV1Kwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                recipient: value.recipient?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaFollowupIsDisabledV1Kwargs>
        for CriteriaFollowupIsDisabledV1Kwargs
    {
        fn from(value: super::CriteriaFollowupIsDisabledV1Kwargs) -> Self {
            Self {
                recipient: Ok(value.recipient),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaGeoipCountryV1 {
        expect: ::std::result::Result<super::Countries, ::std::string::String>,
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<super::CriteriaGeoipCountryV1Kwargs, ::std::string::String>,
        operator: ::std::result::Result<super::Operators, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaGeoipCountryV1 {
        fn default() -> Self {
            Self {
                expect: Err("no value supplied for expect".to_string()),
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
                operator: Err("no value supplied for operator".to_string()),
            }
        }
    }
    impl CriteriaGeoipCountryV1 {
        pub fn expect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Countries>,
            T::Error: ::std::fmt::Display,
        {
            self.expect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expect: {e}"));
            self
        }
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CriteriaGeoipCountryV1Kwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
        pub fn operator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Operators>,
            T::Error: ::std::fmt::Display,
        {
            self.operator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for operator: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaGeoipCountryV1> for super::CriteriaGeoipCountryV1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaGeoipCountryV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expect: value.expect?,
                function: value.function?,
                kwargs: value.kwargs?,
                operator: value.operator?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaGeoipCountryV1> for CriteriaGeoipCountryV1 {
        fn from(value: super::CriteriaGeoipCountryV1) -> Self {
            Self {
                expect: Ok(value.expect),
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
                operator: Ok(value.operator),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaGeoipCountryV1Kwargs {
        remote_ip: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaGeoipCountryV1Kwargs {
        fn default() -> Self {
            Self {
                remote_ip: Err("no value supplied for remote_ip".to_string()),
            }
        }
    }
    impl CriteriaGeoipCountryV1Kwargs {
        pub fn remote_ip<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.remote_ip = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for remote_ip: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaGeoipCountryV1Kwargs> for super::CriteriaGeoipCountryV1Kwargs {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaGeoipCountryV1Kwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                remote_ip: value.remote_ip?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaGeoipCountryV1Kwargs> for CriteriaGeoipCountryV1Kwargs {
        fn from(value: super::CriteriaGeoipCountryV1Kwargs) -> Self {
            Self {
                remote_ip: Ok(value.remote_ip),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaMathRandbelowV1 {
        expect: ::std::result::Result<i64, ::std::string::String>,
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<super::CriteriaMathRandbelowV1Kwargs, ::std::string::String>,
        memoize_id:
            ::std::result::Result<::std::option::Option<super::MemoizeId>, ::std::string::String>,
        operator: ::std::result::Result<super::Operators, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaMathRandbelowV1 {
        fn default() -> Self {
            Self {
                expect: Err("no value supplied for expect".to_string()),
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
                memoize_id: Ok(Default::default()),
                operator: Err("no value supplied for operator".to_string()),
            }
        }
    }
    impl CriteriaMathRandbelowV1 {
        pub fn expect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<i64>,
            T::Error: ::std::fmt::Display,
        {
            self.expect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expect: {e}"));
            self
        }
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CriteriaMathRandbelowV1Kwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
        pub fn memoize_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MemoizeId>>,
            T::Error: ::std::fmt::Display,
        {
            self.memoize_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for memoize_id: {e}"));
            self
        }
        pub fn operator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Operators>,
            T::Error: ::std::fmt::Display,
        {
            self.operator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for operator: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaMathRandbelowV1> for super::CriteriaMathRandbelowV1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaMathRandbelowV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expect: value.expect?,
                function: value.function?,
                kwargs: value.kwargs?,
                memoize_id: value.memoize_id?,
                operator: value.operator?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaMathRandbelowV1> for CriteriaMathRandbelowV1 {
        fn from(value: super::CriteriaMathRandbelowV1) -> Self {
            Self {
                expect: Ok(value.expect),
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
                memoize_id: Ok(value.memoize_id),
                operator: Ok(value.operator),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaMathRandbelowV1Kwargs {
        upperbound: ::std::result::Result<u64, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaMathRandbelowV1Kwargs {
        fn default() -> Self {
            Self {
                upperbound: Err("no value supplied for upperbound".to_string()),
            }
        }
    }
    impl CriteriaMathRandbelowV1Kwargs {
        pub fn upperbound<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<u64>,
            T::Error: ::std::fmt::Display,
        {
            self.upperbound = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for upperbound: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaMathRandbelowV1Kwargs>
        for super::CriteriaMathRandbelowV1Kwargs
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaMathRandbelowV1Kwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                upperbound: value.upperbound?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaMathRandbelowV1Kwargs> for CriteriaMathRandbelowV1Kwargs {
        fn from(value: super::CriteriaMathRandbelowV1Kwargs) -> Self {
            Self {
                upperbound: Ok(value.upperbound),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaMessagemapMessageapiIdV1 {
        expect: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<
            super::CriteriaMessagemapMessageapiIdV1Kwargs,
            ::std::string::String,
        >,
        operator: ::std::result::Result<super::Operators, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaMessagemapMessageapiIdV1 {
        fn default() -> Self {
            Self {
                expect: Err("no value supplied for expect".to_string()),
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
                operator: Err("no value supplied for operator".to_string()),
            }
        }
    }
    impl CriteriaMessagemapMessageapiIdV1 {
        pub fn expect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.expect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expect: {e}"));
            self
        }
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CriteriaMessagemapMessageapiIdV1Kwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
        pub fn operator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Operators>,
            T::Error: ::std::fmt::Display,
        {
            self.operator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for operator: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaMessagemapMessageapiIdV1>
        for super::CriteriaMessagemapMessageapiIdV1
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaMessagemapMessageapiIdV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expect: value.expect?,
                function: value.function?,
                kwargs: value.kwargs?,
                operator: value.operator?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaMessagemapMessageapiIdV1>
        for CriteriaMessagemapMessageapiIdV1
    {
        fn from(value: super::CriteriaMessagemapMessageapiIdV1) -> Self {
            Self {
                expect: Ok(value.expect),
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
                operator: Ok(value.operator),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaMessagemapMessageapiIdV1Kwargs {
        message: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaMessagemapMessageapiIdV1Kwargs {
        fn default() -> Self {
            Self {
                message: Err("no value supplied for message".to_string()),
            }
        }
    }
    impl CriteriaMessagemapMessageapiIdV1Kwargs {
        pub fn message<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.message = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for message: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaMessagemapMessageapiIdV1Kwargs>
        for super::CriteriaMessagemapMessageapiIdV1Kwargs
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaMessagemapMessageapiIdV1Kwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                message: value.message?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaMessagemapMessageapiIdV1Kwargs>
        for CriteriaMessagemapMessageapiIdV1Kwargs
    {
        fn from(value: super::CriteriaMessagemapMessageapiIdV1Kwargs) -> Self {
            Self {
                message: Ok(value.message),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaRecipientAdtrackingV1 {
        expect: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        function: ::std::result::Result<super::FunctionPath, ::std::string::String>,
        kwargs: ::std::result::Result<
            super::CriteriaRecipientAdtrackingV1Kwargs,
            ::std::string::String,
        >,
        operator: ::std::result::Result<super::Operators, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaRecipientAdtrackingV1 {
        fn default() -> Self {
            Self {
                expect: Err("no value supplied for expect".to_string()),
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
                operator: Err("no value supplied for operator".to_string()),
            }
        }
    }
    impl CriteriaRecipientAdtrackingV1 {
        pub fn expect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.expect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expect: {e}"));
            self
        }
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::FunctionPath>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CriteriaRecipientAdtrackingV1Kwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
        pub fn operator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Operators>,
            T::Error: ::std::fmt::Display,
        {
            self.operator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for operator: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaRecipientAdtrackingV1>
        for super::CriteriaRecipientAdtrackingV1
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaRecipientAdtrackingV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expect: value.expect?,
                function: value.function?,
                kwargs: value.kwargs?,
                operator: value.operator?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaRecipientAdtrackingV1> for CriteriaRecipientAdtrackingV1 {
        fn from(value: super::CriteriaRecipientAdtrackingV1) -> Self {
            Self {
                expect: Ok(value.expect),
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
                operator: Ok(value.operator),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaRecipientAdtrackingV1Kwargs {
        account: ::std::result::Result<::std::string::String, ::std::string::String>,
        list: ::std::result::Result<::std::string::String, ::std::string::String>,
        recipient: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaRecipientAdtrackingV1Kwargs {
        fn default() -> Self {
            Self {
                account: Err("no value supplied for account".to_string()),
                list: Err("no value supplied for list".to_string()),
                recipient: Err("no value supplied for recipient".to_string()),
            }
        }
    }
    impl CriteriaRecipientAdtrackingV1Kwargs {
        pub fn account<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.account = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for account: {e}"));
            self
        }
        pub fn list<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.list = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list: {e}"));
            self
        }
        pub fn recipient<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.recipient = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for recipient: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaRecipientAdtrackingV1Kwargs>
        for super::CriteriaRecipientAdtrackingV1Kwargs
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaRecipientAdtrackingV1Kwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                account: value.account?,
                list: value.list?,
                recipient: value.recipient?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaRecipientAdtrackingV1Kwargs>
        for CriteriaRecipientAdtrackingV1Kwargs
    {
        fn from(value: super::CriteriaRecipientAdtrackingV1Kwargs) -> Self {
            Self {
                account: Ok(value.account),
                list: Ok(value.list),
                recipient: Ok(value.recipient),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaRecipientCustomFieldV1 {
        expect: ::std::result::Result<super::ReturnExepctation, ::std::string::String>,
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<
            super::CriteriaRecipientCustomFieldV1Kwargs,
            ::std::string::String,
        >,
        operator: ::std::result::Result<super::Operators, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaRecipientCustomFieldV1 {
        fn default() -> Self {
            Self {
                expect: Err("no value supplied for expect".to_string()),
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
                operator: Err("no value supplied for operator".to_string()),
            }
        }
    }
    impl CriteriaRecipientCustomFieldV1 {
        pub fn expect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ReturnExepctation>,
            T::Error: ::std::fmt::Display,
        {
            self.expect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expect: {e}"));
            self
        }
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CriteriaRecipientCustomFieldV1Kwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
        pub fn operator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Operators>,
            T::Error: ::std::fmt::Display,
        {
            self.operator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for operator: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaRecipientCustomFieldV1>
        for super::CriteriaRecipientCustomFieldV1
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaRecipientCustomFieldV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expect: value.expect?,
                function: value.function?,
                kwargs: value.kwargs?,
                operator: value.operator?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaRecipientCustomFieldV1>
        for CriteriaRecipientCustomFieldV1
    {
        fn from(value: super::CriteriaRecipientCustomFieldV1) -> Self {
            Self {
                expect: Ok(value.expect),
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
                operator: Ok(value.operator),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaRecipientCustomFieldV1Kwargs {
        account: ::std::result::Result<::std::string::String, ::std::string::String>,
        field: ::std::result::Result<::std::string::String, ::std::string::String>,
        list: ::std::result::Result<::std::string::String, ::std::string::String>,
        recipient: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaRecipientCustomFieldV1Kwargs {
        fn default() -> Self {
            Self {
                account: Err("no value supplied for account".to_string()),
                field: Err("no value supplied for field".to_string()),
                list: Err("no value supplied for list".to_string()),
                recipient: Err("no value supplied for recipient".to_string()),
            }
        }
    }
    impl CriteriaRecipientCustomFieldV1Kwargs {
        pub fn account<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.account = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for account: {e}"));
            self
        }
        pub fn field<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.field = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for field: {e}"));
            self
        }
        pub fn list<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.list = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list: {e}"));
            self
        }
        pub fn recipient<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.recipient = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for recipient: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaRecipientCustomFieldV1Kwargs>
        for super::CriteriaRecipientCustomFieldV1Kwargs
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaRecipientCustomFieldV1Kwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                account: value.account?,
                field: value.field?,
                list: value.list?,
                recipient: value.recipient?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaRecipientCustomFieldV1Kwargs>
        for CriteriaRecipientCustomFieldV1Kwargs
    {
        fn from(value: super::CriteriaRecipientCustomFieldV1Kwargs) -> Self {
            Self {
                account: Ok(value.account),
                field: Ok(value.field),
                list: Ok(value.list),
                recipient: Ok(value.recipient),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaRssStateChangedV1 {
        expect: ::std::result::Result<bool, ::std::string::String>,
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs:
            ::std::result::Result<super::CriteriaRssStateChangedV1Kwargs, ::std::string::String>,
        operator: ::std::result::Result<super::Operators, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaRssStateChangedV1 {
        fn default() -> Self {
            Self {
                expect: Err("no value supplied for expect".to_string()),
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
                operator: Err("no value supplied for operator".to_string()),
            }
        }
    }
    impl CriteriaRssStateChangedV1 {
        pub fn expect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.expect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expect: {e}"));
            self
        }
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CriteriaRssStateChangedV1Kwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
        pub fn operator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Operators>,
            T::Error: ::std::fmt::Display,
        {
            self.operator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for operator: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaRssStateChangedV1> for super::CriteriaRssStateChangedV1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaRssStateChangedV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expect: value.expect?,
                function: value.function?,
                kwargs: value.kwargs?,
                operator: value.operator?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaRssStateChangedV1> for CriteriaRssStateChangedV1 {
        fn from(value: super::CriteriaRssStateChangedV1) -> Self {
            Self {
                expect: Ok(value.expect),
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
                operator: Ok(value.operator),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaRssStateChangedV1Kwargs {
        account: ::std::result::Result<::std::string::String, ::std::string::String>,
        categories: ::std::result::Result<
            ::std::option::Option<Vec<::std::string::String>>,
            ::std::string::String,
        >,
        list: ::std::result::Result<::std::string::String, ::std::string::String>,
        min_new_items: ::std::result::Result<::std::num::NonZeroU64, ::std::string::String>,
        ruleset: ::std::result::Result<super::RulesetId, ::std::string::String>,
        subscriber: ::std::result::Result<::std::string::String, ::std::string::String>,
        url: ::std::result::Result<super::TheBlogRssUrl, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaRssStateChangedV1Kwargs {
        fn default() -> Self {
            Self {
                account: Err("no value supplied for account".to_string()),
                categories: Ok(Default::default()),
                list: Err("no value supplied for list".to_string()),
                min_new_items: Err("no value supplied for min_new_items".to_string()),
                ruleset: Err("no value supplied for ruleset".to_string()),
                subscriber: Err("no value supplied for subscriber".to_string()),
                url: Err("no value supplied for url".to_string()),
            }
        }
    }
    impl CriteriaRssStateChangedV1Kwargs {
        pub fn account<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.account = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for account: {e}"));
            self
        }
        pub fn categories<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<Vec<::std::string::String>>>,
            T::Error: ::std::fmt::Display,
        {
            self.categories = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for categories: {e}"));
            self
        }
        pub fn list<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.list = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list: {e}"));
            self
        }
        pub fn min_new_items<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::num::NonZeroU64>,
            T::Error: ::std::fmt::Display,
        {
            self.min_new_items = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for min_new_items: {e}"));
            self
        }
        pub fn ruleset<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RulesetId>,
            T::Error: ::std::fmt::Display,
        {
            self.ruleset = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ruleset: {e}"));
            self
        }
        pub fn subscriber<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.subscriber = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subscriber: {e}"));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::TheBlogRssUrl>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaRssStateChangedV1Kwargs>
        for super::CriteriaRssStateChangedV1Kwargs
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaRssStateChangedV1Kwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                account: value.account?,
                categories: value.categories?,
                list: value.list?,
                min_new_items: value.min_new_items?,
                ruleset: value.ruleset?,
                subscriber: value.subscriber?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaRssStateChangedV1Kwargs>
        for CriteriaRssStateChangedV1Kwargs
    {
        fn from(value: super::CriteriaRssStateChangedV1Kwargs) -> Self {
            Self {
                account: Ok(value.account),
                categories: Ok(value.categories),
                list: Ok(value.list),
                min_new_items: Ok(value.min_new_items),
                ruleset: Ok(value.ruleset),
                subscriber: Ok(value.subscriber),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaRulesetIsCheckpointed {
        expect: ::std::result::Result<bool, ::std::string::String>,
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<
            super::CriteriaRulesetIsCheckpointedKwargs,
            ::std::string::String,
        >,
        operator: ::std::result::Result<super::Operators, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaRulesetIsCheckpointed {
        fn default() -> Self {
            Self {
                expect: Err("no value supplied for expect".to_string()),
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
                operator: Err("no value supplied for operator".to_string()),
            }
        }
    }
    impl CriteriaRulesetIsCheckpointed {
        pub fn expect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.expect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expect: {e}"));
            self
        }
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CriteriaRulesetIsCheckpointedKwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
        pub fn operator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Operators>,
            T::Error: ::std::fmt::Display,
        {
            self.operator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for operator: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaRulesetIsCheckpointed>
        for super::CriteriaRulesetIsCheckpointed
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaRulesetIsCheckpointed,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expect: value.expect?,
                function: value.function?,
                kwargs: value.kwargs?,
                operator: value.operator?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaRulesetIsCheckpointed> for CriteriaRulesetIsCheckpointed {
        fn from(value: super::CriteriaRulesetIsCheckpointed) -> Self {
            Self {
                expect: Ok(value.expect),
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
                operator: Ok(value.operator),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaRulesetIsCheckpointedKwargs {
        ruleset: ::std::result::Result<::std::string::String, ::std::string::String>,
        subscriber: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaRulesetIsCheckpointedKwargs {
        fn default() -> Self {
            Self {
                ruleset: Err("no value supplied for ruleset".to_string()),
                subscriber: Err("no value supplied for subscriber".to_string()),
            }
        }
    }
    impl CriteriaRulesetIsCheckpointedKwargs {
        pub fn ruleset<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.ruleset = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ruleset: {e}"));
            self
        }
        pub fn subscriber<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.subscriber = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subscriber: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaRulesetIsCheckpointedKwargs>
        for super::CriteriaRulesetIsCheckpointedKwargs
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaRulesetIsCheckpointedKwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ruleset: value.ruleset?,
                subscriber: value.subscriber?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaRulesetIsCheckpointedKwargs>
        for CriteriaRulesetIsCheckpointedKwargs
    {
        fn from(value: super::CriteriaRulesetIsCheckpointedKwargs) -> Self {
            Self {
                ruleset: Ok(value.ruleset),
                subscriber: Ok(value.subscriber),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaRulesetReentryAllowed {
        expect: ::std::result::Result<bool, ::std::string::String>,
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<
            super::CriteriaRulesetReentryAllowedKwargs,
            ::std::string::String,
        >,
        operator: ::std::result::Result<super::Operators, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaRulesetReentryAllowed {
        fn default() -> Self {
            Self {
                expect: Err("no value supplied for expect".to_string()),
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
                operator: Err("no value supplied for operator".to_string()),
            }
        }
    }
    impl CriteriaRulesetReentryAllowed {
        pub fn expect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.expect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expect: {e}"));
            self
        }
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CriteriaRulesetReentryAllowedKwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
        pub fn operator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Operators>,
            T::Error: ::std::fmt::Display,
        {
            self.operator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for operator: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaRulesetReentryAllowed>
        for super::CriteriaRulesetReentryAllowed
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaRulesetReentryAllowed,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expect: value.expect?,
                function: value.function?,
                kwargs: value.kwargs?,
                operator: value.operator?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaRulesetReentryAllowed> for CriteriaRulesetReentryAllowed {
        fn from(value: super::CriteriaRulesetReentryAllowed) -> Self {
            Self {
                expect: Ok(value.expect),
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
                operator: Ok(value.operator),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaRulesetReentryAllowedKwargs {
        ruleset: ::std::result::Result<::std::string::String, ::std::string::String>,
        subscriber: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaRulesetReentryAllowedKwargs {
        fn default() -> Self {
            Self {
                ruleset: Err("no value supplied for ruleset".to_string()),
                subscriber: Err("no value supplied for subscriber".to_string()),
            }
        }
    }
    impl CriteriaRulesetReentryAllowedKwargs {
        pub fn ruleset<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.ruleset = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ruleset: {e}"));
            self
        }
        pub fn subscriber<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.subscriber = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subscriber: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaRulesetReentryAllowedKwargs>
        for super::CriteriaRulesetReentryAllowedKwargs
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaRulesetReentryAllowedKwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                ruleset: value.ruleset?,
                subscriber: value.subscriber?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaRulesetReentryAllowedKwargs>
        for CriteriaRulesetReentryAllowedKwargs
    {
        fn from(value: super::CriteriaRulesetReentryAllowedKwargs) -> Self {
            Self {
                ruleset: Ok(value.ruleset),
                subscriber: Ok(value.subscriber),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaTagAllTagsV1 {
        expect: ::std::result::Result<bool, ::std::string::String>,
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<super::CriteriaTagAllTagsV1Kwargs, ::std::string::String>,
        operator: ::std::result::Result<super::Operators, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaTagAllTagsV1 {
        fn default() -> Self {
            Self {
                expect: Err("no value supplied for expect".to_string()),
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
                operator: Err("no value supplied for operator".to_string()),
            }
        }
    }
    impl CriteriaTagAllTagsV1 {
        pub fn expect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.expect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expect: {e}"));
            self
        }
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CriteriaTagAllTagsV1Kwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
        pub fn operator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Operators>,
            T::Error: ::std::fmt::Display,
        {
            self.operator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for operator: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaTagAllTagsV1> for super::CriteriaTagAllTagsV1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaTagAllTagsV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expect: value.expect?,
                function: value.function?,
                kwargs: value.kwargs?,
                operator: value.operator?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaTagAllTagsV1> for CriteriaTagAllTagsV1 {
        fn from(value: super::CriteriaTagAllTagsV1) -> Self {
            Self {
                expect: Ok(value.expect),
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
                operator: Ok(value.operator),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaTagAllTagsV1Kwargs {
        account: ::std::result::Result<::std::string::String, ::std::string::String>,
        labels:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        list: ::std::result::Result<::std::string::String, ::std::string::String>,
        recipient: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaTagAllTagsV1Kwargs {
        fn default() -> Self {
            Self {
                account: Err("no value supplied for account".to_string()),
                labels: Err("no value supplied for labels".to_string()),
                list: Err("no value supplied for list".to_string()),
                recipient: Err("no value supplied for recipient".to_string()),
            }
        }
    }
    impl CriteriaTagAllTagsV1Kwargs {
        pub fn account<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.account = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for account: {e}"));
            self
        }
        pub fn labels<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.labels = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for labels: {e}"));
            self
        }
        pub fn list<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.list = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list: {e}"));
            self
        }
        pub fn recipient<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.recipient = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for recipient: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaTagAllTagsV1Kwargs> for super::CriteriaTagAllTagsV1Kwargs {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaTagAllTagsV1Kwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                account: value.account?,
                labels: value.labels?,
                list: value.list?,
                recipient: value.recipient?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaTagAllTagsV1Kwargs> for CriteriaTagAllTagsV1Kwargs {
        fn from(value: super::CriteriaTagAllTagsV1Kwargs) -> Self {
            Self {
                account: Ok(value.account),
                labels: Ok(value.labels),
                list: Ok(value.list),
                recipient: Ok(value.recipient),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaTagAnyTagsV1 {
        expect: ::std::result::Result<bool, ::std::string::String>,
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<super::CriteriaTagAnyTagsV1Kwargs, ::std::string::String>,
        memoize_id:
            ::std::result::Result<::std::option::Option<super::MemoizeId>, ::std::string::String>,
        operator: ::std::result::Result<super::Operators, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaTagAnyTagsV1 {
        fn default() -> Self {
            Self {
                expect: Err("no value supplied for expect".to_string()),
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
                memoize_id: Ok(Default::default()),
                operator: Err("no value supplied for operator".to_string()),
            }
        }
    }
    impl CriteriaTagAnyTagsV1 {
        pub fn expect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.expect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expect: {e}"));
            self
        }
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CriteriaTagAnyTagsV1Kwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
        pub fn memoize_id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::MemoizeId>>,
            T::Error: ::std::fmt::Display,
        {
            self.memoize_id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for memoize_id: {e}"));
            self
        }
        pub fn operator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Operators>,
            T::Error: ::std::fmt::Display,
        {
            self.operator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for operator: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaTagAnyTagsV1> for super::CriteriaTagAnyTagsV1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaTagAnyTagsV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expect: value.expect?,
                function: value.function?,
                kwargs: value.kwargs?,
                memoize_id: value.memoize_id?,
                operator: value.operator?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaTagAnyTagsV1> for CriteriaTagAnyTagsV1 {
        fn from(value: super::CriteriaTagAnyTagsV1) -> Self {
            Self {
                expect: Ok(value.expect),
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
                memoize_id: Ok(value.memoize_id),
                operator: Ok(value.operator),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaTagAnyTagsV1Kwargs {
        account: ::std::result::Result<::std::string::String, ::std::string::String>,
        labels:
            ::std::result::Result<::std::vec::Vec<::std::string::String>, ::std::string::String>,
        list: ::std::result::Result<::std::string::String, ::std::string::String>,
        recipient: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaTagAnyTagsV1Kwargs {
        fn default() -> Self {
            Self {
                account: Err("no value supplied for account".to_string()),
                labels: Err("no value supplied for labels".to_string()),
                list: Err("no value supplied for list".to_string()),
                recipient: Err("no value supplied for recipient".to_string()),
            }
        }
    }
    impl CriteriaTagAnyTagsV1Kwargs {
        pub fn account<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.account = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for account: {e}"));
            self
        }
        pub fn labels<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.labels = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for labels: {e}"));
            self
        }
        pub fn list<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.list = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list: {e}"));
            self
        }
        pub fn recipient<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.recipient = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for recipient: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaTagAnyTagsV1Kwargs> for super::CriteriaTagAnyTagsV1Kwargs {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaTagAnyTagsV1Kwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                account: value.account?,
                labels: value.labels?,
                list: value.list?,
                recipient: value.recipient?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaTagAnyTagsV1Kwargs> for CriteriaTagAnyTagsV1Kwargs {
        fn from(value: super::CriteriaTagAnyTagsV1Kwargs) -> Self {
            Self {
                account: Ok(value.account),
                labels: Ok(value.labels),
                list: Ok(value.list),
                recipient: Ok(value.recipient),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaUrlUrlContainsV1 {
        expect: ::std::result::Result<bool, ::std::string::String>,
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<super::CriteriaUrlUrlContainsV1Kwargs, ::std::string::String>,
        operator: ::std::result::Result<super::Operators, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaUrlUrlContainsV1 {
        fn default() -> Self {
            Self {
                expect: Err("no value supplied for expect".to_string()),
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
                operator: Err("no value supplied for operator".to_string()),
            }
        }
    }
    impl CriteriaUrlUrlContainsV1 {
        pub fn expect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.expect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expect: {e}"));
            self
        }
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CriteriaUrlUrlContainsV1Kwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
        pub fn operator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Operators>,
            T::Error: ::std::fmt::Display,
        {
            self.operator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for operator: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaUrlUrlContainsV1> for super::CriteriaUrlUrlContainsV1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaUrlUrlContainsV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expect: value.expect?,
                function: value.function?,
                kwargs: value.kwargs?,
                operator: value.operator?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaUrlUrlContainsV1> for CriteriaUrlUrlContainsV1 {
        fn from(value: super::CriteriaUrlUrlContainsV1) -> Self {
            Self {
                expect: Ok(value.expect),
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
                operator: Ok(value.operator),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaUrlUrlContainsV1Kwargs {
        contains: ::std::result::Result<::std::string::String, ::std::string::String>,
        url: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaUrlUrlContainsV1Kwargs {
        fn default() -> Self {
            Self {
                contains: Err("no value supplied for contains".to_string()),
                url: Err("no value supplied for url".to_string()),
            }
        }
    }
    impl CriteriaUrlUrlContainsV1Kwargs {
        pub fn contains<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.contains = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for contains: {e}"));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaUrlUrlContainsV1Kwargs>
        for super::CriteriaUrlUrlContainsV1Kwargs
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaUrlUrlContainsV1Kwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                contains: value.contains?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaUrlUrlContainsV1Kwargs>
        for CriteriaUrlUrlContainsV1Kwargs
    {
        fn from(value: super::CriteriaUrlUrlContainsV1Kwargs) -> Self {
            Self {
                contains: Ok(value.contains),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaUrlUrlEqualsV1 {
        expect: ::std::result::Result<bool, ::std::string::String>,
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<super::CriteriaUrlUrlEqualsV1Kwargs, ::std::string::String>,
        operator: ::std::result::Result<super::Operators, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaUrlUrlEqualsV1 {
        fn default() -> Self {
            Self {
                expect: Err("no value supplied for expect".to_string()),
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
                operator: Err("no value supplied for operator".to_string()),
            }
        }
    }
    impl CriteriaUrlUrlEqualsV1 {
        pub fn expect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.expect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expect: {e}"));
            self
        }
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CriteriaUrlUrlEqualsV1Kwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
        pub fn operator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Operators>,
            T::Error: ::std::fmt::Display,
        {
            self.operator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for operator: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaUrlUrlEqualsV1> for super::CriteriaUrlUrlEqualsV1 {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaUrlUrlEqualsV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expect: value.expect?,
                function: value.function?,
                kwargs: value.kwargs?,
                operator: value.operator?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaUrlUrlEqualsV1> for CriteriaUrlUrlEqualsV1 {
        fn from(value: super::CriteriaUrlUrlEqualsV1) -> Self {
            Self {
                expect: Ok(value.expect),
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
                operator: Ok(value.operator),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaUrlUrlEqualsV1Kwargs {
        match_: ::std::result::Result<::std::string::String, ::std::string::String>,
        url: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaUrlUrlEqualsV1Kwargs {
        fn default() -> Self {
            Self {
                match_: Err("no value supplied for match_".to_string()),
                url: Err("no value supplied for url".to_string()),
            }
        }
    }
    impl CriteriaUrlUrlEqualsV1Kwargs {
        pub fn match_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.match_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for match_: {e}"));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaUrlUrlEqualsV1Kwargs> for super::CriteriaUrlUrlEqualsV1Kwargs {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaUrlUrlEqualsV1Kwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                match_: value.match_?,
                url: value.url?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaUrlUrlEqualsV1Kwargs> for CriteriaUrlUrlEqualsV1Kwargs {
        fn from(value: super::CriteriaUrlUrlEqualsV1Kwargs) -> Self {
            Self {
                match_: Ok(value.match_),
                url: Ok(value.url),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaWebfeedNewFeedItemsV1 {
        expect: ::std::result::Result<bool, ::std::string::String>,
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<
            super::CriteriaWebfeedNewFeedItemsV1Kwargs,
            ::std::string::String,
        >,
        operator: ::std::result::Result<super::Operators, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaWebfeedNewFeedItemsV1 {
        fn default() -> Self {
            Self {
                expect: Err("no value supplied for expect".to_string()),
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
                operator: Err("no value supplied for operator".to_string()),
            }
        }
    }
    impl CriteriaWebfeedNewFeedItemsV1 {
        pub fn expect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.expect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expect: {e}"));
            self
        }
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CriteriaWebfeedNewFeedItemsV1Kwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
        pub fn operator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Operators>,
            T::Error: ::std::fmt::Display,
        {
            self.operator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for operator: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaWebfeedNewFeedItemsV1>
        for super::CriteriaWebfeedNewFeedItemsV1
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaWebfeedNewFeedItemsV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expect: value.expect?,
                function: value.function?,
                kwargs: value.kwargs?,
                operator: value.operator?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaWebfeedNewFeedItemsV1> for CriteriaWebfeedNewFeedItemsV1 {
        fn from(value: super::CriteriaWebfeedNewFeedItemsV1) -> Self {
            Self {
                expect: Ok(value.expect),
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
                operator: Ok(value.operator),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaWebfeedNewFeedItemsV1Kwargs {
        account: ::std::result::Result<::std::string::String, ::std::string::String>,
        list: ::std::result::Result<::std::string::String, ::std::string::String>,
        num_items: ::std::result::Result<::std::num::NonZeroU64, ::std::string::String>,
        ruleset: ::std::result::Result<super::RulesetId, ::std::string::String>,
        webfeed: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaWebfeedNewFeedItemsV1Kwargs {
        fn default() -> Self {
            Self {
                account: Err("no value supplied for account".to_string()),
                list: Err("no value supplied for list".to_string()),
                num_items: Err("no value supplied for num_items".to_string()),
                ruleset: Err("no value supplied for ruleset".to_string()),
                webfeed: Err("no value supplied for webfeed".to_string()),
            }
        }
    }
    impl CriteriaWebfeedNewFeedItemsV1Kwargs {
        pub fn account<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.account = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for account: {e}"));
            self
        }
        pub fn list<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.list = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list: {e}"));
            self
        }
        pub fn num_items<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::num::NonZeroU64>,
            T::Error: ::std::fmt::Display,
        {
            self.num_items = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for num_items: {e}"));
            self
        }
        pub fn ruleset<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RulesetId>,
            T::Error: ::std::fmt::Display,
        {
            self.ruleset = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ruleset: {e}"));
            self
        }
        pub fn webfeed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.webfeed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for webfeed: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaWebfeedNewFeedItemsV1Kwargs>
        for super::CriteriaWebfeedNewFeedItemsV1Kwargs
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaWebfeedNewFeedItemsV1Kwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                account: value.account?,
                list: value.list?,
                num_items: value.num_items?,
                ruleset: value.ruleset?,
                webfeed: value.webfeed?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaWebfeedNewFeedItemsV1Kwargs>
        for CriteriaWebfeedNewFeedItemsV1Kwargs
    {
        fn from(value: super::CriteriaWebfeedNewFeedItemsV1Kwargs) -> Self {
            Self {
                account: Ok(value.account),
                list: Ok(value.list),
                num_items: Ok(value.num_items),
                ruleset: Ok(value.ruleset),
                webfeed: Ok(value.webfeed),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaWebfeedRecurringWaitV1 {
        expect: ::std::result::Result<bool, ::std::string::String>,
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<
            super::CriteriaWebfeedRecurringWaitV1Kwargs,
            ::std::string::String,
        >,
        operator: ::std::result::Result<super::Operators, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaWebfeedRecurringWaitV1 {
        fn default() -> Self {
            Self {
                expect: Err("no value supplied for expect".to_string()),
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
                operator: Err("no value supplied for operator".to_string()),
            }
        }
    }
    impl CriteriaWebfeedRecurringWaitV1 {
        pub fn expect<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.expect = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for expect: {e}"));
            self
        }
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::CriteriaWebfeedRecurringWaitV1Kwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
        pub fn operator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Operators>,
            T::Error: ::std::fmt::Display,
        {
            self.operator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for operator: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaWebfeedRecurringWaitV1>
        for super::CriteriaWebfeedRecurringWaitV1
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaWebfeedRecurringWaitV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                expect: value.expect?,
                function: value.function?,
                kwargs: value.kwargs?,
                operator: value.operator?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaWebfeedRecurringWaitV1>
        for CriteriaWebfeedRecurringWaitV1
    {
        fn from(value: super::CriteriaWebfeedRecurringWaitV1) -> Self {
            Self {
                expect: Ok(value.expect),
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
                operator: Ok(value.operator),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct CriteriaWebfeedRecurringWaitV1Kwargs {
        account: ::std::result::Result<::std::string::String, ::std::string::String>,
        list: ::std::result::Result<::std::string::String, ::std::string::String>,
        rrules: ::std::result::Result<Vec<::std::string::String>, ::std::string::String>,
        ruleset: ::std::result::Result<super::RulesetId, ::std::string::String>,
        timezone: ::std::result::Result<::std::string::String, ::std::string::String>,
        webfeed: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for CriteriaWebfeedRecurringWaitV1Kwargs {
        fn default() -> Self {
            Self {
                account: Err("no value supplied for account".to_string()),
                list: Err("no value supplied for list".to_string()),
                rrules: Err("no value supplied for rrules".to_string()),
                ruleset: Err("no value supplied for ruleset".to_string()),
                timezone: Err("no value supplied for timezone".to_string()),
                webfeed: Err("no value supplied for webfeed".to_string()),
            }
        }
    }
    impl CriteriaWebfeedRecurringWaitV1Kwargs {
        pub fn account<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.account = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for account: {e}"));
            self
        }
        pub fn list<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.list = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list: {e}"));
            self
        }
        pub fn rrules<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<Vec<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.rrules = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for rrules: {e}"));
            self
        }
        pub fn ruleset<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RulesetId>,
            T::Error: ::std::fmt::Display,
        {
            self.ruleset = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ruleset: {e}"));
            self
        }
        pub fn timezone<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.timezone = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for timezone: {e}"));
            self
        }
        pub fn webfeed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.webfeed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for webfeed: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<CriteriaWebfeedRecurringWaitV1Kwargs>
        for super::CriteriaWebfeedRecurringWaitV1Kwargs
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: CriteriaWebfeedRecurringWaitV1Kwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                account: value.account?,
                list: value.list?,
                rrules: value.rrules?,
                ruleset: value.ruleset?,
                timezone: value.timezone?,
                webfeed: value.webfeed?,
            })
        }
    }
    impl ::std::convert::From<super::CriteriaWebfeedRecurringWaitV1Kwargs>
        for CriteriaWebfeedRecurringWaitV1Kwargs
    {
        fn from(value: super::CriteriaWebfeedRecurringWaitV1Kwargs) -> Self {
            Self {
                account: Ok(value.account),
                list: Ok(value.list),
                rrules: Ok(value.rrules),
                ruleset: Ok(value.ruleset),
                timezone: Ok(value.timezone),
                webfeed: Ok(value.webfeed),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Event {
        branch:
            ::std::result::Result<::std::option::Option<super::BranchId>, ::std::string::String>,
        filter: ::std::result::Result<super::Filter, ::std::string::String>,
        id: ::std::result::Result<super::EventId, ::std::string::String>,
        metadata: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        parents: ::std::result::Result<Vec<super::ParentActionIDsItem>, ::std::string::String>,
        recurring: ::std::result::Result<bool, ::std::string::String>,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::EventType, ::std::string::String>,
    }
    impl ::std::default::Default for Event {
        fn default() -> Self {
            Self {
                branch: Ok(Default::default()),
                filter: Err("no value supplied for filter".to_string()),
                id: Err("no value supplied for id".to_string()),
                metadata: Err("no value supplied for metadata".to_string()),
                parents: Err("no value supplied for parents".to_string()),
                recurring: Ok(Default::default()),
                title: Err("no value supplied for title".to_string()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Event {
        pub fn branch<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::BranchId>>,
            T::Error: ::std::fmt::Display,
        {
            self.branch = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for branch: {e}"));
            self
        }
        pub fn filter<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Filter>,
            T::Error: ::std::fmt::Display,
        {
            self.filter = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for filter: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EventId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn parents<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<Vec<super::ParentActionIDsItem>>,
            T::Error: ::std::fmt::Display,
        {
            self.parents = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for parents: {e}"));
            self
        }
        pub fn recurring<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.recurring = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for recurring: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::EventType>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Event> for super::Event {
        type Error = super::error::ConversionError;
        fn try_from(value: Event) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                branch: value.branch?,
                filter: value.filter?,
                id: value.id?,
                metadata: value.metadata?,
                parents: value.parents?,
                recurring: value.recurring?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::Event> for Event {
        fn from(value: super::Event) -> Self {
            Self {
                branch: Ok(value.branch),
                filter: Ok(value.filter),
                id: Ok(value.id),
                metadata: Ok(value.metadata),
                parents: Ok(value.parents),
                recurring: Ok(value.recurring),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Filter {
        criteria:
            ::std::result::Result<::std::vec::Vec<super::CriteriaItems>, ::std::string::String>,
        id: ::std::result::Result<::std::option::Option<super::Id>, ::std::string::String>,
        metadata: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        title: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        type_: ::std::result::Result<super::MatchOn, ::std::string::String>,
    }
    impl ::std::default::Default for Filter {
        fn default() -> Self {
            Self {
                criteria: Err("no value supplied for criteria".to_string()),
                id: Ok(Default::default()),
                metadata: Ok(Default::default()),
                title: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl Filter {
        pub fn criteria<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::CriteriaItems>>,
            T::Error: ::std::fmt::Display,
        {
            self.criteria = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for criteria: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Id>>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn metadata<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.metadata = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for metadata: {e}"));
            self
        }
        pub fn title<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for title: {e}"));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::MatchOn>,
            T::Error: ::std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Filter> for super::Filter {
        type Error = super::error::ConversionError;
        fn try_from(value: Filter) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                criteria: value.criteria?,
                id: value.id?,
                metadata: value.metadata?,
                title: value.title?,
                type_: value.type_?,
            })
        }
    }
    impl ::std::convert::From<super::Filter> for Filter {
        fn from(value: super::Filter) -> Self {
            Self {
                criteria: Ok(value.criteria),
                id: Ok(value.id),
                metadata: Ok(value.metadata),
                title: Ok(value.title),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct IteratorsMailingListRecipientsV1 {
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<
            super::IteratorsMailingListRecipientsV1Kwargs,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for IteratorsMailingListRecipientsV1 {
        fn default() -> Self {
            Self {
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
            }
        }
    }
    impl IteratorsMailingListRecipientsV1 {
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::IteratorsMailingListRecipientsV1Kwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<IteratorsMailingListRecipientsV1>
        for super::IteratorsMailingListRecipientsV1
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: IteratorsMailingListRecipientsV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                function: value.function?,
                kwargs: value.kwargs?,
            })
        }
    }
    impl ::std::convert::From<super::IteratorsMailingListRecipientsV1>
        for IteratorsMailingListRecipientsV1
    {
        fn from(value: super::IteratorsMailingListRecipientsV1) -> Self {
            Self {
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct IteratorsMailingListRecipientsV1Kwargs {
        account: ::std::result::Result<::std::string::String, ::std::string::String>,
        list: ::std::result::Result<::std::string::String, ::std::string::String>,
    }
    impl ::std::default::Default for IteratorsMailingListRecipientsV1Kwargs {
        fn default() -> Self {
            Self {
                account: Err("no value supplied for account".to_string()),
                list: Err("no value supplied for list".to_string()),
            }
        }
    }
    impl IteratorsMailingListRecipientsV1Kwargs {
        pub fn account<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.account = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for account: {e}"));
            self
        }
        pub fn list<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.list = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<IteratorsMailingListRecipientsV1Kwargs>
        for super::IteratorsMailingListRecipientsV1Kwargs
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: IteratorsMailingListRecipientsV1Kwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                account: value.account?,
                list: value.list?,
            })
        }
    }
    impl ::std::convert::From<super::IteratorsMailingListRecipientsV1Kwargs>
        for IteratorsMailingListRecipientsV1Kwargs
    {
        fn from(value: super::IteratorsMailingListRecipientsV1Kwargs) -> Self {
            Self {
                account: Ok(value.account),
                list: Ok(value.list),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct IteratorsMailingListSegmentV1 {
        function: ::std::result::Result<::std::string::String, ::std::string::String>,
        kwargs: ::std::result::Result<
            super::IteratorsMailingListSegmentV1Kwargs,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for IteratorsMailingListSegmentV1 {
        fn default() -> Self {
            Self {
                function: Err("no value supplied for function".to_string()),
                kwargs: Err("no value supplied for kwargs".to_string()),
            }
        }
    }
    impl IteratorsMailingListSegmentV1 {
        pub fn function<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.function = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for function: {e}"));
            self
        }
        pub fn kwargs<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::IteratorsMailingListSegmentV1Kwargs>,
            T::Error: ::std::fmt::Display,
        {
            self.kwargs = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for kwargs: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<IteratorsMailingListSegmentV1>
        for super::IteratorsMailingListSegmentV1
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: IteratorsMailingListSegmentV1,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                function: value.function?,
                kwargs: value.kwargs?,
            })
        }
    }
    impl ::std::convert::From<super::IteratorsMailingListSegmentV1> for IteratorsMailingListSegmentV1 {
        fn from(value: super::IteratorsMailingListSegmentV1) -> Self {
            Self {
                function: Ok(value.function),
                kwargs: Ok(value.kwargs),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct IteratorsMailingListSegmentV1Kwargs {
        account: ::std::result::Result<::std::string::String, ::std::string::String>,
        list: ::std::result::Result<::std::string::String, ::std::string::String>,
        segment: ::std::result::Result<super::Segment, ::std::string::String>,
    }
    impl ::std::default::Default for IteratorsMailingListSegmentV1Kwargs {
        fn default() -> Self {
            Self {
                account: Err("no value supplied for account".to_string()),
                list: Err("no value supplied for list".to_string()),
                segment: Err("no value supplied for segment".to_string()),
            }
        }
    }
    impl IteratorsMailingListSegmentV1Kwargs {
        pub fn account<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.account = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for account: {e}"));
            self
        }
        pub fn list<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.list = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for list: {e}"));
            self
        }
        pub fn segment<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Segment>,
            T::Error: ::std::fmt::Display,
        {
            self.segment = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for segment: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<IteratorsMailingListSegmentV1Kwargs>
        for super::IteratorsMailingListSegmentV1Kwargs
    {
        type Error = super::error::ConversionError;
        fn try_from(
            value: IteratorsMailingListSegmentV1Kwargs,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                account: value.account?,
                list: value.list?,
                segment: value.segment?,
            })
        }
    }
    impl ::std::convert::From<super::IteratorsMailingListSegmentV1Kwargs>
        for IteratorsMailingListSegmentV1Kwargs
    {
        fn from(value: super::IteratorsMailingListSegmentV1Kwargs) -> Self {
            Self {
                account: Ok(value.account),
                list: Ok(value.list),
                segment: Ok(value.segment),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct Ruleset {
        actions: ::std::result::Result<::std::vec::Vec<super::Action>, ::std::string::String>,
        events: ::std::result::Result<::std::vec::Vec<super::Event>, ::std::string::String>,
        id: ::std::result::Result<super::RulesetId, ::std::string::String>,
        iterator: ::std::result::Result<super::Iterator, ::std::string::String>,
        owner: ::std::result::Result<::std::option::Option<super::Owner>, ::std::string::String>,
        parent: ::std::result::Result<
            ::std::option::Option<super::RulesetParent>,
            ::std::string::String,
        >,
        state: ::std::result::Result<super::RulesetState, ::std::string::String>,
        system: ::std::result::Result<bool, ::std::string::String>,
        version: ::std::result::Result<f64, ::std::string::String>,
    }
    impl ::std::default::Default for Ruleset {
        fn default() -> Self {
            Self {
                actions: Err("no value supplied for actions".to_string()),
                events: Err("no value supplied for events".to_string()),
                id: Err("no value supplied for id".to_string()),
                iterator: Err("no value supplied for iterator".to_string()),
                owner: Err("no value supplied for owner".to_string()),
                parent: Err("no value supplied for parent".to_string()),
                state: Err("no value supplied for state".to_string()),
                system: Err("no value supplied for system".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl Ruleset {
        pub fn actions<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Action>>,
            T::Error: ::std::fmt::Display,
        {
            self.actions = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for actions: {e}"));
            self
        }
        pub fn events<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::Event>>,
            T::Error: ::std::fmt::Display,
        {
            self.events = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for events: {e}"));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RulesetId>,
            T::Error: ::std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {e}"));
            self
        }
        pub fn iterator<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::Iterator>,
            T::Error: ::std::fmt::Display,
        {
            self.iterator = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for iterator: {e}"));
            self
        }
        pub fn owner<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::Owner>>,
            T::Error: ::std::fmt::Display,
        {
            self.owner = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for owner: {e}"));
            self
        }
        pub fn parent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::RulesetParent>>,
            T::Error: ::std::fmt::Display,
        {
            self.parent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for parent: {e}"));
            self
        }
        pub fn state<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::RulesetState>,
            T::Error: ::std::fmt::Display,
        {
            self.state = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for state: {e}"));
            self
        }
        pub fn system<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<bool>,
            T::Error: ::std::fmt::Display,
        {
            self.system = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for system: {e}"));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<f64>,
            T::Error: ::std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<Ruleset> for super::Ruleset {
        type Error = super::error::ConversionError;
        fn try_from(value: Ruleset) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                actions: value.actions?,
                events: value.events?,
                id: value.id?,
                iterator: value.iterator?,
                owner: value.owner?,
                parent: value.parent?,
                state: value.state?,
                system: value.system?,
                version: value.version?,
            })
        }
    }
    impl ::std::convert::From<super::Ruleset> for Ruleset {
        fn from(value: super::Ruleset) -> Self {
            Self {
                actions: Ok(value.actions),
                events: Ok(value.events),
                id: Ok(value.id),
                iterator: Ok(value.iterator),
                owner: Ok(value.owner),
                parent: Ok(value.parent),
                state: Ok(value.state),
                system: Ok(value.system),
                version: Ok(value.version),
            }
        }
    }
}
#[doc = r" Generation of default values for serde."]
pub mod defaults {
    pub(super) fn default_bool<const V: bool>() -> bool {
        V
    }
    pub(super) fn actions_schedule_set_branch_v1_kwargs_timezone() -> ::std::string::String {
        "UTC".to_string()
    }
    pub(super) fn actions_schedule_wait_v1_kwargs_action() -> super::ActionId {
        super::ActionId("<action>".to_string())
    }
}
