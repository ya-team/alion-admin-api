use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize};

const DEFAULT_PAGE_SIZE: u64 = 10;
const DEFAULT_PAGE_NUM: u64 = 1;

#[derive(Debug, Serialize, Deserialize)]
pub struct PageRequest {
    #[serde(
        default = "default_current",
        deserialize_with = "deserialize_optional_u64"
    )]
    pub current: u64,
    #[serde(
        default = "default_size",
        deserialize_with = "deserialize_optional_u64"
    )]
    pub size: u64,
}

fn deserialize_optional_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrU64 {
        String(String),
        U64(u64),
    }

    match Option::<StringOrU64>::deserialize(deserializer)? {
        None => Ok(DEFAULT_PAGE_NUM),
        Some(StringOrU64::U64(n)) => Ok(n),
        Some(StringOrU64::String(s)) if s.is_empty() => Ok(DEFAULT_PAGE_NUM),
        Some(StringOrU64::String(s)) => s.parse::<u64>().map_err(DeError::custom),
    }
}

fn default_current() -> u64 {
    DEFAULT_PAGE_NUM
}

fn default_size() -> u64 {
    DEFAULT_PAGE_SIZE
}

impl Default for PageRequest {
    fn default() -> Self {
        Self {
            current: default_current(),
            size: default_size(),
        }
    }
}

#[derive(Debug, Serialize, Default)]
pub struct PaginatedData<T> {
    pub current: u64,
    pub size: u64,
    pub total: u64,
    pub records: Vec<T>,
}
