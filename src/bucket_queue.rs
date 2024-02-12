use crate::priority_queue::{ElemStatus, PriorityQueue};
use ndarray::prelude::Array1;
use ndarray_stats::QuantileExt;
use num_traits::ToPrimitive;
use std::collections::VecDeque;
use std::ops::Sub;

/*
todo:
    - bucket queue does not accept or check for negative values
    - known bug: if multiple updates are done decreaising the value and then increasing it again, there might be nodes in the queue that should not be there
 */

pub struct BucketQueue<'a, T: 'static> {
    values: &'a mut Array1<T>,
    buckets: Vec<VecDeque<usize>>,
    min_priority: usize,
    max_priority: usize,
    pub status: Vec<ElemStatus>,
}

impl<'a, T: 'static> PriorityQueue<'a, T> for BucketQueue<'a, T>
where
    T: Sub<Output = T> + ToPrimitive + PartialOrd + Copy + Sub,
{
    fn is_full(&self) -> bool {
        false
    }

    fn is_empty(&mut self) -> bool {
        while self.min_priority <= self.max_priority {
            let bucket = &mut self.buckets[self.min_priority];
            while !bucket.is_empty() {
                if self.status[bucket[0]] == ElemStatus::IN {
                    return false;
                }
                bucket.pop_front();
            }
            self.min_priority += 1;
        }
        true
    }

    fn insert(&mut self, index: usize, _parent_index: i64) -> Result<(), &'static str> {
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
        Ok(index)
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

    fn update_value(&mut self, index: usize, value: T, parent_index: i64) {
        if self.status[index] != ElemStatus::IN {
            self.values[index] = value;
        } else {
            // if value is smaller than current it doesn't need to be removed
            if value > self.values[index] {
                self.remove(index).unwrap();
            }
            self.values[index] = value;
            self.insert(index, parent_index).unwrap();
        }
    }

    fn reset(&mut self) {
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
        self.values[index]
    }

    #[inline(always)]
    fn get_status(&self, index: usize) -> ElemStatus {
        self.status[index]
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
        let n_buckets = values.max().unwrap().to_usize().unwrap() + 1;
        let buckets = vec![VecDeque::new(); n_buckets];
        let status = vec![ElemStatus::OUT; size];

        BucketQueue {
            values,
            buckets,
            min_priority: n_buckets,
            max_priority: 0,
            status,
        }
    }

    #[inline(always)]
    fn get_bucket(&self, index: usize) -> usize {
        self.values[index].to_usize().unwrap()
    }
}

#[test]
fn test_bucket_queue() {
    // Create an ArrayView1 from a Vec of i32 values
    let mut values = Array1::from(vec![3, 1, 2, 3]);

    // Create a Heap instance with Minimum policy
    let mut queue = BucketQueue::new(&mut values);

    assert!(queue.is_empty());

    // Insert a new element
    queue.insert(0, -1).unwrap();

    assert!(!queue.is_empty());
    assert_eq!(queue.min_priority, 3);
    assert_eq!(queue.buckets[3].len(), 1);
    assert_eq!(queue.status[0], ElemStatus::IN);

    // Insert smaller element
    queue.insert(1, 0).unwrap();

    assert_eq!(queue.buckets[1].len(), 1);
    assert_eq!(queue.status[1], ElemStatus::IN);

    // Insert equal element
    queue.insert(3, 0).unwrap();

    assert_eq!(queue.buckets[3].len(), 2);
    assert_eq!(queue.status[3], ElemStatus::IN);

    // Insert last element
    queue.insert(2, 0).unwrap();

    assert_eq!(queue.buckets[2].len(), 1);
    assert!(!queue.is_full()); // it's never full
    assert!(!queue.is_empty());

    // Pop element
    assert_eq!(queue.pop().unwrap(), 1);
    assert_eq!(queue.status[1], ElemStatus::POPPED);

    // Increase value from 2
    queue.update_value(2, 5, -1);

    assert_eq!(queue.buckets[5].len(), 1);
    assert_eq!(queue.buckets[2].len(), 0); // must be empty because 5 is greater than the previous value

    // Return to original value
    queue.update_value(2, 2, -1);

    queue.is_empty(); // refreching min. priority bucket
    assert_eq!(queue.buckets[2].len(), 1);
    assert_eq!(queue.buckets[5].len(), 1); // because the update is a smaller value this bucket is not empty

    // Poping again
    assert_eq!(queue.pop().unwrap(), 2);

    // Remove element
    queue.remove(0).unwrap();

    assert_eq!(queue.buckets[3].len(), 1); // single element left
    assert!(!queue.is_full());

    queue.reset();
    assert!(queue.is_empty());

    for i in 0..queue.status.len() {
        assert_eq!(queue.status[i], ElemStatus::OUT);
    }
}
