use serde_json;
use serde_json::Error;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Json {
    questions: HashMap<String, String>
}

impl Json {
    pub fn new() -> Result<Json, Error> {
        // Some JSON input data as a &str. Maybe this comes from the user.
        let data = r#"{
                    "questions": {
                        "Kartoffel" : "potato",
                        "Nudel"     : "noodle",
                        "Kuchen"    : "cake",
                        "Brot"      : "bread",
                        "Affe"      : "monkey",
                        "Frankreich": "France"
                    }
                  }"#;

        // Parse the string of data into a Person object. This is exactly the
        // same function as the one that produced serde_json::Value above, but
        // now we are asking it for a Person as output.
        let j: Json = serde_json::from_str(data)?;

        Ok(j)
    }

    pub fn to_vec(mut self) -> Vec<(String, String)> {
        let mut vec: Vec<(String, String)> = Vec::new();
        for (str1, str2) in self.questions {
            vec.push((str1, str2));
        }
        vec
    }
}