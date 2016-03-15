use std::cmp;
// In a triangular tree-like directed graph of numbers (represented by a Vec<Vec<usize>>), where
// each node points to two more in a lower level, find the path from the top to bottom level whose
// nodes have the greatest sum.
pub fn maxtreesum(mut tree: Vec<Vec<usize>>) -> usize {
    let rows = tree.len();
    for i in 1..rows {
        for j in 0..i + 1 {
            match j {
                0 => tree[i][j] += tree[i - 1][j],
                x if x == i => tree[i][j] += tree[i - 1][i - 1],
                _ => tree[i][j] += cmp::max(
                    tree[i - 1][j],
                    tree[i - 1][j - 1]
                    )
            }
        }
    }
    let finalrow = tree[rows - 1].clone();
    finalrow.into_iter().max().unwrap()
}
