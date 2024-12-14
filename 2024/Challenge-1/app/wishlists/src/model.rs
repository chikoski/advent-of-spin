use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Kid {
    name: String,
    items: Vec<String>,
}

impl Kid {
    fn new(name: impl Into<String>, items: Vec<impl Into<String>>) -> Kid {
        let name = name.into();
        let items = items.into_iter().map(|i| i.into()).collect();
        Kid { name, items }
    }
}

impl Default for Kid {
    fn default() -> Self {
        let items = vec![
            "Cozy Gloves",
            "Purple Beanie",
        ];
        Kid::new("John Doe", items)
    }
}

