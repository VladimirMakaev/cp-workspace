#[derive(Clone)]
pub struct XorSet {
    data: Vec<u8>,
}

const MAX_SIZE: u32 = 512; //0b111111111;
const CELL_SIZE: u32 = 8;

impl XorSet {
    pub fn new() -> Self {
        Self { data: vec![0; 72] }
    }

    pub fn union(&self, rhs: &Self) -> Self {
        return Self {
            data: self
                .data
                .iter()
                .zip(rhs.data.iter())
                .map(|(x, y)| x | y)
                .collect(),
        };
    }

    pub fn range_add(&self, value: u32) -> XorSet {
        let mut result = XorSet::new();
        for x in self {
            result.set_value(x ^ value);
        }
        result
    }

    pub fn set_value(&mut self, value: u32) {
        let mask = 1 << value % CELL_SIZE;
        self.data[(value / CELL_SIZE) as usize] = self.data[(value / CELL_SIZE) as usize] | mask;
    }

    pub fn contains_value(&self, value: u32) -> bool {
        let mask = 1 << (value % CELL_SIZE);
        return self.data[(value / CELL_SIZE) as usize] & mask == mask;
    }
}

pub struct XorSetIterator {
    xor: XorSet,
    index: Option<u32>,
}

pub struct XorSetRefIterator<'a> {
    xor: &'a XorSet,
    index: Option<u32>,
}

impl<'a> Iterator for XorSetRefIterator<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.index {
                Some(x) if x > MAX_SIZE => return None,
                Some(value) => {
                    self.index = Some(value + 1);
                    if self.xor.contains_value(value) {
                        return Some(value);
                    }
                }
                None => {
                    self.index = Some(0);
                }
            }
        }
    }
}

impl<'a> IntoIterator for &'a XorSet {
    type Item = u32;

    type IntoIter = XorSetRefIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        XorSetRefIterator {
            index: None,
            xor: self,
        }
    }
}

impl Iterator for XorSetIterator {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.index {
                Some(x) if x > MAX_SIZE => return None,
                Some(value) => {
                    self.index = Some(value + 1);
                    if self.xor.contains_value(value) {
                        return Some(value);
                    }
                }
                None => {
                    self.index = Some(0);
                }
            }
        }
    }
}

impl IntoIterator for XorSet {
    type Item = u32;
    type IntoIter = XorSetIterator;

    fn into_iter(self) -> Self::IntoIter {
        XorSetIterator {
            xor: self,
            index: None,
        }
    }
}

impl FromIterator<u32> for XorSet {
    fn from_iter<T: IntoIterator<Item = u32>>(iter: T) -> Self {
        let mut result = XorSet::new();
        for x in iter.into_iter() {
            result.set_value(x);
        }
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::XorSet;

    #[test]
    pub fn test_1() {
        let set: XorSet = vec![1, 5, 8, 10, 24].into_iter().collect();
        let set_vec: Vec<u32> = set.into_iter().collect();
        assert_eq!(set_vec, vec![1u32, 5, 8, 10, 24]);
    }

    #[test]
    pub fn test_2() {
        let set: XorSet = vec![1, 2, 4, 8].into_iter().collect();
        let result: Vec<u32> = set.range_add(3).into_iter().collect();
        assert_eq!(result, vec![1, 2, 7, 11]);
    }

    #[test]
    pub fn test_union() {
        let set1: XorSet = vec![1, 2, 4, 8].into_iter().collect();
        let set2: XorSet = vec![1, 2, 7, 11].into_iter().collect();
        let result: Vec<u32> = set1.union(&set2).into_iter().collect();
        assert_eq!(result, vec![1, 2, 4, 7, 8, 11]);
    }

    #[test]
    pub fn test_range_add() {
        let set1: XorSet = vec![0, 1, 6, 7].into_iter().collect();
        let result: Vec<u32> = set1.range_add(12).into_iter().collect();
        assert_eq!(result, vec![10, 11, 12, 13]);
    }

    #[test]
    pub fn test_range_add_2() {
        let set1: XorSet = vec![256].into_iter().collect();
        let result: Vec<u32> = set1.range_add(255).into_iter().collect();
        assert_eq!(result, vec![511]);
    }
}
