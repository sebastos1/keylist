use crate::keys::Key;
use indexmap::IndexMap;
use serde::Deserialize;
use serde_yaml::Value;

#[derive(Debug)]
pub struct Category {
    pub name: Option<String>,
    pub bindings: Vec<Binding>,
}

#[derive(Debug)]
pub struct Binding {
    pub description: String,
    pub keys: Vec<KeyEntry>,
}

#[derive(Debug)]
pub enum KeyEntry {
    Single(Key),
    OneOf(Vec<Key>),
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum RawCategory {
    Flat(Vec<Value>),
    Bindings(IndexMap<String, Vec<Value>>),
}

#[derive(Deserialize)]
pub struct RawDocument {
    #[serde(default)]
    pub config: super::Config,
    pub keys: IndexMap<String, RawCategory>,
}

fn parse_entry(val: &Value, mod_key: &str) -> Option<KeyEntry> {
    match val {
        Value::String(s) => Key::from_str(s, mod_key).map(KeyEntry::Single),
        Value::Sequence(seq) => {
            let keys: Vec<Key> = seq
                .iter()
                .filter_map(|v| {
                    let s = match v {
                        Value::String(s) => s.clone(),
                        Value::Number(n) => n.to_string(),
                        _ => return None,
                    };
                    Key::from_str(&s, mod_key)
                })
                .collect();
            (!keys.is_empty()).then_some(KeyEntry::OneOf(keys))
        }
        _ => None,
    }
}

fn parse_keys(raw_keys: &[Value], mod_key: &str) -> Vec<KeyEntry> {
    raw_keys
        .iter()
        .filter_map(|v| parse_entry(v, mod_key))
        .collect()
}

impl From<RawDocument> for super::Document {
    fn from(raw: RawDocument) -> Self {
        let categories = raw
            .keys
            .into_iter()
            .map(|(name, category)| match category {
                RawCategory::Flat(keys) => Category {
                    name: None,
                    bindings: vec![Binding {
                        description: name,
                        keys: parse_keys(&keys, &raw.config.mod_key),
                    }],
                },
                RawCategory::Bindings(map) => Category {
                    name: Some(name),
                    bindings: map
                        .into_iter()
                        .map(|(desc, keys)| Binding {
                            description: desc,
                            keys: parse_keys(&keys, &raw.config.mod_key),
                        })
                        .collect(),
                },
            })
            .collect();

        super::Document {
            config: raw.config,
            categories,
        }
    }
}
