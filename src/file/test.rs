use serde::{Deserialize, Serialize};

use super::{input::Input, output::Output};

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Test {
    pub test_title: String,
    #[serde(
        default,
        alias = "description",
        skip_serializing_if = "String::is_empty"
    )]
    pub desc: String,
    pub stages: Vec<StageWrapper>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StageWrapper {
    pub stage: Stage,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Stage {
    pub input: Input,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<Output>,
}
