#[derive(Clone, PartialEq, Eq)]
enum Version {
    Current(usize),
    Draft(usize),
}

pub struct DSUR {
    current: Vec<usize>,
    draft: Vec<usize>,
    versions: Vec<Version>,
    cur_version: Version,
    count: usize,
}

impl DSUR {
    pub fn len(&self) -> usize {
        return self.count;
    }

    pub fn new(size: usize) -> Self {
        DSUR {
            current: (0..size).collect(),
            draft: (0..size).collect(),
            versions: vec![Version::Current(0); size],
            cur_version: Version::Current(0),
            count: size,
        }
    }

    pub fn save(&mut self) {
        self.cur_version = match self.cur_version {
            Version::Current(x) => Version::Draft(x + 1),
            Version::Draft(_) => panic!("DSU is already saved"),
        };
    }

    pub fn restore(&mut self) {
        self.cur_version = match self.cur_version {
            Version::Draft(x) => Version::Current(x),
            Version::Current(_) => panic!("DSU isn't backed up"),
        }
    }

    fn get(&self, x: usize) -> usize {
        match (&self.cur_version, &self.versions[x]) {
            (Version::Current(_), _) => self.current[x],
            (Version::Draft(_), Version::Current(_)) => self.current[x],
            (Version::Draft(v1), Version::Draft(v2)) if v1 == v2 => self.draft[x],
            (Version::Draft(_), Version::Draft(_)) => self.current[x],
        }
    }

    fn set(&mut self, x: usize, value: usize) -> usize {
        match self.cur_version {
            Version::Current(_) => {
                self.current[x] = value;
                value
            }
            Version::Draft(_) => {
                self.versions[x] = self.cur_version.clone();
                self.draft[x] = value;
                value
            }
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        let mut next = x;
        loop {
            let p = self.get(next);
            if p == next {
                return self.set(x, p);
            }
            next = p;
        }
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let a = self.find(a);
        let b = self.find(b);

        match a.cmp(&b) {
            std::cmp::Ordering::Less => {
                self.set(b, a);
            }
            std::cmp::Ordering::Greater => {
                self.set(a, b);
            }
            _ => {}
        };
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    fn to_vec(dsu: &mut DSUR) -> Vec<usize> {
        (0..dsu.len()).map(|x| dsu.find(x)).collect()
    }

    #[test]
    pub fn test1() {
        let mut set = DSUR::new(5);
        assert_eq!(set.find(4), 4);
    }

    #[test]
    pub fn test2() {
        let mut set = DSUR::new(3);
        set.union(0, 1);
        assert_eq!(to_vec(&mut set), vec![0, 0, 2]);
        set.union(1, 2);
        assert_eq!(to_vec(&mut set), vec![0, 0, 0]);
    }

    #[test]
    pub fn test3() {
        let mut set = DSUR::new(5);
        set.union(0, 1);
        set.union(2, 4);
        assert_eq!(to_vec(&mut set), vec![0, 0, 2, 3, 2]);
        set.union(0, 3);
        assert_eq!(to_vec(&mut set), vec![0, 0, 2, 0, 2]);
    }

    #[test]
    pub fn test4() {
        let mut set = DSUR::new(5);
        set.union(1, 4);
        set.union(2, 3);
        assert_eq!(to_vec(&mut set), vec![0, 1, 2, 2, 1]);
        set.save();
        assert_eq!(to_vec(&mut set), vec![0, 1, 2, 2, 1]);
        set.union(0, 4);
        assert_eq!(to_vec(&mut set), vec![0, 0, 2, 2, 0]);
        set.restore();
        assert_eq!(to_vec(&mut set), vec![0, 1, 2, 2, 1]);
    }

    #[test]
    pub fn test5() {
        let mut set = DSUR::new(5);
        set.save();
        set.union(1, 4);
        set.union(2, 3);
        assert_eq!(to_vec(&mut set), vec![0, 1, 2, 2, 1]);
        set.restore();
        assert_eq!(to_vec(&mut set), vec![0, 1, 2, 3, 4]);
        set.union(0, 1);
        set.union(2, 4);
        assert_eq!(to_vec(&mut set), vec![0, 0, 2, 3, 2]);
        set.save();
        set.union(0, 3);
        assert_eq!(to_vec(&mut set), vec![0, 0, 2, 0, 2]);
        set.restore();
        assert_eq!(to_vec(&mut set), vec![0, 0, 2, 3, 2]);
    }
}
