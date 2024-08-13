# Maze Generator in Rust

This project is a Rust-based maze generator that creates mazes of customizable size and structure. The maze is generated with a start and end point, and the output is printed to the console.

## Features

- **Random Maze Generation**: Generates mazes with randomly shuffled directions.
- **Customizable Configurations**: Define the maze's width, height, and entry/exit points.
- **Performance Metrics**: Prints the time taken to generate the maze.

## Getting Started

### Prerequisites

- Ensure you have [Rust](https://www.rust-lang.org/tools/install) and [Cargo](https://crates.io/) installed on your system.

### Building and Running

1. **Clone the repository**:
   ```sh
   git clone <repository-url>
   cd <project-directory>
   
2. **Build the project**:
   ```sh
   cargo build -r
   
3. **Run the project**:
   ```sh
   cargo run -r
   ```
   
**Maze configuration**
   ```rust
   let maze = Maze::new(MazeConfig {
    width: 30,
    height: 30,
    start_point: (0, 0),
    end_point: (29, 29),
   });
   ```
   
**Example Output**
  ```sh
  ──┬┐│┌───┬───┐┌■─┬─┌──┬─┐┌┐─┬┐
  ┌─┘└┘││┌─┘┌┐─┘│┌┐└┐└┐││─┴┘└─┘│
  ├───┐││└─┐│└──┘│└┐└─┴┘└─┐───┬┘
  ├─┌┐│││┌┐││─┐┌─┴┐└────┐┌┘┌─┐└┐
  │┌┤└┘│││└┤└┐└┘┌┐│──┐┌─┴┘┌┘││┌┘
  │││┌─┘││┌┴─└──┘│├─┐└┴──┌┘┌┤│└─
  ││││┌┐│││┌───┬┐│││└──┐┌┘┌┘│└─┐
  │└┐├┘└┘└┐└─┐┌┘││└┤┌─┐│└┐└──┬┐│
  └┐└┘┌───┼─┌┘└─┐│┌┘└─│└─┴┐┌┐│││
  ││┌┐│─┬─┘┌┘┌─┐││└─┬─┴─┌┐└┘└┤┌┘
  └┴┘│└┐└┐┌┘┌┘─┴┤│──┘┌──┘└┐┌┐│└┐
  ┌──┬─┘┌┘││└──┐│└─┐┌┘┌───┘││├─│
  │┌┐│┌┐│┌┘├┐┌┐└┐┌─┘└┐│┌──┐│││┌┤
  ││└─┘└─┘┌┘││└─└┘┌┬─┘├┘─┐││└┘││
  ││┌┐┌┐┌─┘─┤└─┬──│││┌┘┌┐│││┌─└┐
  │└┘│││└──┐└─┐└──┘│└┘┌┘│├┘└┘┌┐│
  │┌─┘│└┐┌─┤──┴───┐└─┐│┌┘└───┘└┤
  │└──┘┌┘└┐│┌────┐└─┬┘│└─┐│┌─┬─│
  ├──┌─┘┌─┘││───┐│──┘┌┘┌─││││└─┘
  │┌─┘──┴┐┌┐│┌─┬┘└┐┌─┘─┴┐│├┴┘┌─┐
  ││┌┬───┘│└┘└┐│┌─┤└┐┌┐┌┘││┌─┘┌┤
  ├┘│└────┘──┬┘││─┤│└┘└┤┌┘│││┌┘│
  └┐└──┐┌┐┌──┘││└┐└┤─┐┌┘│─┘│││┌┘
  ┌┘┌─┐││└┘│┌─┴┘││─┴┐│└┐└──┴┘│││
  └┐│┌┘└┘┌─┴┘───┤└─┐└┘┌┘│┌───┘└┤
  ┌┘││───┴────┐┌┤┌┐└──┴┐││┌──┬┐│
  │┌┘└─┐┌─┐───┴┘├┘└─┌─┐└┘││┌┐│││
  └┘┌─┌┘│┌┘┌───┐│┌──┘│└──┘││└┘││
  ┌─┴┐└─┘└┐││┌─┘┌┘┌──┼─┌──┘│┌┐││
  └──└────┴┘└┴──┘─┴──└─┴───└┘└─┘
  Maze
  Time in milisecs: 0
  Press Enter to exit...
```
