pub struct DSU {
    parent: Vec<usize>,
    rank: Vec<usize>,
    size: Vec<usize>,
}

impl DSU {
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
            rank: vec![0; size],
            size: vec![1; size],
        }
    }

    pub fn find(&mut self, index: usize) -> usize {
        let (result, _) = self.find_with_size(index);
        return result;
    }

    pub fn find_with_size(&mut self, index: usize) -> (usize, usize) {
        let mut next = index;
        loop {
            if self.parent[next] == next {
                self.parent[index] = next;
                return (next, self.size[next]);
            } else {
                next = self.parent[next];
            }
        }
    }

    pub fn union(&mut self, lhs: usize, rhs: usize) -> usize {
        let left = self.find(lhs);
        let right = self.find(rhs);

        if right == left {
            return left;
        }
        
        self.size[right] = self.size[left] + self.size[right];
        self.size[left] = self.size[right];

        match self.rank[left].cmp(&self.rank[right]) {
            std::cmp::Ordering::Less => {
                self.parent[left] = right;
                right
            }
            std::cmp::Ordering::Greater => {
                self.parent[right] = left;
                left
            }
            std::cmp::Ordering::Equal => {
                self.parent[left] = right;
                self.rank[right] += 1;
                right
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rayon::vec;

    use super::DSU;

    #[test]
    pub fn test1() {
        let mut set = DSU::new(5);
        assert_eq!(set.find(4), 4);
    }

    #[test]
    pub fn test2() {
        let mut set = DSU::new(5);
        set.union(0, 1);
        set.union(2, 4);
        let result: Vec<_> = (0..5).map(|x| set.find(x)).collect();
        assert_eq!(result, vec![1, 1, 4, 3, 4]);
        set.union(0, 3);
        let result: Vec<_> = (0..5).map(|x| set.find(x)).collect();
        assert_eq!(result, vec![1, 1, 4, 1, 4]);
    }

    #[test]
    pub fn test3() {
        let mut set = DSU::new(10);
        set.union(0, 1);
        set.union(2, 3);
        set.union(0, 3);
        set.union(9, 8);
        set.union(7, 6);
        set.union(6, 5);
        set.union(0, 6);
        let result: Vec<_> = (0..10).map(|x| set.find_with_size(x)).collect();
        assert_eq!(
            result,
            vec![
                (3, 7),
                (3, 7),
                (3, 7),
                (3, 7),
                (4, 1),
                (3, 7),
                (3, 7),
                (3, 7),
                (8, 2),
                (8, 2)
            ]
        );
    }
}
