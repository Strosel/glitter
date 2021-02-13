use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;
use std::result::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Arguments {
    pub case: Option<String>,
    pub argument: i32,
}
#[derive(Deserialize, Debug)]
pub struct GlitterRc {
    #[serde(default = "commit_msg")]
    pub commit_message: String,
    #[serde(default = "commit_msg_arguments")]
    pub commit_message_arguments: Vec<Arguments>,
}

pub fn get_and_parse() -> Result<GlitterRc, serde_json::Error> {
    let json_file_path = Path::new("./.glitterrc");

    let file = File::open(json_file_path).expect("Error opening file");

    serde_json::from_reader(file)
}

fn commit_msg() -> String {
    String::from("$RAW_COMMIT_MSG")
}

fn commit_msg_arguments() -> Vec<Arguments> {
    vec![Arguments {
        case: serde::__private::Some("standard".to_owned()),
        argument: 0,
    }]
}
