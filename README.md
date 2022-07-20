# funpar_par_bfs

Parallel Breadth-First Search 

Problem & Solution
Sequential Breadth-First Search is slow for large networks so why not make it faster? :)
Seq BFS is done by iterating through each vertex in each level to find its neighbours and add it into the queue. This is slow and there is an opportunity for us to perform this process in parallel.

Project Scope
To parallelize BFS, it is quite difficult to implement as we need all the workers (threads?) to be able to have access to “global” variables in order to update/create the frontiers/distance without interrupting each other or creating any race conditions. Simple parallelization of BFS may cause race conditions such as duplicating the update call to the distance because the neighbours of two vertices have the possibility of clashing, so we will aim to prevent this from happening while making it parallel.

Therefore, in our project, we will aim to provide synchronizations to all the workers and speed up parallel BFS.

Functions:
The design of BFS was inspired from our A2 which is the frontier, parent and distance version.

Our implementation of BFS will take a graph input and source vertex V
The function will then use nbrs function and return a parent HashMap<usize,usize> and a distance HashMap<usize,usize>
• the neighbor function nbrs is a function that takes a vertex and yields a set of neighboring vertices
of this vertex.
• parent is a map from vertex to vertex, mapping each vertex u to the vertex used immediately
before reaching u on the shortest path from source to u.
• distance is a map from a vertex to an integer representing the distance from source. Distance is
measured in terms of the number of edges used. This means, source is at distance 0 away from src
itself.

bfs() is the normal (sequential) version of BFS

par_bfs() is the parallel version of BFS

Both functions are tested in main()
