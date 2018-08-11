mod json;

use rand::prelude::*;
use std::vec::Vec;
use self::json::Json;
use std::fs::File;
use std::io::prelude::*;

pub struct QuestionBuffer {
    questions: Vec<(String, String)>,
    done: Vec<(String, String)>,
    current: usize
}

impl QuestionBuffer {

    pub fn new(vec: Vec<(&str, &str)>) -> QuestionBuffer {
        let mut questions = Vec::new();
        for strings in vec {
            questions.push((String::from(strings.0), String::from(strings.1)));
        }
        QuestionBuffer{
            questions,
            done: Vec::new(),
            current: 0
        }
    }

    pub fn new_from_json(name: &str) -> Result<QuestionBuffer, String> {
        let mut file = match File::open(name) {
            Ok(file)    => file,
            Err(_msg)    => return Err(String::from("Failed to open file."))
        };
        let mut data = String::new();
        match file.read_to_string(&mut data) {
            Ok(_m)    => println!("file read."),
            Err(_m)   => return Err(String::from("Failed to read file."))
        }
        let json = match Json::new(&data) {
            Ok(json)    => json,
            Err(_msg)    => return Err(String::from("Serialization failed."))
        };
        Ok(QuestionBuffer {
            questions: json.to_vec(),
            done: Vec::new(),
            current: 0
        })
    }

    pub fn next(&mut self) -> Option<&(String, String)> {
        match self.questions.len() {
            0   => {
                None
            },
            len => {
                let mut rng = thread_rng();
                let index = rng.gen_range(0, len);
                self.current = index;
                Some(&self.questions[index])
            }
        }
    }

    pub fn evaluate(&mut self, input: &str) -> &str {
        if input == &self.questions[self.current].1 {
            let question = self.questions.remove(self.current);
            self.done.push(question);
            &self.done[self.done.len() - 1].1
        } else {
            &self.questions[self.current].1
        }
    }

    //pub fn restart(&mut self) {
        //self.questions = self.done.clone();
        //self.done = Vec::new();
    //}

}