use std::collections::{HashMap, HashSet};

pub trait Edge {
    fn get_id(&self) -> usize;

    fn get_from(&self) -> usize;

    fn get_to(&self) -> usize;

    fn other(&self, v: usize) -> usize {
        self.get_from() ^ self.get_to() ^ v
    }
}

pub trait UniEdge {}
pub trait BiEdge {}

struct Graph<'a, E>
where
    E: Edge,
{
    vertices: HashSet<usize>,
    edges: HashMap<usize, Vec<&'a E>>,
    edge_count: usize,
}

impl<'a, E: Edge> Graph<'a, E> {
    pub fn new() -> Self {
        Self {
            vertices: HashSet::new(),
            edges: HashMap::new(),
            edge_count: 0,
        }
    }
}

impl<'a, E> Graph<'a, E>
where
    E: Edge + UniEdge,
{
    pub fn add_unidirected_edge(&mut self, edge: &'a E) -> &mut Self {
        self.edges
            .entry(edge.get_from())
            .or_insert_with(|| vec![edge])
            .push(edge);

        self.edge_count += 1;
        self
    }
}

impl<'a, E> Graph<'a, E>
where
    E: Edge + BiEdge,
{
    pub fn add_edge(&mut self, edge: &'a E) -> &mut Self {
        self.edges
            .entry(edge.get_from())
            .or_insert_with(|| vec![edge])
            .push(edge);

        self.edges
            .entry(edge.get_to())
            .or_insert_with(|| vec![edge])
            .push(edge);

        self.edge_count += 1;
        self
    }
}

pub struct DfsTree<'a, E>
where
    E: Edge,
{
    graph: &'a Graph<'a, E>,
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Edge {
        from: usize,
        to: usize,
        weight: usize,
        index: usize,
    }

    impl super::UniEdge for Edge {}

    impl super::Edge for Edge {
        fn get_id(&self) -> usize {
            self.index
        }

        fn get_from(&self) -> usize {
            self.from
        }

        fn get_to(&self) -> usize {
            self.to
        }
    }

    #[test]
    fn test1() {
        let mut g = Graph::<Edge>::new();
        g.add_unidirected_edge(&Edge {
            from: 0,
            to: 10,
            weight: 1,
            index: 10,
        });
    }
}
