use std::collections::HashMap;

use super::Id;

pub struct FileGraph {
    pub nodes: HashMap<Id, (bool, Vec<Id>)>,
}
