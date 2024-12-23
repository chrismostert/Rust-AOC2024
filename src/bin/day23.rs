use std::collections::HashSet;

use aoc_2024::graph::Graph;

fn max_clique(graph: &Graph) -> Vec<String> {
    let mut prev_cliques: HashSet<Vec<String>> = HashSet::new();
    let mut cliques: HashSet<Vec<String>> =
        graph.nodes.iter().map(|node| vec![node.clone()]).collect();
    let mut len = 1;

    loop {
        if len == 3 {
            println!(
                "Part 1: {}",
                cliques
                    .iter()
                    .filter(|clique| { clique.iter().any(|node| &node[0..=0] == "t") })
                    .count()
            );
        }
        if cliques.is_empty() {
            return prev_cliques.into_iter().next().unwrap();
        }
        let mut new_cliques: HashSet<Vec<String>> = HashSet::new();
        for clique in cliques.iter() {
            for out in &graph.edges[clique.last().unwrap()] {
                if !clique.contains(out)
                    && clique.iter().all(|node| graph.edges[out].contains(node))
                {
                    let mut n = clique.clone();
                    n.push(out.clone());
                    n.sort();
                    new_cliques.insert(n);
                }
            }
        }
        prev_cliques = cliques;
        cliques = new_cliques;
        len += 1;
    }
}

fn main() {
    let graph = include_str!("../../inputs/day23.txt")
        .parse::<Graph>()
        .unwrap();

    let p2 = max_clique(&graph).join(",");

    println!("Part 2: {}", p2);
}
