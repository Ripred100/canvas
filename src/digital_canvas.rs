use rand::Rng;

#[derive(Copy, Clone, Debug)]
pub struct DigitalCanvas<const N: usize> {
    pub pixels: [[RgbPixel; N]; N],
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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


/// An iterator implementation that gives the contained [RgbPixel]'s in order.
/// In the futire this will probably be configurable to acomodate different hardware implementations
/// of physical LED pannels.
/// A pixel's position in canvas.pixels[n][m] corresponds to ROW n and COLUMN m. 
/// 
/// The current implementation starts at the top right corner of the canvas, going to the left, returns to the right onevery new row
/// i.e: in a 3x3 canvas, the first element returned from the iterator is row 0, column 2, followed by row 0, column 1
/// ```rust
/// use canvy::DigitalCanvas;
/// let canvas = DigitalCanvas::<3>::new();
/// let pxl0_0 = canvas.pixels[0][0];
/// let pxl0_1 = canvas.pixels[0][1];
/// let pxl0_2 = canvas.pixels[0][2];
/// let pxl1_0 = canvas.pixels[1][0];
/// let pxl1_1 = canvas.pixels[1][1];
/// let pxl1_2 = canvas.pixels[1][2];
/// let pxl2_0 = canvas.pixels[2][0];
/// let pxl2_1 = canvas.pixels[2][1];
/// let pxl2_2 = canvas.pixels[2][2];
/// 
/// let ordered = [pxl0_2, pxl0_1, pxl0_0, pxl1_2, pxl1_1, pxl1_0, pxl2_2, pxl2_1, pxl2_0];
/// 
/// for (i, pixel) in canvas.into_iter().enumerate() {
/// assert_eq!(ordered[i], pixel);
/// }
/// 

impl<'a, const N: usize> Iterator for CanvasIntoIterator<N> {
    type Item = RgbPixel;
    fn next(&mut self) -> Option<RgbPixel> {
        if self.index >= N*N {
            return None
        }
        else {
            // Rows always index from top to bottom for now
            let row = self.index / N;
            // columns index start from the right.
            // This is not intuitive at all, and should definitely be controlled by a config

            // This match statement does't do anything for now, but will allow snaking in the future
            let column = match row%2 {
                0 => N - 1 - (self.index % N),
                1 => N - 1 - (self.index % N), //self.index % N ,
                _ => unreachable!()
            };
            self.index += 1;
            return Some(self.pixels[row][column])
        }

    }
}

