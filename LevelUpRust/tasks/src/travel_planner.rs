use std::collections::{HashMap, HashSet, VecDeque};

pub fn shortest_path(
    edges: Vec<(usize, usize, usize)>,
    start: usize,
    goal: usize,
) -> Option<HashMap<usize, Vec<usize>>> {
    let mut graph: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    let mut path:HashMap<usize, Vec<usize>> = HashMap::new();
    let mut visited = HashSet::from([start]);

    edges.iter().for_each(|(a, b, cost)| {
        graph
            .entry(*a)
            .and_modify(|k| k.push((*b, *cost)))
            .or_insert(vec![(*b, *cost)]);
    });
    let mut queue = VecDeque::from([(start, 0)]);

    while queue.len() > 0 {
        let (node, cost) = queue.remove(0).unwrap();
        graph.get(&node).unwrap().iter().for_each(|(a, c)| {
            if !visited.contains(a) {
                if *a == goal {
                    path.entry(cost + c)
                        .or_insert(queue.iter().map(|node| node.0).collect());
                } 
                visited.insert(*a);
                queue.push_back((*a, cost + c));
                
            }
        });
    }
    if path.keys().len() > 0{
        return Some(path)
    }
    None
}
