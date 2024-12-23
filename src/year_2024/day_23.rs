
use fxhash::FxHashMap;

#[derive(Default)]
struct Graph<'a> {
    node_edges: Vec<Vec<usize>>,
    node_names: Vec<&'a str>,
    node_ids: FxHashMap<&'a str, usize>
}

#[derive(Default)]
struct CliqueSearchState {
    nodes: Vec<usize>,
    current_maximum: Vec<usize>
}

impl<'a> Graph<'a> {

    pub fn add_undirected_edge(&mut self, from: &'a str, to: &'a str) {
        let from_index = self.get_node_id(from);
        let to_index = self.get_node_id(to);

        self.node_edges[from_index].push(to_index);
        self.node_edges[to_index].push(from_index);
    }

    pub fn finalize_graph(&mut self) {
        for edges in &mut self.node_edges {
            edges.sort();
        }
    }

    pub fn from_input(input: &'a str) -> Self {
        let mut graph = Graph::default();
        for line in input.lines() {
            let mut parts = line.split("-");
            let left = parts.next().unwrap();
            let right = parts.next().unwrap();
    
            graph.add_undirected_edge(left, right);
        }
    
        graph.finalize_graph();
        graph
    }

    pub fn get_three_clique_count(&self) -> u64 {
        let mut count= 0;
        for starting_node in 0..self.node_ids.len() {
            let edges = &self.node_edges[starting_node];
            for i in 0..edges.len() {
                if edges[i] < starting_node {
                    continue;
                }

                for j in (i + 1)..edges.len() {
                    let first_node = edges[i];
                    let second_node = edges[j];
                    if !self.node_names[starting_node].starts_with("t") &&
                       !self.node_names[first_node].starts_with("t") &&
                       !self.node_names[second_node].starts_with("t") {

                        continue;
                    }

                    if self.node_edges[first_node].binary_search(&second_node).is_ok() {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    pub fn get_maximum_clique(&self) -> String {
        let mut search_state = CliqueSearchState::default();
        for starting_node in 0..self.node_edges.len() {
            self.get_maximum_clique_recurse(starting_node, &mut search_state);
        }

        let mut clique_nodes =
            search_state.current_maximum
                        .into_iter()
                        .map(|node_id| self.node_names[node_id])
                        .collect::<Vec<_>>();

        clique_nodes.sort();
        clique_nodes.join(",")
    }

    fn get_maximum_clique_recurse(&self, current_node: usize, search_state: &mut CliqueSearchState) {
        let edges = &self.node_edges[current_node];
        for existing_edge in &search_state.nodes {
            if edges.binary_search(&existing_edge).is_err() {
                return;
            }
        }
    
        let start_index = edges.binary_search(&current_node).unwrap_err();
        if (edges.len() - start_index + search_state.nodes.len() + 1) <= search_state.current_maximum.len() {
            return;
        }
    
        search_state.nodes.push(current_node);
        for edge_index in 0..edges.len() {
            if edges[edge_index] < current_node {
                continue;
            }

            if (edges.len() - edge_index + search_state.nodes.len()) <= search_state.current_maximum.len() {
                break;
            }

            self.get_maximum_clique_recurse(edges[edge_index], search_state);
        }
    
        if search_state.nodes.len() > search_state.current_maximum.len() {
            search_state.current_maximum = search_state.nodes.clone();
        }
    
        search_state.nodes.pop();
    }

    fn get_node_id(&mut self, node_name: &'a str) -> usize {
        *self.node_ids
             .entry(node_name)
             .or_insert_with(|| {
                self.node_names.push(node_name);
                self.node_edges.push(Vec::default());
                self.node_edges.len() - 1
             })
    }

}

pub fn part1(input: &str) -> u64 {
    Graph::from_input(input).get_three_clique_count()
}

pub fn part2(input: &str) -> String {
    Graph::from_input(input).get_maximum_clique()
}
