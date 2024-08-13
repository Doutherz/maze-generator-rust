use maze_project::*;
use std::fs::File;
use std::io::Write;
use std::thread::Builder;
use std::io;
use std::io::Result;

fn main() -> Result<()>{
    const STACK_SIZE: usize = 512 * 1024 * 1024;
    let handle = Builder::new()
        .stack_size(STACK_SIZE)
        .spawn(|| {
            let maze = Maze::new(MazeConfig{
                width: 30,
                height: 30,
                start_point: (0, 15),
                end_point: (9, 9),
            });
            println!("{}", maze);
            println!("Time in milisecs: {}", maze.gen_time);

            //let text = maze.to_string();
            //let mut file = File::create("maze.txt").unwrap();
            //file.write_all(text.as_bytes()).unwrap();
            
        }).expect("Error with thread");

    handle.join().expect("Thread panicked");
    println!("Press Enter to exit...");
    let _ = io::stdin().read_line(&mut String::new());

    Ok(())
}
