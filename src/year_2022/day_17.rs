
use crate::utils::Point2D;

use fxhash::FxHashMap;

struct Shape {
    mask: u32,
    height: usize,
    width: usize
}

impl Shape {

    fn new(mask: [u8; 4], height: usize, width: usize) -> Self {
        Self {
            mask: u32::from_le_bytes(mask),
            height,
            width
        }
    }

}

struct TowerState<'a> {
    grid: Vec<u8>,
    input_stream: &'a [u8],
    input_index: usize,
    shapes: [Shape; 5],
    shape_index: usize,
    top_index: usize
}

impl<'a> TowerState<'a> {

    fn new(max_height: usize, input_stream: &'a [u8]) -> Self {
        Self {
            grid: vec![0u8; max_height],
            input_stream,
            input_index: 0,
            shapes: [
                Shape::new([0b01111000, 0b00000000, 0b00000000, 0b00000000], 1, 4),
                Shape::new([0b00100000, 0b01110000, 0b00100000, 0b00000000], 3, 3),
                Shape::new([0b01110000, 0b00010000, 0b00010000, 0b00000000], 3, 3),
                Shape::new([0b01000000, 0b01000000, 0b01000000, 0b01000000], 4, 1),
                Shape::new([0b01100000, 0b01100000, 0b00000000, 0b00000000], 2, 2)
            ],
            shape_index: 0,
            top_index: 0
        }
    }

    fn drop_block(&mut self) {
        let shape = &self.shapes[self.shape_index];
        self.shape_index = (self.shape_index + 1) % self.shapes.len();

        let mut shape_position = Point2D::new(2, (self.top_index + 3) as isize);
        let mut grid_mask = 0;
        loop {
            let mut next_position = shape_position;
            match self.input_stream[self.input_index] {
                b'<' => {
                    if next_position.column_index() > 0 {
                        next_position.x -= 1;
                    }
                },
                b'>' => {
                    if next_position.column_index() + shape.width < 7 {
                        next_position.x += 1;
                    }
                },
                dir => { panic!("Unrecognized direction: {}", dir); }
            }

            self.input_index = (self.input_index + 1) % self.input_stream.len();
            if (grid_mask & (shape.mask >> next_position.x)) != 0 {
                next_position = shape_position;

            } else {
                shape_position = next_position;
            }

            if next_position.y == 0 {
                break;
            }

            next_position.y -= 1;
            let next_grid_mask = (grid_mask << 8) | (self.grid[next_position.row_index()] as u32);
            if (next_grid_mask & (shape.mask >> shape_position.x)) != 0 {
                break;
            }

            shape_position = next_position;
            grid_mask = next_grid_mask;
        }

        grid_mask |= shape.mask >> shape_position.x;
        self.grid[shape_position.row_index()..shape_position.row_index() + 4].copy_from_slice(&grid_mask.to_le_bytes());
        self.top_index = std::cmp::max(self.top_index, shape_position.row_index() + shape.height);
    }

    fn get_state_key(&self) -> (usize, usize, [u8; 16]) {
        let mut grid_state = [0; 16];
        for r in 0..std::cmp::min(self.top_index, grid_state.len()) {
            grid_state[grid_state.len() - r - 1] = self.grid[self.top_index - r - 1];
        }

        (self.input_index, self.shape_index, grid_state)
    }

}

fn run_simulation<const MAX_ITERATION: usize>(input: &str) -> u64 {
    let mut iteration_count = 0;
    let mut height_adjustment = 0;
    let mut state_cache = Some(FxHashMap::default());
    let mut tower = TowerState::new(10000, input.trim().as_bytes());
    while iteration_count < MAX_ITERATION {
        tower.drop_block();
        iteration_count += 1;
        if let Some(mut cache) = state_cache.take() {
            let state_key = tower.get_state_key();
            if let Some((previous_iteration_count, previous_tower_height)) = cache.get(&state_key) {
                let cycle_iteration_count = iteration_count - previous_iteration_count;
                let cycle_tower_height = tower.top_index - previous_tower_height;

                let repeat_count = (MAX_ITERATION - iteration_count) / cycle_iteration_count;
                height_adjustment = repeat_count * cycle_tower_height;
                iteration_count += cycle_iteration_count * repeat_count;  

            } else {
                cache.insert(state_key, (iteration_count, tower.top_index));
                state_cache = Some(cache);
            }
        }
    }

    (tower.top_index + height_adjustment) as u64
}

pub fn part1(input: &str) -> u64 {
    run_simulation::<2022>(input)
}

pub fn part2(input: &str) -> u64 {
    run_simulation::<1000000000000>(input)
}