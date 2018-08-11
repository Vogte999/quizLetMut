use serde_json;
use serde_json::Error;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Json {
    questions: HashMap<String, String>
}

impl Json {
    pub fn new(data: &str) -> Result<Json, Error> {
        // Parse the string of data into a Person object. This is exactly the
        // same function as the one that produced serde_json::Value above, but
        // now we are asking it for a Person as output.
        let j: Json = serde_json::from_str(data)?;

        Ok(j)
    }

    pub fn to_vec(self) -> Vec<(String, String)> {
        let mut vec: Vec<(String, String)> = Vec::new();
        for (str1, str2) in self.questions {
            vec.push((str1, str2));
        }
        vec
    }
}