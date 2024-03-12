use std::path::Path;

use serde::{Deserialize, Serialize};

use self::{
    input::Input,
    meta::Meta,
    test::{Stage, Test},
};

pub mod input;
pub mod meta;
pub mod output;
pub mod test;

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct File {
    pub meta: Meta,
    pub tests: Vec<Test>,
    // disallow construction outside of this module?
    #[serde(default, skip)]
    _private: (),
}

#[derive(Debug, thiserror::Error)]
pub enum FileError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    YamlError(#[from] serde_yaml::Error),
    #[error(transparent)]
    HttpError(#[from] http::Error),
}

impl File {
    pub fn from_str(s: &str) -> Result<Self, FileError> {
        serde_yaml::from_str(s)
            .map_err(Into::into)
            .map(|mut file: Self| {
                // FIXME: some sort of initialization magic
                file.inputs_mut().for_each(Input::do_magic);
                file
            })
    }

    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, FileError> {
        Self::from_str(&std::fs::read_to_string(path)?)
    }

    pub fn stages(&self) -> impl Iterator<Item = &Stage> {
        self.tests
            .iter()
            .flat_map(|t| t.stages.iter().map(|s| &s.stage))
    }

    pub fn stages_mut(&mut self) -> impl Iterator<Item = &mut Stage> {
        self.tests
            .iter_mut()
            .flat_map(|t| t.stages.iter_mut().map(|s| &mut s.stage))
    }

    pub fn inputs(&self) -> impl Iterator<Item = &Input> {
        self.stages().map(|s| &s.input)
    }

    pub fn inputs_mut(&mut self) -> impl Iterator<Item = &mut Input> {
        self.stages_mut().map(|s| &mut s.input)
    }
}

fn is_false(&b: &bool) -> bool {
    b == false
}

fn is_true(&b: &bool) -> bool {
    b == true
}
