/// Used as a wrapper for data from/to Database
#[derive(Clone, Debug)]
pub struct Credentials {
    pub key: String,
    pub value: String,
    pub info: Option<String>,
}

impl Credentials {
    /// Create new instance of `Credentials`
    pub fn new(key: String, value: String, info: String) -> Self {
        if info.is_empty() {
            Credentials {
                key: key,
                value: value,
                info: None,
            }
        } else {
            Credentials {
                key: key,
                value: value,
                info: Some(info),
            }
        }
    }

    /// Returns tuple for data in current struct by
    /// dealing with info which is `Option<String>
    pub fn provide(&self) -> (String, String, String) {
        if let Some(info) = self.info.clone() {
            (self.key.clone(), self.value.clone(), info.clone())
        } else {
            (self.key.clone(), self.value.clone(), String::new())
        }
    }
}
