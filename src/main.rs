use eframe::egui;
use image::{RgbImage, Rgb};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct GUI {
    lat: f64,
    lon: f64,
    img_buf: Arc<Mutex<Option<egui::ColorImage>>>,
    original_img: Arc<Mutex<Option<RgbImage>>>,
    hidden_pixels: Arc<Mutex<HashMap<(u32, u32), Rgb<u8>>>>,
}

impl GUI {
    fn load_map_image(&self, path: &str) -> Option<RgbImage> {
        image::open(path).ok().map(|img| img.to_rgb8())
    }

    fn update_image(&mut self) {
        let mut img = self.original_img.lock().unwrap().clone().unwrap_or_else(|| {
            eprintln!("Error: No original image loaded.");
            return RgbImage::new(1, 1);
        });

        let (px, py) = self.map_to_pixel(self.lat, self.lon);
        self.place_marker(&mut img, px, py);

        let color_image = Self::convert_to_color_image(&img);
        *self.img_buf.lock().unwrap() = Some(color_image);
    }

    fn map_to_pixel(&self, lat: f64, lon: f64) -> (u32, u32) {
        use std::f64::consts::{PI, SQRT_2};
        let (leftmost, rightmost, topmost, bottommost) = (44.0, 1336.0, 5.0, 653.0);
        let r = (rightmost - leftmost) / (4.0 * SQRT_2);
        let phi = lat.to_radians();
        let lambda = lon.to_radians();

        let mut theta = phi;
        if theta.abs() < PI / 2.0 - 1e-6 {
            for _ in 0..10 {
                let num = 2.0 * theta + (2.0 * theta).sin() - PI * phi.sin();
                let denom = 2.0 + 2.0 * (2.0 * theta).cos();
                if denom.abs() < 1e-6 { break; }
                theta -= num / denom;
            }
        } else {
            theta = phi.signum() * PI / 2.0;
        }

        let x = r * (2.0 * SQRT_2 / PI) * lambda * theta.cos();
        let y = r * SQRT_2 * theta.sin();

        let pixel_x = ((x + 2.0 * r * SQRT_2) / (4.0 * r * SQRT_2)) * (rightmost - leftmost) + leftmost;
        let pixel_y = ((1.0 - (y / (SQRT_2 * r))) / 2.0) * (bottommost - topmost) + topmost;

        (pixel_x.round() as u32, pixel_y.round() as u32)
    }

    fn place_marker(&mut self, img: &mut RgbImage, px: u32, py: u32) {
        let marker_color = Rgb([255, 0, 0]);
        let border_color = Rgb([0, 0, 0]);
        let marker_radius = 5;
        let border_thickness = 2;

        let (width, height) = img.dimensions();
        let mut hidden_pixels = self.hidden_pixels.lock().unwrap();

        let keys_to_restore: Vec<(u32, u32)> = hidden_pixels.keys().cloned().collect();
        for (x, y) in keys_to_restore {
            if let Some(color) = hidden_pixels.remove(&(x, y)) {
                img.put_pixel(x, y, color);
            }
        }

        for dx in -(marker_radius as i32)..=(marker_radius as i32) {
            for dy in -(marker_radius as i32)..=(marker_radius as i32) {
                let nx = px as i32 + dx;
                let ny = py as i32 + dy;
                
                if nx >= 0 && nx < width as i32 && ny >= 0 && ny < height as i32 {
                    let pos = (nx as u32, ny as u32);
                    let dist = ((dx * dx + dy * dy) as f64).sqrt();

                    if dist <= marker_radius as f64 {
                        if !hidden_pixels.contains_key(&pos) {
                            hidden_pixels.insert(pos, *img.get_pixel(pos.0, pos.1));
                        }

                        if dist >= (marker_radius - border_thickness) as f64 {
                            img.put_pixel(pos.0, pos.1, border_color);
                        } else {
                            img.put_pixel(pos.0, pos.1, marker_color);
                        }
                    }
                }
            }
        }
    }

    fn convert_to_color_image(img: &RgbImage) -> egui::ColorImage {
        let (w, h) = img.dimensions();
        egui::ColorImage::from_rgb([w as usize, h as usize], img.as_raw())
    }
}

impl eframe::App for GUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.heading("Control panel");

            let mut changed = false;
            changed |= ui.add(egui::Slider::new(&mut self.lat, -90.0..=90.0).text("Latitude")).changed();
            changed |= ui.add(egui::Slider::new(&mut self.lon, -180.0..=180.0).text("Longitude")).changed();

            if changed {
                self.update_image();
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let available_size = ui.available_size();
            if let Some(image) = &*self.img_buf.lock().unwrap() {
                let texture = ctx.load_texture("map_texture", image.clone(), egui::TextureOptions::default());
                let image_size = texture.size_vec2();
                let scale = (available_size.x / image_size.x).min(available_size.y / image_size.y);
        
                ui.add(egui::Image::new(&texture).fit_to_exact_size(image_size * scale));
            }
        });

        ctx.request_repaint();
    }
}

fn main() -> Result<(), eframe::Error> {
    let app = GUI {
        lat: 0.0,
        lon: 0.0,
        img_buf: Arc::new(Mutex::new(None)),
        original_img: Arc::new(Mutex::new(None)),
        hidden_pixels: Arc::new(Mutex::new(HashMap::new())),
    };

    if let Some(img) = app.load_map_image("data/titan.png") {
        *app.original_img.lock().unwrap() = Some(img.clone());
        let color_image = GUI::convert_to_color_image(&img);
        *app.img_buf.lock().unwrap() = Some(color_image);
    }

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    
    eframe::run_native("Mollweide mapper", options, Box::new(|_| Ok(Box::new(app))))
}
