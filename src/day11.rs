use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

struct Graph {
    nodes: HashSet<Rc<String>>,
    out_edges: HashMap<Rc<String>, Vec<Rc<String>>>,
    // in_edges: HashMap<Rc<String>, Vec<Rc<String>>>,
}

impl Graph {
    fn new() -> Self {
        Self {
            nodes: HashSet::new(),
            out_edges: HashMap::new(), // from node to dest
                                       // in_edges: HashMap::new(),
        }
    }

    fn add_node(&mut self, node: String, edges: Vec<String>) {
        let node = Rc::new(node);
        self.nodes.insert(Rc::clone(&node));

        // let in_edges = self.in_edges.entry(Rc::clone(&node)).or_insert(Vec::new());
        let out_edges = self.out_edges.entry(Rc::clone(&node)).or_insert(Vec::new());

        for edge in edges {
            let edge_rc = Rc::new(edge.clone());
            if !self.nodes.contains(&edge_rc) {
                self.nodes.insert(edge_rc.clone());
                // in_edges.push(edge_rc.clone());
                out_edges.push(edge_rc.clone());
            } else {
                // in_edges.push(edge_rc.clone());
                out_edges.push(edge_rc.clone());
            }
        }
    }

    #[allow(dead_code)]
    fn print_graph(&self) {
        for node in self.nodes.iter() {
            println!("Node: {}", node);
        }

        for (node, edges) in self.out_edges.iter() {
            println!("{}: {:?}", node, edges);
        }
    }
}

#[derive(Clone)]
struct Edge {
    from: Rc<String>,
    to: Rc<String>,
}

impl Edge {
    fn new(from: Rc<String>, to: Rc<String>) -> Self {
        Self { from, to }
    }
}

#[derive(Clone)]
struct CurrentPath {
    path: Vec<Edge>,
}

impl CurrentPath {
    fn new() -> Self {
        Self { path: Vec::new() }
    }

    fn add_edge(&mut self, edge: Edge) {
        self.path.push(edge);
    }

    fn is_valid_edge(&self, edge: &Edge) -> bool {
        !self.path.iter().any(|e| e.from == edge.from && e.to == edge.to)
    }

    #[allow(dead_code)]
    fn print_path(&self) {
        for edge in self.path.iter() {
            println!("{} -> {}", edge.from, edge.to);
        }
    }
}

#[allow(dead_code)]
fn get_num_paths(graph: &Graph, curr_node: Rc<String>, curr_path: CurrentPath) -> i64 {
    if curr_node == Rc::new("out".to_string()) {
        return 1;
    }

    match graph.out_edges.get(&curr_node) {
        Some(out_edges) => {
            let mut num_paths = 0;
            for out_node in out_edges.iter() {
                if curr_path.is_valid_edge(&Edge::new(Rc::clone(&curr_node), Rc::clone(out_node))) {
                    let mut new_path = curr_path.clone();
                    new_path.add_edge(Edge::new(Rc::clone(&curr_node), Rc::clone(out_node)));
                    num_paths += get_num_paths(graph, Rc::clone(out_node), new_path);
                }
            }
            return num_paths;
        }
        None => {
            return 0;
        }
    }
}

pub fn part1(input: &str) -> i64 {
    let mut graph = Graph::new();
    input
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .for_each(|(node, edges)| {
            graph.add_node(
                node.to_string(),
                edges.split(" ").map(|edge| edge.to_string()).collect(),
            );
        });

    let curr_path = CurrentPath::new();
    get_num_paths(&graph, Rc::new("you".to_string()), curr_path)
}

#[allow(dead_code)]
fn get_num_paths2(graph: &Graph, curr_node: Rc<String>, curr_path: CurrentPath) -> i64 {
    if curr_node == Rc::new("out".to_string()) {
        let has_dac = curr_path.path.iter().any(|e| e.to == Rc::new("dac".to_string()));
        let has_fft = curr_path.path.iter().any(|e| e.to == Rc::new("fft".to_string()));
        if has_dac && has_fft {
            return 1;
        }
        return 0;
    }

    match graph.out_edges.get(&curr_node) {
        Some(out_edges) => {
            let mut num_paths = 0;
            for out_node in out_edges.iter() {
                if curr_path.is_valid_edge(&Edge::new(Rc::clone(&curr_node), Rc::clone(out_node))) {
                    let mut new_path = curr_path.clone();
                    new_path.add_edge(Edge::new(Rc::clone(&curr_node), Rc::clone(out_node)));
                    num_paths += get_num_paths2(graph, Rc::clone(out_node), new_path);
                }
            }
            return num_paths;
        }
        None => {
            return 0;
        }
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
struct MemoiKey {
    curr_node: Rc<String>,
    has_dac: bool,
    has_fft: bool,
}

impl MemoiKey {
    fn new(curr_node: Rc<String>, has_dac: bool, has_fft: bool) -> Self {
        Self {
            curr_node,
            has_dac,
            has_fft,
        }
    }
}

fn get_num_paths2_memoi(
    graph: &Graph,
    curr_node: Rc<String>,
    curr_path: CurrentPath,
    memoi: &mut HashMap<MemoiKey, i64>,
) -> i64 {
    let has_dac: bool = curr_path.path.iter().any(|e| e.to == Rc::new("dac".to_string()));
    let has_fft: bool = curr_path.path.iter().any(|e| e.to == Rc::new("fft".to_string()));
    let memoi_key = MemoiKey::new(curr_node.clone(), has_dac, has_fft);
    if let Some(&cached) = memoi.get(&memoi_key) {
        return cached;
    }

    if curr_node == Rc::new("out".to_string()) {
        let result = if has_dac && has_fft { 1 } else { 0 };
        memoi.insert(memoi_key, result);
        return result;
    }

    match graph.out_edges.get(&curr_node) {
        Some(out_edges) => {
            let mut num_paths = 0;
            for out_node in out_edges.iter() {
                if curr_path.is_valid_edge(&Edge::new(Rc::clone(&curr_node), Rc::clone(out_node))) {
                    let mut new_path = curr_path.clone();
                    new_path.add_edge(Edge::new(Rc::clone(&curr_node), Rc::clone(out_node)));
                    num_paths += get_num_paths2_memoi(graph, Rc::clone(out_node), new_path, memoi);
                }
            }
            memoi.insert(memoi_key, num_paths);
            return num_paths;
        }
        None => {
            memoi.insert(memoi_key, 0);
            return 0;
        }
    }
}

pub fn part2(input: &str) -> i64 {
    let mut graph = Graph::new();
    input
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .for_each(|(node, edges)| {
            graph.add_node(
                node.to_string(),
                edges.split(" ").map(|edge| edge.to_string()).collect(),
            );
        });

    let curr_path = CurrentPath::new();
    let mut memoi = HashMap::new();
    get_num_paths2_memoi(&graph, Rc::new("svr".to_string()), curr_path, &mut memoi)
}
