// Licensed under the Apache License, Version 2.0 (the "License"); you may
// not use this file except in compliance with the License. You may obtain
// a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations
// under the License.

use hashbrown::HashSet;
use std::collections::VecDeque;
use std::hash::Hash;

use petgraph::visit::{GraphProp, IntoNeighbors, IntoNodeIdentifiers, VisitMap, Visitable};

/// Given an graph, a node in the graph, and a visit_map,
/// return the set of nodes connected to the given node.
///
/// Arguments:
///
/// * `graph` - The graph object to run the algorithm on
/// * `node` - The node index to find the connected nodes for
/// * `graph.visit_map()` - The visit map for the graph
///
/// # Example
/// ```rust
/// use std::iter::FromIterator;
/// use hashbrown::HashSet;
/// use petgraph::graph::Graph;
/// use petgraph::graph::node_index as ndx;
/// use petgraph::visit::Visitable;
/// use petgraph::Directed;
/// use retworkx_core::connectivity::bfs_undirected;
///
/// let graph = Graph::<(), (), Directed>::from_edges(&[
///     (0, 1),
///     (1, 2),
///     (2, 3),
///     (3, 0),
///     (4, 5),
///     (5, 6),
///     (6, 7),
///     (7, 4),
/// ]);
/// let node_idx = ndx(3);
/// let component = bfs_undirected(&graph, node_idx, &mut graph.visit_map());
/// let expected = HashSet::from_iter([ndx(0), ndx(1), ndx(3), ndx(2)]);
/// assert_eq!(expected, component);
/// ```
pub fn bfs_undirected<G>(graph: G, start: G::NodeId, discovered: &mut G::Map) -> HashSet<G::NodeId>
where
    G: GraphProp + IntoNeighbors + Visitable,
    G::NodeId: Eq + Hash,
{
    let mut component = HashSet::new();
    component.insert(start);
    let mut stack = VecDeque::new();
    stack.push_front(start);

    while let Some(node) = stack.pop_front() {
        for succ in graph.neighbors(node) {
            if discovered.visit(succ) {
                stack.push_back(succ);
                component.insert(succ);
            }
        }
    }

    component
}

/// Given a graph, return a list of sets of all the
/// connected components.
///
/// Arguments:
///
/// * `graph` - The graph object to run the algorithm on
///
/// # Example
/// ```rust
/// use std::iter::FromIterator;
/// use hashbrown::HashSet;
/// use petgraph::graph::Graph;
/// use petgraph::graph::NodeIndex;
/// use petgraph::{Undirected, Directed};
/// use petgraph::graph::node_index as ndx;
/// use retworkx_core::connectivity::connected_components;
///
/// let graph = Graph::<(), (), Undirected>::from_edges(&[
///     (0, 1),
///     (1, 2),
///     (2, 3),
///     (3, 0),
///     (4, 5),
///     (5, 6),
///     (6, 7),
///     (7, 4),
/// ]);
/// let components = connected_components(&graph);
/// let exp1 = HashSet::from_iter([ndx(0), ndx(1), ndx(3), ndx(2)]);
/// let exp2 = HashSet::from_iter([ndx(7), ndx(5), ndx(4), ndx(6)]);
/// let expected = vec![exp1, exp2];
/// assert_eq!(expected, components);
/// ```
pub fn connected_components<G>(graph: G) -> Vec<HashSet<G::NodeId>>
where
    G: GraphProp + IntoNeighbors + Visitable + IntoNodeIdentifiers,
    G::NodeId: Eq + Hash,
{
    let mut conn_components = Vec::new();
    let mut discovered = graph.visit_map();

    for start in graph.node_identifiers() {
        if !discovered.visit(start) {
            continue;
        }

        let component = bfs_undirected(graph, start, &mut discovered);
        conn_components.push(component)
    }

    conn_components
}

/// Given a graph, return the number of connected components of the graph.
///
/// Arguments:
///
/// * `graph` - The graph object to run the algorithm on
///
/// # Example
/// ```rust
/// use retworkx_core::petgraph::{Graph, Undirected};
/// use retworkx_core::connectivity::number_connected_components;
///
/// let graph = Graph::<(), (), Undirected>::from_edges([(0, 1), (1, 2), (3, 4)]);
/// assert_eq!(number_connected_components(&graph), 2);
/// ```
pub fn number_connected_components<G>(graph: G) -> usize
where
    G: GraphProp + IntoNeighbors + Visitable + IntoNodeIdentifiers,
    G::NodeId: Eq + Hash,
{
    let mut num_components = 0;

    let mut discovered = graph.visit_map();
    for start in graph.node_identifiers() {
        if !discovered.visit(start) {
            continue;
        }

        num_components += 1;
        bfs_undirected(graph, start, &mut discovered);
    }

    num_components
}

#[cfg(test)]
mod test_conn_components {
    use std::iter::FromIterator;
    use hashbrown::HashSet;
    use petgraph::graph::{Graph, NodeIndex};
    use petgraph::graph::node_index as ndx;
    use petgraph::visit::Visitable;
    use petgraph::Directed;

    use crate::connectivity::{connected_components, number_connected_components, bfs_undirected};

    #[test]
    fn test_number_connected() {
        let graph = Graph::<(), (), Directed>::from_edges([(0, 1), (1, 2), (3, 4)]);
        assert_eq!(number_connected_components(&graph), 2);
    }

    #[test]
    fn test_number_node_holes() {
        let mut graph = Graph::<(), (), Directed>::from_edges([(0, 1), (1, 2)]);
        graph.remove_node(NodeIndex::new(1));
        assert_eq!(number_connected_components(&graph), 2);
    }

    #[test]
    fn test_connected_components() {
        let graph = Graph::<(), (), Directed>::from_edges(&[
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0),
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 4),
        ]);
        let components = connected_components(&graph);
        let exp1 = HashSet::from_iter([ndx(0), ndx(1), ndx(3), ndx(2)]);
        let exp2 = HashSet::from_iter([ndx(7), ndx(5), ndx(4), ndx(6)]);
        let expected = vec![exp1, exp2];
        assert_eq!(expected, components);
    }

    #[test]
    fn test_bfs_undirected() {
        let graph = Graph::<(), (), Directed>::from_edges(&[
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0),
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 4),
        ]);
        let node_idx = NodeIndex::new(3);
        let component = bfs_undirected(&graph, node_idx, &mut graph.visit_map());
        let expected = HashSet::from_iter([ndx(0), ndx(1), ndx(3), ndx(2)]);
        assert_eq!(expected, component);
    }
}