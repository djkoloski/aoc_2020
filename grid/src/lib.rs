#[derive(Clone, Debug)]
pub struct Grid<T> {
    width: usize,
    height: usize,
    values: Vec<T>,
}

impl<T: Default> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self::new_with(width, height, |_, _| Default::default())
    }
}

impl<T> Grid<T> {
    pub fn new_with(width: usize, height: usize, get: impl Fn(i32, i32) -> T) -> Self {
        let mut values = Vec::with_capacity(width * height);
        for y in 0..height as i32 {
            for x in 0..width as i32 {
                values.push(get(x, y));
            }
        }

        Self {
            width,
            height,
            values,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get(&self, x: i32, y: i32) -> &T {
        self.try_get(x, y).unwrap()
    }

    pub fn try_get(&self, x: i32, y: i32) -> Option<&T> {
        if self.contains(x, y) {
            Some(&self.values[x as usize + y as usize * self.width])
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: i32, y: i32) -> &mut T {
        self.try_get_mut(x, y).unwrap()
    }

    pub fn try_get_mut(&mut self, x: i32, y: i32) -> Option<&mut T> {
        if self.contains(x, y) {
            Some(&mut self.values[x as usize + y as usize * self.width])
        } else {
            None
        }
    }

    pub fn contains(&self, x: i32, y: i32) -> bool {
        x >= 0 && (x as usize) < self.width && y >= 0 && (y as usize) < self.height
    }

    pub fn enumerate(&self) -> Enumerate {
        Enumerate {
            width: self.width,
            height: self.height,
            x: 0,
            y: 0,
        }
    }

    pub fn neighbors<'a>(&'a self, x: i32, y: i32) -> Neighbors<'a, T> {
        Neighbors {
            grid: self,
            x,
            y,
            index: 0,
        }
    }

    pub fn rotate_ccw(&mut self) {
        assert_eq!(self.width, self.height);

        let size = self.width;
        for x in 0..(size / 2) {
            let tx = size - x - 1;
            for y in 0..(size / 2) {
                let ty = size - y - 1;
                let indices = [
                    x + y * size,
                    ty + x * size,
                    tx + ty * size,
                    y + tx * size,
                ];
                self.values.swap(indices[0], indices[1]);
                self.values.swap(indices[0], indices[2]);
                self.values.swap(indices[0], indices[3]);
            }
        }
    }

    pub fn rotate_half(&mut self) {
        assert_eq!(self.width, self.height);

        let size = self.width;
        for x in 0..(size / 2) {
            let tx = size - x - 1;
            for y in 0..size {
                let ty = size - y - 1;
                self.values.swap(x + y * size, tx + ty * size);
            }
        }
    }

    pub fn rotate_cw(&mut self) {
        assert_eq!(self.width, self.height);

        let size = self.width;
        for x in 0..(size / 2) {
            let tx = size - x - 1;
            for y in 0..(size / 2) {
                let ty = size - y - 1;
                let indices = [
                    x + y * size,
                    ty + x * size,
                    tx + ty * size,
                    y + tx * size,
                ];
                self.values.swap(indices[0], indices[3]);
                self.values.swap(indices[0], indices[2]);
                self.values.swap(indices[0], indices[1]);
            }
        }
    }

    pub fn flip_horiz(&mut self) {
        for x in 0..(self.width / 2) {
            let tx = self.width - x - 1;
            for y in 0..self.height {
                self.values.swap(x + y * self.width, tx + y * self.width);
            }
        }
    }

    pub fn flip_vert(&mut self) {
        for x in 0..self.width {
            for y in 0..(self.height / 2) {
                let ty = self.height - y - 1;
                self.values.swap(x + y * self.width, x + ty * self.width);
            }
        }
    }
}

impl<T: Clone> Grid<T> {
    pub fn slice(&self, x: i32, y: i32, width: usize, height: usize) -> Self {
        Self::new_with(width, height, |dx, dy| self.get(x + dx, y + dy).clone())
    }

    pub fn blit(&mut self, x: i32, y: i32, other: &Grid<T>) {
        for dx in 0..other.width {
            for dy in 0..other.height {
                *self.get_mut(x + dx as i32, y + dy as i32) = other.get(dx as i32, dy as i32).clone();
            }
        }
    }
}

pub struct Enumerate {
    width: usize,
    height: usize,
    x: usize,
    y: usize,
}

impl Iterator for Enumerate {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y < self.height {
            let result = (self.x as i32, self.y as i32);
            if self.x + 1 < self.width {
                self.x += 1;
                Some(result)
            } else {
                self.x = 0;
                self.y += 1;
                Some(result)
            }
        } else {
            None
        }
    }
}

pub struct Neighbors<'a, T> {
    grid: &'a Grid<T>,
    x: i32,
    y: i32,
    index: u8,
}

impl<'a, T> Iterator for Neighbors<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        const OFFSET_X: [i32; 8] = [
            -1,  0,  1,
            -1,      1,
            -1,  0,  1,
        ];
        const OFFSET_Y: [i32; 8] = [
            -1, -1, -1,
             0,      0,
             1,  1,  1,
        ];

        loop {
            if self.index < 8 {
                let value = self.grid.try_get(self.x + OFFSET_X[self.index as usize], self.y + OFFSET_Y[self.index as usize]);
                self.index += 1;
                if value.is_some() {
                    break value;
                }
            } else {
                break None;
            }
        }
    }
}
