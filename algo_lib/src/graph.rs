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
    edges: Vec<Vec<InnerEdge>>,
    edge_count: usize,
    adjacency: Vec<Vec<InnerEdge>>,
}

impl Graph {
    pub fn new(vertex_count: usize) -> Self {
        Self {
            edges: vec![Vec::new(); vertex_count],
            edge_count: 0,
            adjacency: vec![Vec::new(); vertex_count],
        }
    }

    pub fn add_unidirected_edge<E: Edge>(&mut self, edge: &E) -> &mut Self {
        self.adjacency[edge.get_to()].push(InnerEdge::new(edge.get_from(), edge.get_id()));
        self.adjacency[edge.get_from()].push(InnerEdge::new(edge.get_to(), edge.get_id()));
        self.edge_count += 1;
        self
    }

    pub fn add_edge(&mut self, from: usize, to: usize) -> &mut Self {
        self.edges[from].push(InnerEdge::new(to, self.edge_count + 1));
        self.edges[to].push(InnerEdge::new(from, self.edge_count + 1));
        self.edge_count += 1;
        self
    }

    pub fn vertex_count(&self) -> usize {
        self.edges.len()
    }

    pub fn build_low_link(&self) -> LowLink {
        LowLink::new(self).search()
    }
}

pub struct LowLink<'a> {
    counter: usize,
    graph: &'a Graph,
    low_links: Vec<usize>,
    pre_order: Vec<usize>,
    bridges: Vec<usize>,
}

impl<'a> LowLink<'a> {
    fn new(g: &'a Graph) -> Self {
        Self {
            counter: 0,
            graph: g,
            low_links: vec![0; g.edges.len()],
            pre_order: vec![0; g.edges.len()],
            bridges: vec![],
        }
    }

    fn search(mut self) -> Self {
        for v in 0..self.pre_order.len() {
            if self.pre_order[v] == 0 {
                self.dfs(v, v, 0);
            }
        }
        self
    }

    fn dfs(&mut self, cur: usize, parent: usize, id: usize) -> usize {
        eprintln!("dfs: {}->{}", parent, cur);
        if self.pre_order[cur] != 0 {
            self.low_links[parent] = std::cmp::min(self.low_links[parent], self.pre_order[cur]);
            return self.low_links[parent];
        }
        self.counter += 1;
        self.pre_order[cur] = self.counter;
        self.low_links[cur] = self.pre_order[cur];

        for edge in self.graph.edges[cur].iter() {
            if edge.id == id {
                continue;
            }

            if self.dfs(edge.to, cur, edge.id) == 0 {
                self.low_links[cur] = std::cmp::min(self.low_links[cur], self.low_links[edge.to]);

                if self.low_links[edge.to] == self.pre_order[edge.to] {
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
        let mut g = Graph::new(3);
        g.add_edge(0, 1).add_edge(0, 2).add_edge(1, 2);
        let low_link = g.build_low_link();
        let links = (0..g.vertex_count())
            .map(|x| (low_link.pre_order[x], low_link.low_links[x]))
            .collect::<Vec<(usize, usize)>>();
        assert_eq!(links, vec![]);
    }

    #[test]
    fn test1() {
        let mut g = Graph::new(10);
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

        let low_link = g.build_low_link();
        let result = (0..10)
            .map(|x| (low_link.pre_order[x], low_link.low_links[x]))
            .collect::<Vec<(usize, usize)>>();
        assert_eq!(result, vec![]);
    }
}
