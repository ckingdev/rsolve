#![feature(collections)]

use std::collections::VecMap;
use std::vec;
use std::ops;

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::VecMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

#[derive(Clone)]
struct Puzzle {
    elements: VecMap<u16>
}

impl Puzzle {
    fn new() -> Puzzle {
        let elements: VecMap<u16> = VecMap::new();
        return Puzzle{elements: elements};
    }

    fn new_full_solved(n_pieces: u16) -> Puzzle {
        let elements = (0..n_pieces).map(|i| (i as usize, i)).collect();
        return Puzzle{elements: elements};
    }

    fn is_solved(&self) -> bool {
        return self.elements.iter().all(|(i, j)| i == *j as usize);
    }

    fn inverse(&self) -> Puzzle {
        let swapped = self.elements.iter().map(|(i, j)| (*j as usize, i as u16)).collect();
        return Puzzle{elements: swapped};
    }
}

impl<'a, 'b> ops::Mul<&'a Puzzle> for &'b Puzzle {
    type Output = Puzzle;
    fn mul(self, rhs: &'a Puzzle) -> Puzzle {
        let mut result = Puzzle::new();
        for (i, j) in self.elements.iter() {
            let _ = match rhs.elements.get(&(*j as usize)) {
                Some(&k) => result.elements.insert(i, k),
                None => result.elements.insert(i, *j)
            };
        }
        return result
    }
}

impl ops::Mul for Puzzle {
    type Output = Puzzle;
    fn mul(self, rhs: Puzzle) -> Puzzle {
        return &self * &rhs; 
    }
}

impl<'a> ops::Mul<Puzzle> for &'a Puzzle {
    type Output = Puzzle;
    fn mul(self, rhs: Puzzle) -> Puzzle {
        return self.mul(rhs);
    }
}

impl<'a> ops::Mul<&'a Puzzle> for Puzzle {
    type Output = Puzzle;
    fn mul(self, rhs: &'a Puzzle) -> Puzzle {
        return (&self).mul(rhs);
    }
}

#[derive(Clone)]
struct SearchNode {
    p: Puzzle,
    moves_taken: vec::Vec<usize>
}

impl SearchNode {
    fn multiply(&self, i: usize, move_set: &Vec<Puzzle>) -> SearchNode {
        let new_p = &self.p * &move_set[i];
        let mut new_moves_taken = self.moves_taken.clone();
        new_moves_taken.push(i);
        return SearchNode{p: new_p, moves_taken: new_moves_taken};  
    }
}

fn dfs(depth: u8, max_depth: u8, node: &SearchNode, move_set: &Vec<Puzzle>) -> Option<SearchNode> {
    if depth == max_depth {
        // We're at max depth- check for solved and return appropriately
        if node.p.is_solved() {
            return Some(node.clone());
        } else {
            return None;
        }
    } else {
        for i in 0..move_set.len() {
            let new_node = node.multiply(i, move_set);
            match dfs(depth+1, max_depth, &new_node, move_set) {
                Some(solved) => return Some(solved),
                None => continue
            };
        }
        return None;
    }
}

fn iddfs(max_depth: u8, start_pos: Puzzle, move_set: &Vec<Puzzle>) -> Option<Vec<usize>> {
    let start_node = SearchNode{p: start_pos, moves_taken: vec::Vec::new()};
    for depth in 0..max_depth {
        match dfs(depth, max_depth, &start_node, move_set) {
            Some(solved) => return Some(solved.moves_taken),
            None => continue
        }
    }
    return None;
}

fn main() {
    // define moves
    let mapU = map!{0 => 1, 1 =>  3,  3 => 2, 2 => 0,
                    7 => 5, 5 => 11, 11 => 9, 9 => 7,
                    6 => 4, 4 => 10, 10 => 8, 8 => 6};
    let MoveU = Puzzle{elements: mapU};

                        
    let mapR = map!{15 => 3, 3 => 10, 10 => 23, 23 => 15,
                     7 => 1, 1 => 18, 18 => 21, 21 =>  7,
                     8 => 9, 9 => 17, 17 => 16, 16 =>  8};

    let MoveR = Puzzle{elements: mapR};
    
    let mapF = map!{6 =>  7,  7 => 15, 15 => 14, 14 => 6,
                    2 =>  8,  8 => 21, 21 => 13, 13 => 2,
                    3 => 16, 16 => 20, 20 =>  5,  5 => 3};
    let MoveF = Puzzle{elements: mapF};

    // define move sets
    let qtm = [MoveR.clone(), MoveR.inverse(),
               MoveU.clone(), MoveU.inverse(),
               MoveF.clone(), MoveF.inverse()];

    let htm = vec![MoveR.clone(), MoveR.clone() * &MoveR, MoveR.clone() * &MoveR * &MoveR,
                MoveU.clone(), MoveU.clone() * &MoveU, MoveU.clone() * &MoveU * &MoveU,
                MoveF.clone(), MoveF.clone() * &MoveF, MoveF.clone() * &MoveF * &MoveF];
    

    let mut p = Puzzle::new_full_solved(24);
    println!("{:?}", p.is_solved());
    p = p * &htm[0] * &htm[3] * &htm[2] * &htm[3] * &htm[0] * &htm[4] * &htm[0];
    let startNode = SearchNode{p: p, moves_taken: vec::Vec::new()};
    match dfs(0, 7, &startNode, &htm) {
        Some(solvedNode) => println!("{:?}", solvedNode.moves_taken),
        None => println!("No solution found.")
    }
}
