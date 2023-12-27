use std::collections::HashSet;
use std::iter::repeat;
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
    fn get_inside(&self, heading: &Direction, inside: &Side) -> Vec<Direction> {
        match self {
            Tile::PIPE(dir1, dir2) => {
                let n_intermediate = repeat(&Side::LEFT)
                    .scan(dir1.clone(), |dir, side| {
                        *dir = dir.turn(side);
                        match dir == dir2 {
                            true => None,
                            false => Some(*dir),
                        }
                    })
                    .count();
                let is_corner = match n_intermediate {
                    0 | 2 => true,
                    1 => false,
                    _ => panic!(),
                };
                match is_corner {
                    true => {
                        let inside_dir = heading.turn(inside);
                        match [dir1, dir2].into_iter().find(|&&dir| dir == inside_dir) {
                            Some(_) => Vec::new(),
                            None => DIRECTIONS
                                .into_iter()
                                .filter(|dir| dir != dir1 && dir != dir2)
                                .collect(),
                        }
                    }
                    false => vec![heading.turn(inside)],
                }
            }
            Tile::GROUND => Vec::new(),
            Tile::START => Vec::new(),
        }
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
    fn turn(&self, side: &Side) -> Direction {
        match (self, side) {
            (Direction::NORTH, Side::LEFT) | (Direction::SOUTH, Side::RIGHT) => Direction::WEST,
            (Direction::EAST, Side::LEFT) | (Direction::WEST, Side::RIGHT) => Direction::NORTH,
            (Direction::SOUTH, Side::LEFT) | (Direction::NORTH, Side::RIGHT) => Direction::EAST,
            (Direction::WEST, Side::LEFT) | (Direction::EAST, Side::RIGHT) => Direction::SOUTH,
        }
    }
}

enum OffSet {
    POS(usize),
    NEG(usize),
}

#[derive(Debug, Eq, Hash, Clone, Copy, PartialEq)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn get_heading(&self, next: &Position) -> Option<Direction> {
        match (
            next.row.checked_sub(self.row),
            next.col.checked_sub(self.col),
        ) {
            (None, Some(0)) => Some(Direction::NORTH),
            (Some(0), Some(1)) => Some(Direction::EAST),
            (Some(1), Some(0)) => Some(Direction::SOUTH),
            (Some(0), None) => Some(Direction::WEST),
            _ => None,
        }
    }
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
    let n_cols = grid[0].len();
    let start = ravel(
        grid.iter()
            .flatten()
            .enumerate()
            .find(|(_, &tile)| tile == Tile::START)
            .unwrap()
            .0,
        n_cols,
    );
    let loop_positions = get_loop_positions(&grid, &start, &Direction::WEST);
    println!("Part 1 answer: {}", loop_positions.len() / 2);
    let enclosed = get_enclosed(&grid, &loop_positions);
    println!("Part 2 answer: {}", enclosed.len());
}

fn ravel(index: usize, n_cols: usize) -> Position {
    let row = index / n_cols;
    let col = index % n_cols;
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

fn get_loop_positions(
    grid: &Vec<Vec<Tile>>,
    current_pos: &Position,
    previous_dir: &Direction,
) -> Vec<Position> {
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
            let are_connected = current_tile.are_connected(candidate_tile, candidate_dir);
            are_connected
        })
        .next()
        .unwrap();
    let mut positions = match next_tile {
        Tile::START => Vec::new(),
        _ => get_loop_positions(grid, &next_pos, &current_dir.opposite()),
    };
    positions.push(next_pos);
    positions
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Side {
    LEFT,
    RIGHT,
}
fn find_inside(perimeter: &Vec<Position>) -> Side {
    let headings: Vec<Direction> = perimeter
        .iter()
        .zip(perimeter.iter().cycle().skip(1))
        .map(|(pos1, pos2)| {
            pos1.get_heading(pos2).unwrap()
        })
        .collect();
    let turns: Vec<Side> = headings
        .iter()
        .zip(headings.iter().cycle().skip(1))
        .filter_map(|(current, next)| match (current, next) {
            (Direction::NORTH, Direction::WEST)
            | (Direction::WEST, Direction::SOUTH)
            | (Direction::SOUTH, Direction::EAST)
            | (Direction::EAST, Direction::NORTH) => Some(Side::LEFT),
            (Direction::NORTH, Direction::EAST)
            | (Direction::EAST, Direction::SOUTH)
            | (Direction::SOUTH, Direction::WEST)
            | (Direction::WEST, Direction::NORTH) => Some(Side::RIGHT),
            _ => None,
        })
        .collect();
    let n_left_turns = turns.iter().filter(|&&turn| turn == Side::LEFT).count();
    let n_right_turns = turns.len() - n_left_turns;
    match n_left_turns > n_right_turns {
        true => Side::LEFT,
        false => Side::RIGHT,
    }
}

fn walk<'a>(perimeter: &'a Vec<Position>) -> impl Iterator<Item = (Position, Direction)> + 'a {
    perimeter
        .iter()
        .zip(perimeter.iter().cycle().skip(1))
        .map(move |(current, next)| (current.clone(), current.get_heading(next).unwrap()))
}

fn get_enclosed(grid: &Vec<Vec<Tile>>, perimeter: &Vec<Position>) -> HashSet<Position> {
    let inside = find_inside(perimeter);
    walk(perimeter)
        .map(|(pos, heading)| {
            let tile = get(grid, &pos).unwrap();
            tile.get_inside(&heading, &inside)
                .into_iter()
                .map(move |dir| {
                    repeat(dir).scan(pos.clone(), |pos, dir| {
                        *pos = dir.offset(pos).unwrap();
                        match perimeter.iter().find(|&candidate| pos == candidate) {
                            Some(_) => None,
                            None => Some(*pos),
                        }
                    })
                })
                .flatten()
        })
        .flatten()
        .collect()
}
