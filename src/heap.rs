use crate::priority_queue::{ElemStatus, PriorityQueue};
use ndarray::prelude::*;
use num_traits::Bounded;
use numpy::Element;

pub struct Heap<'a, T: 'static>
where
    T: Bounded + Copy + Clone + Element + PartialOrd,
{
    values: &'a mut Array1<T>,
    nodes: Vec<usize>,
    pos: Vec<usize>,
    last: usize,
    size: usize,
    pub status: Vec<ElemStatus>,
    ages: Vec<i64>,
}

impl<'a, T: 'static> PriorityQueue<'a, T> for Heap<'a, T>
where
    T: Bounded + Copy + Clone + Element + PartialOrd,
{
    fn is_full(&self) -> bool {
        self.last + 1 == self.size
    }

    fn is_empty(&mut self) -> bool {
        self.last == self.size
    }

    fn insert(&mut self, index: usize, parent_index: i64) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("Heap is full");
        }

        self.try_update_age(index, parent_index);

        if self.is_empty() {
            self.last = 0;
        } else {
            self.last += 1;
        }

        self.nodes[self.last] = index;
        self.status[index] = ElemStatus::IN;
        self.pos[index] = self.last;
        self.move_up_from_position(self.last);

        Ok(())
    }

    fn pop(&mut self) -> Result<usize, &'static str> {
        if self.is_empty() {
            return Err("Heap is empty");
        }

        let index = self.nodes[0];
        self.status[index] = ElemStatus::POPPED;
        self.swap(0, self.last);

        // making removed invalid
        self.pos[index] = self.size;
        self.nodes[self.last] = self.size;

        if self.last == 0 {
            self.last = self.size;
        } else {
            self.last -= 1;
            // FIXME: this step is extremelly slow on watershed from minima
            self.move_down_from_position(0);
        }
        Ok(index)
    }

    fn remove(&mut self, index: usize) -> Result<(), &'static str> {
        if self.pos[index] == self.size {
            return Err("Element not in heap");
        }

        let value = self.values[index];
        self.values[index] = T::min_value();
        self.move_up_from_position(self.pos[index]);

        self.pop()?;

        self.values[index] = value;
        self.status[index] = ElemStatus::OUT;

        Ok(())
    }

    fn update_value(&mut self, index: usize, value: T, parent_index: i64) -> () {
        let prev_value = self.values[index];

        self.values[index] = value;

        if self.status[index] == ElemStatus::IN {
            if value < prev_value {
                self.move_up(index, parent_index)
            } else if value > prev_value {
                self.move_down(index, parent_index)
            }
        }
    }

    fn reset(&mut self) -> () {
        self.last = self.size;
        for i in 0..self.size {
            self.nodes[i] = i;
            self.pos[i] = i;
            self.status[i] = ElemStatus::OUT;
            self.ages[i] = 0;
        }
    }

    #[inline(always)]
    fn get_value(&self, index: usize) -> T {
        return self.values[index];
    }

    #[inline(always)]
    fn get_status(&self, index: usize) -> ElemStatus {
        return self.status[index];
    }
}

impl<'a, T> Heap<'a, T>
where
    T: Bounded + Copy + Clone + Element + PartialOrd,
{
    pub fn new(values: &'a mut Array1<T>) -> Self {
        let size = values.len();
        if size < 1 {
            panic!("Heap size must be greater than 0");
        }
        let mut nodes = vec![0; size];
        let mut pos = vec![0; size];
        let mut status = vec![ElemStatus::OUT; size];
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

    fn try_update_age(&mut self, index: usize, parent_index: i64) -> () {
        if parent_index >= 0 {
            self.ages[index] = self.ages[parent_index as usize] + 1;
        } else {
            self.ages[index] = 0;
        }
    }

    fn move_up(&mut self, index: usize, parent_index: i64) -> () {
        self.try_update_age(index, parent_index);
        self.move_up_from_position(self.pos[index]);
    }

    fn move_down(&mut self, index: usize, parent_index: i64) -> () {
        self.try_update_age(index, parent_index);
        self.move_down_from_position(self.pos[index]);
    }

    fn parent(&self, i: usize) -> i64 {
        (i as i64 - 1) / 2
    }

    fn left_child(&self, i: usize) -> usize {
        2 * i + 1
    }

    fn right_child(&self, i: usize) -> usize {
        2 * i + 2
    }

    fn swap(&mut self, i: usize, j: usize) -> () {
        let tmp = self.nodes[i];
        self.nodes[i] = self.nodes[j];
        self.nodes[j] = tmp;
        self.pos[self.nodes[i]] = i;
        self.pos[self.nodes[j]] = j;
    }

    fn lower(&self, i: usize, j: usize) -> bool {
        let node_i = self.nodes[i];
        let node_j = self.nodes[j];
        (self.values[node_i] < self.values[node_j])
            || (self.values[node_i] == self.values[node_j] && self.ages[node_i] < self.ages[node_j])
    }

    fn greater(&self, i: usize, j: usize) -> bool {
        let node_i = self.nodes[i];
        let node_j = self.nodes[j];
        (self.values[node_i] > self.values[node_j])
            || (self.values[node_i] == self.values[node_j] && self.ages[node_i] > self.ages[node_j])
    }

    // moves towards root (smaller values) of heap
    fn move_up_from_position(&mut self, pos: usize) -> () {
        let mut current = pos;
        let mut parent = self.parent(current);
        while (parent >= 0) && self.greater(parent as usize, current) {
            self.swap(parent as usize, current);
            current = parent as usize;
            parent = self.parent(current);
        }
    }

    fn move_down_from_position(&mut self, pos: usize) -> () {
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
            self.move_down_from_position(next);
        }
    }
}

#[test]
fn test_heap() {
    // Create an ArrayView1 from a Vec of i32 values
    let mut values = Array1::from(vec![3, 1, 2, 4]);

    // Create a Heap instance with Minimum policy
    let mut heap = Heap::new(&mut values);

    heap.is_empty();

    // Assert that the Heap was created correctly
    assert_eq!(heap.size, 4);

    // Insert a new element
    heap.insert(0, -1).unwrap();

    assert!(!heap.is_empty());
    assert_eq!(heap.last, 0);
    assert_eq!(heap.pos[0], 0);
    assert_eq!(heap.status[0], ElemStatus::IN);

    // Insert smaller element
    heap.insert(1, 0).unwrap();

    assert_eq!(heap.last, 1);
    assert_eq!(heap.pos[1], 0);
    assert_eq!(heap.pos[0], 1);

    // Insert larger element
    heap.insert(3, 0).unwrap();

    assert_eq!(heap.last, 2);
    assert_eq!(heap.pos[0], 1);
    assert_eq!(heap.pos[1], 0);
    assert_eq!(heap.pos[3], 2);

    // Insert last element
    heap.insert(2, 0).unwrap();

    assert!(heap.is_full());
    assert!(!heap.is_empty());

    // Pop element
    assert_eq!(heap.pop().unwrap(), 1);
    assert_eq!(heap.status[1], ElemStatus::POPPED);

    // Increase value
    assert_eq!(heap.pos[2], 0);
    heap.update_value(2, 5, -1);

    assert_eq!(heap.pos[0], 0);

    // Return to original value
    heap.update_value(2, 2, -1);

    assert_eq!(heap.pos[2], 0);

    // Poping again
    assert_eq!(heap.pop().unwrap(), 2);

    // Remove element
    heap.remove(0).unwrap();

    assert_eq!(heap.last, 0); // single element left
    assert!(!heap.is_full());

    heap.reset();
    assert!(heap.is_empty());

    for i in 0..heap.size {
        assert_eq!(heap.status[i], ElemStatus::OUT);
    }
}
