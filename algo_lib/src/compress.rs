/// A simple data structure for coordinate compression
pub struct SparseIndex<T>
where
    T: Ord + Copy + Clone,
{
    coords: Vec<T>,
}

impl<T> SparseIndex<T>
where
    T: Ord + Copy + Clone,
{
    /// Builds an index, given the full set of coordinates to compress.
    pub fn new(mut coords: Vec<T>) -> Self {
        coords.sort_unstable();
        coords.dedup();
        Self { coords }
    }

    /// Returns Ok(i) if the coordinate q appears at index i
    /// Returns Err(i) if q appears between indices i-1 and i
    pub fn compress(&self, q: &T) -> Result<usize, usize> {
        self.coords.binary_search(q)
    }

    pub fn compress_unw(&self, q: &T) -> usize {
        self.compress(q).unwrap()
    }

    pub fn max(&self) -> usize {
        self.coords.last().map(|x| self.compress_unw(x)).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_sparse_1() {
        let res = vec![2, 54, 4, 2, 1, 7, 9];
        let index = SparseIndex::new(res);
        assert_eq!(index.compress(&54), Ok(5))
    }

    #[test]
    pub fn test_sparse_2() {
        let res = vec![2, 54, 4, 2, 1, 7, 9];
        let index = SparseIndex::new(res);
        assert_eq!(index.max(), 5)
    }
}
