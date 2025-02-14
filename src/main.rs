use titan_mapper::extract_boundary;
use std::env;
use image::Rgb;

fn map_to_pixel(
    lat: f64, lon: f64, 
    leftmost: f64, rightmost: f64, topmost: f64, bottommost: f64
) -> (u32, u32) {
    use std::f64::consts::{PI, SQRT_2};
    
    let r = (rightmost - leftmost) / (4.0 * SQRT_2);
    println!("Computed R: {r}");

    let phi = lat.to_radians();
    let lambda = (lon).to_radians();

    let mut theta = phi;
    if theta.abs() < PI / 2.0 - 1e-6 {
        for _ in 0..10 {
            let num = 2.0 * theta + (2.0 * theta).sin() - PI * phi.sin();
            let denom = 2.0 + 2.0 * (2.0 * theta).cos();
            if denom.abs() < 1e-6 {
                break;
            }
            theta -= num / denom;
        }
    } else {
        theta = phi.signum() * PI / 2.0;
    }

    let x = (r * (2.0 * SQRT_2 / PI) * lambda * theta.cos());
    let y = r * SQRT_2 * theta.sin();

    let pixel_x = ((x + 2.0 * r * SQRT_2) / (4.0 * r * SQRT_2)) * (rightmost - leftmost) + leftmost;
    let pixel_y = ((1.0 - (y / (SQRT_2 * r))) / 2.0) * (bottommost - topmost) + topmost;

    (pixel_x.round() as u32, pixel_y.round() as u32)
}



fn place_marker_on_image(image_path: &str, output_path: &str, px: u32, py: u32) {
    let mut img = image::open(image_path).expect("Failed to open image").to_rgb8();

    let marker_color = Rgb([255, 0, 147]);
    let border_color = Rgb([0, 0, 0]);

    let marker_size = 5;
    let border_thickness = 2;
    let (width, height) = img.dimensions();

    for dx in -(marker_size as i32)..=(marker_size as i32) {
        for dy in -(marker_size as i32)..=(marker_size as i32) {
            let nx = px as i32 + dx;
            let ny = py as i32 + dy;
            if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                if dx.abs() >= marker_size - border_thickness || dy.abs() >= marker_size - border_thickness {
                    img.put_pixel(nx as u32, ny as u32, border_color);
                } else {
                    img.put_pixel(nx as u32, ny as u32, marker_color);
                }
            }
        }
    }

    img.save(output_path).expect("Failed to save output image");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_path = "data/titan.png";
    let output_path = "data/out.png";
    let (w,e,n,s) = ((44,328),(1336,329),(692,5),(688,653)); // pre-computed using `extract_boundary` in lib.rs
    println!("west: {:?}, east: {:?}, north: {:?}, south: {:?}", w, e, n, s);
    let latitude: f64 = args[1].parse().expect("Invalid latitude");
    let longitude: f64 = args[2].parse().expect("Invalid longitude");

    let (px, py) = map_to_pixel(latitude, longitude, w.0.into(), e.0.into(), n.1.into(), s.1.into());
    println!("Mapped coordinates: ({}, {})", px, py);
    place_marker_on_image(input_path, "data/marked.png", px, py);
}
