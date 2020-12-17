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

    pub fn set(&mut self, x: i32, y: i32, value: T) {
        if self.contains(x, y) {
            self.values[x as usize + y as usize * self.width] = value;
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
