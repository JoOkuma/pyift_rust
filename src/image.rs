
pub struct Image2D
{
    pub width: usize,
    pub height: usize,
}


pub struct Image3D
{
    pub width: usize,
    pub height: usize,
    pub depth: usize,
}


impl Image2D
{
    pub fn new(width: usize, height: usize) -> Self
    {
        Image2D { width, height }
    }

    #[inline]
    fn to_index(&self, x: usize, y: usize) -> usize
    {
        x + y * self.width
    }

    #[inline]
    pub fn neighbors(&self, p: usize) -> Vec<usize> 
    {
        let x = p % self.width;
        let y = p / self.width;
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

    #[inline]
    fn is_valid(&self, x: usize, y: usize) -> bool
    {
        x < self.width && y < self.height
    }
}


impl Image3D
{
    pub fn new(width: usize, height: usize, depth: usize) -> Self
    {
        Image3D { width, height, depth }
    }

    #[inline]
    fn to_index(&self, x: usize, y: usize, z: usize) -> usize
    {
        x + y * self.width + z * self.width * self.height
    }

    #[inline]
    pub fn neighbors(&self, p: usize) -> Vec<usize> 
    {
        let x = p % self.width;
        let y = (p / self.width) % self.height;
        let z = p / (self.width * self.height);
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

    #[inline]
    fn is_valid(&self, x: usize, y: usize, z: usize) -> bool
    {
        x < self.width && y < self.height && z < self.depth
    }
}



#[test]
fn test_2d_neighborhood()
{
    let image = Image2D::new(3, 3);
    let neighbors = image.neighbors(4);
    assert_eq!(neighbors.len(), 4);
    assert_eq!(neighbors[0], 3);
    assert_eq!(neighbors[1], 5);
    assert_eq!(neighbors[2], 1);
    assert_eq!(neighbors[3], 7);

    // check invalid neighbors
    let neighbors = image.neighbors(0);
    assert_eq!(neighbors.len(), 2);

    let neighbors = image.neighbors(8);
    assert_eq!(neighbors.len(), 2);

    let neighbors = image.neighbors(1);
    assert_eq!(neighbors.len(), 3);
}


#[test]
fn test_3d_neighboorhood()
{
    let image = Image3D::new(3, 3, 3);
    let neighbors = image.neighbors(13);
    assert_eq!(neighbors.len(), 6);
    assert_eq!(neighbors[0], 12);
    assert_eq!(neighbors[1], 14);
    assert_eq!(neighbors[2], 10);
    assert_eq!(neighbors[3], 16);
    assert_eq!(neighbors[4], 4);
    assert_eq!(neighbors[5], 22);

    // check invalid neighbors
    let neighbors = image.neighbors(0);
    assert_eq!(neighbors.len(), 3);

    let neighbors = image.neighbors(26);
    assert_eq!(neighbors.len(), 3);

    let neighbors = image.neighbors(1);
    assert_eq!(neighbors.len(), 4);

    let neighbors = image.neighbors(4);
    assert_eq!(neighbors.len(), 5);
}