pub trait Adjacency {
    fn neighbors(&self, p: usize) -> Vec<usize>;
}

pub struct AdjacencyGrid2D {
    shape: [usize; 2],
}

impl AdjacencyGrid2D {
    pub fn new(shape: &[usize]) -> Self {
        AdjacencyGrid2D {
            shape: [shape[0], shape[1]],
        }
    }

    #[inline(always)]
    fn to_index(&self, x: usize, y: usize) -> usize {
        x + y * self.shape[1]
    }

    #[inline(always)]
    fn is_valid(&self, x: usize, y: usize) -> bool {
        y < self.shape[0] && x < self.shape[1]
    }
}

impl Adjacency for AdjacencyGrid2D {
    fn neighbors(&self, p: usize) -> Vec<usize> {
        let x = p % self.shape[1];
        let y = p / self.shape[1];
        // array of neighbors

        // wraps around so validation is done upper bound only
        let mut valid_neighbors = Vec::<usize>::with_capacity(4);

        // (0, 1)
        let ny = y.wrapping_add(1);
        let nx = x;
        if self.is_valid(nx, ny) {
            valid_neighbors.push(self.to_index(nx, ny));
        }

        // (1, 0)
        let ny = y;
        let nx = x.wrapping_add(1);
        if self.is_valid(nx, ny) {
            valid_neighbors.push(self.to_index(nx, ny));
        }

        // (0, -1)
        let ny = y.wrapping_sub(1);
        let nx = x;
        if self.is_valid(nx, ny) {
            valid_neighbors.push(self.to_index(nx, ny));
        }

        // (-1, 0)
        let ny = y;
        let nx = x.wrapping_sub(1);
        if self.is_valid(nx, ny) {
            valid_neighbors.push(self.to_index(nx, ny));
        }

        valid_neighbors
    }
}

pub struct AdjacencyGrid3D {
    shape: [usize; 3],
}

impl AdjacencyGrid3D {
    pub fn new(shape: &[usize]) -> Self {
        AdjacencyGrid3D {
            shape: [shape[0], shape[1], shape[2]],
        }
    }

    #[inline(always)]
    fn to_index(&self, x: usize, y: usize, z: usize) -> usize {
        x + y * self.shape[2] + z * self.shape[1] * self.shape[2]
    }

    #[inline(always)]
    fn is_valid(&self, x: usize, y: usize, z: usize) -> bool {
        z < self.shape[0] && y < self.shape[1] && x < self.shape[2]
    }
}

impl Adjacency for AdjacencyGrid3D {
    fn neighbors(&self, p: usize) -> Vec<usize> {
        let x = p % self.shape[2];
        let y = (p / self.shape[2]) % self.shape[1];
        let z = p / (self.shape[2] * self.shape[1]);
        // array of neighbors

        // wraps around so validation is done upper bound only
        let mut valid_neighbors = Vec::<usize>::with_capacity(6);

        // (0, 0, 1)
        let nz = z.wrapping_add(1);
        let ny = y;
        let nx = x;
        if self.is_valid(nx, ny, nz) {
            valid_neighbors.push(self.to_index(nx, ny, nz));
        }

        // (0, 1, 0)
        let nz = z;
        let ny = y.wrapping_add(1);
        let nx = x;
        if self.is_valid(nx, ny, nz) {
            valid_neighbors.push(self.to_index(nx, ny, nz));
        }

        // (1, 0, 0)
        let nz = z;
        let ny = y;
        let nx = x.wrapping_add(1);
        if self.is_valid(nx, ny, nz) {
            valid_neighbors.push(self.to_index(nx, ny, nz));
        }

        // (0, 0, -1)
        let nz = z.wrapping_sub(1);
        let ny = y;
        let nx = x;
        if self.is_valid(nx, ny, nz) {
            valid_neighbors.push(self.to_index(nx, ny, nz));
        }

        // (0, -1, 0)
        let nz = z;
        let ny = y.wrapping_sub(1);
        let nx = x;
        if self.is_valid(nx, ny, nz) {
            valid_neighbors.push(self.to_index(nx, ny, nz));
        }

        // (-1, 0, 0)
        let nz = z;
        let ny = y;
        let nx = x.wrapping_sub(1);
        if self.is_valid(nx, ny, nz) {
            valid_neighbors.push(self.to_index(nx, ny, nz));
        }

        valid_neighbors
    }
}

#[test]
fn test_2d_neighborhood() {
    let shape = [3, 3];
    let adj = AdjacencyGrid2D::new(&shape);
    let neighbors = adj.neighbors(4);
    assert_eq!(neighbors.len(), 4);
    assert_eq!(neighbors[0], 7);
    assert_eq!(neighbors[1], 5);
    assert_eq!(neighbors[2], 1);
    assert_eq!(neighbors[3], 3);

    // check invalid neighbors
    let neighbors = adj.neighbors(0);
    assert_eq!(neighbors.len(), 2);

    let neighbors = adj.neighbors(8);
    assert_eq!(neighbors.len(), 2);

    let neighbors = adj.neighbors(1);
    assert_eq!(neighbors.len(), 3);
}

#[test]
fn test_3d_neighboorhood() {
    let shape = [3, 3, 3];
    let adj = AdjacencyGrid3D::new(&shape);
    let neighbors = adj.neighbors(13);
    assert_eq!(neighbors.len(), 6);
    assert_eq!(neighbors[0], 22);
    assert_eq!(neighbors[1], 16);
    assert_eq!(neighbors[2], 14);
    assert_eq!(neighbors[3], 4);
    assert_eq!(neighbors[4], 10);
    assert_eq!(neighbors[5], 12);

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
