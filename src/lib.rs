use core::{fmt, panic};
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::time::{Instant, Duration};


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Direction {
    LEFT,
    RIGHT,
    UP,
    DOWN,
    NONE,
}

impl Direction {
    fn random() -> Vec<Direction> {
        let mut rng = rand::thread_rng();
        let mut variants = vec![Direction::LEFT, Direction::RIGHT, Direction::UP, Direction::DOWN];
        variants.shuffle(&mut rng);
        variants
    }

    fn oposite(&self) -> Self {
        match self {
            Direction::LEFT => Direction::RIGHT,
            Direction::RIGHT => Direction::LEFT,
            Direction::UP => Direction::DOWN,
            Direction::DOWN => Direction::UP,
            _ => Direction::NONE,
        }
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::LEFT
    }
}

#[derive(Default, Clone, Eq, PartialEq, Hash)]
pub struct Cell {
    pub enter: Direction,
    pub paths: Vec<(Direction, (usize, usize))>, // Store the direction and the position of the next cell
    pub is_end: bool,
}

pub struct Maze {
    pub cells: Vec<Vec<Cell>>,
    maze_config: MazeConfig,
    pub gen_time: u128,
}

pub struct MazeConfig {
    pub width: usize,
    pub height: usize,
    pub start_point: (usize, usize),
    pub end_point: (usize, usize),
}

impl Maze {
    pub fn new(maze_config: MazeConfig) -> Maze {
        let cells: Vec<Vec<Cell>> = vec![vec![Cell::default(); maze_config.width]; maze_config.height];
        let mut maze = Maze {
            cells,
            maze_config,
            gen_time: 0,
        };
        let time = Instant::now();
        maze.depth_first_search(maze.maze_config.start_point, &mut HashMap::new());
        maze.gen_time = time.elapsed().as_millis();
        maze
    }

    fn depth_first_search(&mut self, cell_pos: (usize, usize), cells_visited: &mut HashMap<(usize, usize), ()>) {
        // Mark cell as visited
        cells_visited.insert(cell_pos, ());


        let ran_directions = Direction::random();

        for ran in ran_directions {
            let next_pos = match ran {
                Direction::LEFT => (cell_pos.0, cell_pos.1.saturating_sub(1)),
                Direction::RIGHT => (cell_pos.0, cell_pos.1 + 1),
                Direction::UP => (cell_pos.0.saturating_sub(1), cell_pos.1),
                Direction::DOWN => (cell_pos.0 + 1, cell_pos.1),
                _ => continue,
            };

            if let Some(row) = self.cells.get_mut(next_pos.0) {
                if let Some(_next_cell) = row.get_mut(next_pos.1) {
                    // Check if next cell has been visited
                    if cells_visited.contains_key(&next_pos) {
                        continue;
                    }

                    if cell_pos == self.maze_config.end_point {
                        self.cells[cell_pos.0][cell_pos.1].is_end = true;
                    }

                    // Update direction and add to paths
                    self.cells[cell_pos.0][cell_pos.1].paths.push((ran, next_pos));
                    self.cells[next_pos.0][next_pos.1].enter = ran.oposite();

                    // Recursive call
                    self.depth_first_search(next_pos, cells_visited);
                }
            }
        }
    }
}

fn cell_drawing(directions: &mut Vec<Direction>) -> &'static str{
    directions.sort();
    
    match directions.as_slice() {
        // Single Direction
        [Direction::LEFT] => "─",
        [Direction::RIGHT] => "─",
        [Direction::UP] => "│",
        [Direction::DOWN] => "│",
        
        // Two Directions
        [Direction::LEFT, Direction::RIGHT] => "─",
        [Direction::LEFT, Direction::UP] => "┘",
        [Direction::LEFT, Direction::DOWN] => "┐",
        [Direction::RIGHT, Direction::UP] => "└",
        [Direction::RIGHT, Direction::DOWN] => "┌",
        [Direction::UP, Direction::DOWN] => "│",
        
        // Three Directions
        [Direction::LEFT, Direction::RIGHT, Direction::UP] => "┴",
        [Direction::LEFT, Direction::RIGHT, Direction::DOWN] => "┬",
        [Direction::LEFT, Direction::UP, Direction::DOWN] => "┤",
        [Direction::RIGHT, Direction::UP, Direction::DOWN] => "├",
        
        // Four Directions
        [Direction::LEFT, Direction::RIGHT, Direction::UP, Direction::DOWN] => "┼",

        [Direction::LEFT, Direction::LEFT] => "■",
        
        // Default case if no match
        _ => panic!("{:?}", directions),
    }
}

//display maze to terminal
impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        for rows in self.cells.clone() {
            for c in rows {
                let mut directions: Vec<Direction> = c.paths.iter()
                    .map(|(direction, _)| direction.clone()) // Clone the direction to create a new Vec
                    .collect();
                directions.push(c.enter);
                write!(f, "{}", cell_drawing(&mut directions))?;

            }
            write!(f, "\n")?;
        }
        write!(f, "Maze") 
    }
}

