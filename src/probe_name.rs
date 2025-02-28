use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub(crate) enum ProbeName {
    archived,
    blocksDeleteOnBranches,
    codeApproved,
}
