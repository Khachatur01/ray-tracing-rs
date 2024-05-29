use crate::screen::Screen;
use crate::vec2d::Vec2D;

mod screen;
mod vec2d;
mod vec3d;

fn main() {
    std::process::Command::new("clear").status().unwrap();

    let mut screen: Screen = Screen::new(110, 28, 8.0 / 15.0);

    for t in 0..10000 {
        for i in 0..screen.width {
            for j in 0..screen.height {
                let mut uv: Vec2D = Vec2D::new(i as f64, j as f64) / Vec2D::new(screen.width as f64, screen.height as f64) * 2.0 - 1.0;

                uv.x = uv.x * screen.aspect;
                uv.x = uv.x + f64::sin((t as f64) * 0.001);

                let distance: f64 = uv.x * uv.x + uv.y * uv.y;

                let color: u8 = (1.0 / distance) as u8;

                screen.set_pixel(i, j, color);
            }
        }

        println!("{}", screen.get_screen());
    }
}