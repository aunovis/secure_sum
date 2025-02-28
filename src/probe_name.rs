use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[allow(non_camel_case_types)]
pub(crate) enum ProbeName {
    archived,
    codeApproved,
}
