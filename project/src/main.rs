/*
General idea of BFS

def bfs(nbrs: V => Set[V], src: V) =
    distFromSrc = 0
    frontier = { src } // set of vertices on the frontier
    visited = { src } // set of already-visited vertices
    while (frontier.nonEmpty) {
        frontier_ = expand(frontier, visited, nbrs)
        visited_ = visited + frontier
        frontier, visited, distFromSrc = frontier_, visited_, distFromSrc + 1
    }
    

def bfs[V](nbrs: V => Set[V], src: V) = {
    def expand(frontier: Set[V], parent: Map[V, V]): (Set[V], Map[V, V]) =
    // derive new frontier and new parent map
    def iterate(frontier: Set[V], parent: Map[V, V], distance: Map[V, Int], d: Int) =
        if frontier.isEmpty then
            (parent, distance)
        else {
            val (frontier_, parent_) = expand(frontier, parent)
            val distance_ = // derive new distance map   
            iterate(frontier_, parent_, distance_, d + 1)
        }
    iterate(Set(src), Map(src -> src), Map(), 0)
}
*/

use std::{collections::{HashMap, HashSet}};


//input a vertex and return all its neighbours
fn nbrs(vertex: usize, graph: &HashMap<usize, HashSet<usize>>) -> HashSet<usize> {
    graph.get(&vertex).unwrap().clone()
}

//bfs will print out the path it traverses the graph
fn bfs(source: usize, graph: HashMap<usize, HashSet<usize>>) {

}

fn main() {
    let mut graph: HashMap<usize, HashSet<usize>> = HashMap::new();
    graph.insert(1, HashSet::from([2,3]));
    graph.insert(2, HashSet::from([1]));
    graph.insert(3, HashSet::from([1,4]));
    graph.insert(4, HashSet::from([3]));

    let vertex: usize = 1;
    println!("Neighbours of Vertex: {} are {:?}", vertex, nbrs(vertex, &graph))
}
