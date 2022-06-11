use crate::image_wrapper::Image;

pub struct ImageCrop<T> where T: Image {
    image: T,
    width: u32,
    height: u32,
    offset_x: u32,
    offset_y: u32,
}

impl<T: Image> ImageCrop<T> {
    pub fn new(image: T, width: u32, height: u32, offset_x: u32, offset_y: u32) -> Self {
        Self {
            image,
            width,
            height,
            offset_x,
            offset_y,
        }
    }

    /// Execute Image Crop.
    /// # return value
    /// cropping image count or error.
    pub fn run(&self) -> Result<u32, String> {
        let vc = Self::vertical_count(self.image.height(), self.height, self.offset_y);
        let hc = Self::horizontal_count(self.image.width(), self.width, self.offset_x);
        for v in 0..vc {
            for h in 0..hc {}
        }
        Ok(vc * hc)
    }

    fn vertical_count(image_height: u32, crop_height: u32, offset_y: u32) -> u32 {
        let target_height = image_height - offset_y;
        let mod_num = target_height % crop_height;
        target_height / crop_height
    }

    fn horizontal_count(image_width: u32, crop_width: u32, offset_x: u32) -> u32 {
        let target_width = image_width - offset_x;
        target_width / crop_width
    }
}

mod tests {
    use std::ptr::null;

    use image::DynamicImage;

    use crate::image_crop::Image;
    use crate::ImageCrop;

    #[test]
    fn test_run_success() {
        struct MockImage {}
        impl Image for MockImage {
            fn height(&self) -> u32 {
                100
            }

            fn width(&self) -> u32 {
                100
            }

            fn crop(&mut self, x: u32, y: u32, width: u32, height: u32) -> DynamicImage {
                DynamicImage::new_rgb8(33, 33)
            }
        }
        let image = MockImage {};
        let target = ImageCrop::new(image, 33, 33, 0, 1);
        let result = target.run();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 9);
    }
}