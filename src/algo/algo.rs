// use std::io;
// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::fmt; // Import `fmt`
// use std::cmp;

// use crate::ds::sequence::Sequence;
// use crate::ds::tile::Tile;
// use crate::ds::graph::Graph;
// use crate::ds::graph::Ukonen;
// use crate::ds::graph::UkonenNode;
// use crate::ds::graph::UkonenEdge;
// use crate::ds::graph::GraphProperties;
// use crate::ds::graph::Dfs;


// Greedy search for shortest common superstring
// pub fn shortest_common_superstring(sequences: &Tile) -> Sequence {

//     let mut subsequence_set = HashSet::<Sequence>::new();
//     let mut k : usize = 0;
//     for seq in sequences {
//         subsequence_set.insert(seq.clone());
//         // find max length of subsequence used
//         if seq.len() > k {
//             k = seq.len();
//         }
//     }

//     let mut x = k / 2;

//     while x < k {
//         let graph = overlap_graph(sequences,  k);
//         if graph.edge_count() > 0 {
//             let e = graph.edges().next().unwrap();
//             graph.write_dot("abc.dot");
//             let mut input = String::new();
//             io::stdin().read_line(&mut input).expect("error: unable to read user input");
//         }
//         // print!("ALOHA");
//         // k = k-1;
//         x += 1;
//     }

//     Sequence::new()
// }


// N{P}[ST]{P}
// pub fn generate_motifs(target: &Vec<u8>, result: &mut Vec<String>, i: usize, temp: &mut Vec<u8>) {
//
//     if i == target.len() {
//         result.push(std::str::from_utf8(temp).unwrap().to_string());
//         return;
//     }
//
//     let alphabet = String::from("FLSYCWPHQRITMNKVADEG").into_bytes();
//     let mut alternatives = Vec::<u8>::new();
//     let mut j = i;
//     if target[j] == b'[' {
//         j += 1;
//         while j < target.len() && target[j] != b']' {
//             alternatives.push(target[j]);
//             j += 1;
//         }
//         for elem in alternatives {
//             temp.push(elem);
//             generate_motifs(target, result, j + 1, temp);
//             temp.pop();
//         }
//     }
//     else if target[j] == b'{' {
//         j += 1;
//         for elem in alphabet {
//             if elem != target[j] {
//                 temp.push(elem);
//                 generate_motifs(target, result, j + 2, temp);
//                 temp.pop();
//             }
//         }
//         j += 2;
//     }
//     else{
//         temp.push(target[i]);
//         generate_motifs(target, result, i + 1, temp);
//         temp.pop();
//     }
// }

// fn search_motifs(st: String, pat: String) -> Vec<usize> {
//
//     let st = st.into_bytes();
//     let pat = pat.into_bytes();
//     let mut res = Vec::<String>::new();
//     let mut temp = Vec::<u8>::new();
//
//     generate_motifs(&pat, &mut res, 0, &mut temp);
//
//     let mut pos = Vec::<usize>::new();
//     for elem in &res {
//         pos.append(&mut knuth_morris_pratt(&st, elem.as_bytes()));    
//     }
//     pos
// }