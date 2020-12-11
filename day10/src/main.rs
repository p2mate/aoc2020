use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::{
    collections::{HashMap, HashSet},
};

struct ReachabilityGraph {
    graph: HashMap<i32, Vec<i32>>
}

impl ReachabilityGraph {
    fn from_input(sorted_input: &Vec<i32>) -> Self {
        let graph = sorted_input
        .iter()
        .enumerate()
        .filter_map(|e| {
            let end = sorted_input[e.0..]
                .iter()
                .enumerate()
                .take_while(|(_, x)| **x - e.1 <= 3)
                .map(|e2| e2.0)
                .last()
                .unwrap();

            Some((*e.1, sorted_input[e.0 + 1..e.0 + end + 1].to_vec()))
        })
        .collect::<HashMap<_, _>>();
        ReachabilityGraph { graph }
    }

    fn get_reachability(&self, target: i32) -> Vec<i32> {
        self.graph.get(&target).unwrap().to_vec()
    }
}

fn read_input<T: AsRef<Path>>(name: T) -> io::Result<Vec<String>> {
    BufReader::new(std::fs::File::open(name)?).lines().collect()
}

fn day10_1(input: &[i32], target: i32) -> (i32, i32) {
    let mut current = 0;
    let mut jolt1_diff = 0;
    let mut jolt3_diff = 0;
    let mut used = HashSet::new();
    while target != current {
        let adapter_jolt = input
            .iter()
            .filter(|n| (**n - current) <= 3 && !used.contains(*n))
            .min()
            .unwrap();
        used.insert(adapter_jolt);
        match adapter_jolt - current {
            1 => jolt1_diff += 1,
            3 => jolt3_diff += 1,
            _ => {}
        }
        current = *adapter_jolt;
    }
    jolt3_diff += 1;
    (jolt1_diff, jolt3_diff)
}

fn count_paths(
    start: i32,
    target: i32,
    cache: &mut HashMap<i32, usize>,
    graph: &ReachabilityGraph,
) -> usize {
    if let Some(result) = cache.get(&target) {
        return *result;
    }

    if start == target {
        return 1;
    }

    let total = graph.get_reachability(target)
        .iter()
        .map(|e| count_paths(start, *e, cache, graph))
        .sum();
    cache.insert(target, total);

    total
}

fn day10_2(input: &[i32], target: i32) -> usize {
    let mut sorted_input = Vec::from(input);
    sorted_input.push(0);
    sorted_input.sort();

    let graph = ReachabilityGraph::from_input(&sorted_input);
    let mut cache = HashMap::new();

    count_paths(target, sorted_input[0] , &mut cache, &graph)
}
fn main() -> io::Result<()> {
    let name = std::env::args().skip(1).next().unwrap();
    let numbers = read_input(name)?
        .iter()
        .map(|l| l.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let target_jolts = *numbers.iter().max().unwrap();
    let day10_1 = day10_1(&numbers, target_jolts);
    println!(
        "1 jolt difference: {}, 3 jolt difference: {} product: {}",
        day10_1.0,
        day10_1.1,
        day10_1.0 * day10_1.1
    );
    println!(
        "possible combinations: {}",
        day10_2(&numbers, target_jolts)
    );
    Ok(())
}
