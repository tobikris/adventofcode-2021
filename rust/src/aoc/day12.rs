use crate::aoc::input;

use petgraph::prelude::*;
use std::collections::BTreeMap;

pub fn main(day: usize) {
    println!("...1: {}", challenge1(day));
    println!("...2: {}", challenge2(day));
}

pub fn challenge1(day: usize) -> String {
    let read = input::read_file(day, 1);
    let unique_paths = count_unique_paths(read, lowercase_only_once);
    format!(
        "Total unique paths visiting small caves once: {}",
        unique_paths
    )
}

pub fn challenge2(day: usize) -> String {
    let read = input::read_file(day, 1);
    let unique_paths = count_unique_paths(read, lowercase_only_one_twice);
    format!(
        "Total unique paths visiting small caves twice: {}",
        unique_paths
    )
}

pub fn lowercase_only_once(
    graph: &Graph<&str, i32, Undirected>,
    path: &Vec<NodeIndex>,
    n: NodeIndex,
) -> bool {
    graph[n].to_lowercase().eq(graph[n]) && path.contains(&n)
}

pub fn lowercase_only_one_twice(
    graph: &Graph<&str, i32, Undirected>,
    path: &Vec<NodeIndex>,
    n: NodeIndex,
) -> bool {
    if graph[n].eq("start") || graph[n].eq("end") {
        return true;
    }
    if graph[n].to_lowercase().eq(graph[n]) {
        if !path.contains(&n) {
            return false;
        }
        let mut counts = BTreeMap::new();
        for cave in path.iter().map(|x| graph[*x]) {
            if cave.to_lowercase().eq(cave) {
                *counts.entry(cave).or_insert(0) += 1;
            }
        }

        let max = counts
            .clone()
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .unwrap()
            .0;
        return counts[max] >= 2;
    }
    false
}

pub fn count_unique_paths(
    read: String,
    exclude: fn(graph: &Graph<&str, i32, Undirected>, path: &Vec<NodeIndex>, n: NodeIndex) -> bool,
) -> usize {
    let mut graph = Graph::<&str, i32, Undirected>::new_undirected();

    for line in read.lines() {
        let left_name = line.split("-").nth(0).unwrap();
        let right_name = line.split("-").nth(1).unwrap();
        let left = graph
            .node_indices()
            .find(|i| graph[*i] == left_name)
            .unwrap_or(graph.add_node(left_name));
        let right = graph
            .node_indices()
            .find(|i| graph[*i] == right_name)
            .unwrap_or(graph.add_node(right_name));
        graph.add_edge(left, right, 1);
    }

    let start = graph.node_indices().find(|i| graph[*i] == "start").unwrap();
    let end = graph.node_indices().find(|i| graph[*i] == "end").unwrap();

    let paths = all_paths(&graph, start, end, &mut vec![start], exclude);

    paths.len()
}

pub fn all_paths(
    graph: &Graph<&str, i32, Undirected>,
    start: NodeIndex,
    end: NodeIndex,
    path: &mut Vec<NodeIndex>,
    exclude: fn(graph: &Graph<&str, i32, Undirected>, path: &Vec<NodeIndex>, n: NodeIndex) -> bool,
) -> Vec<Vec<NodeIndex>> {
    let mut paths = vec![];
    for n in graph.neighbors(start) {
        if n == end {
            let mut path = path.clone();
            path.push(n);
            paths.push(path);
        } else if path.len() < 50 {
            if exclude(&graph, &path, n) {
                continue;
            }
            let mut path = path.clone();
            path.push(n);
            paths.append(&mut all_paths(graph, n, end, &mut path, exclude));
        }
    }
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn challenge1() {
        let read = input::read_file(12, 0);
        let unique_paths = count_unique_paths(read, lowercase_only_once);
        assert_eq!(unique_paths, 19);
    }

    #[test]
    fn challenge2() {
        let read = input::read_file(12, 0);
        let unique_paths = count_unique_paths(read, lowercase_only_one_twice);
        assert_eq!(unique_paths, 103);
    }
}
