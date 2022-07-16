use std::{collections::{HashMap, HashSet}};

//input a vertex and return all its neighbours
fn nbrs(vertex: usize, graph: &HashMap<usize, HashSet<usize>>) -> HashSet<usize> {
    graph.get(&vertex).unwrap().clone()
}

//bfs will print out the path it traverses the graph
//return hashmap of parent and distance
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

    let (parent, distance) = bfs(vertex, graph1);
    println!("Parent Map");
    for (key, value) in parent.iter() {
        println!("{} -> {}", key, value);
    }
    println!("Distance Map");
    for (keys, values) in distance.iter() {
        println!("{} -> {}", keys, values);
    }
    // let mut frontier = HashSet::from([2,3]);
    // let mut parent: HashMap<usize,usize> = HashMap::from([(1,1),(2,1),(3,1)]);
    // let mut visited: HashSet<usize> = HashSet::from([1]);
    // let (new_frontier, new_parent) = expand(frontier,&graph,parent,visited);
    // println!("{:?}",new_frontier);
    // for (key, value) in new_parent.iter() {
    //     println!("{} -> {}",key, value);
    // }
}
