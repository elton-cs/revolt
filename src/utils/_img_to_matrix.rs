use image::GenericImageView;
use image::Pixel;

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
    map
}
