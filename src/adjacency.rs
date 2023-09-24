
pub trait Adjacency
{
    fn neighbors(&self, p: usize) -> Vec<usize>;
}


pub struct AdjacencyGrid2D
{
    shape: [usize; 2],
}


impl AdjacencyGrid2D
{
    pub fn new(shape: &[usize]) -> Self
    {
        AdjacencyGrid2D {
            shape: [shape[0], shape[1]],
        }
    }

    #[inline]
    fn to_index(&self, x: usize, y: usize) -> usize
    {
        x + y * self.shape[1]
    }

    #[inline]
    fn is_valid(&self, x: usize, y: usize) -> bool
    {
        y < self.shape[0] && x < self.shape[1]
    }
}


impl Adjacency for AdjacencyGrid2D
{
    fn neighbors(&self, p: usize) -> Vec<usize> 
    {
        let x = p % self.shape[1];
        let y = p / self.shape[1];
        // array of neighbors

        // wraps around so validation is done upper bound only
        let neighbors = vec![
            (x.wrapping_sub(1), y),
            (x.wrapping_add(1), y),
            (x, y.wrapping_sub(1)),
            (x, y.wrapping_add(1)),
        ];

        let valid_neighbors = neighbors.into_iter()
            .filter(|&(x, y)| self.is_valid(x, y))
            .map(|(x, y)| self.to_index(x, y))
            .collect::<Vec<_>>();
        
        valid_neighbors
    }
}


pub struct AdjacencyGrid3D
{
    shape: [usize; 3],
}


impl AdjacencyGrid3D
{
    pub fn new(shape: &[usize]) -> Self
    {
        AdjacencyGrid3D {
             shape: [shape[0], shape[1], shape[2]],
        }
    }

    #[inline]
    fn to_index(&self, x: usize, y: usize, z: usize) -> usize
    {
        x + y * self.shape[2] + z * self.shape[1] * self.shape[2]
    }

    #[inline]
    fn is_valid(&self, x: usize, y: usize, z: usize) -> bool
    {
        z < self.shape[0] && y < self.shape[1] && x < self.shape[2]
    }
}


impl Adjacency for AdjacencyGrid3D
{
    #[inline]
    fn neighbors(&self, p: usize) -> Vec<usize> 
    {
        let x = p % self.shape[2];
        let y = (p / self.shape[2]) % self.shape[1];
        let z = p / (self.shape[2] * self.shape[1]);
        // array of neighbors

        // wraps around so validation is done upper bound only
        let neighbors = vec![
            (x.wrapping_sub(1), y, z),
            (x.wrapping_add(1), y, z),
            (x, y.wrapping_sub(1), z),
            (x, y.wrapping_add(1), z),
            (x, y, z.wrapping_sub(1)),
            (x, y, z.wrapping_add(1)),
        ];

        let valid_neighbors = neighbors.into_iter()
            .filter(|&(x, y, z)| self.is_valid(x, y, z))
            .map(|(x, y, z)| self.to_index(x, y, z))
            .collect::<Vec<_>>();
        
        valid_neighbors
    }
}



#[test]
fn test_2d_neighborhood()
{
    let shape = [3, 3];
    let adj = AdjacencyGrid2D::new(&shape);
    let neighbors = adj.neighbors(4);
    assert_eq!(neighbors.len(), 4);
    assert_eq!(neighbors[0], 3);
    assert_eq!(neighbors[1], 5);
    assert_eq!(neighbors[2], 1);
    assert_eq!(neighbors[3], 7);

    // check invalid neighbors
    let neighbors = adj.neighbors(0);
    assert_eq!(neighbors.len(), 2);

    let neighbors = adj.neighbors(8);
    assert_eq!(neighbors.len(), 2);

    let neighbors = adj.neighbors(1);
    assert_eq!(neighbors.len(), 3);
}


#[test]
fn test_3d_neighboorhood()
{
    let shape = [3, 3, 3];
    let adj = AdjacencyGrid3D::new(&shape);
    let neighbors = adj.neighbors(13);
    assert_eq!(neighbors.len(), 6);
    assert_eq!(neighbors[0], 12);
    assert_eq!(neighbors[1], 14);
    assert_eq!(neighbors[2], 10);
    assert_eq!(neighbors[3], 16);
    assert_eq!(neighbors[4], 4);
    assert_eq!(neighbors[5], 22);

    // check invalid neighbors
    let neighbors = adj.neighbors(0);
    assert_eq!(neighbors.len(), 3);

    let neighbors = adj.neighbors(26);
    assert_eq!(neighbors.len(), 3);

    let neighbors = adj.neighbors(1);
    assert_eq!(neighbors.len(), 4);

    let neighbors = adj.neighbors(4);
    assert_eq!(neighbors.len(), 5);
}