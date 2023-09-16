
use ndarray::prelude::*;
use numpy::{Element};
use pyo3::prelude::*;


#[derive(Debug, Clone)]
enum ElemStatus
{
    UNSEEN,
    SEEN,
    DONE,
}


pub struct Heap<'a, T>
where
    T: Element + Clone + PartialOrd,
{
    values: ArrayView1<'a, T>,
    nodes: Vec<usize>,
    pos: Vec<usize>,
    last: usize,
    size: usize,
    status: Vec<ElemStatus>,
    ages: Vec<i64>,
}


impl<'a, T> Heap<'a, T>
where
    T: Element + Clone + PartialOrd,
{
    fn new(values: ArrayView1<'a, T>) -> Self {
        let size = values.len();
        if size < 1 {
            panic!("Heap size must be greater than 0");
        }
        let mut nodes = vec![0; size];
        let mut pos = vec![0; size];
        let mut status = vec![ElemStatus::UNSEEN; size];
        let mut ages = vec![0; size];
        let mut heap = Heap {
            values,
            nodes,
            pos,
            last: size,
            size,
            status,
            ages,
        };
        heap.reset();
        heap
    }

    #[inline]
    fn is_full(&self) -> bool
    {
        self.last == self.size - 1
    }

    #[inline]
    fn is_empty(&self) -> bool
    {
        self.last == self.size
    }

    fn reset(&mut self) -> ()
    {
        self.last = self.size;
        for i in 0..self.size {
            self.nodes[i] = i;
            self.pos[i] = i;
            self.status[i] = ElemStatus::UNSEEN;
            self.ages[i] = 0;
        }
    }

    #[inline]
    fn parent(&self, i: usize) -> i64
    {
        (i as i64 - 1) / 2
    }

    #[inline]
    fn left_child(&self, i: usize) -> usize
    {
        2 * i + 1
    }

    #[inline]
    fn right_child(&self, i: usize) -> usize
    {
        2 * i + 2
    }

    #[inline]
    fn swap(&mut self, i: usize, j: usize) -> ()
    {
        let tmp = self.nodes[i];
        self.nodes[i] = self.nodes[j];
        self.nodes[j] = tmp;
        self.pos[self.nodes[i]] = i;
        self.pos[self.nodes[j]] = j;
    }

    #[inline]
    fn lower(&self, i: usize, j: usize) -> bool
    {
        let node_i = self.nodes[i];
        let node_j = self.nodes[j];
        if (self.values[node_i] < self.values[node_j]) ||
           (self.values[node_i] == self.values[node_j] && self.ages[node_i] < self.ages[node_j]) {
            return true;
        }
        return false;
    }

    #[inline]
    fn greater(&self, i: usize, j: usize) -> bool
    {
        let node_i = self.nodes[i];
        let node_j = self.nodes[j];
        if (self.values[node_i] > self.values[node_j]) ||
           (self.values[node_i] == self.values[node_j] && self.ages[node_i] > self.ages[node_j]) {
            return true;
        }
        return false;
    }

    // moves towards root (smaller values) of heap
    fn move_up(&mut self, pos: usize) -> ()
    {
        let mut parent = self.parent(pos);
        while (parent >= 0) && self.lower(parent as usize, pos) {
            self.swap(parent as usize, pos);
            parent = self.parent(pos);
        }
    }

    fn move_down(&mut self, pos: usize) -> ()
    {
        let mut next = pos;
        let left = self.left_child(pos);
        let right = self.right_child(pos);
        if (left <= self.last) && self.lower(left, next) {
            next = left;
        }
        if (right <= self.last) && self.lower(right, next) {
            next = right;
        }
        if next != pos {
            self.swap(next, pos);
            self.move_down(next);
        }
    }

    #[inline]
    fn try_update_age(&mut self, index: usize, parent_index: i64) -> ()
    {
        if parent_index >= 0 {
            self.ages[index] = self.ages[parent_index as usize] + 1;
        } else {
            self.ages[index] = 0;
        }
    }

    fn insert(&mut self, index: usize, parent_index: i64) -> ()
    {
        if self.is_full()
        {
            panic!("Heap is full");
        }

        self.try_update_age(index, parent_index);

        if self.last == self.size {
            self.last = 0;
        }

        self.last += 1;
        self.nodes[self.last] = index;
        self.status[index] = ElemStatus::SEEN;
        self.pos[index] = self.last;
        self.move_up(self.last);
    }

    // TODO
    // fn pop(&mut self) -> usize
    // {
    //     if self.is_empty()
    //     {
    //         panic!("Heap is empty");
    //     }
    // }

}


#[test]
fn test_heap_creation()
{
    // Create an ArrayView1 from a Vec of i32 values
    let values = Array1::from(vec![3, 1, 2, 4]);

    // Create a Heap instance with Minimum policy
    let heap = Heap::new(values.view());

    // Assert that the Heap was created correctly
    assert_eq!(heap.size, 4);
    // Add more assertions to verify the initialization of other fields...
}