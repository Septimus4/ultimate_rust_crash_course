use std::process::exit;
use clap::{CommandFactory, Parser, Subcommand};
use image::DynamicImage;

#[derive(Parser)]
#[command(name = "ImageProcessor")]
#[command(about = "A command line tool to process images", long_about = None)]
struct Cli {
    #[arg(short = 'u', long, help = "")]
    blur: Option<f32>,
    #[arg(short, long)]
    brighten: Option<i32>,
    #[arg(short, long, value_parser = parse_crop)]
    crop: Option<(u32, u32, u32, u32)>,
    #[arg(short, long)]
    rotate: Option<i32>,
    #[arg(short, long)]
    invert: bool,
    #[arg(short, long)]
    grayscale: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Clone)]
enum Commands {
    // Transform an image
    Transform {
        infile: String,
        outfile: String,
    },
    // Generate a fractal image
    Fractal {
        outfile: String,
    },
    // Generate a simple image
    Generate {
        outfile: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Transform { .. } => {
            handle_image_processing(cli);
        }
        command => handle_image_generation(command),
    }
}

fn handle_image_processing(cli: Cli) {
    if let Commands::Transform { infile, outfile } = &cli.command {
        let img = load_image(&infile);
        let img = process_image(img, &cli);
        save_image(img, &outfile);
    } else {
        print_usage_and_exit();
    }
}

fn handle_image_generation(command: Commands) {
    match command {
        Commands::Fractal { outfile } => {
            let img = fractal();
            save_image(img, &outfile);
        }
        Commands::Generate { outfile } => {
            let img = generate();
            save_image(img, &outfile);
        }
        _ => print_usage_and_exit(),
    }
}

fn process_image(img: DynamicImage, cli: &Cli) -> DynamicImage {
    let mut img = img;

    if let Some(sigma) = cli.blur {
        img = blur(img, sigma);
    }

    if let Some(value) = cli.brighten {
        img = brighten(img, value);
    }

    if let Some((x, y, width, height)) = cli.crop {
        img = crop(img, x, y, width, height);
    }

    if let Some(value) = cli.rotate {
        img = rotate(img, value);
    }

    if cli.invert {
        img = invert(img);
    }

    if cli.grayscale {
        img = grayscale(img);
    }

    img
}

fn print_usage_and_exit() {
    Cli::command().print_help().unwrap();
    exit(-1);
}

fn load_image(infile: &str) -> DynamicImage {
    image::open(infile).expect("Failed to open INFILE.")
}

fn save_image(img: DynamicImage, outfile: &str) {
    img.save(outfile).expect("Failed writing OUTFILE.")
}

fn blur(img: DynamicImage, sigma: f32) -> DynamicImage {
    img.blur(sigma)
}

fn brighten(img: DynamicImage, value: i32) -> DynamicImage {
    img.brighten(value)
}

fn crop(mut img: DynamicImage, x: u32, y: u32, width: u32, height: u32) -> DynamicImage {
    img.crop(x, y, width, height)
}

fn rotate(img: DynamicImage, value: i32) -> DynamicImage {
    match value {
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        _ => img,
    }
}

fn invert(mut img: DynamicImage) -> DynamicImage {
    img.invert();
    img
}

fn grayscale(img: DynamicImage) -> DynamicImage {
    img.grayscale()
}

fn generate() -> DynamicImage {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let red = (0.5 * (x as f32 * 0.01).sin() * 255.0) as u8;
        let green = (0.5 * (y as f32 * 0.01).sin() * 255.0) as u8;
        let blue = (0.5 * (x as f32 * 0.01 + y as f32 * 0.01).sin() * 255.0) as u8;

        *pixel = image::Rgb([red, green, blue]);
    }

    DynamicImage::ImageRgb8(imgbuf)
}

fn fractal() -> DynamicImage {
    let width = 800;
    let height = 800;
    let mut imgbuf = image::ImageBuffer::new(width, height);
    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        *pixel = image::Rgb([red, green, blue]);
    }

    DynamicImage::ImageRgb8(imgbuf)
}

fn parse_crop(s: &str) -> Result<(u32, u32, u32, u32), String> {
    let parts: Vec<&str> = s.split(',').collect();
    if parts.len() != 4 {
        return Err(format!("Invalid crop value: {}", s));
    }
    let x = parts[0].parse().map_err(|_| format!("Invalid x value: {}", parts[0]))?;
    let y = parts[1].parse().map_err(|_| format!("Invalid y value: {}", parts[1]))?;
    let width = parts[2].parse().map_err(|_| format!("Invalid width value: {}", parts[2]))?;
    let height = parts[3].parse().map_err(|_| format!("Invalid height value: {}", parts[3]))?;
    Ok((x, y, width, height))
}
