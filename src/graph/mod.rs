use std::collections::HashMap;

use petgraph::{
    algo::astar,
    dot::Dot,
    graphmap::{DiGraphMap, GraphMap},
    Directed, EdgeDirection,
};

use crate::{RelationshipType, SPDX};

pub(crate) fn create_graph(spdx: &SPDX) -> DiGraphMap<&str, &RelationshipType> {
    let mut g = DiGraphMap::<&str, &RelationshipType>::new();
    let mut nodes: HashMap<&str, &str> = HashMap::new();
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

pub(crate) fn find_path<'a>(
    graph: &'a DiGraphMap<&'a str, &'a RelationshipType>,
    start: &'a str,
    end: &'a str,
) -> Option<(i32, Vec<&'a str>)> {
    astar(graph, start, |goal| end == goal, |_| 1, |_| 0)
}

pub(crate) fn path_with_relationships<'a>(
    graph: &'a DiGraphMap<&'a str, &'a RelationshipType>,
    path: Vec<&'a str>,
) -> Vec<&'a str> {
    let mut path_with_relationships: Vec<&str> = Vec::new();
    for spdx_id in path {
        if !path_with_relationships.is_empty() {
            let edge = graph
                .edges_directed(
                    path_with_relationships.last().unwrap(),
                    EdgeDirection::Outgoing,
                )
                .find(|edge| edge.1 == spdx_id)
                .unwrap();

            path_with_relationships.push(edge.2.as_ref())
        }
        path_with_relationships.push(spdx_id);
    }
    path_with_relationships
}

/// Create graphviz dot.
pub fn graphviz<'a>(
    graph: &'a DiGraphMap<&'a str, &'a RelationshipType>,
) -> Dot<&GraphMap<&str, &RelationshipType, Directed>> {
    Dot::new(graph)
}

#[cfg(test)]
mod test {
    use petgraph::dot::Dot;

    use super::*;

    #[test]
    fn create_graph_succeeds() {
        let spdx = SPDX::from_file("tests/data/SPDXForGraph.spdx.json").unwrap();
        let graph = create_graph(&spdx);
        let dot = Dot::new(&graph);
        dbg!(dot);
    }

    #[test]
    fn create_complex_graph_succeeds() {
        let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
        let graph = create_graph(&spdx);
        let dot = Dot::new(&graph);
        dbg!(dot);
    }

    #[test]
    fn find_path_works() {
        let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
        let graph = create_graph(&spdx);
        let path = find_path(&graph, "SPDXRef-DOCUMENT", "SPDXRef-Saxon").unwrap();
        dbg!(path);
    }

    #[test]
    fn find_complex_path_works() {
        let spdx = SPDX::from_file("tests/data/SPDXForGraph.spdx.json").unwrap();
        let graph = create_graph(&spdx);
        let path = find_path(&graph, "SPDXRef-Package-1", "SPDXRef-File-1").unwrap();
        dbg!(path);
    }

    #[test]
    fn find_path_with_relationships_works() {
        let spdx = SPDX::from_file("tests/data/SPDXJSONExample-v2.2.spdx.json").unwrap();
        let graph = create_graph(&spdx);
        let path = find_path(&graph, "SPDXRef-DOCUMENT", "SPDXRef-Saxon").unwrap();
        let path = path_with_relationships(&graph, path.1);
        dbg!(path);
    }

    #[test]
    fn find_complex_path_with_relationships_works() {
        let spdx = SPDX::from_file("tests/data/SPDXForGraph.spdx.json").unwrap();
        let graph = create_graph(&spdx);
        let path = find_path(&graph, "SPDXRef-Package-1", "SPDXRef-File-1").unwrap();
        let path = path_with_relationships(&graph, path.1);
        dbg!(path);
    }
}
