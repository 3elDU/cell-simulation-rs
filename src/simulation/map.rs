/// A structure containing map with all the cells.
/// It is just a wrapper around [`Vec`] with some helper methods.
#[derive(Clone)]
pub struct Map<T> {
    map: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T> Default for Map<T>
where
    T: Default,
{
    fn default() -> Self {
        // Return an empty map
        Self::new(0, 0)
    }
}

impl<T> Map<T>
where
    T: Default,
{
    pub fn new(width: usize, height: usize) -> Self {
        let mut map = Vec::with_capacity(width);
        for i in 0..width {
            map.push(Vec::with_capacity(height));
            for _j in 0..height {
                map[i].push(T::default());
            }
        }

        Map { map, width, height }
    }

    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }

    // Returns a cell at the specified coordinates
    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.map.get(x)?.get(y)
    }
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.map.get_mut(x)?.get_mut(y)
    }

    /// Set a cell at specified coordinates
    pub fn set(&mut self, x: usize, y: usize, cell: T) {
        self.map[x][y] = cell;
    }
}
