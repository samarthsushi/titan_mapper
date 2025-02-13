use image::{GrayImage, Luma, RgbImage, Rgb};
use std::collections::HashSet;
use std::u32::MAX;
use std::u32::MIN;

fn is_white(pixel: &Rgb<u8>) -> bool {
    pixel[0] > 230 && pixel[1] > 230 && pixel[2] > 230
}

fn is_boundary_pixel(img: &RgbImage, x: u32, y: u32) -> bool {
    if is_white(&img.get_pixel(x, y)) {
        return false;
    }
    let (width, height) = img.dimensions();
    let directions = [
        (-1, -1), (0, -1), (1, -1),
        (-1,  0),         (1,  0),
        (-1,  1), (0,  1), (1,  1),
    ];
    for (dx, dy) in directions.iter() {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx >= 0 && ny >= 0 && nx < width as i32 && ny < height as i32 {
            if is_white(&img.get_pixel(nx as u32, ny as u32)) {
                return true;
            }
        }
    }
    false
}

fn flood_fill(img: &GrayImage, x: u32, y: u32, visited: &mut HashSet<(u32, u32)>) -> Vec<(u32, u32)> {
    let (width, height) = img.dimensions();
    let mut stack = vec![(x, y)];
    let mut cluster = Vec::new();

    while let Some((cx, cy)) = stack.pop() {
        if visited.contains(&(cx, cy)) || img.get_pixel(cx, cy)[0] != 128 {
            continue;
        }
        visited.insert((cx, cy));
        cluster.push((cx, cy));
        let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        for &(dx, dy) in &neighbors {
            let nx = cx as i32 + dx;
            let ny = cy as i32 + dy;
            if nx >= 0 && ny >= 0 && nx < width as i32 && ny < height as i32 {
                stack.push((nx as u32, ny as u32));
            }
        }
    }
    cluster
}

fn extract_boundary(input_path: &str, output_path: &str) {
    let img = image::open(input_path).expect("Failed to open image").to_rgb8();
    let (width, height) = img.dimensions();
    let mut boundary_img = GrayImage::new(width, height);
    let mut clusters = Vec::new();
    let mut visited = HashSet::new();

    for y in 0..height {
        for x in 0..width {
            if is_boundary_pixel(&img, x, y) {
                boundary_img.put_pixel(x, y, Luma([128]));
            } else {
                boundary_img.put_pixel(x, y, Luma([255]));
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            if boundary_img.get_pixel(x, y)[0] == 128 && !visited.contains(&(x, y)) {
                let cluster = flood_fill(&boundary_img, x, y, &mut visited);
                clusters.push(cluster);
            }
        }
    }
    
    let largest_cluster = clusters.into_iter().max_by_key(|c| c.len()).unwrap_or_default();
    let mut final_img = GrayImage::from_fn(width, height, |_,_| Luma([255]));
    for &(x, y) in &largest_cluster {
        final_img.put_pixel(x, y, Luma([128]));
    }

    let mut westmost = (MAX, 0);
    let mut eastmost = (MIN, 0);
    let mut northmost = (0, MAX);
    let mut southmost = (0, MIN);

    let mut west_candidates = Vec::new();
    let mut east_candidates = Vec::new();
    let mut north_candidates = Vec::new();
    let mut south_candidates = Vec::new();

    for &(x, y) in &largest_cluster {
        if x < westmost.0 {
            westmost = (x, y);
            west_candidates.clear();
            west_candidates.push((x, y));
        } else if x == westmost.0 {
            west_candidates.push((x, y));
        }

        if x > eastmost.0 {
            eastmost = (x, y);
            east_candidates.clear();
            east_candidates.push((x, y));
        } else if x == eastmost.0 {
            east_candidates.push((x, y));
        }

        if y < northmost.1 {
            northmost = (x, y);
            north_candidates.clear();
            north_candidates.push((x, y));
        } else if y == northmost.1 {
            north_candidates.push((x, y));
        }

        if y > southmost.1 {
            southmost = (x, y);
            south_candidates.clear();
            south_candidates.push((x, y));
        } else if y == southmost.1 {
            south_candidates.push((x, y));
        }
    }

    westmost = *west_candidates.get(west_candidates.len() / 2).unwrap_or(&westmost);
    eastmost = *east_candidates.get(east_candidates.len() / 2).unwrap_or(&eastmost);
    northmost = *north_candidates.get(north_candidates.len() / 2).unwrap_or(&northmost);
    southmost = *south_candidates.get(south_candidates.len() / 2).unwrap_or(&southmost);

    final_img.put_pixel(westmost.0, westmost.1, Luma([0]));
    final_img.put_pixel(eastmost.0, eastmost.1, Luma([0]));
    final_img.put_pixel(northmost.0, northmost.1, Luma([0]));
    final_img.put_pixel(southmost.0, southmost.1, Luma([0]));
    
    final_img.save(output_path).expect("Failed to save output image");
}

fn main() {
    let input_path = "data/titan.png";
    let output_path = "data/out.png";
    extract_boundary(input_path, output_path);
    println!("Boundary extraction complete. Output saved as {}", output_path);
}
