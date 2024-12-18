
use crate::utils::{Vector2, gcd};

use std::collections::VecDeque;
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
    Unknown
}

impl Direction {

    fn all() -> [Direction; 4] {
        [
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::Up
        ]
    }

    fn reverse(self) -> Direction {
        use Direction::*;
        match self {
            Right => Left,
            Down => Up,
            Left => Right,
            Up => Down,
            Unknown => unreachable!()
        }
    }

    fn rotate_left(self) -> Direction {
        use Direction::*;
        match self {
            Right => Up,
            Down => Right,
            Left => Down,
            Up => Left,
            Unknown => unreachable!()
        }
    }

    fn rotate_right(self) -> Direction {
        use Direction::*;
        match self {
            Right => Down,
            Down => Left,
            Left => Up,
            Up => Right,
            Unknown => unreachable!()
        }
    }

    fn to_index(self) -> usize {
        use Direction::*;
        match self {
            Right => 0,
            Down => 1,
            Left => 2,
            Up => 3,
            Unknown => unreachable!()
        }
    }

    fn to_index_with_bias(self, bias: usize) -> usize {
        (self.to_index() + bias) % 4
    }

    fn to_unit_vector(self) -> Vector2 {
        use Direction::*;
        match self {
            Right => Vector2::new(1, 0),
            Down => Vector2::new(0, 1),
            Left => Vector2::new(-1, 0),
            Up => Vector2::new(0, -1),
            Unknown => unreachable!()
        }
    }

}

impl Default for Direction {

    fn default() -> Self {
        Direction::Unknown
    }

}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CubeFaceId {
    Top,
    Front,
    Bottom,
    Back,
    Left,
    Right,
    Unknown
}

impl CubeFaceId {

    fn all() -> [CubeFaceId; 6] {
        [
            CubeFaceId::Top,
            CubeFaceId::Front,
            CubeFaceId::Bottom,
            CubeFaceId::Back,
            CubeFaceId::Left,
            CubeFaceId::Right,
        ]
    }

    fn get_neighbors(self) -> [CubeFaceId; 4] {
        use CubeFaceId::*;
        match self {
            Top => [ Right, Front, Left, Back ],
            Front => [ Right, Bottom, Left, Top ],
            Bottom => [ Right, Back, Left, Front ],
            Back => [ Right, Bottom, Left, Top ],
            Left => [ Front, Bottom, Back, Top ],
            Right => [ Back, Bottom, Front, Top ],
            Unknown => unreachable!()
        }
    }

    fn to_index(self) -> usize {
        use CubeFaceId::*;
        match self {
            Top => 0,
            Front => 1,
            Bottom => 2,
            Back => 3,
            Left => 4,
            Right => 5,
            Unknown => unreachable!()
        }
    }

}

impl Default for CubeFaceId {

    fn default() -> Self {
        CubeFaceId::Unknown
    }

}

#[derive(Clone, Copy, Debug, Default)]
struct EdgeLink {
    dest_face: CubeFaceId,
    dest_dir: Direction
}

#[derive(Clone, Copy, Debug)]
struct CubeFace {
    x_offset: usize,
    y_offset: usize,
    edges: [EdgeLink; 4]
}

impl Default for CubeFace {

    fn default() -> Self {
        CubeFace {
            x_offset: usize::MAX,
            y_offset: usize::MAX,
            edges: Default::default()
        }
    }

}

impl Index<Direction> for CubeFace {

    type Output = EdgeLink;

    fn index(&self, index: Direction) -> &Self::Output {
        &self.edges[index.to_index()]
    }

}

impl IndexMut<Direction> for CubeFace {

    fn index_mut(&mut self, index: Direction) -> &mut Self::Output {
        &mut self.edges[index.to_index()]
    }

}

struct Cube<'a> {
    map: Vec<&'a [u8]>,
    instructions: &'a [u8],
    faces: [CubeFace; 6],
    face_size: isize
}

impl<'a> Index<CubeFaceId> for Cube<'a> {

    type Output = CubeFace;

    fn index(&self, index: CubeFaceId) -> &Self::Output {
        &self.faces[index.to_index()]
    }

}

impl<'a> IndexMut<CubeFaceId> for Cube<'a> {

    fn index_mut(&mut self, index: CubeFaceId) -> &mut Self::Output {
        &mut self.faces[index.to_index()]
    }

}

impl<'a> Cube<'a> {

    fn new(input: &'a str) -> Self {
        let map =
            input
            .lines()
            .take_while(|line| !line.is_empty())
            .map(|line| line.as_bytes())
            .collect::<Vec<_>>();

        let instructions =
            input
            .lines()
            .rev()
            .next()
            .unwrap()
            .as_bytes();

        let height = map.len();
        let width =
            map
            .iter()
            .map(|row| row.len())
            .fold(0, std::cmp::max);

        let mut cube = Cube {
            map,
            instructions,
            faces: Default::default(),
            face_size: gcd(height, width) as isize
        };

        cube.find_faces();
        cube
    }

    fn find_faces(&mut self) {
        let starting_col = self.map[0].iter().position(|v| *v == b'.').unwrap();
        let mut queue = VecDeque::with_capacity(6);
        queue.push_back((Vector2::new(starting_col as isize, 0), CubeFaceId::Top, CubeFaceId::Left, Direction::Right));
        while let Some((offset, face_id, prev_face_id, prev_face_dir)) = queue.pop_front() {
            {
                let face = &mut self[face_id];
                if face.x_offset != usize::MAX {
                    continue;
                }

                face.x_offset = offset.x_index();
                face.y_offset = offset.y_index();
            }

            let neighbors = face_id.get_neighbors();
            let mut face_map_bias = 0;
            while neighbors[prev_face_dir.reverse().to_index_with_bias(face_map_bias)] != prev_face_id {
                face_map_bias += 1;
            }

            for dir in Direction::all() {
                let new_offset =
                    Vector2::new(
                        offset.x() + (dir.to_unit_vector().x() * self.face_size),
                        offset.y() + (dir.to_unit_vector().y() * self.face_size)
                    );

                if new_offset.y() < 0 ||
                   new_offset.row_index() >= self.map.len() ||
                   new_offset.x() < 0 ||
                   new_offset.column_index() >= self.map[new_offset.row_index()].len() ||
                   self.map[new_offset.row_index()][new_offset.column_index()] == b' ' {

                    continue;
                }

                let new_dir = dir.reverse();
                let new_face_id = neighbors[dir.to_index_with_bias(face_map_bias)];

                {
                    let current_face = &mut self[face_id];

                    current_face[dir].dest_dir = dir;
                    current_face[dir].dest_face = new_face_id;
                }

                {
                    let new_face = &mut self[new_face_id];
                    new_face[new_dir].dest_dir = new_dir;
                    new_face[new_dir].dest_face = face_id;
                    queue.push_back((new_offset, new_face_id, face_id, dir));
                }
            }
        }
    }

    fn to_map_coords(&self, cube_point: (Vector2, CubeFaceId)) -> Vector2 {
        let face = &self[cube_point.1];
        Vector2::new(
            cube_point.0.x() + (face.x_offset as isize),
            cube_point.0.y() + (face.y_offset as isize))
    }

    fn translate_edge(&self, cube_point: (Vector2, CubeFaceId), dir: Direction) -> ((Vector2, CubeFaceId), Direction) {
        use Direction::*;

        let edge_mapping = &self[cube_point.1][dir];
        let new_coords = match (dir, edge_mapping.dest_dir) {
            (Right, Right) => {
                Vector2::new(0, cube_point.0.y())
            },
            (Down, Down) => {
                Vector2::new(cube_point.0.x(), 0)
            },
            (Left, Left) => {
                Vector2::new(self.face_size - 1, cube_point.0.y())
            },
            (Up, Up) => {
                Vector2::new(cube_point.0.x(), self.face_size - 1)
            },
            (Right, Down) => {
                Vector2::new(self.face_size - cube_point.0.y() - 1, 0)
            },
            (Down, Right) => {
                Vector2::new(0, self.face_size - cube_point.0.x() - 1)
            },
            (Down, Up) => {
                Vector2::new(self.face_size - cube_point.0.x() - 1, self.face_size - 1)
            },
            (Left, Up) => {
                Vector2::new(self.face_size - cube_point.0.y() - 1, self.face_size - 1)
            },
            (Up, Right) => {
                Vector2::new(0, cube_point.0.x())
            },
            (Left, Right) => {
                Vector2::new(0, self.face_size - cube_point.0.y() - 1)
            },
            (Left, Down) => {
                Vector2::new(cube_point.0.y(), 0)
            },
            (Down, Left) => {
                Vector2::new(self.face_size - 1, cube_point.0.x())
            },
            (Right, Left) => {
                Vector2::new(self.face_size - 1, self.face_size - cube_point.0.y() - 1)
            },
            (Right, Up) => {
                Vector2::new(cube_point.0.y(), self.face_size - 1)
            },
            (Up, Down) => {
                Vector2::new(self.face_size - cube_point.0.x() - 1, 0)
            },
            (Up, Left) => {
                Vector2::new(self.face_size - 1, cube_point.0.x())
            },
            (from_dir, to_dir) => panic!("Unhandled mapping {:?} -> {:?}", from_dir, to_dir)
        };

        ((new_coords, edge_mapping.dest_face), edge_mapping.dest_dir)
    }

    fn walk_map(&self) -> u64 {
        let mut current_dir = Direction::Right;
        let mut current_pos = (Vector2::new(0, 0), CubeFaceId::Top);
        let mut index = 0;
        while index < self.instructions.len() {
            if self.instructions[index] >= b'0' && self.instructions[index] <= b'9' {
                let mut distance = 0;
                while index < self.instructions.len() &&
                      self.instructions[index] >= b'0' &&
                      self.instructions[index] <= b'9' {

                    distance = (distance * 10) + ((self.instructions[index] - b'0') as u64);
                    index += 1;
                }

                for _ in 0..distance {
                    let mut next_pos = (current_pos.0 + current_dir.to_unit_vector(), current_pos.1);
                    let mut next_dir = current_dir;

                    if next_pos.0.x() < 0 ||
                       next_pos.0.x() >= self.face_size ||
                       next_pos.0.y() < 0 ||
                       next_pos.0.y() >= self.face_size {

                        let translated = self.translate_edge(next_pos, next_dir);
                        next_pos = translated.0;
                        next_dir = translated.1;
                    }

                    let map_coords = self.to_map_coords(next_pos);
                    if self.map[map_coords.row_index()][map_coords.column_index()] == b'#' {
                        break;
                    }

                    current_pos = next_pos;
                    current_dir = next_dir;
                }

            } else {
                current_dir = match self.instructions[index] {
                    b'R' => current_dir.rotate_right(),
                    b'L' => current_dir.rotate_left(),
                    _ => unreachable!()
                };

                index += 1;
            }
        }

        let map_coords = self.to_map_coords(current_pos);
        (((map_coords.row_index() + 1) * 1000) + ((map_coords.column_index() + 1) * 4) + current_dir.to_index()) as u64
    }

    fn try_stitch_face_right(&mut self, face_id: CubeFaceId, dir: Direction) -> bool {
        let right_rotate_edge = self[face_id][dir.rotate_right()];
        if right_rotate_edge.dest_face == CubeFaceId::Unknown {
            return false;
        }

        let seek_dir = right_rotate_edge.dest_dir.rotate_left();

        let seek_edge = self[right_rotate_edge.dest_face][seek_dir];
        if seek_edge.dest_face == CubeFaceId::Unknown {
            return false;
        }

        let dest_stitched_face = seek_edge.dest_face;
        let dest_stitched_dir = seek_edge.dest_dir.rotate_right();
        self[face_id][dir].dest_face = dest_stitched_face;
        self[face_id][dir].dest_dir = dest_stitched_dir;

        self[dest_stitched_face][dest_stitched_dir.reverse()].dest_face = face_id;
        self[dest_stitched_face][dest_stitched_dir.reverse()].dest_dir = dir.reverse();

        true
    }

    fn try_stitch_face_left(&mut self, face_id: CubeFaceId, dir: Direction) -> bool {
        let left_rotate_edge = self[face_id][dir.rotate_left()];
        if left_rotate_edge.dest_face == CubeFaceId::Unknown {
            return false;
        }

        let seek_dir = left_rotate_edge.dest_dir.rotate_right();

        let seek_edge = self[left_rotate_edge.dest_face][seek_dir];
        if seek_edge.dest_face == CubeFaceId::Unknown {
            return false;
        }

        let dest_stitched_face = seek_edge.dest_face;
        let dest_stitched_dir = seek_edge.dest_dir.rotate_left();
        self[face_id][dir].dest_face = dest_stitched_face;
        self[face_id][dir].dest_dir = dest_stitched_dir;

        self[dest_stitched_face][dest_stitched_dir.reverse()].dest_face = face_id;
        self[dest_stitched_face][dest_stitched_dir.reverse()].dest_dir = dir.reverse();

        true
    }

    fn stitch_cube_map(&mut self) {
        loop {
            let mut stitched_edge = false;
            for face_id in CubeFaceId::all() {
                for dir in Direction::all() {
                    if self[face_id][dir].dest_face == CubeFaceId::Unknown {
                        if self.try_stitch_face_right(face_id, dir) ||
                           self.try_stitch_face_left(face_id, dir) {

                            stitched_edge = true;
                        }
                    }
                }
            }

            if !stitched_edge {
                break;
            }
        }
    }

    fn stitch_flat_map(&mut self) {
        for face_id in CubeFaceId::all() {
            {
                let face = &self[face_id];
                if face[Direction::Up].dest_face != CubeFaceId::Unknown {
                    continue;
                }
            }

            let mut bottom_face = face_id;
            while self[bottom_face][Direction::Down].dest_face != CubeFaceId::Unknown {
                bottom_face = self[bottom_face][Direction::Down].dest_face;
            }

            {
                let top_face = &mut self[face_id];

                top_face[Direction::Up].dest_face = bottom_face;
                top_face[Direction::Up].dest_dir = Direction::Up;
            }

            {
                let bottom_face = &mut self[bottom_face];

                bottom_face[Direction::Down].dest_face = face_id;
                bottom_face[Direction::Down].dest_dir = Direction::Down;
            }
        }

        for face_id in CubeFaceId::all() {
            {
                let face = self[face_id];
                if face[Direction::Left].dest_face != CubeFaceId::Unknown {
                    continue;
                }
            }

            let mut right_face = face_id;
            while self[right_face][Direction::Right].dest_face != CubeFaceId::Unknown {
                right_face = self[right_face][Direction::Right].dest_face;
            }

            {
                let left_face = &mut self[face_id];

                left_face[Direction::Left].dest_face = right_face;
                left_face[Direction::Left].dest_dir = Direction::Left;
            }

            {
                let right_face = &mut self[right_face];

                right_face[Direction::Right].dest_face = face_id;
                right_face[Direction::Right].dest_dir = Direction::Right;
            }
        }
    }

}

pub fn part1(input: &str) -> u64 {
    let mut cube = Cube::new(input);
    cube.stitch_flat_map();
    cube.walk_map()
}

pub fn part2(input: &str) -> u64 {
    let mut cube = Cube::new(input);
    cube.stitch_cube_map();
    cube.walk_map()
}
