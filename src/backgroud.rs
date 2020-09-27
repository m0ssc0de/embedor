use image::imageops;
use image::DynamicImage;

struct Background {
    image: DynamicImage,
    slot: Slot,
}

struct Slot {
    first_last: ((u32, u32), (u32, u32)),
    rotate: Option<String>,
}

impl Background {
    pub fn new(image: DynamicImage, first_last: ((u32, u32), (u32, u32))) -> Self {
        Self {
            image,
            slot: Slot {
                first_last,
                rotate: None,
            },
        }
    }
    pub fn fill_slot(mut self, foreground: DynamicImage) {
        let foreground = self.resize_foreground(foreground);
        let first_pixel: (u32, u32) = self.slot.first_last.0;
        imageops::overlay(&mut self.image, &foreground, first_pixel.0, first_pixel.1)
    }
    fn resize_foreground(&self, foreground: DynamicImage) -> DynamicImage {
        let first_pixel: (u32, u32) = self.slot.first_last.0;
        let last_pixel: (u32, u32) = self.slot.first_last.1;
        // error, negative number,
        let nwidth = last_pixel.0 - first_pixel.0;
        let nheight = last_pixel.1 - first_pixel.0;
        foreground.resize(nwidth, nheight, imageops::FilterType::Nearest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
