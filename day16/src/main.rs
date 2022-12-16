use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Node {
    rate: usize,
    tunnels: Vec<String>,
}

impl Node {
    fn new(rate: usize, tunnels: Vec<String>) -> Node {
        Node { rate, tunnels }
    }
}

fn main() {
    let nodes: HashMap<&str, Node> = include_str!("input2.txt")
        .lines()
        .map(|line| {
            let mut halves = line.split("; ");

            let mut name_rate = halves
                .next()
                .unwrap()
                .trim_start_matches("Valve ")
                .split(" has flow rate=");
            let name = name_rate.next().unwrap();
            let rate = name_rate.next().unwrap().parse::<usize>().unwrap();

            let tunnels = halves
                .next()
                .unwrap()
                .trim_start_matches("tunnels lead to ")
                .trim_start_matches("tunnel leads to ")
                .trim_start_matches("valves ")
                .trim_start_matches("valve ")
                .split(", ")
                .map(|s| s.into())
                .collect::<Vec<_>>();

            (name, Node::new(rate, tunnels))
        })
        .collect();

    let p = get_max_pressure(&nodes, &mut HashMap::new(), 30, "AA", HashSet::new());
    println!("Pressure: {p}");
}

fn get_max_pressure(
    nodes: &HashMap<&str, Node>,
    cache: &mut HashMap<String, usize>,
    time: usize,
    current: &str,
    opened: HashSet<&str>,
) -> usize {
    // Create cache key
    let mut cache_key: String = format!("{current}|{time}|").into();
    let mut temp_opened = opened.iter().collect::<Vec<_>>();
    temp_opened.sort();
    temp_opened
        .iter()
        .for_each(|n| cache_key += format!(" {n}").as_str());

    // early return
    if cache.contains_key(&cache_key) {
        return *cache.get(&cache_key).unwrap();
    }
    if time == 0 {
        return 0;
    }

    let current_node = nodes.get(current).unwrap();
    let mut all_max_pressure = vec![];

    // If open current valve
    if !opened.contains(&current) && current_node.rate != 0 && time > 1 {
        let p = current_node.rate * (time - 1);

        let mut cur_opened = opened.clone();
        cur_opened.insert(current);

        let ps = current_node
            .tunnels
            .iter()
            .map(|t| p + get_max_pressure(nodes, cache, time - 2, t, cur_opened.clone()))
            .collect::<Vec<_>>();

        all_max_pressure.extend(ps);
    }
    // If skip current valve
    let temp = current_node
        .tunnels
        .iter()
        .map(|t| get_max_pressure(nodes, cache, time - 1, t, opened.clone()))
        .collect::<Vec<_>>();
    all_max_pressure.extend(temp);

    // Get max pressure
    let max = *all_max_pressure.iter().max().unwrap();

    println!("Storing result for {cache_key}: {max}");
    cache.insert(cache_key, max);

    max
}
