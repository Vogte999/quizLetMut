mod json;

use rand::prelude::*;
use std::vec::Vec;
use self::json::Json;
use serde_json::Error;


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

    pub fn new_from_json() -> Result<QuestionBuffer, Error> {
        let mut json = Json::new()?;
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