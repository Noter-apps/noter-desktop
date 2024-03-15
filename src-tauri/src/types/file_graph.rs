use std::collections::HashMap;

use super::{id, Id};

pub struct FileGraph {
    nodes: HashMap<Id, bool>
}
