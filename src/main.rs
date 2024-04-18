use std::path::PathBuf;
use clap::Parser;
use image::Pixel;

//This defines command line arguments
#[derive(Parser)]
struct Cli{
	path:PathBuf,
}

//use an unsigned 64 bit integer to avoid overflowing while adding together millions of pixels
struct RGB{
	r:u64,
	g:u64,
	b:u64,
}

//the main function returns nothing () or an image error
fn main()->Result<(),image::ImageError>{
	//use clap to parse command line arguments (just the path to the input image in this case)
	let cli=Cli::parse();

	//read an image from the path specified via cli
	//The ? operator checks for an error and returns early if there is one
	let img=image::open(cli.path)?;
	//convert to rgb
	let img_rgb=img.to_rgb8();
	//get the pixels and save the count for later
	let pixels=img_rgb.pixels();
	let pixel_count=pixels.len() as u64;

	//compute the average color
	//mut means the value is mutable i.e. can be overwritten/changed
	let mut avg=RGB{r:0,g:0,b:0};
	for pixel in pixels{
		//peek into the pixel data structure provided by the "image" library and write to the avg color
		avg.r+=pixel.channels()[0] as u64;
		avg.g+=pixel.channels()[1] as u64;
		avg.b+=pixel.channels()[2] as u64;
	}

	//print out three numbers to the console
	println!("Average color={},{},{}",
		avg.r/pixel_count,
		avg.g/pixel_count,
		avg.b/pixel_count
	);

	//This satisfies the return type of the main function
	Ok(())
}
