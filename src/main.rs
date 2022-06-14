use clap::Parser;

use crate::image_crop::ImageCrop;
use crate::image_wrapper::ImageWrapper;

mod image_crop;
mod image_wrapper;

#[derive(clap::Parser)]
struct Args {
    #[clap(subcommand)]
    action: Action,

}

#[derive(clap::Subcommand)]
enum Action {
    /// image_crop the image.
    Crop {
        #[clap(short, long, help = "target image path.")]
        path: String,
        #[clap(short, long, help = "image image_crop width.")]
        width: u32,
        #[clap(long, help = "image image_crop height.")]
        height: u32,
        #[clap(short = 'x', long, help = "image image_crop offset x position.")]
        offset_x: u32,
        #[clap(short = 'y', long, help = "image image_crop offset y position.")]
        offset_y: u32,
        #[clap(short = 'o', long, help = "output directory for Split image.")]
        output_dir: String,
    }
}

fn main() {
    let cli: Args = Args::parse();

    match cli.action {
        Action::Crop { path, width, height, offset_x, offset_y, output_dir } => {
            println!("{}", path);
            let image;
            match image::open(path) {
                Ok(i) => image = i,
                Err(e) => panic!("{}", e.to_string())
            }
            let img = ImageWrapper::new(image);
            let mut crop = ImageCrop::new(img, width, height, offset_x, offset_y);
            let result = crop.run();
            match result {
                Ok(count) => println!("The number of images processed is {0}.", count),
                Err(message) => panic!("An error occurred while cropping the image. reason:{0}", message)
            }
        }
    }
}