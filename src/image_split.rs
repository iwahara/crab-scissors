use std::path::Path;

use crate::image_wrapper::Image;

pub struct ImageSplit<T> where T: Image {
    image: T,
    width: u32,
    height: u32,
    offset_x: u32,
    offset_y: u32,
    output_dir: String,
    extension: String,
}

impl<T: Image> ImageSplit<T> {
    pub fn new(image: T, width: u32, height: u32, offset_x: u32, offset_y: u32, output_dir: String, extension: String) -> Self {
        Self {
            image,
            width,
            height,
            offset_x,
            offset_y,
            output_dir,
            extension,
        }
    }

    /// Execute Image Split.
    /// # return value
    /// split image count or error.
    pub fn run(&mut self) -> Result<u32, String> {
        let vc = Self::vertical_count(self.image.height(), self.height, self.offset_y);
        let hc = Self::horizontal_count(self.image.width(), self.width, self.offset_x);
        let output_path = Path::new(self.output_dir.as_str());
        for v in 0..vc {
            for h in 0..hc {
                let y = self.height * v;
                let x = self.width * h;
                let split_image = self.image.crop(x, y, self.width, self.height);

                let file_path = output_path.join(format!("{}_{}.{}", v, h, self.extension));
                let result = split_image.save(file_path);
                match result {
                    Ok(_i) => continue,
                    Err(e) => return Err(format!("Image save error.{}", e.to_string())),
                }
            }
        }
        Ok(vc * hc)
    }

    fn vertical_count(image_height: u32, crop_height: u32, offset_y: u32) -> u32 {
        let target_height = image_height - offset_y;
        target_height / crop_height
    }

    fn horizontal_count(image_width: u32, crop_width: u32, offset_x: u32) -> u32 {
        let target_width = image_width - offset_x;
        target_width / crop_width
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use image::DynamicImage;
    use tempfile::tempdir;

    use crate::{ImageSplit, ImageWrapper};

    #[test]
    fn test_run_success() {
        let image = DynamicImage::new_rgb8(100, 200);
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path();
        let str_temp_path = temp_path.to_str().unwrap().to_string();

        let mut target = ImageSplit::new(ImageWrapper::new(image), 20, 20, 0, 0, str_temp_path, String::from("png"));
        let result = target.run();

        assert_eq!(result.unwrap(), 50);
        assert_eq!(temp_path.read_dir().unwrap().count(), 50);
        for v in 0..10 {
            for h in 0..5 {
                let file_name = format!("{}_{}.png", v, h);
                assert!(temp_path.join(Path::new(&file_name)).exists());
            }
        }
    }
}