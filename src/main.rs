extern crate image;
extern crate photon_rs as photon;
extern crate time;
use std::path::Path;
use std::fs;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {

  let args: Vec<String> = env::args().collect();
  let image_input_directory = &args[1];
  let image_output_directory = &args[2];

  for entry in fs::read_dir(image_input_directory).expect("Directory not exists") {
    let entry = entry.expect("Couldn't read inside directory");

    let image_path = &entry.path().display().to_string(); // Get path reference
    let p = Path::new(image_path).file_stem(); // Get filename in path
    let parse_filename = format!("{:?}", p)
    .replace("\"", "")
    .replace("-", "_")
    .replace("é", "e")
    .replace("à", "a")
    .replace("è", "e")
    .replace(",", "_")
    .replace(" ", "")
    .replace("(", "")
    .replace(")", "")
    .replace("Some", ""); // Parse the destination image name to a convenient std unix name

    let colour_spaces: [&str; 5] = [
        "saturate_hsl",
        "desaturate",
        "lighten", 
        "darken", 
        "shift_hue",
    ];

    for &color in colour_spaces.iter() {
        // Open image path (&str) in photon native engine
        let mut img = photon::native::open_image(image_path)?;
        
        photon::colour_spaces::hsl(&mut img, color, 0.2_f32);
        
        photon::native::save_image(img, &format!(
            "{}/{}_{}.jpg", 
            image_output_directory,
            parse_filename, 
            color
        )[..]);
        println!(
            "Generate {}/{}_{}.jpg image.",
            image_output_directory,
            parse_filename,
            color
        );
    }

    // Apply effect and save image for each following
    solarize(image_path, image_output_directory, &parse_filename)
    .expect("Couldn't create solarize image");
    
    colorize(image_path, image_output_directory, &parse_filename)
    .expect("Couldn't create colorize image");

    halftone(image_path, image_output_directory, &parse_filename)
    .expect("Couldn't create halftone image");

    inc_brightness(image_path, image_output_directory, &parse_filename, 10)
    .expect("Couldn't create inc_brightness image");
    
    vertical_strips(image_path, image_output_directory, &parse_filename, 5)
    .expect("Couldn't create vertical_strips image");

    horizontal_strips(image_path, image_output_directory, &parse_filename, 7)
    .expect("Couldn't create horizontal_strips image");

    tint(image_path, image_output_directory, &parse_filename, 10, 20, 15)
    .expect("Couldn't create tint image");

    offset(image_path, image_output_directory, &parse_filename, 0, 30)
    .expect("Couldn't create offset image");

    offset_blue(image_path, image_output_directory, &parse_filename, 30)
    .expect("Couldn't create offset_blue image");

    offset_red(image_path, image_output_directory, &parse_filename, 30)    
    .expect("Couldn't create offset_red image");

    offset_green(image_path, image_output_directory, &parse_filename, 30)
    .expect("Couldn't create offset_green image");

    multiple_offsets(image_path, image_output_directory, &parse_filename, 30, 0, 2)
    .expect("Couldn't create mutliple_offsets image");

    primary(image_path, image_output_directory, &parse_filename)
    .expect("Couldn't create primary image");

    println!("You can compare outputs images with the original in {:?}", entry);
  }
  Ok(())
}

fn solarize(image_path: &str, image_output_directory: &str, parse_filename: &str) -> Result<(), Box<dyn std::error::Error>> {
  let mut img = photon::native::open_image(image_path)?;
  photon::effects::solarize(&mut img);
  photon::native::save_image(img, &format!(
    "{}/{}_solarize.jpg", 
    image_output_directory,
    parse_filename
  )[..]);
  println!(
      "Generate {}/{}_solarize.jpg image.",
      image_output_directory,
      parse_filename
  );
  Ok(())
}

fn colorize(image_path: &str, image_output_directory: &str, parse_filename: &str) -> Result<(), Box<dyn std::error::Error>> {
  let mut img = photon::native::open_image(image_path)?;
  photon::effects::colorize(&mut img);
  photon::native::save_image(img, &format!(
    "{}/{}_colorize.jpg", 
    image_output_directory,
    parse_filename
  )[..]);
  println!(
      "Generate {}/{}_colorize.jpg image.",
      image_output_directory,
      parse_filename
  );
  Ok(())
}

fn halftone(image_path: &str, image_output_directory: &str, parse_filename: &str) -> Result<(), Box<dyn std::error::Error>> {
  let mut img = photon::native::open_image(image_path)?;
  photon::effects::halftone(&mut img);
  photon::native::save_image(img, &format!(
    "{}/{}_halftone.jpg", 
    image_output_directory,
    parse_filename
  )[..]);
  println!(
      "Generate {}/{}_halftone.jpg image.",
      image_output_directory,
      parse_filename
  );
  Ok(())
}

fn horizontal_strips(image_path: &str, image_output_directory: &str, parse_filename: &str, num_strips: u8) -> Result<(), Box<dyn std::error::Error>> {
  let mut img = photon::native::open_image(image_path)?;
  photon::effects::horizontal_strips(&mut img, num_strips);
  photon::native::save_image(img, &format!(
    "{}/{}_horizontal_strips.jpg", 
    image_output_directory,
    parse_filename
  )[..]);
  println!(
      "Generate {}/{}_horizontal_strips.jpg image.",
      image_output_directory,
      parse_filename
  );
  Ok(())
}

fn inc_brightness(image_path: &str, image_output_directory: &str, parse_filename: &str,  brightness: u8) -> Result<(), Box<dyn std::error::Error>> {
  let mut img = photon::native::open_image(image_path)?;
  photon::effects::inc_brightness(&mut img, brightness);
  photon::native::save_image(img, &format!(
    "{}/{}_inc_brightness.jpg", 
    image_output_directory,
    parse_filename
  )[..]);
  println!(
      "Generate {}/{}_inc_brightness.jpg image.",
      image_output_directory,
      parse_filename
  );
  Ok(())
}

fn multiple_offsets(
  image_path: &str, 
  image_output_directory: &str, 
  parse_filename: &str, 
  offset: u32,
  channel_index: usize,
  channel_index2: usize) -> Result<(), Box<dyn std::error::Error>> {
  let mut img = photon::native::open_image(image_path)?;
  photon::effects::multiple_offsets(&mut img, offset, channel_index, channel_index2);
  photon::native::save_image(img, &format!(
    "{}/{}_multiple_offsets.jpg", 
    image_output_directory,
    parse_filename
  )[..]);
  println!(
      "Generate {}/{}_multiple_offsets.jpg image.",
      image_output_directory,
      parse_filename
  );
  Ok(())
}

fn offset(
  image_path: &str, 
  image_output_directory: &str, 
  parse_filename: &str, 
  channel_index: usize, 
  offset: u32) -> Result<(), Box<dyn std::error::Error>> {
  let mut img = photon::native::open_image(image_path)?;
  photon::effects::offset(&mut img, channel_index, offset);
  photon::native::save_image(img, &format!(
    "{}/{}_offset.jpg", 
    image_output_directory,
    parse_filename
  )[..]);
  println!(
      "Generate {}/{}_offset.jpg image.",
      image_output_directory,
      parse_filename
  );
  Ok(())
}

fn offset_blue(
  image_path: &str, 
  image_output_directory: &str, 
  parse_filename: &str,
  offset_amt: u32) -> Result<(), Box<dyn std::error::Error>> {
  let mut img = photon::native::open_image(image_path)?;
  photon::effects::offset_blue(&mut img, offset_amt);
  photon::native::save_image(img, &format!(
    "{}/{}_offset_blue.jpg", 
    image_output_directory,
    parse_filename
  )[..]);
  println!(
      "Generate {}/{}_offset_blue.jpg image.",
      image_output_directory,
      parse_filename
  );
  Ok(())
}

fn offset_green(
  image_path: &str, 
  image_output_directory: &str, 
  parse_filename: &str,
  offset_amt: u32) -> Result<(), Box<dyn std::error::Error>> {
  let mut img = photon::native::open_image(image_path)?;
  photon::effects::offset_green(&mut img, offset_amt);
  photon::native::save_image(img, &format!(
    "{}/{}_offset_green.jpg", 
    image_output_directory,
    parse_filename
  )[..]);
  println!(
      "Generate {}/{}_offset_green.jpg image.",
      image_output_directory,
      parse_filename
  );
  Ok(())
}

fn offset_red(
  image_path: &str, 
  image_output_directory: &str, 
  parse_filename: &str,
  offset_amt: u32) -> Result<(), Box<dyn std::error::Error>> {
  let mut img = photon::native::open_image(image_path)?;
  photon::effects::offset_red(&mut img, offset_amt);
  photon::native::save_image(img, &format!(
    "{}/{}_offset_red.jpg", 
    image_output_directory,
    parse_filename
  )[..]);
  println!(
      "Generate {}/{}_offset_red.jpg image.",
      image_output_directory,
      parse_filename
  );
  Ok(())
}

fn primary(image_path: &str, image_output_directory: &str, parse_filename: &str) -> Result<(), Box<dyn std::error::Error>> {
  let mut img = photon::native::open_image(image_path)?;
  photon::effects::primary(&mut img);
  photon::native::save_image(img, &format!(
    "{}/{}_primary.jpg", 
    image_output_directory,
    parse_filename
  )[..]);
  println!(
      "Generate {}/{}_primary.jpg image.",
      image_output_directory,
      parse_filename
  );
  Ok(())
}

fn tint(
  image_path: &str, 
  image_output_directory: &str, 
  parse_filename: &str,
  r_offset: u32,
  g_offset: u32,
  b_offset: u32) -> Result<(), Box<dyn std::error::Error>> {
  let mut img = photon::native::open_image(image_path)?;
  photon::effects::tint(&mut img, r_offset, g_offset, b_offset);
  photon::native::save_image(img, &format!(
    "{}/{}_tint.jpg", 
    image_output_directory,
    parse_filename
  )[..]);
  println!(
      "Generate {}/{}_tint.jpg image.",
      image_output_directory,
      parse_filename
  );
  Ok(())
}

fn vertical_strips(
  image_path: &str, 
  image_output_directory: &str, 
  parse_filename: &str,
  num_strips: u8) -> Result<(), Box<dyn std::error::Error>> {
  let mut img = photon::native::open_image(image_path)?;
  photon::effects::vertical_strips(&mut img, num_strips);
  photon::native::save_image(img, &format!(
    "{}/{}_vertical_strips.jpg", 
    image_output_directory,
    parse_filename
  )[..]);
  println!(
      "Generate {}/{}_vertical_strips.jpg image.",
      image_output_directory,
      parse_filename
  );
  Ok(())
}
