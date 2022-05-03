use std::ops::RangeBounds;

pub trait SegTreeNode<D>: Copy {
    fn combine(lhr: &Self, rhs: &Self) -> Self;

    fn zero() -> Self;

    fn leaf(data: &D) -> Self;
}

pub trait UpdateRange<Node, D>
where
    Node: SegTreeNode<D>,
{
}

pub struct SegTree<T: SegTreeNode<Data>, Data> {
    size: usize,
    inner: Vec<T>,
    #[allow(dead_code)]
    data: Vec<Data>,
}

impl<T: SegTreeNode<Data>, Data> SegTree<T, Data> {
    #[allow(dead_code)]
    pub fn new(data: Vec<Data>) -> Self {
        let mut tree_data = Vec::with_capacity(4 * data.len());
        tree_data.resize_with(4 * data.len(), || <T as SegTreeNode<Data>>::zero());
        let result = Self {
            size: data.len(),
            inner: tree_data,
            data: data,
        };
        result
    }

    #[inline]
    fn left(cur_node: usize) -> usize {
        cur_node << 1
    }

    #[inline]
    fn right(cur_node: usize) -> usize {
        (cur_node << 1) + 1
    }

    fn build_internal(&mut self, cur_node: usize, lx: usize, rx: usize) {
        if lx == rx {
            self.inner[cur_node] = <T as SegTreeNode<Data>>::leaf(&self.data[lx]);
            return;
        }
        let mid = (lx + rx) / 2;

        self.build_internal(Self::left(cur_node), lx, mid);
        self.build_internal(Self::right(cur_node), mid + 1, rx);
        self.inner[cur_node] = <T as SegTreeNode<Data>>::combine(
            &self.inner[Self::left(cur_node)],
            &self.inner[Self::right(cur_node)],
        );
    }

    #[allow(dead_code)]
    pub fn build(&mut self) {
        self.build_internal(1, 0, self.size - 1);
    }

    fn query_internal(&mut self, cur_node: usize, l: usize, r: usize, lx: usize, rx: usize) -> T {
        if lx >= l && rx <= r {
            return self.inner[cur_node];
        }

        if rx < l || lx > r {
            return <T as SegTreeNode<Data>>::zero();
        }

        let mid = (lx + rx) / 2;
        let left = self.query_internal(Self::left(cur_node), l, r, lx, mid);
        let right = self.query_internal(Self::right(cur_node), l, r, mid + 1, rx);
        return <T as SegTreeNode<Data>>::combine(&left, &right);
    }

    fn update_data_internal<F>(
        &mut self,
        cur_node: usize,
        index: usize,
        l: usize,
        r: usize,
        mut update: F,
    ) where
        F: FnMut(&mut Data),
    {
        if l == r {
            update(&mut self.data[l]);
            self.inner[cur_node] = T::leaf(&self.data[l]);
            return;
        }
        let mid = (l + r) / 2;
        if index <= mid {
            self.update_data_internal(Self::left(cur_node), index, l, mid, update);
        } else {
            self.update_data_internal(Self::right(cur_node), index, mid + 1, r, update);
        }
        self.inner[cur_node] = <T as SegTreeNode<Data>>::combine(
            &self.inner[Self::left(cur_node)],
            &self.inner[Self::right(cur_node)],
        );
    }

    fn get_left_range<B: RangeBounds<usize>>(&self, range: &B) -> usize {
        match range.start_bound() {
            std::ops::Bound::Included(l) => *l,
            std::ops::Bound::Excluded(l) => *l + 1,
            std::ops::Bound::Unbounded => 0,
        }
    }

    fn get_right_range<B: RangeBounds<usize>>(&self, range: &B) -> usize {
        match range.end_bound() {
            std::ops::Bound::Included(r) => *r,
            std::ops::Bound::Excluded(r) => *r - 1,
            std::ops::Bound::Unbounded => self.size - 1,
        }
    }

    #[allow(dead_code)]
    pub fn query<B: RangeBounds<usize>>(&mut self, query_range: B) -> T {
        if query_range.end_bound() == std::ops::Bound::Excluded(&0) {
            return <T as SegTreeNode<Data>>::zero();
        }
        let l = self.get_left_range(&query_range);
        let r = self.get_right_range(&query_range);
        self.query_internal(1, l, r, 0, self.size - 1)
    }

    pub fn update_data<F>(&mut self, index: usize, update: F)
    where
        F: FnMut(&mut Data),
    {
        self.update_data_internal(1, index, 0, self.size - 1, update);
    }

    fn update_range_internal<Op: UpdateRange<T, Data>, B: RangeBounds<usize>>(
        &mut self,
        query_l: usize,
        query_r: usize,
        lx: usize,
        rx: usize,
    ) {
    }

    pub fn update_range<Op: UpdateRange<T, Data>, B: RangeBounds<usize>>(
        &mut self,
        query_range: B,
        operation: Op,
    ) {
    }
}

#[cfg(test)]
mod test {
    use super::{SegTree, SegTreeNode};

    #[derive(Clone, Copy)]
    struct Node {
        sum: usize,
    }

    impl SegTreeNode<usize> for Node {
        fn combine(lhr: &Self, rhs: &Self) -> Self {
            Self {
                sum: lhr.sum + rhs.sum,
            }
        }

        fn zero() -> Self {
            Self { sum: 0 }
        }

        fn leaf(data: &usize) -> Self {
            Self { sum: data.clone() }
        }
    }

    #[derive(Clone, Copy)]
    struct MinNode {
        min: usize,
    }

    struct Data {
        value: usize,
        count: usize,
    }

    impl SegTreeNode<Data> for MinNode {
        fn combine(lhr: &Self, rhs: &Self) -> Self {
            Self {
                min: std::cmp::min(lhr.min, rhs.min),
            }
        }

        fn zero() -> Self {
            Self {
                min: std::usize::MAX,
            }
        }

        fn leaf(data: &Data) -> Self {
            if data.count > 0 {
                Self { min: data.value }
            } else {
                Self::zero()
            }
        }
    }

    #[test]
    pub fn test_tree_1() {
        let mut tree = SegTree::<Node, usize>::new(vec![4, 1, 7, 2, 6, 3, 2, 4]);
        tree.build();
        assert_eq!(tree.query(0..).sum, 29);
        assert_eq!(tree.query(..).sum, 29);
        assert_eq!(tree.query(1..=1).sum, 1);
        assert_eq!(tree.query(2..=4).sum, 15);
        assert_eq!(tree.query(2..3).sum, 7);
        assert_eq!(tree.query(0..1).sum, 4);
    }

    #[test]
    pub fn test_tree_update() {
        let mut tree = SegTree::<Node, usize>::new(vec![0, 0, 0, 0]);
        tree.build();

        tree.update_data(0, |x| *x += 1);
        tree.update_data(1, |x| *x += 1);
        tree.update_data(2, |x| *x += 1);
        tree.update_data(3, |x| *x += 1);
        assert_eq!(tree.query(0..).sum, 4);
    }

    #[test]
    pub fn test_min() {
        let data = vec![
            Data { value: 0, count: 0 },
            Data { value: 1, count: 0 },
            Data { value: 2, count: 0 },
            Data { value: 3, count: 0 },
        ];
        let mut tree = SegTree::<MinNode, Data>::new(data);
        assert_eq!(tree.query(0..).min, std::usize::MAX);
        tree.update_data(1, |x| x.count += 1);
        assert_eq!(tree.query(1..).min, 1);
        tree.update_data(1, |x| x.count -= 1);
        assert_eq!(tree.query(0..).min, std::usize::MAX);
    }
}
