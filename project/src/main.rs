use std::{collections::{HashMap, HashSet}, clone};
use dashmap::DashSet;
use dashmap::DashMap;
use rayon::prelude::*;
use rand::{Rng, SeedableRng};

//input a vertex and return all its neighbours
fn nbrs(vertex: usize, graph: &HashMap<usize, HashSet<usize>>) -> HashSet<usize> {
    graph.get(&vertex).unwrap().clone()
}

fn bfs(source: usize, graph: HashMap<usize, HashSet<usize>>) -> (HashMap<usize,usize>,HashMap<usize,usize>) {
    let frontier: HashSet<usize> = HashSet::from([source]); 
    let parent: HashMap<usize,usize> = HashMap::from([(source, source)]);
    let visited: HashSet<usize> = HashSet::new(); //Visited nodes
    let distance: HashMap<usize, usize> = HashMap::new(); //Hashmap of each vertices and its distances from the source 
    fn expand(frontier: HashSet<usize>, graph: &HashMap<usize, HashSet<usize>>, parent: HashMap<usize,usize>, visited: HashSet<usize>) -> (HashSet<usize>, HashMap<usize,usize>, HashSet<usize>) {
        let mut new_frontier: HashSet<usize> = HashSet::new();
        let mut new_parent: HashMap<usize, usize> = HashMap::new();
        let mut new_visited: HashSet<usize> = HashSet::new();

        new_visited.extend(visited);
        new_parent.extend(parent);
        frontier.iter().for_each(|&v| {
            let neighbours = nbrs(v, graph);
            new_visited.insert(v);
            for n in neighbours {
                if !new_visited.contains(&n) {
                    new_parent.insert(n, v);
                    new_frontier.insert(n);
                }
            }
        });
        for v in frontier {
            new_frontier.remove(&v);
        }
        (new_frontier, new_parent, new_visited)
    }
    fn iterate(graph: &HashMap<usize, HashSet<usize>>, frontier: HashSet<usize>, parent: HashMap<usize,usize>, distance: HashMap<usize, usize>, d: usize, visited: HashSet<usize>) -> (HashMap<usize,usize>, HashMap<usize, usize>) {
        if frontier.is_empty() {
            (parent, distance)
        }
        else {
            let (new_frontier, new_parent, new_visited) = expand(frontier, graph, parent,visited);
            let mut new_distance: HashMap<usize, usize> = HashMap::new();
            new_distance.extend(distance);
            new_frontier.iter().for_each(|&v| {
                new_distance.insert(v,d+1);
            });
            iterate(&graph, new_frontier, new_parent,new_distance, d+1, new_visited)
        }
    }
    iterate(&graph, frontier, parent, distance, 0, visited)
}

//bfs return hashmap of parent and distance
fn par_bfs(source: usize, graph: HashMap<usize, HashSet<usize>>) -> (HashMap<usize,usize>,HashMap<usize,usize>) {
    let frontier: HashSet<usize> = HashSet::from([source]); 
    let parent: HashMap<usize,usize> = HashMap::from([(source, source)]);
    let visited: HashSet<usize> = HashSet::new(); //Visited nodes
    let distance: HashMap<usize, usize> = HashMap::new(); //Hashmap of each vertices and its distances from the source 
    fn expand(frontier: HashSet<usize>, graph: &HashMap<usize, HashSet<usize>>, parent: HashMap<usize,usize>, visited: HashSet<usize>) -> (HashSet<usize>, HashMap<usize,usize>, HashSet<usize>) {
        let new_frontier: DashSet<usize> = DashSet::new();
        let mut new_visited: DashSet<usize> = DashSet::new();
        let mut new_parent = DashMap::new();
        new_visited.extend(visited);
        new_parent.extend(parent);
        
        frontier.par_iter().for_each(|&v| {
            new_visited.insert(v);
            let neighbours = nbrs(v, graph);
            for &n in neighbours.iter() {
                if !new_visited.contains(&n) {
                    new_parent.insert(n,v);
                    new_frontier.insert(n);
                }
            }
        });

        for v in frontier {
            new_frontier.remove(&v);
        }

        let mut n_frontier: HashSet<usize> = HashSet::new();
        for n in new_frontier.iter() {
            n_frontier.insert(*n.key());
        }
        let mut n_parent: HashMap<usize,usize> = HashMap::new();
        for n in new_parent.iter() {
            n_parent.insert(*n.key(), *n.value());
        }
        let mut n_visited: HashSet<usize> = HashSet::new();
        for n in new_visited.iter() {
            n_visited.insert(*n.key());
        }
        
        (n_frontier, n_parent, n_visited)
    }
    fn iterate(graph: &HashMap<usize, HashSet<usize>>, frontier: HashSet<usize>, parent: HashMap<usize,usize>, distance: HashMap<usize, usize>, d: usize, visited: HashSet<usize>) -> (HashMap<usize,usize>, HashMap<usize, usize>) {
        if frontier.is_empty() {
            (parent, distance)
        }
        else {
            let (new_frontier, new_parent, new_visited) = expand(frontier, graph, parent,visited);
            let mut new_distance: HashMap<usize, usize> = HashMap::new();
            new_distance.extend(distance);
            new_frontier.iter().for_each(|&v| {
                new_distance.insert(v,d+1);
            });
            iterate(&graph, new_frontier, new_parent,new_distance, d+1, new_visited)
        }
    }
    iterate(&graph, frontier, parent, distance, 0, visited)
}


fn rand_range(start: usize, end: usize) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(start..end)
}

fn gen_hashset(n: usize) -> HashSet<usize> {
    use rand::Rng;
    use rand::distributions::Standard;
    let mut o: HashSet<usize> = HashSet::new();
    let rng = rand_range(1, n+1);
    (1..=rng).for_each(|num| {
        o.insert(num);
    });
    o
}

fn create_graph(nodes: usize) -> HashMap<usize,HashSet<usize>> {
    let mut graph: HashMap<usize,HashSet<usize>> = HashMap::new();
    for i in 1..nodes {
        let num = rand_range(1, nodes+1);
        graph.insert(i, gen_hashset(num));
    }
    graph
}

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

    let mut graph2: HashMap<usize, HashSet<usize>> = HashMap::new();
    graph2.insert(1, HashSet::from([2,3,4,5]));
    graph2.insert(2, HashSet::from([1,12]));
    graph2.insert(3, HashSet::from([1,11]));
    graph2.insert(4, HashSet::from([1,7]));
    graph2.insert(5, HashSet::from([1,6]));
    graph2.insert(6, HashSet::from([5,8]));
    graph2.insert(7, HashSet::from([4,8,10]));
    graph2.insert(8, HashSet::from([6,7,9]));
    graph2.insert(9, HashSet::from([8,10]));
    graph2.insert(10, HashSet::from([7,9]));
    graph2.insert(11, HashSet::from([3,12,13]));
    graph2.insert(12, HashSet::from([2,11]));
    graph2.insert(13, HashSet::from([11,14]));
    graph2.insert(14, HashSet::from([13,15]));
    graph2.insert(15, HashSet::from([14]));

    let random_graph = create_graph(20);

    let vertex: usize = 1;
    // println!("Neighbours of Vertex: {} are {:?}", vertex, nbrs(vertex, &graph))

    use std::time::Instant;
    let now = Instant::now();

    // Code block to measure.
    {
        let (parent, distance) = bfs(vertex, random_graph.clone());
        let elapseds = now.elapsed();
        println!("normal_bfs Elapsed: {:.2?}", elapseds);

        println!("Parent Map");
        for (key, value) in parent.iter() {
            println!("{} -> {}", key, value);
        }
        println!("Distance Map");
        for (keys, values) in distance.iter() {
            println!("{} -> {}", keys, values);
        }
    }
    
    let n = Instant::now();
    {
        let (parent, distance) = par_bfs(vertex, random_graph.clone());
        let elapsed = n.elapsed();
        println!("par_bfs Elapsed: {:.2?}", elapsed);

        println!("Parent Map");
        for (key, value) in parent.iter() {
            println!("{} -> {}", key, value);
        }
        println!("Distance Map");
        for (keys, values) in distance.iter() {
            println!("{} -> {}", keys, values);
        }
    }
    
}
