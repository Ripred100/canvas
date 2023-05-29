use rand::Rng;

#[derive(Copy, Clone)]
pub struct DigitalCanvas {
    pub pixels: [[RgbPixel; 10]; 10],
}

#[derive(Copy, Clone)]
pub struct RgbPixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl RgbPixel {
    pub fn default() -> RgbPixel {
        let mut rng = rand::thread_rng();
        RgbPixel {
            red: rng.gen_range(0..255),
            green: rng.gen_range(0..255),
            blue: rng.gen_range(0..255),
        }
    }

    pub fn red() -> RgbPixel {
        RgbPixel {
            red: 255,
            green: 0,
            blue: 0,
        }
    }

    pub fn green() -> RgbPixel {
        RgbPixel {
            red: 0,
            green: 255,
            blue: 0,
        }
    }

    pub fn blue() -> RgbPixel {
        RgbPixel {
            red: 0,
            green: 0,
            blue: 255,
        }
    }

    pub fn set_rgb(mut self, (red, green, blue): (u8, u8, u8)) -> RgbPixel {
        self.red = red;
        self.green = green;
        self.blue = blue;
        self
    }

    pub fn color(self) -> (u8, u8, u8) {
        (self.red, self.green, self.blue)
    }

    pub fn cycle(&mut self) {
        self.red = self.red.wrapping_add(10);
        self.green = self.green.wrapping_add(20);
        self.blue = self.blue.wrapping_add(30);
    }
}

impl DigitalCanvas {
    pub fn new() -> DigitalCanvas {
        let mut canvas = DigitalCanvas {
            pixels: [[RgbPixel::default(); 10]; 10],
        };
        for row in &mut canvas.pixels {
            for pixel in row {
                *pixel = RgbPixel::default();
            }
        }
        canvas
    }
}
