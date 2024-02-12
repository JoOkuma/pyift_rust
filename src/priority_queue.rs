#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ElemStatus {
    OUT,
    IN,
    POPPED,
}

pub trait PriorityQueue<'a, T: 'static> {
    fn is_full(&self) -> bool;
    fn is_empty(&mut self) -> bool;
    fn insert(&mut self, index: usize, parent_index: i64) -> Result<(), &'static str>;
    fn pop(&mut self) -> Result<usize, &'static str>;
    fn remove(&mut self, index: usize) -> Result<(), &'static str>;
    fn update_value(&mut self, index: usize, value: T, parent_index: i64);
    fn reset(&mut self);
    fn get_value(&self, index: usize) -> T;
    fn get_status(&self, index: usize) -> ElemStatus;
}
