use std::{collections::{HashMap, HashSet}};

use rayon::prelude::*;
//use flurry::{HashMap, HashSet};

//input a vertex and return all its neighbours
fn nbrs(vertex: usize, graph: &HashMap<usize, HashSet<usize>>) -> HashSet<usize> {
    graph.get(&vertex).unwrap().clone()
}

//bfs return hashmap of parent and distance
// fn bfs(source: usize, graph: HashMap<usize, HashSet<usize>>) -> (HashMap<usize,usize>,HashMap<usize,usize>) {
//     let frontier: HashSet<usize> = HashSet::from([source]); 
//     let parent: HashMap<usize,usize> = HashMap::from([(source, source)]);
//     let visited: HashSet<usize> = HashSet::new(); //Visited nodes
//     let distance: HashMap<usize, usize> = HashMap::new(); //Hashmap of each vertices and its distances from the source 
//     fn expand(frontier: HashSet<usize>, graph: &HashMap<usize, HashSet<usize>>, parent: HashMap<usize,usize>, visited: HashSet<usize>) -> (HashSet<usize>, HashMap<usize,usize>, HashSet<usize>) {

//         use dashmap::DashSet;
//         use dashmap::DashMap;

//         let mut new_frontier = DashSet::new();
//         let mut new_visited = DashSet::new();
//         let mut new_parent = DashMap::new();
//         new_visited.extend(visited);
//         new_parent.extend(parent);

//         frontier.par_iter().map(|&v| {
//             let neighbours = nbrs(v, graph);
//             new_visited.insert(v);
//             for n in neighbours {
//                 if !new_visited.contains(&n) {
//                     new_parent.insert(n, v);
//                     new_frontier.insert(n);
//                 }
//             }
//         });

//         for v in frontier {
//             new_frontier.remove(&v);
//         }

//         let n_frontier: HashSet<usize> = HashSet::new();
        
//         (new_frontier, new_parent, new_visited)
//     }
//     fn iterate(graph: &HashMap<usize, HashSet<usize>>, frontier: HashSet<usize>, parent: HashMap<usize,usize>, distance: HashMap<usize, usize>, d: usize, visited: HashSet<usize>) -> (HashMap<usize,usize>, HashMap<usize, usize>) {
//         if frontier.is_empty() {
//             (parent, distance)
//         }
//         else {
//             let (new_frontier, new_parent, new_visited) = expand(frontier, graph, parent,visited);
//             let mut new_distance: HashMap<usize, usize> = HashMap::new();
//             new_distance.extend(distance);
//             new_frontier.iter().for_each(|&v| {
//                 new_distance.insert(v,d+1);
//             });
//             iterate(&graph, new_frontier, new_parent,new_distance, d+1, new_visited)
//         }
//     }
//     iterate(&graph, frontier, parent, distance, 0, visited)
// }
fn main() {
    let mut graph: HashMap<usize, HashSet<usize>> = HashMap::new();
    graph.insert(1, HashSet::from([2,3]));
    graph.insert(2, HashSet::from([1]));
    graph.insert(3, HashSet::from([1,4]));
    graph.insert(4, HashSet::from([3]));


    let mut graph1: HashMap<usize, HashSet<usize>> = HashMap::new();
    graph1.insert(1,HashSet::from([2,3]));
    graph1.insert(2,HashSet::from([1,5]));
    graph1.insert(3,HashSet::from([1,4,5]));
    graph1.insert(4, HashSet::from([3,6]));
    graph1.insert(5, HashSet::from([2,3]));
    graph1.insert(6, HashSet::from([4]));

    let vertex: usize = 1;
    // println!("Neighbours of Vertex: {} are {:?}", vertex, nbrs(vertex, &graph))

    // use std::time::Instant;
    // let now = Instant::now();

    // // Code block to measure.
    // {
    //     let (parent, distance) = bfs(vertex, graph);
    //     // println!("Parent Map");
    //     // for (key, value) in parent.iter() {
    //     //     println!("{} -> {}", key, value);
    //     // }
    //     // println!("Distance Map");
    //     // for (keys, values) in distance.iter() {
    //     //     println!("{} -> {}", keys, values);
    //     // }
    // }

    // let elapsed = now.elapsed();
    // println!("Elapsed: {:.2?}", elapsed);

    use dashmap::DashSet;
    use dashmap::DashMap;

    let mut frontier: HashSet<usize> = HashSet::new();
    frontier.insert(1);
    frontier.insert(2);
    let mut new_frontier: DashSet<usize> = DashSet::new();
    new_frontier.extend(frontier.clone());

    println!("{:?}",frontier);
    println!("{:?}", new_frontier);

    let mut parent: HashMap<usize,usize> = HashMap::new();
    parent.insert(1,1);
    parent.insert(2,2);

    let mut new_parent: DashMap<usize,usize> = DashMap::new();
    new_parent.extend(parent.clone());

    for (&key, &value) in parent.iter() {
        println!("{} -> {}", key, value);
    }

    new_parent.iter().for_each(|n| {
        println!("{} -> {}", n.key(), n.value());
    })
    
}
