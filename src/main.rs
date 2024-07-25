use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use rand::Rng; // Añadir esta línea para usar la biblioteca rand
mod framebuffer;
use framebuffer::Framebuffer;

fn render(framebuffer: &mut Framebuffer) {
    let width = framebuffer.width;
    let height = framebuffer.height;
    let mut new_buffer = framebuffer.buffer.clone();

    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            let cell = framebuffer.buffer[idx];
            let neighbors = count_neighbors(framebuffer, x, y);
            
            if cell == 0xFFFF00 && (neighbors < 2 || neighbors > 3) {
                new_buffer[idx] = 0x1D1B41; // Muere por subpoblación o sobrepoblación
            } else if cell == 0x1D1B41 && neighbors == 3 {
                new_buffer[idx] = 0xFFFF00; // Nace por reproducción
            } else if cell == 0xFFFF00 && (neighbors == 2 || neighbors == 3) {
                new_buffer[idx] = 0xFFFF00; // Sobrevive
            } else {
                new_buffer[idx] = 0x1D1B41; // Muere por defecto
            }
        }
    }
    framebuffer.buffer = new_buffer;
}

fn count_neighbors(framebuffer: &Framebuffer, x: usize, y: usize) -> usize {
    let width = framebuffer.width;
    let height = framebuffer.height;
    let mut count = 0;

    for j in (y as isize - 1)..=(y as isize + 1) {
        for i in (x as isize - 1)..=(x as isize + 1) {
            if i == x as isize && j == y as isize {
                continue;
            }
            let ni = i.rem_euclid(width as isize) as usize;
            let nj = j.rem_euclid(height as isize) as usize;
            let idx = nj * width + ni;
            if framebuffer.buffer[idx] == 0xFFFF00 {
                count += 1;
            }
        }
    }
    count
}

fn create_initial_pattern(framebuffer: &mut Framebuffer) {
    let block = vec![(1, 1), (1, 2), (2, 1), (2, 2)];
    let bee_hive = vec![(5, 5), (6, 4), (7, 4), (8, 5), (6, 6), (7, 6)];
    let boat = vec![(10, 10), (11, 10), (10, 11), (12, 11), (11, 12)];
    let loaf = vec![(15, 15), (16, 15), (17, 16), (15, 16), (16, 17), (18, 17), (17, 18)];
    let tub = vec![(20, 20), (21, 21), (20, 22), (19, 21)];
    let hwss = vec![
        (25, 25), (26, 25), (27, 25), (28, 25), (29, 25), (29, 26), (29, 27), (28, 28), 
        (24, 28), (24, 27), (24, 26), (25, 29)
    ];
    let mwss = vec![
        (30, 30), (31, 30), (32, 30), (33, 30), (34, 30), (34, 31), (34, 32), (33, 33), 
        (29, 33), (29, 32), (29, 31), (30, 34)
    ];
    let lwss = vec![
        (35, 35), (36, 35), (37, 35), (38, 35), (39, 35), (39, 36), (39, 37), (38, 38), 
        (34, 38), (34, 37), (34, 36), (35, 39)
    ];
    let beacon = vec![
        (40, 40), (41, 40), (40, 41), (41, 41), (42, 42), (43, 42), (42, 43), (43, 43)
    ];

    let patterns = vec![
        block, bee_hive, boat, loaf, tub, hwss, mwss, lwss, beacon
    ];

    framebuffer.set_current_color(0xFFFF00);
    for pattern in patterns {
        for &(x, y) in &pattern {
            framebuffer.point(x, y);
        }
    }

    // Añadir más células aleatorias para llenar más la pantalla
    let mut rng = rand::thread_rng();
    for _ in 0..5000 {
        let x = rng.gen_range(0..framebuffer.width);
        let y = rng.gen_range(0..framebuffer.height);
        framebuffer.point(x, y);
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;

    let framebuffer_width = 100;
    let framebuffer_height = 100;

    let frame_delay = Duration::from_millis(100);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Conway's Game of Life - Rust",
        window_width, window_height,
        WindowOptions::default(),
    ).unwrap();

    framebuffer.set_background_color(0x1D1B41);
    framebuffer.clear();
    create_initial_pattern(&mut framebuffer);

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        render(&mut framebuffer);

        window.update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height).unwrap();
        std::thread::sleep(frame_delay);
    }
}
