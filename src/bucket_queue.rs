use crate::priority_queue::{ElemStatus, PriorityQueue};
use ndarray::prelude::Array1;
use ndarray_stats::QuantileExt;
use num_traits::ToPrimitive;
use std::collections::VecDeque;
use std::ops::Sub;

/*
todo:
    - avoid removing from queue, create a list of invalid nodes and ignore them when popping / checking empty
    - bucket queue ta buggada
 */

pub struct BucketQueue<'a, T: 'static> {
    values: &'a mut Array1<T>,
    buckets: Vec<VecDeque<usize>>,
    min_value: T,
    min_priority: usize,
    max_priority: usize,
    pub status: Vec<ElemStatus>,
}

impl<'a, T: 'static> PriorityQueue<'a, T> for BucketQueue<'a, T>
where
    T: Sub<Output = T> + ToPrimitive + PartialOrd + Copy + Sub,
{
    fn is_full(&self) -> bool {
        return false;
    }

    fn is_empty(&mut self) -> bool {
        while self.min_priority <= self.max_priority {
            if !self.buckets[self.min_priority].is_empty() {
                return false;
            }
            self.min_priority += 1;
        }
        return true;
    }

    fn insert(&mut self, index: usize, parent_index: i64) -> Result<(), &'static str> {
        let bucket = self.get_bucket(index);

        if bucket < self.min_priority {
            self.min_priority = bucket;
        }

        if bucket > self.max_priority {
            self.max_priority = bucket;
        }

        if bucket >= self.buckets.len() {
            for _ in self.buckets.len()..(bucket + 1) {
                self.buckets.push(VecDeque::new());
            }
        }

        self.status[index] = ElemStatus::IN;
        self.buckets[bucket].push_back(index);

        Ok(())
    }

    fn pop(&mut self) -> Result<usize, &'static str> {
        if self.is_empty() {
            return Err("Queue is empty");
        }
        let index = self.buckets[self.min_priority].pop_front().unwrap();
        self.status[index] = ElemStatus::POPPED;
        return Ok(index);
    }

    fn remove(&mut self, index: usize) -> Result<(), &'static str> {
        if self.status[index] != ElemStatus::IN {
            return Err("Removing element not in queue.");
        }
        let bucket = self.get_bucket(index);
        let index_in_bucket = self.buckets[bucket]
            .iter()
            .position(|&x| x == index)
            .unwrap();
        self.buckets[bucket].remove(index_in_bucket);
        self.status[index] = ElemStatus::OUT;
        Ok(())
    }

    fn update_value(&mut self, index: usize, value: T, parent_index: i64) -> () {
        if self.status[index] != ElemStatus::IN {
            self.values[index] = value;
        } else {
            self.remove(index).unwrap();
            self.values[index] = value;
            self.insert(index, parent_index).unwrap();
        }
    }

    fn reset(&mut self) -> () {
        self.min_priority = self.values.len();
        self.max_priority = 0;
        for i in 0..self.status.len() {
            self.status[i] = ElemStatus::OUT;
        }
        for i in 0..self.buckets.len() {
            self.buckets[i].clear();
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

impl<'a, T> BucketQueue<'a, T>
where
    T: Sub<Output = T> + ToPrimitive + Copy + PartialOrd,
{
    pub fn new(values: &'a mut Array1<T>) -> Self {
        let size = values.len();
        if size < 1 {
            panic!("Heap size must be greater than 0");
        }
        let min_value = *values.min().unwrap();
        let n_buckets = (*values.max().unwrap() - min_value).to_usize().unwrap() + 1;
        let buckets = vec![VecDeque::new(); n_buckets];
        let status = vec![ElemStatus::OUT; size];
        let mut queue = BucketQueue {
            values,
            buckets: buckets,
            min_value: min_value,
            min_priority: n_buckets,
            max_priority: 0,
            status: status,
        };
        queue
    }

    #[inline(always)]
    fn get_bucket(&self, index: usize) -> usize {
        return (self.values[index] - self.min_value).to_usize().unwrap();
    }
}

#[test]
fn test_bucket_queue() {
    // Create an ArrayView1 from a Vec of i32 values
    let mut values = Array1::from(vec![3, 1, 2, 3]);

    // Create a Heap instance with Minimum policy
    let mut queue = BucketQueue::new(&mut values);

    assert!(queue.is_empty());
    assert_eq!(queue.min_value, 1);

    // Insert a new element
    queue.insert(0, -1).unwrap();

    assert!(!queue.is_empty());
    assert_eq!(queue.min_priority, 2);
    assert_eq!(queue.buckets[2].len(), 1);
    assert_eq!(queue.status[0], ElemStatus::IN);

    // Insert smaller element
    queue.insert(1, 0).unwrap();

    assert_eq!(queue.buckets[0].len(), 1);
    assert_eq!(queue.status[1], ElemStatus::IN);

    // Insert equal element
    queue.insert(3, 0).unwrap();

    assert_eq!(queue.buckets[2].len(), 2);
    assert_eq!(queue.status[3], ElemStatus::IN);

    // Insert last element
    queue.insert(2, 0).unwrap();

    assert_eq!(queue.buckets[1].len(), 1);
    assert!(!queue.is_full()); // it's never full
    assert!(!queue.is_empty());

    // Pop element
    assert_eq!(queue.pop().unwrap(), 1);
    assert_eq!(queue.status[1], ElemStatus::POPPED);

    // Increase value from 2
    queue.update_value(2, 5, -1);

    assert_eq!(queue.buckets[4].len(), 1);
    assert_eq!(queue.buckets[1].len(), 0);

    // Return to original value
    queue.update_value(2, 2, -1);

    assert_eq!(queue.buckets[1].len(), 1);
    assert_eq!(queue.buckets[4].len(), 0);

    // Poping again
    assert_eq!(queue.pop().unwrap(), 2);

    // Remove element
    queue.remove(0).unwrap();

    assert_eq!(queue.buckets[2].len(), 1); // single element left
    assert!(!queue.is_full());

    queue.reset();
    assert!(queue.is_empty());

    for i in 0..queue.status.len() {
        assert_eq!(queue.status[i], ElemStatus::OUT);
    }
}
