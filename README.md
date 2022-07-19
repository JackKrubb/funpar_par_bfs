# funpar_par_bfs

The main idea to make bfs parallel is to expand the frontier in parallel and merge it together.

par_bfs was implemented taking input a regular std::collection hashset/hashmap and then converting to dashset and dashmap for local variables and return
as a regular std::collection data structures.

However, when using par_iter with dashset and dashmap on the frontier, it doesnt seem to be faster than normal_bfs probably because of spinlock and the way Rust works for parallel codes.



