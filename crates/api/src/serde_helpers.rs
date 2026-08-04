use serde::Deserialize;

pub fn deserialize_string_or_int<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt {
        Int(i64),
        String(String),
    }

    Option::<StringOrInt>::deserialize(deserializer).map(|opt| {
        opt.map(|v| match v {
            StringOrInt::String(s) => s,
            StringOrInt::Int(i) => i.to_string(),
        })
    })
}
