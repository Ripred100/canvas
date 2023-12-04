use rand::Rng;

#[derive(Copy, Clone)]
pub struct DigitalCanvas<const N: usize> {
    pub pixels: [[RgbPixel; N]; N],
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

    pub fn set_rgb(&mut self, (red, green, blue): (u8, u8, u8)){
        self.red = red;
        self.green = green;
        self.blue = blue;
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

impl<const N: usize> DigitalCanvas<N> {
    pub fn new() -> DigitalCanvas<N> {
        let mut canvas = DigitalCanvas {
            pixels: [[RgbPixel::default(); N]; N],
        };
        for row in &mut canvas.pixels {
            for pixel in row {
                *pixel = RgbPixel::default();
            }
        }
        canvas
    }
}

impl<const N: usize> IntoIterator for DigitalCanvas<N> {
    type Item = RgbPixel;
    type IntoIter = CanvasIntoIterator<N>;
    fn into_iter(self) -> Self::IntoIter {
        CanvasIntoIterator {
            pixels: self.pixels,
            index: 0
        }
    }
}
pub struct CanvasIntoIterator<const N: usize> {
    pixels: [[RgbPixel; N]; N],
    index: usize, 
}

impl<'a, const N: usize> Iterator for CanvasIntoIterator<N> {
    type Item = RgbPixel;
    fn next(&mut self) -> Option<RgbPixel> {
        if self.index >= N*N {
            return None
        }
        else {
            // Rows always index from top to bottom for now
            let y = self.index / N;
            // columns index start from the right, and snake every row.
            // This is not intuitive at all, and should definitely be controlled by a config
            let x = match y%2 {
                0 =>  N - 1 - (self.index % N),
                1 => self.index % N ,
                _ => unreachable!()
            };
            self.index += 1;
            return Some(self.pixels[x][y])
        }

    }
}

