extern crate image;

use image::GenericImageView;

fn main() {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.

    let path = "resources/image.png";
    let img = image::open(path);
    if img.is_err() {
        println!("Failed to open the image {}: {}", path, img.err().unwrap());
        return;
    }
    let img = img.unwrap();
    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());

    // Write the contents of this image to the Writer in PNG format.
    img.save("test.png").unwrap();
}

// //! An example of generating julia fractals.
// extern crate image;
// extern crate num_complex;
//
// fn main() {
//     let imgx = 1200;
//     let imgy = 1200;
//
//     let scalex = 3.0 / imgx as f32;
//     let scaley = 3.0 / imgy as f32;
//
//     // Create a new ImgBuf with width: imgx and height: imgy
//     let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
//
//     // Iterate over the coordinates and pixels of the image
//     for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
//         let r = (0.3 * x as f32) as u8;
//         let b = (0.3 * y as f32) as u8;
//         *pixel = image::Rgb([r, 0, b]);
//     }
//
//     // A redundant loop to demonstrate reading image data
//     for x in 0..imgx {
//         for y in 0..imgy {
//             let cx = y as f32 * scalex - 1.5;
//             let cy = x as f32 * scaley - 1.5;
//
//             let c = num_complex::Complex::new(-0.4, 0.6);
//             let mut z = num_complex::Complex::new(cx, cy);
//
//             let mut i = 0;
//             while i < 255 && z.norm() <= 2.0 {
//                 z = z * z + c;
//                 i += 1;
//             }
//
//             let pixel = imgbuf.get_pixel_mut(x, y);
//             let image::Rgb(data) = *pixel;
//             *pixel = image::Rgb([data[0], i as u8, data[2]]);
//         }
//     }
//
//     // Save the image as “fractal.png”, the format is deduced from the path
//     imgbuf.save("fractal.png").unwrap();
// }