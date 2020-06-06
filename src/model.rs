// Copyright 2018 Sandeep Datta
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use serde_yaml;
use std::fs::File;
use std::io::prelude::*;
use std::vec::Vec;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Quiz {
    pub title: String,
    pub bank: Vec<QAndA>,
}

impl Quiz {
    pub fn new(title: &str, bank: Vec<QAndA>) -> Quiz {
        Quiz {
            title: title.to_owned(),
            bank,
        }
    }
}


#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct QAndA {
    pub question: String,
    pub answer: String,
}

impl QAndA {
    pub fn new(question: &str, answer: &str) -> Self {
        QAndA {
            question: question.to_owned(),
            answer: answer.to_owned(),
        }
    }
}

pub fn save_sample_yaml(file_name: &str) {
    let quiz = Quiz::new("Vim Quiz!", 
                            vec![ QAndA::new("How do you quit vim?", ":q")
                                , QAndA::new("How do you enter insert mode from normal mode?", "'i' or 'a'")]);

    let s = serde_yaml::to_string(&quiz).unwrap();

    let mut file = File::create(file_name).unwrap();
    write!(file, "{}", s).unwrap();
}

pub fn load_file(file_name: &str) -> Quiz {
    let mut file = File::open(file_name).unwrap();

    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();

    return serde_yaml::from_str(&contents).unwrap();
}