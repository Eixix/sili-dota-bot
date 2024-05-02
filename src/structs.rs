use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Punlines {
    pub(crate) dodo_poll: PollOptions,
    match_outcome: MatchOptions,
    performance_verbs: PerformanceVerbs,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MatchOptions {
    win: Vec<String>,
    lose: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PollOptions {
    pub(crate) ja: Vec<String>,
    pub(crate) nein: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PerformanceVerbs {
    #[serde(rename = "0.5")]
    first: Vec<String>,
    #[serde(rename = "1")]
    second: Vec<String>,
    #[serde(rename = "2")]
    third: Vec<String>,
    #[serde(rename = "5")]
    fourth: Vec<String>,
    #[serde(rename = "10")]
    fifth: Vec<String>,
    inf: Vec<String>,
}
