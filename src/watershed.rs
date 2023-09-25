use std::ops::Add;
use numpy::{Element};
use ndarray::*;

use num_traits::{Bounded, Zero};
use crate::heap::{ElemStatus, Heap};
use crate::adjacency::{Adjacency, AdjacencyGrid2D, AdjacencyGrid3D};


pub fn watershed_from_minima<T, D>(
    topology: &ArrayView<T, D>,
    mask: &ArrayView<bool, D>,
    h: T,
) -> Array<usize, D>
    where
    T: Add<Output = T> + Bounded + Copy + Clone + Element + PartialOrd + Ord + Zero,
    D: Dimension,
{
    let shape = topology.dim();

    let adj: Box<dyn Adjacency> = match D::NDIM.unwrap() {
        2 => Box::new(AdjacencyGrid2D::new(topology.shape())),
        3 => Box::new(AdjacencyGrid3D::new(topology.shape())),
        _ => panic!("Unsupported dimension!"),
    };

    let topology = topology.to_shape(topology.len()).unwrap();
    let mask = mask.to_shape(mask.len()).unwrap();

    if h <= T::zero() {
        panic!("h must be greater than 0");
    }

    let size: usize = topology.len();
    let mut cost = topology
        .to_shape(size)
        .unwrap()
        .mapv(|x| x + h);

    let mut root: Array1<usize> = Array1::from_shape_vec(
        Ix1(size),
        (0..size).collect()
    ).unwrap();

    let mut heap = Heap::new(& mut cost);

    for i in 0..size {
        if mask[i] {
            heap.insert(i, -1).unwrap();
        }
    }

    while !heap.is_empty()
    {
        let p = heap.pop().unwrap();

        if root[p] == p
        {
            heap.values[p] = topology[p];
        }

        for q in adj.neighbors(p)
        {
            if mask[q] && heap.status[q] != ElemStatus::POPPED 
            {
                let path_cost = std::cmp::max(topology[q], heap.values[p]);
                if path_cost < heap.values[q]
                {
                    root[q] = root[p];
                    heap.values[q] = path_cost;
                    heap.move_up(q, p as i64);
                }
            }
        }
    }

    for i in 0..size
    {
        if mask[i] {
            root[i] += 1;
        } else {
            root[i] = 0;
        }
    }

    root.into_shape(shape).unwrap()
}


#[test]
fn test_watershed_from_minima() {
    // TODO: Test 3D and do a real 2D example
    // Test 1
    let image = array![[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    let shape = image.dim();
    let mask = Array2::from_elem(shape, true);

    let expected_labels = array![[1, 1, 1], [1, 1, 1], [1, 1, 1]];

    let result = watershed_from_minima(
        &image.view(),
        &mask.view(),
        1,
    );
    assert_eq!(result, expected_labels);
}