#[derive(Debug, PartialEq, Eq, Default)]
pub struct SortedVec<T>
where
    T: Ord,
{
    vec: Vec<T>,
}

impl<T> SortedVec<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        SortedVec { vec: Vec::new() }
    }

    pub fn push(&mut self, val: T) {
        if self.vec.is_empty() {
            self.vec.push(val);
            return;
        }
        for i in 0..self.vec.len() {
            if self.vec[i] < val {
                self.vec.insert(i, val);
                return;
            }
        }
        self.vec.push(val);
    }
}
