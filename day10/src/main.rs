use std::{env, fs};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Tile {
    PIPE(Direction, Direction),
    GROUND,
    START,
}

impl Tile {
    fn connects_from(&self, direction: &Direction) -> bool {
        match self {
            Tile::PIPE(dir1, dir2) => dir1 == direction || dir2 == direction,
            Tile::GROUND => false,
            Tile::START => true,
        }
    }
    fn are_connected(&self, other: &Tile, direction: &Direction) -> bool {
        self.connects_from(direction) && other.connects_from(&direction.opposite())
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

impl Direction {
    fn offset(&self, pos: &Position) -> Option<Position> {
        let (row_offset, col_offset) = match self {
            Direction::NORTH => (OffSet::NEG(1), OffSet::POS(0)),
            Direction::EAST => (OffSet::POS(0), OffSet::POS(1)),
            Direction::SOUTH => (OffSet::POS(1), OffSet::POS(0)),
            Direction::WEST => (OffSet::POS(0), OffSet::NEG(1)),
        };
        let row = match row_offset {
            OffSet::POS(offset) => pos.row.checked_add(offset),
            OffSet::NEG(offset) => pos.row.checked_sub(offset),
        };
        let col = match col_offset {
            OffSet::POS(offset) => pos.col.checked_add(offset),
            OffSet::NEG(offset) => pos.col.checked_sub(offset),
        };
        match (row, col) {
            (Some(row), Some(col)) => Some(Position { row, col }),
            _ => None,
        }
    }
    fn opposite(&self) -> Direction {
        match self {
            Direction::NORTH => Direction::SOUTH,
            Direction::EAST => Direction::WEST,
            Direction::SOUTH => Direction::NORTH,
            Direction::WEST => Direction::EAST,
        }
    }
}

enum OffSet {
    POS(usize),
    NEG(usize),
}

#[derive(Debug)]
struct Position {
    row: usize,
    col: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(&args[1]).unwrap();
    let grid: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '|' => Tile::PIPE(Direction::NORTH, Direction::SOUTH),
                    '-' => Tile::PIPE(Direction::WEST, Direction::EAST),
                    'L' => Tile::PIPE(Direction::NORTH, Direction::EAST),
                    'J' => Tile::PIPE(Direction::NORTH, Direction::WEST),
                    '7' => Tile::PIPE(Direction::WEST, Direction::SOUTH),
                    'F' => Tile::PIPE(Direction::SOUTH, Direction::EAST),
                    '.' => Tile::GROUND,
                    'S' => Tile::START,
                    _ => panic!(),
                })
                .collect()
        })
        .collect();
    let size = (grid.len(), grid[0].len());
    let start = ravel(
        grid.iter()
            .flatten()
            .enumerate()
            .find(|(_, &tile)| tile == Tile::START)
            .unwrap()
            .0,
        size,
    );
    let loop_length = find_loop_length(&grid, &start, &Direction::NORTH);
    println!("Part 1 answer: {}", loop_length / 2)
}

fn ravel(index: usize, size: (usize, usize)) -> Position {
    let row = index / size.0;
    let col = index % size.1;
    Position { row, col }
}

fn get(grid: &Vec<Vec<Tile>>, pos: &Position) -> Option<Tile> {
    match grid.get(pos.row) {
        Some(&ref row) => row.get(pos.col).copied(),
        None => None,
    }
}

const DIRECTIONS: [Direction; 4] = [
    Direction::NORTH,
    Direction::WEST,
    Direction::SOUTH,
    Direction::EAST,
];

fn find_loop_length(
    grid: &Vec<Vec<Tile>>,
    current_pos: &Position,
    previous_dir: &Direction,
) -> usize {
    let current_tile = get(grid, current_pos).unwrap();
    let (current_dir, next_pos, next_tile) = DIRECTIONS
        .iter()
        .filter(|&candidate_dir| candidate_dir != previous_dir)
        .filter_map(|candidate_dir| match candidate_dir.offset(current_pos) {
            Some(candidate_pos) => Some((candidate_dir, candidate_pos)),
            None => None,
        })
        .filter_map(
            |(candidate_dir, candidate_pos)| match get(grid, &candidate_pos) {
                Some(candidate_tile) => Some((candidate_dir, candidate_pos, candidate_tile)),
                None => None,
            },
        )
        .filter(|(candidate_dir, _, candidate_tile)| {
            current_tile.are_connected(candidate_tile, candidate_dir)
        })
        .next()
        .unwrap();
    match next_tile {
        Tile::START => 1,
        _ => find_loop_length(grid, &next_pos, &current_dir.opposite()) + 1,
    }
}
