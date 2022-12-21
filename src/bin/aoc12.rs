use std::{fmt::Display, fs};

type Coord = (usize, usize);

struct Node {
    coord: Coord,
    val: char,
    visited: bool,
    distance: i32,
    prev: Option<Coord>,
    next: Option<Coord>,
    connected: Vec<Coord>,
}

impl Display for Node {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("Node {},{} = {}", self.coord.0, self.coord.1, self.val);
        println!("\tDistance: {}", self.distance);
        println!("\tConnected: {:?}", self.connected);
        println!("\tPrev: {:?}", self.prev);
        Ok(())
    }
}

impl Node {
    fn new(coord: Coord, val: char) -> Node {
        Node {
            coord,
            val,
            visited: false,
            distance: i32::MAX,
            prev: None,
            next: None,
            connected: Vec::new(),
        }
    }

    fn reset(&mut self) {
        self.visited = false;
        self.distance = i32::MAX;
        self.prev = None;
        self.next = None;
    }

    fn try_add(&mut self, coord: Coord, c: char) {
        let self_val = self.val as i32;
        let other_val = c as i32;

        if self.val == 'S' {
            if c == 'a' {
                self.connected.push(coord);
            }
            return;
        }

        if self.val == 'E' || c == 'S' {
            return;
        }

        if c == 'E' {
            if self.val == 'z' {
                self.connected.push(coord);
            }
            return;
        }

        if other_val - self_val <= 1 {
            self.connected.push(coord);
        }
    }

    fn visit(&mut self) {
        self.visited = true;
    }
}

struct Graph {
    head: Coord,
    goal: Coord,
    nodes: Vec<Vec<Node>>,
}

impl Display for Graph {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("Graph, Head: {:?}, Tail: {:?}", self.head, self.goal);
        for i in &self.nodes {
            for node in i {
                println!("{node}");
            }
        }

        Ok(())
    }
}

impl Graph {
    pub fn from(s: &str) -> Graph {
        // Create 2d table to make it easier to build graph
        let graph2d = s
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut nodes: Vec<Vec<Node>> = Vec::new();
        let mut head = (0, 0);
        let mut goal = (0, 0);
        for i in graph2d.iter().enumerate() {
            let mut row = Vec::new();
            for j in i.1.iter().enumerate() {
                let mut node = Node::new((i.0, j.0), *j.1);

                // Check node above
                if i.0 != 0 {
                    node.try_add((i.0 - 1, j.0), graph2d[i.0 - 1][j.0]);
                }

                // Check node below
                if i.0 != graph2d.len() - 1 {
                    node.try_add((i.0 + 1, j.0), graph2d[i.0 + 1][j.0]);
                }

                // Check node to the left
                if j.0 != 0 {
                    node.try_add((i.0, j.0 - 1), graph2d[i.0][j.0 - 1]);
                }

                // Check node to the right
                if j.0 != graph2d[0].len() - 1 {
                    node.try_add((i.0, j.0 + 1), graph2d[i.0][j.0 + 1]);
                }

                if graph2d[i.0][j.0] == 'S' {
                    head = (i.0, j.0)
                }

                if graph2d[i.0][j.0] == 'E' {
                    goal = (i.0, j.0)
                }

                row.push(node);
            }
            nodes.push(row);
        }

        Graph { head, goal, nodes }
    }

    fn node_mut(&mut self, coord: Coord) -> &mut Node {
        &mut self.nodes[coord.0][coord.1]
    }

    fn node(&self, coord: Coord) -> &Node {
        &self.nodes[coord.0][coord.1]
    }

    fn shortest_path(&mut self, start_node: Coord) -> i32 {
        // Reset everything
        for node in self.nodes.iter_mut().flatten() {
            node.reset();
        }

        self.node_mut(start_node).distance = 0;

        let mut to_visit = Vec::new();
        let mut coord = start_node;

        while coord != self.goal {
            if self.node(coord).visited {
                continue;
            }
            self.node_mut(coord).visit();

            let path_len = self.node(coord).distance + 1;
            for i in 0..self.node(coord).connected.len() {
                let connected_coord = self.node(coord).connected[i];

                if self.node(connected_coord).visited {
                    continue;
                }

                if path_len < self.node(connected_coord).distance {
                    self.node_mut(connected_coord).distance = path_len;
                    self.node_mut(connected_coord).prev = Some(coord);
                    to_visit.push(connected_coord);
                }
            }

            to_visit.sort_by(|a: &(usize, usize), b| {
                self.node(*a)
                    .distance
                    .cmp(&self.node(*b).distance)
                    .reverse()
            });
            let next = to_visit.pop();

            if next.is_none() {
                return i32::MAX;
            }

            coord = next.unwrap();
        }

        self.node(self.goal).distance
    }

    fn print_path(&self) {
        let mut coord = self.goal;
        let mut line_len = 0;
        loop {
            line_len += 1;
            if line_len == 10 {
                println!("");
                line_len = 0;
            }

            let node = self.node(coord);

            if node.val != 'S' {
                print!("{} ({}) <- ", node.val, node.distance);
            } else {
                println!("{}", node.val);
                break;
            }

            coord = node.prev.unwrap();
        }
    }

    fn print_grid(&mut self, start_node: Coord) {
        if self.node(start_node).next.is_some() {
            return;
        }

        let mut coord = self.goal;

        while coord != start_node {
            self.node_mut(self.node(coord).prev.unwrap()).next = Some(coord);

            coord = self.node(coord).prev.unwrap();
        }

        for row in &self.nodes {
            for node in row {
                let mut c = '.';

                if let Some(x) = node.next {
                    if x.0 < node.coord.0 {
                        c = '^';
                    } else if x.0 > node.coord.0 {
                        c = 'V';
                    } else if x.1 < node.coord.1 {
                        c = '<';
                    } else {
                        c = '>';
                    }
                }

                if node.val == 'E' {
                    c = 'E';
                }

                print!("{c}");
            }
            println!("");
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to open file");
    let mut graph = Graph::from(&input);
    let mut shortest_path = graph.shortest_path(graph.head);
    //graph.print_path();
    //graph.print_grid(graph.head);
    println!("Part 1: {shortest_path}");

    for i in 0..graph.nodes.len() {
        for j in 0..graph.nodes[0].len() {
            let coord = (i, j);
            if graph.node(coord).val != 'a' {
                continue;
            }

            let path_len = graph.shortest_path(coord);

            if path_len < shortest_path {
                shortest_path = path_len;
            }
        }
    }
    println!("Part 2: {shortest_path}");
}
