use std::path::Path;

use clap::Parser;

use crate::image_split::ImageSplit;
use crate::image_wrapper::ImageWrapper;

mod image_split;
mod image_wrapper;

#[derive(clap::Parser)]
struct Args {
    #[clap(subcommand)]
    action: Action,

}

#[derive(clap::Subcommand)]
enum Action {
    /// image_split the image.
    Split {
        #[clap(short, long, help = "target image path.")]
        path: String,
        #[clap(short, long, help = "image image_split width.")]
        width: u32,
        #[clap(long, help = "image image_split height.")]
        height: u32,
        #[clap(short = 'x', long, default_value_t = 0, help = "image image_split offset x position.")]
        offset_x: u32,
        #[clap(short = 'y', long, default_value_t = 0, help = "image image_split offset y position.")]
        offset_y: u32,
        #[clap(short = 'o', long, help = "output directory for Split image.")]
        output_dir: String,
    }
}

fn main() {
    let cli: Args = Args::parse();

    match cli.action {
        Action::Split { path, width, height, offset_x, offset_y, output_dir } => {
            println!("{}", path);
            let image;
            match image::open(&path) {
                Ok(i) => image = i,
                Err(e) => panic!("{}", e.to_string())
            }
            let extension = match Path::new(&path).extension() {
                Some(ext) => ext,
                None => panic!("extension not fount.")
            };

            let output_ext = match extension.to_str() {
                Some(str) => str.to_string(),
                None => panic!("extension not fount")
            };

            let img = ImageWrapper::new(image);
            let mut split = ImageSplit::new(img, width, height, offset_x, offset_y, output_dir, output_ext);
            let result = split.run();
            match result {
                Ok(count) => println!("The number of images processed is {0}.", count),
                Err(message) => panic!("An error occurred while split the image. reason:{0}", message)
            }
        }
    }
}