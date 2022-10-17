use num_complex::Complex32;
use image::ImageBuffer;
use bevy::{
    prelude::*,
    render::{render_resource::TextureUsages},
    render::render_resource::{Extent3d, TextureFormat, TextureDescriptor},
};
pub struct MandlebrotNative {
    pub width: u32,
    pub height: u32,
    pub max_iterations: u32,
    pub scalex: f32,
    pub scaley: f32,
}

impl MandlebrotNative {
    pub fn new(w: u32, h: u32, iter: u32) -> Self {
        let x = 3.0 / w as f32;
        let y = 3.0 / w as f32;

        MandlebrotNative { width: w, height: h, max_iterations: iter , scalex: x, scaley: y }
    }
    ///NATIVE CALCULATION
    pub fn native(&self) -> bevy::prelude::Image {
        let mut image = ImageBuffer::new(self.width, self.height);
    
        for(x, y, pixel) in image.enumerate_pixels_mut() {

            //USED TO SCALE DOWN TO MANDELBROT SIZE
            let cx = (x as f32)  as f32 * self.scalex - 2.0;
            let cy = (y as f32) as f32 * self.scaley -1.5;
            
            //Litterlay the mandelbrot function
            let (_, i) = self.get(complex_from_coord(cx, cy), self.max_iterations);
    
            if i < self.max_iterations {
                let x = x as f32;
                let y = y as f32;
    
                let radius = 2.0f32;
    
                let i = i as f32 + 1.0 - ((x * x + y * y).ln() - (2.0f32).ln()) / (radius).ln();
                
                //Outside OR border
                *pixel = image::Rgba([
                    ((i as f32 / self.max_iterations as f32) *  255.0) as u8,
                    ((i as f32 / self.max_iterations as f32) *  255.0) as u8,
                    ((i as f32 / self.max_iterations as f32) *  255.0) as u8,
                    255_u8,
                ]);
            } else {
                //INSIDE OF MANDELBROT 
                *pixel = image::Rgba([
                    ((x as f32 / self.width as f32) * 255.0) as u8,
                    ((x as f32 / self.width as f32) * 255.0) as u8,
                    ((x as f32 / self.width as f32) * 255.0) as u8,
                    255_u8,
                ])
            }
        }

        let size = Extent3d {
            width: self.width,
            height: self.height,
            depth_or_array_layers: 1,
        };
    
        let image_descriptor = Image {
            data: image.as_raw().clone(),
            
            texture_descriptor: TextureDescriptor {
                label: None,
                size: size,
                dimension: bevy::render::render_resource::TextureDimension::D2,
                format: TextureFormat::Bgra8UnormSrgb,
                mip_level_count: 1,
                sample_count: 1,
                usage: TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_DST
                    | TextureUsages::RENDER_ATTACHMENT,
            },
            ..default()
        };
    
        image_descriptor
    }

    //SUPPOESED TO BE A OPTIMIZED VERSION
     pub fn native_optimized(&self)-> bevy::prelude::Image {
        let mut image = ImageBuffer::new(self.width, self.height);
    
        for c in 0..self.width {
            let x0 = ((c as f32) / (self.width as f32)) * 3.5 - 2.5;
            for r in 0..self.height {
                let y0 = ((r as f32) / (self.height as f32)) * 2.0 - 1.0;
                let mut x = 0.0;
                let mut y = 0.0;
                let mut iteration: u32 = 0;
                while x * x + y * y <= 4.0 && iteration < self.max_iterations {
                    let xtemp = x * x - y * y + x0;
                    y = 2.0 * x * y + y0;
                    x = xtemp;
                    iteration = iteration + 1;
                }
                 let rgb = self.get_color(iteration);
                image.put_pixel(c, r, rgb);
            }
        }

        println!("Done with calc");

        let size = Extent3d {
            width: self.width,
            height: self.height,
            depth_or_array_layers: 1,
        };
    
        let image_descriptor = Image {
            data: image.as_raw().clone(),
            
            texture_descriptor: TextureDescriptor {
                label: None,
                size: size,
                dimension: bevy::render::render_resource::TextureDimension::D2,
                format: TextureFormat::Bgra8UnormSrgb,
                mip_level_count: 1,
                sample_count: 1,
                usage: TextureUsages::TEXTURE_BINDING
                    | TextureUsages::COPY_DST
                    | TextureUsages::RENDER_ATTACHMENT,
            },
            ..default()
        };
        
        println!("Done with IMAGE");
        image_descriptor
     }

     fn get_color(&self, i: u32) -> image::Rgb<u8> {
        if i > self.max_iterations {
            return image::Rgb([255, 255, 255]);
        }
        if self.max_iterations == 255 {
            let idx = i as u8;
            return image::Rgb([idx, idx, idx]);
        }
        let idx = (((i as f32) / (self.max_iterations as f32)) * 255.0).round() as u8;
        return image::Rgb([idx, idx, idx]);
    }

    pub fn get(&self, c: Complex32, iterations: u32) -> (Complex32, u32) {
        let mut zn = Complex32::new(0.0, 0.0);
        let mut iteration = 0;
        while iteration < iterations && zn.norm() <= 2.0 {
            zn = mandelbrot(zn, c);
            iteration += 1;
        }

        (zn, iteration)
    }

}

fn complex_from_coord(x: f32, y: f32) -> Complex32 {
    Complex32::new(x, y)
}

fn mandelbrot(zn: Complex32, c: Complex32) -> Complex32 {
    zn * zn + c
}

