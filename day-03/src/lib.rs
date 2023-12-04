
/* I'm not proud of this code.
 *
 * I believe this question has exposed some pretty
 * major holes in my understanding of graph theory,
 * and made me realized I have a lot to learn.
 *
 */

use std::fs;
use std::str::FromStr;
use std::hash::Hash;
use std::collections::HashSet;

pub fn run_all() {
    let file_path = "input/3.txt";
    let input = fs::read_to_string(&file_path).expect("failed to read input file");
    let graph = input.parse::<Graph>().expect("failed to parse input");
    println!("Day 03");
    println!("    Part One: {}", part_one(&graph));
    println!("    Part Two: {}", part_two(&graph));
}

pub fn part_one(graph: &Graph) -> u64 {
    graph.nodes
        .iter()
        .filter(|node| node.has_symbol_neighbor(graph))
        .map(|node| node.to_string(&graph).parse::<u64>().expect("failed to parse int"))
        .sum()
}

pub fn part_two(graph: &Graph) -> u64 {
    let mut sum = 0;
    for (row, line) in graph.chars.iter().enumerate() {
        for (col, x) in line.iter().enumerate() {
            if *x == '*' {
                let all_points = get_matching_neighbors(&graph.chars, row, col, char::is_numeric);

                let points = all_points
                    .iter()
                    .filter_map(|point| graph.get_node_at(point.0, point.1));

                let nodes: Vec<_> = unique(points).collect();

                if nodes.len() == 2 {
                    let a = graph.nodes[nodes[0]].to_string(graph).parse::<u64>().unwrap();
                    let b = graph.nodes[nodes[1]].to_string(graph) .parse::<u64>().unwrap();
                    sum += a * b;
                }
            }
        }
    }
    sum
}

#[derive(Debug)]
pub struct Node {
    points: Vec<(usize, usize)>,
}

impl Node {
    fn new() -> Self {
        Self {
            points: Vec::new(),
        }
    }

    fn has_symbol_neighbor(&self, graph: &Graph) -> bool {
        for point in &self.points {
            if get_matching_neighbors(&graph.chars, point.0, point.1, is_symbol).len() > 0 {
                return true;
            }
        }
        false
    }

    fn to_string(&self, graph: &Graph) -> String {
        let mut result = String::new();
        for point in &self.points {
            result.push(graph.chars[point.0][point.1]);
        }
        result
    }
}

#[derive(Debug)]
pub struct Graph {
    chars: Vec<Vec<char>>,
    nodes: Vec<Node>,
}

impl Graph {
    fn new() -> Self {
        Self {
            chars: Vec::new(),
            nodes: Vec::new(),
        }
    }

    fn get_node_at(&self, row: usize, col: usize) -> Option<usize> {
        for (i, node) in self.nodes.iter().enumerate() {
            for &(r, c) in &node.points {
                if r == row && c == col {
                    return Some(i);
                }
            }
        }
        None
    }
}

fn get_matching_neighbors<F> (grid: &Vec<Vec<char>>, row: usize, col: usize, predicate: F) -> Vec<(usize, usize)>
    where F: Fn(char) -> bool
{
    let rows = grid.len();
    let cols = grid[0].len();

    let mut result = Vec::new();

    for i in 0..3 {
        for j in 0..3 {
            // Skip the center cell (col, row)
            if i == 1 && j == 1 {
                continue;
            }

            let new_row = row as i32 + i - 1;
            let new_col = col as i32 + j - 1;

            // Check boundaries
            if new_row >= 0 && new_row < rows as i32 && new_col >= 0 && new_col < cols as i32 {
                let nbor = grid[new_row as usize][new_col as usize];
                if predicate(nbor) {
                    result.push((new_row as usize, new_col as usize));
                }
            }
        }
    }
    
    result
}

impl FromStr for Graph {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Graph::new();
        for (row, line) in s.lines().map(|line| line.trim().chars().collect::<Vec<char>>()).enumerate() {
            let mut col = 0;
            while col < line.len() {
                if line[col].is_numeric() {
                    let mut node = Node::new();
                    while col < line.len() && line[col].is_numeric() {
                        node.points.push((row, col));
                        col += 1;
                    }
                    result.nodes.push(node);
                } else {
                    col += 1;
                }
            }
            result.chars.push(line);
        }
        Ok(result)
    }
}

fn unique<T, I>(iter: I) -> impl Iterator<Item = T>
where
    T: Hash + Eq + Clone,
    I: Iterator<Item = T>,
{
    let mut seen = HashSet::new();
    iter.filter(move |item| seen.insert(item.clone()))
}

fn is_symbol(x: char) -> bool {
    !x.is_numeric() && x != '.'
}

