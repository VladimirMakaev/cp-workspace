use std::collections::HashMap;

#[derive(Clone)]
struct InnerEdge {
    pub to: usize,
    pub id: usize,
}

pub trait Edge {
    fn get_from(&self) -> usize;
    fn get_to(&self) -> usize;
    fn get_id(&self) -> usize;
}

impl InnerEdge {
    fn new(to: usize, id: usize) -> Self {
        Self { id, to }
    }
}

pub struct Graph {
    edges: HashMap<usize, Vec<InnerEdge>>,
    edge_count: usize,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
            edge_count: 0,
        }
    }

    pub fn add_unidirected_edge<E: Edge>(&mut self, edge: &E) -> &mut Self {
        let inner_edge = InnerEdge::new(edge.get_to(), edge.get_id());
        let reverse_edge = InnerEdge::new(edge.get_from(), edge.get_id());

        self.edges
            .entry(edge.get_from())
            .or_insert_with(|| vec![inner_edge.clone()])
            .push(inner_edge);

        self.edges
            .entry(edge.get_to())
            .or_insert_with(|| vec![reverse_edge.clone()])
            .push(reverse_edge);
        self.edge_count += 1;
        self
    }

    pub fn add_edge(&mut self, from: usize, to: usize) -> &mut Self {
        let inner_edge = InnerEdge::new(to, self.edge_count);
        let reverse_edge = InnerEdge::new(from, self.edge_count);

        self.edges
            .entry(from)
            .or_insert_with(|| vec![inner_edge.clone()])
            .push(inner_edge);

        self.edges
            .entry(to)
            .or_insert_with(|| vec![reverse_edge.clone()])
            .push(reverse_edge);
        self.edge_count += 1;
        self
    }

    pub fn vertex_count(&self) -> usize {
        self.edges.len()
    }

    pub fn build_low_link<I>(&self, vertices: I) -> LowLink
    where
        I: Iterator<Item = usize>,
    {
        LowLink::new(self).search(vertices)
    }
}

pub struct LowLink<'a> {
    counter: usize,
    graph: &'a Graph,
    low_links: HashMap<usize, usize>,
    pre_order: HashMap<usize, usize>,
    bridges: Vec<usize>,
}

impl<'a> LowLink<'a> {
    fn new(g: &'a Graph) -> Self {
        Self {
            counter: 0,
            graph: g,
            low_links: HashMap::new(),
            pre_order: HashMap::new(),
            bridges: vec![],
        }
    }

    fn search<I>(mut self, vertices: I) -> Self
    where
        I: Iterator<Item = usize>,
    {
        for v in vertices {
            if self.pre_order.get(&v).is_none() {
                self.dfs(v, v, 0);
            }
        }
        self
    }

    fn dfs(&mut self, cur: usize, parent: usize, id: usize) -> usize {
        //eprintln!("{0}->{1}", parent, cur);
        if let Some(cur_pre_order) = self.pre_order.get(&cur) {
            return match self
                .low_links
                .entry(parent)
                .and_modify(|link| *link = std::cmp::min(*link, *cur_pre_order))
            {
                std::collections::hash_map::Entry::Occupied(res) => *res.get(),
                std::collections::hash_map::Entry::Vacant(_) => panic!("unreachable"),
            };
        }
        self.counter += 1;
        self.pre_order.insert(cur, self.counter);
        self.low_links.insert(cur, self.counter);

        for edge in self.graph.edges.get(&cur).into_iter().flatten() {
            if edge.id == id {
                continue;
            }

            if self.dfs(edge.to, cur, edge.id) == 0 {
                let to = *self.low_links.get(&edge.to).unwrap();
                self.low_links
                    .entry(cur)
                    .and_modify(|link| *link = std::cmp::min(*link, to));

                if self.low_links[&edge.to] == self.pre_order[&edge.to] {
                    self.bridges.push(edge.id)
                }
            }
        }

        return 0;
    }

    pub fn bridges(&self) -> impl Iterator<Item = &usize> {
        self.bridges.iter()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test0() {
        let mut g = Graph::new();
        g.add_edge(0, 1).add_edge(0, 2).add_edge(1, 2);
        let low_link = g.build_low_link(0..3);
        assert_eq!(low_link.bridges().cloned().collect::<Vec<usize>>(), vec![]);
    }

    #[test]
    fn test1() {
        let mut g = Graph::new();
        g.add_edge(0, 1)
            .add_edge(0, 2)
            .add_edge(1, 2)
            .add_edge(1, 3)
            .add_edge(2, 3)
            .add_edge(1, 4)
            .add_edge(4, 5)
            .add_edge(5, 6)
            .add_edge(6, 4)
            .add_edge(2, 7)
            .add_edge(7, 8)
            .add_edge(7, 9);

        let low_link = g.build_low_link(0..10);
        assert_eq!(
            low_link.bridges().cloned().collect::<Vec<usize>>(),
            vec![5, 10, 11, 9]
        );
    }
}
