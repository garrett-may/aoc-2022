use itertools::Itertools; // for `.cartesian_product`
use std::fs;

type Coord = (usize, usize);

struct Tree {
    height: usize,
    north: Coord, // Furthest tree north
    east: Coord,  // Furthest tree east
    south: Coord, // Furthest tree south
    west: Coord,  // Furthest tree west
}

// Read in a file as a string, and then parse it
fn read_and_parse(filepath: &str) -> Vec<Vec<Tree>> {
    fs::read_to_string(["res/", filepath].join(""))
        .expect("Unable to_xy read file")
        .replace('\r', "") // Strip all carriage returns (found on WSL)
        .split('\n')
        .filter(|line| line != &"") // Remove extraneous empty lines
        .enumerate()
        .map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .map(|(x, c)| {
                    let tree_xy = (x, y);
                    Tree {
                        height: (c - b'0') as usize,
                        north: tree_xy,
                        east: tree_xy,
                        south: tree_xy,
                        west: tree_xy,
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

// Given a tree and a function `f` that represents a certain direction,
// find the furthest tree that tree can see in that direction
//
// By finding the furthest tree, we can solve both part 1 and part 2, as:
// - that tree must be taller than the furthest tree it can see (part 1)
// - the distance between that tree and the furthest tree it can see (part 2)
//
// To find the furthest tree a tree can see, we essentially store a linked list
// intertwined within the `Tree` type. It goes back to see the furthest tree for
// other trees in that direction, and finds the smallest one of these.
//
// There are better ways to solve part 1, which do not involve the linked list
// (instead computing the _largest_ tree in a certain direction). And for part 2,
// you can completely brute force it and check all four directions for any given tree.
// However, this solution is the only one that seems to unite the two parts with a
// common function.
fn furthest_tree(
    trees: &[Vec<Tree>],
    mut from_xy: Coord,
    to_xy: Coord,
    f: fn(&Tree) -> Coord,
) -> Coord {
    while trees[from_xy.1][from_xy.0].height < trees[to_xy.1][to_xy.0].height {
        let far_tree_xy = f(&trees[from_xy.1][from_xy.0]);
        if far_tree_xy == from_xy {
            return from_xy;
        }
        from_xy = far_tree_xy;
    }
    from_xy
}

// A tree is visible from_xy the edges of the grid if:
// - that tree is taller than the furthest tree that tree can see
// - that tree is the same tree as the furthest tree that tree can see
//
// Trees on the edge have themselves as the furthest tree, so they are *visible*.
fn is_visible(trees: &[Vec<Tree>], tree_xy: Coord) -> bool {
    let tree = &trees[tree_xy.1][tree_xy.0];
    [tree.north, tree.east, tree.south, tree.west]
        .iter()
        .any(|far_tree_xy| {
            tree.height > trees[far_tree_xy.1][far_tree_xy.0].height || *far_tree_xy == tree_xy
        })
}

// A tree calculates its scenic score by doing the following:
// - find the furthest trees that tree can see in all four directions,
//   find the distances between them and that tree, and then find the
//   product
//
// Trees on the edge have themselves as the furthest tree, so their scenic score becomes *0*.
fn scenic_score(trees: &[Vec<Tree>], tree_xy: Coord) -> usize {
    let tree = &trees[tree_xy.1][tree_xy.0];
    [tree.north, tree.east, tree.south, tree.west]
        .iter()
        .map(|far_tree_xy| tree_xy.1.abs_diff(far_tree_xy.1) + tree_xy.0.abs_diff(far_tree_xy.0))
        .product()
}

fn visibility(filepath: &str) -> (usize, usize) {
    let mut trees = read_and_parse(filepath);
    let size = trees.len();

    for (x, y) in (1..size - 1).cartesian_product(1..size - 1) {
        let (a, b) = (size - x - 1, size - y - 1);
        trees[y][x].north = furthest_tree(&trees, (x, y - 1), (x, y), |tree: &Tree| tree.north);
        trees[y][a].east = furthest_tree(&trees, (a + 1, y), (a, y), |tree: &Tree| tree.east);
        trees[b][a].south = furthest_tree(&trees, (a, b + 1), (a, b), |tree: &Tree| tree.south);
        trees[y][x].west = furthest_tree(&trees, (x - 1, y), (x, y), |tree: &Tree| tree.west);
    }

    (
        (0..size)
            .cartesian_product(0..size)
            .filter(|tree_xy| is_visible(&trees, *tree_xy))
            .count(),
        (0..size)
            .cartesian_product(0..size)
            .map(|tree_xy| scenic_score(&trees, tree_xy))
            .max()
            .expect("Cannot retrieve max"),
    )
}

fn main() {
    println!(
        "Visible trees/Most scenic score: {:?}",
        visibility("test_input.txt")
    );
    println!("=========================");
    println!(
        "Visible trees/Most scenic score {:?}",
        visibility("input.txt")
    );
}
