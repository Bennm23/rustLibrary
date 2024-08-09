use std::{ops::{Deref, DerefMut}, sync::{atomic::{AtomicBool, AtomicI8}, Arc, Mutex, MutexGuard}, thread, time::Duration};
extern crate gtk4 as gtk;

use gtk::{glib::timeout_add_local, prelude::{ApplicationExt, ApplicationExtManual, BoxExt, DrawingAreaExtManual, GtkWindowExt, WidgetExt}, prelude::GestureDragExt, Application, ApplicationWindow, Box, DrawingArea, GestureDrag};

use gtk::glib::clone;
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};


fn main() {
    println!("Hello, world!");

    gtk::init().expect("failed to init gtk");

    let app = Application::builder()
        .application_id("local.benn.falling_sand")
        .build();
    app.connect_activate(build_ui);
    app.run();
}

#[derive(Clone, Copy)]
struct Sand {
    present : bool,
    falling : bool,
}

// const ROWS: usize = 40;
// const COLS: usize = 40;
const SAND_SIZE: usize = 2;//Grain of sand size in pixels
const DRAW_SIZE: usize = 800;
const BRUSH_SIZE: usize = 10; //How many pixels to draw in
const COUNT : usize = DRAW_SIZE / SAND_SIZE;//Row/Col count

type Grid = Arc<Mutex<Vec<Vec<Sand>>>>;

fn build_ui(application : &gtk::Application) {

    let window = ApplicationWindow::builder()
        .application(application)
        .title("Falling Sand")
        .default_width(1000)
        .default_height(1000)
        .build();

    let drawing_area = DrawingArea::builder()
        .content_width(DRAW_SIZE as i32)
        .content_height(DRAW_SIZE as i32)
        .build();

    let grid : Grid = Arc::new(Mutex::new(vec![vec![Sand{present:false, moved:false, falling:false}; COUNT]; COUNT]));

    // let grid : Arc<Mutex<&[[bool; COLS]]>> = Arc::new(Mutex::new(&[[false; COLS]; ROWS]));
    let write = Arc::clone(&grid);

    let drag_controller = GestureDrag::new();
    let start_x = Arc::new(Mutex::new(0.0));
    let start_y = Arc::new(Mutex::new(0.0));
    drag_controller.connect_drag_begin(clone!(
        @strong start_x,
        @strong start_y,
        =>
        move |_, x, y| {
            let mut new_x = start_x.lock().unwrap();
            *new_x = x;
            let mut new_y = start_y.lock().unwrap();
            *new_y = y;
        }
    ));

    drag_controller.connect_drag_update(clone!(
        @strong grid,
        @strong start_x,
        @strong start_y,
        =>
        move |_, offset_x, offset_y| {

            let mut write = grid.lock().unwrap();

            let row = ((*start_y.lock().unwrap() + offset_y) / SAND_SIZE as f64).floor() as usize;
            let col = ((*start_x.lock().unwrap() + offset_x) / SAND_SIZE as f64).floor() as usize;

            if row >= COUNT || col >= COUNT { return; }

            write[row][col].present = true;

            let mut x = BRUSH_SIZE as i32;
            let mut y = 0 as i32;
            let mut decision_over_2 : i32 = 1 - x;
            let col = col as i32;
            let row = row as i32;

            while x >= y {

                for i in -x .. x {
                    set_pixel(&mut write, col + i, row + y);
                    set_pixel(&mut write, col + i, row - y);
                }
                for i in -y .. y {
                    set_pixel(&mut write, col + i, row + x);
                    set_pixel(&mut write, col + i, row - x);
                }

                y += 1;

                if decision_over_2 <= 0 {
                    decision_over_2 += 2 * y + 1;
                } else {
                    x -= 1;
                    decision_over_2 += 2 * (y - x) + 1;
                }

            }
        }
    ));

    drawing_area.set_draw_func(move |_, cr, _width, _height| {
        let res = grid.lock().unwrap();
        cr.set_source_rgb(1.0, 1.0, 1.0);
        cr.rectangle(0.0, 0.0, DRAW_SIZE as f64, DRAW_SIZE as f64);
        cr.stroke().expect("Stroke Failed");

        for y in 0 .. COUNT {
            for x in 0 .. COUNT {

                if res[y][x].present {
                    cr.set_source_rgb(1.0, 1.0, 1.0);
                    cr.rectangle((x * SAND_SIZE) as f64, (y * SAND_SIZE) as f64, SAND_SIZE as f64, SAND_SIZE as f64);
                    cr.fill().expect("Fill Failed");
                    cr.stroke().expect("Stoke Failed");
                }
            }
        }
    });

    let container = Box::builder()
        .orientation(gtk::Orientation::Horizontal)
        .margin_top(100)
        .spacing(10)
        .halign(gtk::Align::Center)
        .build();

    container.append(&drawing_area);

    window.set_child(Some(&container));
    window.show();
    window.present();
    drawing_area.add_controller(drag_controller);

    timeout_add_local(Duration::from_millis(25), move || {
        let mut data = write.lock().unwrap();
        
        //TODO: add impact of sand particle so stagnant sand still moves
        //TODO: split into chunks
        for y in (1 .. COUNT).rev() {
            for x in (0 .. COUNT).rev() {
                if !data[y - 1][x].present {continue;}

                if !data[y][x].present {
                    data[y - 1][x].present = false;
                    data[y][x].present = true;
                } else if x > 0 && !data[y][x - 1].present {
                    data[y - 1][x].present = false;
                    data[y][x - 1].present = true;
                } else if x < COUNT - 1 && !data[y][x + 1].present {
                    data[y - 1][x].present = false;
                    data[y][x + 1].present = true;
                }
            }
        }

        drawing_area.queue_draw();

        gtk::glib::ControlFlow::Continue
    });
}

fn set_pixel(write : &mut MutexGuard<Vec<Vec<Sand>>>, x : i32, y : i32) {
    if x >= 0 && x < COUNT as i32 && y >= 0 && y < COUNT as i32 {
        write[y as usize][x as usize].present = true;
        write[y as usize][x as usize].falling = true;
    }
}
// fn main() {

//     let mut grid : Vec<Vec<usize>> = vec![vec![0; 10]; 10];

//     for x in 0 .. 10 {
//         for y in 0 .. 10 {
//             grid[x][y] = y;
//         }
//     }
//     // let chunks

//     printer(&grid);

//     let chunks = split::<usize>(&mut grid, 2);

//     for c in chunks {
//         println!("---------");
//         printer(&c);
//     }

// }

// fn split<T : Copy>(grid : &mut Vec<Vec<T>>, chunk_size : usize) -> Vec<Vec<Vec<T>>> {

//     let num_rows = grid.len();
//     let num_cols = if num_rows > 0 { grid[0].len() } else { 0 };

//     // Initialize the result
//     let mut result = Vec::new();

//     // Iterate over column chunks
//     for chunk_start in (0..num_cols).step_by(chunk_size) {
//         let chunk_end = std::cmp::min(chunk_start + chunk_size, num_cols);
//         let mut column_chunk = vec![Vec::with_capacity(num_rows); chunk_end - chunk_start];

//         // Populate the chunk with columns
//         for (_, row) in grid.iter().enumerate() {
//             for col in chunk_start..chunk_end {
//                 column_chunk[col - chunk_start].push(row[col]);
//             }
//         }

//         result.push(column_chunk);
//     }

//     result

// }

// fn printer(grid : & Vec<Vec<usize>>) {
//     for x in 0 .. grid.len() {
//         print!("[ ");
//         for y in 0 .. grid[0].len() {
//             if y == 9 {
//                 print!("{}", grid[x][y])
//             } else {
//                 print!("{}, ", grid[x][y])

//             }
//         }
//         println!(" ]");
//     }
// }