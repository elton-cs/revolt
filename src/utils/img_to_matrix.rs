use image::GenericImageView;
use image::Pixel;

const BLOCK_SIZE: u32 = 26;

pub fn pixel_convertor(path_to_image: &str) -> Vec<Vec<u32>> {
    let img = image::open(path_to_image).expect("Failed to open image");
    let (width, height) = img.dimensions();

    let mut map = vec![vec![0u32; width as usize]; height as usize];
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let luma = pixel.to_luma().0[0];

            if luma == 0 {
                map[y as usize][x as usize] = 1;
            } else {
                map[y as usize][x as usize] = 0;
            }
        }
    }

    map.reverse();

    // let mut map = vec![vec![0u32; width as usize]; height as usize];
    // for y in 0..height {
    //     let mut x_vec = vec![0u32; width as usize];

    //     for x in 0..width {
    //         let pixel = img.get_pixel(x, y);
    //         let luma = pixel.to_luma().0[0];

    //         if luma == 0 {
    //             x_vec.push(1);
    //         } else {
    //             x_vec.push(0);
    //         }
    //     }
    //     // map[(height - 1 - y) as usize] = x_vec;
    //     map.push(x_vec);
    // }

    map
}

pub fn convertor(path_to_image: &str) -> Vec<Vec<u32>> {
    // Load the image
    let img = image::open(path_to_image).expect("Failed to open image");

    // Get the dimensions of the image
    let (width, height) = img.dimensions();

    // Calculate the dimensions of the map
    let map_width = width / BLOCK_SIZE;
    let map_height = height / BLOCK_SIZE;

    // Create a 2D array to store the map
    let mut map = vec![vec![0u32; map_width as usize]; map_height as usize];

    // Iterate over the blocks and convert them to the corresponding values
    for by in 0..map_height {
        for bx in 0..map_width {
            let mut is_wall = false;

            let pixel = img.get_pixel(bx * BLOCK_SIZE, by * BLOCK_SIZE);
            let luma = pixel.to_luma().0[0];

            // If the top-left pixel of the block is black, mark the block as a wall
            if luma == 0 {
                is_wall = true;
            }

            // // Check the pixels within the block
            // for y in 0..BLOCK_SIZE {
            //     for x in 0..BLOCK_SIZE {
            //         let pixel_x = bx * BLOCK_SIZE + x;
            //         let pixel_y = by * BLOCK_SIZE + y;
            //         let pixel = img.get_pixel(pixel_x, pixel_y);
            //         let luma = pixel.to_luma().0[0];

            //         // If any pixel in the block is black, mark the block as a wall
            //         if luma == 0 {
            //             is_wall = true;
            //             break;
            //         }
            //     }
            //     if is_wall {
            //         break;
            //     }
            // }

            // Set the map value based on the block's content
            if is_wall {
                map[by as usize][bx as usize] = 1;
            } else {
                map[by as usize][bx as usize] = 0;
            }
        }
    }

    // Print the map
    // for row in map {
    //     for cell in row {
    //         print!("{}", cell);
    //     }
    //     println!();
    // }

    map
}
