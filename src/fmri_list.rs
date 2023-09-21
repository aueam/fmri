use std::fmt::{Debug, Display, Formatter};
use serde::{Deserialize, Serialize};
use crate::FMRI;

/// [`FMRIList`] contains more [`FMRIs`][FMRI]
#[derive(PartialEq, Serialize, Deserialize, Clone)]
pub struct FMRIList(Vec<FMRI>);

impl FMRIList {
    /// Creates new [`FMRIList`]
    pub fn new() -> Self {
        Self(vec![])
    }

    /// Adds new [`FMRI`] into [`FMRIList`]
    pub fn add(&mut self, fmri: FMRI) {
        self.0.push(fmri)
    }

    /// Returns [`FMRIList`] as [`Vec`]<[`FMRI`]>
    pub fn get(self) -> Vec<FMRI> {
        self.0
    }

    /// Returns [`FMRIList`] as &[`Vec`]<[`FMRI`]>
    pub fn get_ref(&self) -> &Vec<FMRI> {
        &self.0
    }

    /// Returns [FMRIList`] as &mut [`Vec`]<[`FMRI`]>
    pub fn get_ref_mut(&mut self) -> &mut Vec<FMRI> {
        &mut self.0
    }

    /// Returns length of [`FMRIList`]
    pub fn len(&self) -> usize {
        self.get_ref().len()
    }

    /// Checks if [`FMRIList`] is empty
    pub fn is_empty(&self) -> bool {
        if self.get_ref().len() == 0 {
            return true;
        }
        false
    }

    /// Checks if [`FMRIList`] has specified [`FMRI`]
    pub fn is_fmri_in_list(&self, checking_fmri: &FMRI) -> bool {
        for fmri in self.get_ref() {
            if fmri.package_name_eq(checking_fmri) {
                return true;
            }
        }
        false
    }
}

/// Implementation of [`Display`] for [`FMRIList`]
impl Display for FMRIList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string: String = "".to_owned();

        for (index, fmri) in self.get_ref().iter().enumerate() {
            string.push_str(&*format!("{}. {}\n", index, fmri))
        }

        write!(f, "{}", string)
    }
}

/// Implementation of [`Debug`] for [`FMRIList`]
impl Debug for FMRIList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{}", self))
    }
}