use ndarray::*;
use num_traits::{Bounded, ToPrimitive, Zero};
use numpy::{Element, IntoPyArray, PyArray, PyReadonlyArray};
use pyo3::prelude::*;
use std::any::TypeId;
use std::fmt::Debug;
use std::ops::{Add, Sub};

use crate::adjacency::{Adjacency, AdjacencyGrid2D, AdjacencyGrid3D};
use crate::bucket_queue::BucketQueue;
use crate::heap::Heap;
use crate::priority_queue::{ElemStatus, PriorityQueue};

fn is_float<T: 'static>() -> bool {
    let type_id = TypeId::of::<T>();
    type_id == TypeId::of::<f32>() || type_id == TypeId::of::<f64>()
}

pub fn watershed_from_minima<T, D>(
    topology: &ArrayView<T, D>,
    mask: &ArrayView<bool, D>,
    h: T,
) -> Array<usize, D>
where
    T: Add<Output = T>
        + Sub<Output = T>
        + Bounded
        + Copy
        + Clone
        + Debug
        + Element
        + PartialOrd
        + ToPrimitive
        + Zero
        + 'static,
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
    let mut root: Array1<usize> = Array1::from_shape_vec(Ix1(size), (0..size).collect()).unwrap();

    let mut cost = topology.to_shape(size).unwrap().mapv(|x| x + h);

    let mut queue: Box<dyn PriorityQueue<T>> = match is_float::<T>() {
        false => Box::new(BucketQueue::new(&mut cost)),
        true => Box::new(Heap::new(&mut cost)),
        _ => panic!("Unsupported type!"),
    };

    for i in 0..size {
        if mask[i] {
            queue.insert(i, -1).unwrap();
        }
    }

    while !queue.is_empty() {
        let p = queue.pop().unwrap();

        if root[p] == p {
            queue.update_value(p, topology[p], -1);
        }

        for q in adj.neighbors(p) {
            if mask[q] && queue.get_status(q) != ElemStatus::POPPED {
                let path_cost: T;

                // fmax
                if topology[q] > queue.get_value(p) {
                    path_cost = topology[q];
                } else {
                    path_cost = queue.get_value(p);
                }

                if path_cost < queue.get_value(q) {
                    root[q] = root[p];
                    queue.update_value(q, path_cost, p as i64);
                }
            }
        }
    }

    // avoiding zero label in segmentation label
    for i in 0..size {
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

    let result = watershed_from_minima(&image.view(), &mask.view(), 1);
    assert_eq!(result, expected_labels);
}

macro_rules! impl_watershed_from_minima {
    ($new_name:ident, $ty:ty, $dim:expr) => {
        #[pyfunction]
        pub fn $new_name<'py>(
            py: Python<'py>,
            topology: PyReadonlyArray<$ty, Dim<[usize; $dim]>>,
            mask: PyReadonlyArray<bool, Dim<[usize; $dim]>>,
            h: $ty,
        ) -> PyResult<&'py PyArray<usize, Dim<[usize; $dim]>>> {
            let arr = watershed_from_minima(&topology.as_array(), &mask.as_array(), h);
            let py_array = arr.into_pyarray(py);
            Ok(py_array)
        }
    };
}

impl_watershed_from_minima!(watershed_from_minima_u8_2d, u8, 2);
impl_watershed_from_minima!(watershed_from_minima_u8_3d, u8, 3);
impl_watershed_from_minima!(watershed_from_minima_u16_2d, u16, 2);
impl_watershed_from_minima!(watershed_from_minima_u16_3d, u16, 3);
impl_watershed_from_minima!(watershed_from_minima_u32_2d, u32, 2);
impl_watershed_from_minima!(watershed_from_minima_u32_3d, u32, 3);

impl_watershed_from_minima!(watershed_from_minima_i16_2d, i16, 2);
impl_watershed_from_minima!(watershed_from_minima_i16_3d, i16, 3);
impl_watershed_from_minima!(watershed_from_minima_i32_2d, i32, 2);
impl_watershed_from_minima!(watershed_from_minima_i32_3d, i32, 3);
impl_watershed_from_minima!(watershed_from_minima_i64_2d, i64, 2);
impl_watershed_from_minima!(watershed_from_minima_i64_3d, i64, 3);

impl_watershed_from_minima!(watershed_from_minima_f32_2d, f32, 2);
impl_watershed_from_minima!(watershed_from_minima_f32_3d, f32, 3);
impl_watershed_from_minima!(watershed_from_minima_f64_2d, f64, 2);
impl_watershed_from_minima!(watershed_from_minima_f64_3d, f64, 3);
