fn main() {
    let hill = {
        let input = include_str!("input.txt");
        let cols = input.find('\n').unwrap();
        let trees: Vec<bool> = input
            .replace('\n', "")
            .chars()
            .map(|c| c == '#')
            .collect();
        let rows = trees.len() / cols;
        Hill {
            size: (cols, rows),
            trees,
        }
    };

    let slopes = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2)
    ];

    let result: usize = slopes.iter().map(|s| {
        let count = hill.count_trees(*s);
        println!("slope ({},{}) = {} trees", s.0, s.1, count);
        count
    }).product();

    println!("product of tree counts: {}", result);
}

struct Hill {
    size: (usize, usize),
    trees: Vec<bool>,
}

impl Hill {
    fn has_tree(&self, p: &(usize, usize)) -> bool {
        self.trees[p.1 * self.size.0 + p.0]
    }
    fn count_trees(&self, slope: (usize, usize)) -> usize {
        Slope {
            pos: (0, 0),
            size: self.size,
            slope
        }.filter(|t| self.has_tree(t)).count()
    }
}

struct Slope {
    pos: (usize, usize),
    slope: (usize, usize),
    size: (usize, usize),
}

impl Iterator for Slope {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let p = self.pos;
        if p.1 < self.size.1 {
            self.pos.0 = (self.pos.0 + self.slope.0) % self.size.0;
            self.pos.1 += self.slope.1;
            Some(p)
        } else {
            None
        }
    }
}