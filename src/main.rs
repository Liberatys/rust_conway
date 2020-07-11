use ggez::event::{self, EventHandler};

use ggez::event::{KeyCode, KeyMods};
use ggez::{graphics, Context, ContextBuilder, GameResult};
use rand::prelude::*;
use std::time::{Duration, Instant};
fn main() {
    // Make a Context.
    let (mut ctx, mut event_loop) = ContextBuilder::new("Conway's", "Game of life")
        .window_setup(ggez::conf::WindowSetup::default().title("Conway's Game of life"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(SCREEN_LENGTH_AND_WIDTH, SCREEN_LENGTH_AND_WIDTH),
        )
        .build()
        .expect("aieee, could not create ggez context!");
    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let mut my_game = Conway::new(&mut ctx);
    // Run!
    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occured: {}", e),
    }
}

struct Cell {}

impl Cell {
    pub fn new_state(old_state: bool, sur_cells: Vec<bool>) -> bool {
        let truth_count = sur_cells.iter().filter(|&n| *n == true).count();
        if old_state {
            if truth_count < 2 {
                false
            } else if truth_count == 2 || truth_count == 3 {
                true
            } else {
                false
            }
        } else {
            if truth_count == 3 {
                true
            } else {
                false
            }
        }
    }
}

impl Grid {
    pub fn new() -> Self {
        Self { cells: Vec::new() }
    }

    pub fn print(&self) {
        for x in 0..self.cells.len() {
            println!("{:?}", self.cells[x]);
        }
    }

    pub fn random_pop(&mut self, size_x: usize, size_y: usize) {
        let mut cells: Vec<Vec<bool>> = Vec::new();
        let mut rng = thread_rng();
        for x in 0..size_x {
            cells.push(Vec::new());
            for y in 0..size_y {
                let random_value = rng.gen_range(0, 6);
                if random_value == 2 {
                    cells[x].push(true);
                } else {
                    cells[x].push(false);
                }
            }
        }
        self.cells = cells;
    }

    pub fn get_cells(&self) -> Vec<Vec<bool>> {
        return self.cells.clone();
    }

    pub fn get_cell_value(&self, x: usize, y: usize) -> bool {
        return self.cells[y][x];
    }

    pub fn get_sur_cells(&self, x: usize, y: usize) -> Vec<bool> {
        let mut sur_cells: Vec<bool> = Vec::new();
        // 0 == top / left and len() - 1 == right / bottom
        let left_right_x_start = if x == 0 {
            (false, true)
        } else if x == self.cells.len() - 1 {
            (true, false)
        } else {
            (true, true)
        };
        let top_bottom_y_start = if y == 0 {
            (false, true)
        } else if y == self.cells[x].len() - 1 {
            (true, false)
        } else {
            (true, true)
        };
        //check if surrounding tiles are available and not cut off by a border
        if left_right_x_start.0 == true {
            sur_cells.push(self.cells[x - 1][y]);
        }
        if left_right_x_start.1 == true {
            sur_cells.push(self.cells[x + 1][y]);
        }
        if top_bottom_y_start.0 == true {
            sur_cells.push(self.cells[x][y - 1]);
        }
        if top_bottom_y_start.1 == true {
            sur_cells.push(self.cells[x][y + 1]);
        }
        if left_right_x_start.0 == true && top_bottom_y_start.0 == true {
            sur_cells.push(self.cells[x - 1][y - 1]);
        }
        if left_right_x_start.0 == true && top_bottom_y_start.1 == true {
            sur_cells.push(self.cells[x - 1][y + 1]);
        }
        if left_right_x_start.1 == true && top_bottom_y_start.0 == true {
            sur_cells.push(self.cells[x + 1][y - 1]);
        }
        if left_right_x_start.1 == true && top_bottom_y_start.1 == true {
            sur_cells.push(self.cells[x + 1][y + 1]);
        }
        sur_cells
    }

    pub fn set_cells(&mut self, cells: Vec<Vec<bool>>) {
        self.cells = cells;
    }
}

const UPDATES_PER_SECOND: f32 = 8.0;
const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;
const SCREEN_LENGTH_AND_WIDTH: f32 = 1200.0;
struct Conway {
    grid: Grid,
    time_past_since_update: Instant,
    ratio: f32,
    spaced: bool,
}

struct Grid {
    // col x row
    cells: Vec<Vec<bool>>,
}
impl Conway {
    pub fn new(_ctx: &mut Context) -> Self {
        let mut con = Self {
            grid: Grid::new(),
            ratio: 0.0,
            spaced: false,
            time_past_since_update: Instant::now(),
        };
        //ration 1 to 8 for screen size
        con.grid.random_pop(50, 50);
        con.ratio = SCREEN_LENGTH_AND_WIDTH / 50.0;
        con
    }
}

impl EventHandler for Conway {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        if (self.spaced) {
            self.spaced = false;
            self.grid.random_pop(50, 50);
        }
        if Instant::now() - self.time_past_since_update >= Duration::from_millis(MILLIS_PER_UPDATE)
        {
            let mut new_state = self.grid.get_cells();
            for x in 0..new_state.len() {
                for y in 0..new_state[x].len() {
                    new_state[x][y] =
                        Cell::new_state(new_state[x][y], self.grid.get_sur_cells(x, y));
                }
            }
            self.grid.set_cells(new_state);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        for x in 0..self.grid.cells.len() {
            for y in 0..self.grid.cells[x].len() {
                if self.grid.cells[x][y] == true {
                    let rectangle = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new(
                            x as f32 * self.ratio,
                            y as f32 * self.ratio,
                            self.ratio,
                            self.ratio,
                        ),
                        graphics::WHITE,
                    )?;
                    graphics::draw(ctx, &rectangle, (ggez::mint::Point2 { x: 0.0, y: 0.0 },))?;
                }
            }
        }
        graphics::present(ctx)
    }

    /// key_down_event gets fired when a key gets pressed.
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        // Here we attempt to convert the Keycode into a Direction using the helper
        // we defined earlier.
        if keycode == KeyCode::Space {
            self.spaced = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cell_collection() {
        let mut grid = Grid::new();
        grid.set_cells(vec![
            vec![false, true, true],
            vec![false, true, true],
            vec![false, true, true],
        ]);
        assert_eq!(grid.get_sur_cells(1, 1).len(), 8);
        assert_eq!(grid.get_sur_cells(0, 0).len(), 3);
        assert_eq!(grid.get_sur_cells(0, 1).len(), 5);
        assert_eq!(grid.get_sur_cells(0, 2).len(), 3);
        assert_eq!(grid.get_sur_cells(1, 0).len(), 5);
        assert_eq!(grid.get_sur_cells(1, 0).len(), 5);
    }
}

