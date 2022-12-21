use std::fs;

struct Forest {
    // Stored as row, col
    trees: Vec<Vec<i32>>,

    // Stored as col, row
    col_first_trees: Vec<Vec<i32>>,

    visible_trees: Vec<Vec<i32>>,

    scenic_scores: Vec<Vec<i32>>,
}

impl Forest {
    pub fn from_file(input: &'static str) -> Forest {
        let trees = fs::read_to_string(input)
            .expect("Unable to read file")
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_string().parse::<i32>().unwrap())
                    .collect()
            })
            .collect::<Vec<Vec<i32>>>();

        // Store col, row version to iterate easily over columns
        let mut cols: Vec<Vec<i32>> = vec![Vec::new(); trees[0].len()];
        trees
            .iter()
            .for_each(|i| i.iter().enumerate().for_each(|j| cols[j.0].push(*j.1)));

        let mut result = Forest {
            visible_trees: vec![vec![0; trees[0].len()]; trees.len()],
            scenic_scores: vec![vec![0; trees[0].len()]; trees.len()],
            trees: trees,
            col_first_trees: cols,
        };

        result.calculate_visible_trees();
        result.calculate_scenic_scores();

        result
    }

    fn calculate_scenic_score_row<'a, I>(&self, row: I, idx: usize) -> i32
    where
        I: Iterator<Item = &'a i32>,
    {
        let mut iter = row.skip(idx);
        let tree = iter.next().unwrap();
        let mut score = 0;

        for t in iter {
            score += 1;
            if t >= tree {
                break;
            }
        }

        score
    }

    fn calculate_scenic_score(&self, row: usize, col: usize) -> i32 {
        self.calculate_scenic_score_row(self.trees[row].iter(), col)
            * self.calculate_scenic_score_row(
                self.trees[row].iter().rev(),
                self.scenic_scores[0].len() - col - 1,
            )
            * self.calculate_scenic_score_row(
                self.col_first_trees[col].iter().rev(),
                self.col_first_trees[0].len() - row - 1,
            )
            * self.calculate_scenic_score_row(self.col_first_trees[col].iter(), row)
    }

    fn calculate_scenic_scores(&mut self) {
        // Filter row-wise left to right
        for tree_row in self.trees.iter().enumerate() {
            for tree_col in self.trees.iter().enumerate() {
                self.scenic_scores[tree_row.0][tree_col.0] =
                    self.calculate_scenic_score(tree_row.0, tree_col.0);
            }
        }
    }

    fn calculate_visible_trees(&mut self) {
        // Filter row-wise left to right
        self.trees.iter().enumerate().for_each(|i| {
            let mut max = -1;
            i.1.iter().enumerate().for_each(|j| {
                if *j.1 > max {
                    self.visible_trees[i.0][j.0] = 1;
                    max = *j.1;
                }
            });
        });

        // Filter row-wise right to left
        self.trees.iter().enumerate().for_each(|i| {
            let mut max = -1;
            i.1.iter().enumerate().rev().for_each(|j| {
                if *j.1 > max {
                    self.visible_trees[i.0][j.0] = 1;
                    max = *j.1;
                }
            });
        });

        // Filter col-wise top down
        self.col_first_trees.iter().enumerate().for_each(|i| {
            let mut max = -1;
            i.1.iter().enumerate().for_each(|j| {
                if *j.1 > max {
                    self.visible_trees[j.0][i.0] = 1;
                    max = *j.1;
                }
            });
        });

        // Filter col-wise buttom up
        self.col_first_trees.iter().enumerate().for_each(|i| {
            let mut max = -1;
            i.1.iter().enumerate().rev().for_each(|j| {
                if *j.1 > max {
                    self.visible_trees[j.0][i.0] = 1;
                    max = *j.1;
                }
            });
        });
    }

    pub fn highest_scenic_score(&self) -> i32 {
        *self.scenic_scores.iter().flatten().max().unwrap()
    }

    pub fn visible_trees(&self) -> i32 {
        self.visible_trees.iter().flatten().sum()
    }
}

fn main() {
    let forest = Forest::from_file("input.txt");

    println!(
        "Part 1: {}\nPart 2: {}",
        forest.visible_trees(),
        forest.highest_scenic_score()
    );
}
