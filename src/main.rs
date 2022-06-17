extern crate piston_window;

use piston::WindowSettings;
use piston_window::*;

#[derive(Debug, Copy, Clone)]
struct Block {

}

impl Block {
    fn new() -> Block {
        Block {}
    }
}

struct Grid {
    size: u32,
    cells: Vec<Block>,
}

impl Grid {
    fn new(size: u32) -> Grid {
        Grid { size: size, cells: vec![Block::new(); (size * size) as usize] }
    }
}

struct App
{
    size: u32,
    grid: Grid,
}

impl App {
    fn new(window_rez: u32, grid_size: u32) -> App {
        if grid_size == 0 || window_rez == 0 {
            panic!("App size cannot be 0!");
        }

        if grid_size > window_rez {
            panic!("grid_size > window_rez makes the blocks smaller than a pixel!");
        }

        App { size: window_rez, grid: Grid::new(grid_size) }
    }
}

fn main() {
    let app = App::new(700, 101);

    let mut window: PistonWindow = WindowSettings::new("Maze Generator", (app.size, app.size))
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .expect("Failed to build window");

    while let Some(event) = window.next() {
        window.draw_2d(&event, |ctx, graphics, _| {
            const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
            clear(BLACK, graphics);

            let rectangle = Rectangle::new([1.0, 1.0, 1.0, 1.0]);
            let rect_size = app.size as f64 / app.grid.size as f64;
            for i in 0..app.grid.size as i32 {
                for j in 0..app.grid.size as i32 {
                    if i % 2 == 0 || j % 2 == 0 {
                        rectangle.draw([i as f64 * rect_size, j as f64 * rect_size, rect_size, rect_size], &ctx.draw_state, ctx.transform, graphics);
                    }
                }
            }
        });
    }
}
