use maze_project::*;

#[test]
fn test_maze() {
    let maze = Maze::new(MazeConfig{
        width: 10,
        height: 10,
        start_point: (0, 0),
        end_point: (9, 9),
    });

    for row in maze.cells {
        for c in row {
            assert!(c.is_end == Direction::NONE);
        }
    }
}

#[test]
fn end_test() {
    let maze = Maze::new(MazeConfig{
        width: 10,
        height: 10,
        start_point: (0, 0),
        end_point: (9, 9),
    });

    assert!(maze.cells[9][9].is_end);
}
