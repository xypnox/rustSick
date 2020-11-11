#![allow(dead_code)]

extern crate image;

use image::GenericImageView;

fn get_path(path: &str) -> String {
    format!(
        "/home/xypnox/Projects/learn/rustSick/experiments/pxsort_v1/src/{}",
        path,
    )
}

fn main() {
    // Use the open function to load an image from a Path.
    // `open` returns a `DynamicImage` on success.
    let img = image::open(get_path("images/1.png")).unwrap();

    // The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());

    // The color method returns the image's `ColorType`.
    println!("{:?}", img.color());

    // Use RBGA for everything so YOLO
    // let mut buf = img.to_rgba();
    // let buf2 = buf.clone();
    // let mut sorted_pixels: Vec<_> = buf2.pixels().collect();
    // sorted_pixels.sort_by_key(|p| );

    // for (i, pixel) in buf.pixels_mut().enumerate() {
    //     *pixel = *sorted_pixels[i];
    // }

    img.save(get_path("images/output/1.png"))
        .unwrap_or_else(|_| {
            println!("Could not save image!");
        });
}
