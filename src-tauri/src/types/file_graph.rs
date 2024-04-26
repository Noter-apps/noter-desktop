use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{FilePreview, Id};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub from: Id,
    pub to: Id,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Graph {
    /// id represents file in graph, if it exists, the value is Some, if it exists only as an to
    /// part of an edge, the value is None
    pub nodes: HashMap<Id, Option<FilePreview>>,
    pub edges: Vec<Edge>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    pub fn get_node(&self, id: &Id) -> Option<&FilePreview> {
        self.nodes.get(id).unwrap_or(&None).as_ref()
    }

    pub fn add_node(&mut self, node: FilePreview) {
        self.nodes.insert(node.id.clone(), Some(node));
    }

    pub fn add_edge(&mut self, from: Id, to: Id) {
        if let None = self.get_node(&from) {
            self.nodes.insert(from.clone(), None);
        }
        if let None = self.get_node(&to) {
            self.nodes.insert(to.clone(), None);
        }

        self.edges.push(Edge { from, to });
    }

    pub fn get_node_list(&self) -> Vec<Option<FilePreview>> {
        self.nodes.values().cloned().collect()
    }

    pub fn get_edge_list(&self) -> Vec<Edge> {
        self.edges.clone()
    }
}
