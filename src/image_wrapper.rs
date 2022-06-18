use image::DynamicImage;

pub trait Image {
    fn height(&self) -> u32;
    fn width(&self) -> u32;
    fn crop(&mut self, x: u32, y: u32, width: u32, height: u32) -> DynamicImage;
}

pub struct ImageWrapper {
    image: DynamicImage,
}

impl ImageWrapper {
    pub fn new(image: DynamicImage) -> Self {
        Self {
            image
        }
    }
}

impl Image for ImageWrapper {
    fn height(&self) -> u32 {
        self.image.height()
    }
    fn width(&self) -> u32 {
        self.image.width()
    }
    fn crop(&mut self, x: u32, y: u32, width: u32, height: u32) -> DynamicImage {
        self.image.crop(x, y, width, height)
    }
}
