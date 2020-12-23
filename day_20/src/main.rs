use std::{io, num::ParseIntError};
use grid::Grid;
use problem::{Problem, ProblemInput, solve};

#[derive(Clone, Copy, Debug)]
struct Transform {
    rotation: u8,
    reflection: bool,
}

impl Transform {
    fn combine(&self, other: &Self) -> Self {
        if !self.reflection {
            Self {
                rotation: (self.rotation + other.rotation) % 4,
                reflection: other.reflection,
            }
        } else {
            Self {
                rotation: (self.rotation + 4 - other.rotation) % 4,
                reflection: self.reflection != other.reflection,
            }
        }
    }

    fn transform(&self, x: i32, y: i32) -> (i32, i32) {
        if !self.reflection {
            let tx = [x, -y, -x, y];
            let ty = [y, x, -y, -x];
            (tx[self.rotation as usize], ty[self.rotation as usize])
        } else {
            let tx = [y, x, -y, -x];
            let ty = [x, -y, -x, y];
            (tx[self.rotation as usize], ty[self.rotation as usize])
        }
    }
}

#[derive(Debug)]
struct Tile {
    id: u64,
    sides: [u16; 4],
    inner: Grid<bool>,
}

impl Tile {
    fn reversed_sides(&self) -> [u16; 4] {
        [
            self.sides[1].reverse_bits() >> 6,
            self.sides[0].reverse_bits() >> 6,
            self.sides[3].reverse_bits() >> 6,
            self.sides[2].reverse_bits() >> 6,
        ]
    }
}

struct Input {
    tiles: Vec<Tile>,
}

#[derive(Debug)]
enum ParseInputError {
    ParseIntError(ParseIntError),
    IoError(io::Error),
    MissingTileLine(usize),
}

impl From<ParseIntError> for ParseInputError {
    fn from(e: ParseIntError) -> Self {
        Self::ParseIntError(e)
    }
}

impl From<io::Error> for ParseInputError {
    fn from(e: io::Error) -> Self {
        Self::IoError(e)
    }
}

impl ProblemInput for Input {
    type Error = ParseInputError;

    fn parse<R: io::BufRead>(reader: R) -> Result<Self, Self::Error> {
        let mut lines = reader.lines();
        let mut tiles = Vec::new();
        while let Some(line) = lines.next() {
            let id = line?[5..9].parse()?;

            let mut grid = Grid::new(10, 10);
            for y in (0..10).rev() {
                let line = lines.next().ok_or(ParseInputError::MissingTileLine(y))??;
                for (x, c) in line.chars().enumerate() {
                    *grid.get_mut(x as i32, y as i32) = c == '#';
                }
            }

            let mut sides = [0, 0, 0, 0];
            for i in 0..10 {
                if *grid.get(9, i) {
                    sides[0] |= 1 << i;
                }
                if *grid.get(9 - i, 9) {
                    sides[1] |= 1 << i;
                }
                if *grid.get(0, 9 - i) {
                    sides[2] |= 1 << i;
                }
                if *grid.get(i, 0) {
                    sides[3] |= 1 << i;
                }
            }

            tiles.push(Tile {
                id,
                sides,
                inner: grid.slice(1, 1, 8, 8),
            });

            lines.next();
        }
        Ok(Self { tiles })
    }
}

fn reconstruct_image(tiles: &Vec<Tile>) -> Option<(Grid<bool>, Grid<u64>)> {
    let size = (tiles.len() as f32).sqrt().floor() as usize;

    let mut neighbor_transforms = vec![[None, None, None, None]; tiles.len()];

    for (i, tile) in tiles.iter().enumerate() {
        for (s, side) in tile.sides.iter().enumerate() {
            let pair = side.reverse_bits() >> 6;
            for (j, other) in tiles.iter().enumerate().filter(|&(j, _)| j != i) {
                if let Some(r) = other.sides.iter().position(|&s| s == pair) {
                    neighbor_transforms[i][s] = Some((j, Transform { rotation: (s as u8 + 4 - r as u8 + 2) % 4, reflection: false }));
                    break;
                } else if let Some(r) = other.reversed_sides().iter().position(|&s| s == pair) {
                    neighbor_transforms[i][s] = Some((j, Transform { rotation: (4 - s as u8 + r as u8 + 2) % 4, reflection: true }));
                    break;
                }
            }
        }
    }

    if let Some(corner) = neighbor_transforms.iter().position(|ns| ns.iter().filter(|n| n.is_none()).count() == 2) {
        if let Some(rotation) = match &neighbor_transforms[corner] {
            [_, _, None, None] => Some(0),
            [_, None, None, _] => Some(1),
            [None, None, _, _] => Some(2),
            [None, _, _, None] => Some(3),
            _ => None,
        } {
            let mut result_image = Grid::new(size * 8, size * 8);
            let mut result_ids = Grid::new(size, size);
    
            let mut queue = vec![((0, 0), corner, Transform { rotation, reflection: false })];

            while let Some(((x, y), index, tile_to_world)) = queue.pop() {
                if *result_ids.get(x, y) == 0 {
                    let mut image = tiles[index].inner.clone();
                    match tile_to_world.rotation {
                        0 => (),
                        1 => image.rotate_ccw(),
                        2 => image.rotate_half(),
                        3 => image.rotate_cw(),
                        _ => unreachable!(),
                    }
                    if tile_to_world.reflection {
                        image.flip_vert();
                        image.rotate_ccw();
                    }

                    result_image.blit(x * 8, y * 8, &image);
                    *result_ids.get_mut(x, y) = tiles[index].id;

                    const DX: [i32; 4] = [1, 0, -1, 0];
                    const DY: [i32; 4] = [0, 1, 0, -1];
                    for d in 0..4 {
                        if let Some((nindex, neighbor_to_normal)) = &neighbor_transforms[index][d] {
                            let (dx, dy) = tile_to_world.transform(DX[d], DY[d]);
                            let target = neighbor_to_normal.combine(&tile_to_world);
                            queue.push(((x + dx, y + dy), *nindex, target));
                        }
                    }
                }
            }
    
            Some((result_image, result_ids))
        } else {
            None
        }
    } else {
        None
    }
}

fn check_pattern(grid: &Grid<bool>, x: i32, y: i32, pattern: &Grid<bool>) -> bool {
    for (px, py) in pattern.enumerate() {
        if *pattern.get(px, py) && !grid.get(x + px, y + py) {
            return false;
        }
    }
    true
}

struct Day20;
impl Problem for Day20 {
    type Input = Input;
    type Part1Output = u64;
    type Part2Output = usize;
    type Error = ();

    fn part_1(input: &Self::Input) -> Result<Self::Part1Output, Self::Error> {
        if let Some((_, ids)) = reconstruct_image(&input.tiles) {
            Ok(
                ids.get(0, 0)
                * ids.get(ids.width() as i32 - 1, 0)
                * ids.get(0, ids.height() as i32 - 1)
                * ids.get(ids.width() as i32 - 1, ids.height() as i32 - 1)
            )
        } else {
            Err(())
        }
    }

    fn part_2(input: &Self::Input) -> Result<Self::Part2Output, Self::Error> {
        if let Some((mut image, _)) = reconstruct_image(&input.tiles) {
            const LINES: [&'static str; 3] = [
                "                  # ",
                "#    ##    ##    ###",
                " #  #  #  #  #  #   ",
            ];
            let pattern = Grid::new_with(20, 3, |x, y| LINES[2 - y as usize].chars().nth(x as usize).unwrap() == '#');

            let mut pattern_count = 0;
            'outer: for _ in 0..2 {
                for _ in 0..4 {
                    for x in 0..image.width() - pattern.width() {
                        for y in 0..image.height() - pattern.height() {
                            if check_pattern(&image, x as i32, y as i32, &pattern) {
                                pattern_count += 1;
                            }
                        }
                    }
                    if pattern_count > 0 {
                        break 'outer;
                    }
                    image.rotate_ccw();
                }
                image.flip_vert();
            }

            Ok(image.enumerate().filter(|&(x, y)| *image.get(x, y)).count() - pattern_count * 15)
        } else {
            Err(())
        }
    }
}

fn main() {
    solve::<Day20>("input").unwrap();
}
