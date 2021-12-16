use std::{collections::HashMap};

use crate::model::{HintId, PlayerId};

pub enum Phase {
    SelectPlacingHint(HashMap<PlayerId,SelectPlacingHint>)
}

pub struct SelectPlacingHint {
    pub hint: HintId
}