use clap::Parser;
use graphics::{clear, Rectangle};
use piston::WindowSettings;
use piston_window::PistonWindow;

mod grid;
mod maze_gen_algos;

/// View Maze generator algorithms
#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct CmdOpts {
    /// the resolution of the app in pixels, where (w, h) = (rez, rez)
    #[arg(default_value_t = 720)]
    win_size: u32,
    /// Number of rows/columns the maze will have
    #[arg(default_value_t = 36)]
    nr_rows: u32,
}

fn main() {
    let args = CmdOpts::parse();

    const BG_COL: [f32; 4] = [0.; 4];
    const FG_COL: [f32; 4] = [1.; 4];

    let block_widget = Rectangle::new(FG_COL);
    let rect_size: f64 = (args.win_size as f64) / (args.nr_rows * 2 + 1) as f64;

    let mut window: PistonWindow =
        WindowSettings::new("Maze Generator", (args.win_size, args.win_size))
            .exit_on_esc(true)
            .resizable(true)
            .build()
            .expect("Failed to build window");

    let maze = maze_gen_algos::create_rand_prim(args.nr_rows, None, None);

    while let Some(event) = window.next() {
        window.draw_2d(&event, |ctx, graphics, _| {
            clear(BG_COL, graphics);

            for i in 0..maze.size {
                for j in 0..maze.size {
                    if let grid::Cell::Wall = maze.get_cell(i, j).unwrap() {
                        let rect_pos = ((rect_size * i as f64), (rect_size * j as f64));
                        block_widget.draw(
                            [rect_pos.0, rect_pos.1, rect_size, rect_size],
                            &ctx.draw_state,
                            ctx.transform,
                            graphics,
                        );
                    }
                }
            }
        });
    }
}
