use std::collections::HashMap;

use petgraph::{graph::NodeIndex, Graph};

use crate::{RelationshipType, SPDX};

pub(crate) fn create_graph(spdx: &SPDX) -> Graph<&str, &RelationshipType> {
    let mut g = Graph::<&str, &RelationshipType>::new();
    let mut nodes: HashMap<&str, NodeIndex> = HashMap::new();
    for relationship in &spdx.relationships {
        let a = *nodes
            .entry(&relationship.spdx_element_id)
            .or_insert_with(|| g.add_node(&relationship.spdx_element_id));
        let b = *nodes
            .entry(&relationship.related_spdx_element)
            .or_insert_with(|| g.add_node(&relationship.related_spdx_element));
        g.add_edge(a, b, &relationship.relationship_type);
    }
    g
}

#[cfg(test)]
mod test {
    use petgraph::dot::Dot;

    use super::*;

    #[test]
    fn create_graph_succeeds() {
        let spdx = SPDX::from_file("tests/data/SPDXForGraph.spdx.json").unwrap();
        let graph = create_graph(&spdx);
        Dot::new(&graph);
    }

    #[test]
    fn create_complex_graph_succeeds() {
        let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
        let graph = create_graph(&spdx);
        Dot::new(&graph);
    }
}
