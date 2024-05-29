use std::cmp;

pub struct Screen {
    pub width: u16,
    pub height: u16,
    pub aspect: f64,
    screen: Vec<char>,
    gradient: [char; 17]
}

impl Screen {
    pub fn new(width: u16, height: u16, pixel_aspect: f64) -> Screen {
        return Screen {
            width, height,
            aspect: ((width as f64) / (height as f64)) * pixel_aspect,
            screen: vec![' '; (width * height) as usize],
            gradient: [' ', '.', ':', '!', '/', 'r', '(', ';', '1', 'Z', '4', 'H', '9', 'W', '8', '$', '@']
        }
    }

    pub fn set_pixel(&mut self, x: u16, y: u16, color: u8) {
        let color: u8 = Self::clump(color, 0, (self.gradient.len() - 1) as u8);

        self.screen[(x + y * self.width) as usize] = self.gradient[color as usize];
    }

    pub fn get_screen(&self) -> String {
        self.screen.iter().collect()
    }

    fn clump(value: u8, min: u8, max: u8) -> u8 {
        cmp::max(cmp::min(value, max), min)
    }
}
