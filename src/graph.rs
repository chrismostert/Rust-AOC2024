use std::{collections::{HashMap, HashSet}, str::FromStr};

#[derive(Debug)]
pub struct Graph {
    pub nodes: HashSet<String>,
    pub edges: HashMap<String, HashSet<String>>,
}

// Todo error handling
impl FromStr for Graph {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph: Graph = Graph {
            nodes: HashSet::new(),
            edges: HashMap::new(),
        };

        s.lines()
            .map(|line| {
                let (from, to) = line.split_once("-").unwrap();
                (from, to)
            })
            .for_each(|(from, to)| {
                graph.nodes.insert(from.into());
                graph.nodes.insert(to.into());
                graph
                    .edges
                    .entry(from.into())
                    .or_default()
                    .insert(to.into());
                graph
                    .edges
                    .entry(to.into())
                    .or_default()
                    .insert(from.into());
            });

        Ok(graph)
    }
}