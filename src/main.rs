use clap::{App, Arg};
use image;

fn into_bayered(img: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>) -> Vec<u8> {
    let mut bayered_img = Vec::<u8>::with_capacity((img.height() * img.width() * 4) as usize);
    
    for row in img.rows() {
        let mut row_2 = vec![];

        for value in row {
            bayered_img.push(value[2]);
            bayered_img.push(value[1]);
            row_2.push(value[1]);
            row_2.push(value[0]);
        }
        bayered_img.append(&mut row_2);
    }

    bayered_img
}

fn upscale(height: usize, width: usize, bayered_img: Vec<u8>) -> (usize, usize, Vec<u8>) {
    let new_height = height * 2 - 1;
    let new_width = width * 2 - 1;

    let mut new_img: Vec<u8> = Vec::with_capacity((new_height * new_width * 3) as usize);

    enum RowType {
        A,
        B
    }

    enum GridType {
        BGGR,
        GBRG,
        GRBG,
        RGGB
    };

    let mut row = RowType::A;

    macro_rules! index {
        ($x:expr, $y:expr) => {
            (($x * width * 2) + $y) as usize
        };
    };

    for i in 0..new_height {
        match row {
            RowType::A => {
                let mut grid = GridType::BGGR;
                
                for j in 0..new_width {
                    match grid {
                        GridType::BGGR => {
                            let blue = bayered_img[index!(i, j)];
                            let green1 = bayered_img[index!(i, j + 1)] as usize;
                            // let green2 = bayered_img[index!(i + 1, j)] as usize;
                            // let green = (green1 + green2) / 2;
                            let green = green1;
                            let green = green as u8;
                            let red = bayered_img[index!(i + 1, j + 1)];

                           new_img.push(red);
                           new_img.push(green);
                           new_img.push(blue);

                           grid = GridType::GBRG;
                        },
                        GridType::GBRG => {
                            let blue = bayered_img[index!(i, j + 1)];
                            // let green1 = bayered_img[index!(i, j)] as usize;
                            let green2 = bayered_img[index!(i + 1, j + 1)] as usize;
                            // let green = (green1 + green2) / 2;
                            let green = green2;
                            let green = green as u8;
                            let red = bayered_img[index!(i + 1, j)];

                           new_img.push(red);
                           new_img.push(green);
                           new_img.push(blue);

                           grid = GridType::BGGR;
                        },
                        GridType::GRBG => panic!("I should not be at this RowType (GRBG)"),
                        GridType::RGGB => panic!("I should not be at this RowType (RGGB)")
                    };
                }

                row = RowType::B;
            },
            RowType::B => {
                let mut grid = GridType::GRBG;
                
                for j in 0..new_width {
                    match grid {
                        GridType::BGGR => panic!("I should not bee at this RowType (BGGR)"),
                        GridType::GBRG => panic!("I should not be at this RowType (GBRG)"),
                        GridType::GRBG => {
                            let blue = bayered_img[index!(i + 1, j)];
                            let green1 = bayered_img[index!(i, j)] as usize;
                            // let green2 = bayered_img[index!(i + 1, j + 1)] as usize;
                            // let green = (green1 + green2) / 2;
                            let green = green1;
                            let green = green as u8;
                            let red = bayered_img[index!(i, j + 1)];

                           new_img.push(red);
                           new_img.push(green);
                           new_img.push(blue);

                           grid = GridType::RGGB;
                        },
                        GridType::RGGB => {
                            let blue = bayered_img[index!(i + 1, j + 1)];
                            // let green1 = bayered_img[index!(i, j + 1)] as usize;
                            let green2 = bayered_img[index!(i + 1, j)] as usize;
                            // let green = (green1 + green2) / 2;
                            let green = green2;
                            let green = green as u8;
                            let red = bayered_img[index!(i, j)];

                           new_img.push(red);
                           new_img.push(green);
                           new_img.push(blue);

                           grid = GridType::GRBG;
                        }
                    };
                }

                row = RowType::A;
            }
        };
    }

    (new_height, new_width, new_img)
}

fn main() {
    let matches = App::new("Upscaler")
        .version("0.1.0")
        .author("Dhruv D. <ddhamani@uncc.edu>")
        .about("Upscales images")
        .arg_from_usage("<INPUT>    'The input image to upscale [possible types: hdr, bmp tga, tiff, dxt, gif, jpeg, png, pnm, webp]'")
        .arg_from_usage("<OUTPUT>    'The output image to write [possible types: jpeg, png]'")
        .arg(Arg::with_name("scale")
             .short("s")
             .long("scale")
             .help("Scaling factor")
             .takes_value(true)
             .default_value("2")
             .possible_values(&["2", "4", "8", "16", "32"])
             )
        .get_matches();

    let img_path = matches.value_of("INPUT").unwrap();
    let img_path = std::path::Path::new(img_path);

    let out_path = matches.value_of("OUTPUT").unwrap();
    let out_path = std::path::Path::new(out_path);
    
    let img = match img_path.extension() {
        Some(_) => image::open(img_path),
        None => {
            clap::Error::with_description(
                "Input File does not have an extension I could parse.",
                clap::ErrorKind::InvalidValue,
            )
            .exit();
        }
    };

    match out_path.extension() {
        Some(ext) => {
            if ext == "jpeg" || ext == "jpg" || ext == "png" {}
            else {
                clap::Error::with_description(
                    "Output File does not have an extension I support [png, jpeg, jpg].",
                    clap::ErrorKind::InvalidValue,
                ).exit()
            }
        },
        None => {
            clap::Error::with_description(
                "Output File does not have an extension I could parse.",
                clap::ErrorKind::InvalidValue,
            )
            .exit();
        }
    }

    let scale = matches.value_of("scale").unwrap();
    let mut scale: usize = scale.parse().unwrap();

    let mut img = match img {
        Ok(image) => image.into_rgb(),
        Err(e) => {
            clap::Error::with_description(
                format!("I could not decode the file at the given path. \n{:?}", e).as_str(),
                clap::ErrorKind::InvalidValue,
            )
            .exit();
        }
    };

    let rescaled_img = loop {
        let height = img.height();
        let width = img.width();
        let bayered_img = into_bayered(img);
        
        let (new_height, new_width, new_img) = upscale(height as usize, width as usize, bayered_img);
        
        img = match image::ImageBuffer::from_vec(new_width as u32, new_height as u32, new_img) {
            Some(i) => i,
            None => clap::Error::with_description(
                format!("Something went wrong while creating the new image buffer, scale: {}", scale).as_str(),
                clap::ErrorKind::HelpDisplayed,
            )
            .exit(),
        };
        
        if scale / 2 == 1 {
            break img;
        } else {
            scale = scale / 2;
        }
    };

    match rescaled_img.save(out_path) {
        Ok(_) => println!("Success!"),
        Err(e) => clap::Error::with_description(
            format!("Something went wrong while writing.\n{}", e).as_str(),
            clap::ErrorKind::HelpDisplayed,
        )
        .exit(),
    };
}
