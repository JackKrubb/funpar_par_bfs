# funpar_par_bfs

The main idea to make bfs parallel is to expand the frontier in parallel and merge it together.

par_bfs was implemented taking input a regular std::collection hashset/hashmap and then converting to dashset and dashmap for local variables and return
as a regular std::collection data structures.

With several tests, the magic number for the number of nodes seems to be ~180. Which means that if the number of nodes is less than 180 (<180), the normal_bfs version will be faster than the parallel version. Beyond 180, the parallel version seems to be faster.



