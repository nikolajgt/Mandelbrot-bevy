use image::{self};
use num_complex::Complex64;
use bevy::{
    prelude::*,
    render::{render_resource::TextureUsages},
    render::render_resource::{Extent3d, TextureFormat, TextureDescriptor},
};
// use palette::rgb::Rgb;
use std::{format, vec};
use std::io::{self, Write };
use std::path::Path;
use std::time::Instant;
// use palette::{Srgb, Hsv, Mix};
use std::thread;
// use color_space::{Rgb as rgb_convert, Lch};

pub struct RunMandelbrot {
    name: String,
    img_width: u32,
    img_height: u32,
    max_iterations: u64,
    save_image: bool,
    zoom: f64,
    xmove: f64,
    ymove: f64,
}

// width and height of the rendered image
// const WIDTH: u32 = 400;
// const HEIGHT: u32 = 400;

// // what part of the set to render
// const RMIN: f64 = -1.8;
// const RMAX: f64 = 0.8;
// const ICENTER: f64 = 0.0;



// // how many iterations before including a number in the set
// const ITERATIONS: u32 = 1000;

// // the "darkness" of the area just outside the set
// const COLORFACTOR: u32 = 300;

// // amount of threads to use
// const THREADS: u32 = 8;

// // calculates how "wide" the views are on the axi
// const RWIDTH: f64 = RMAX - RMIN;
// const IWIDTH: f64 = RWIDTH * HEIGHT as f64 / WIDTH as f64;

// // what imaginary coordinates to include
// const IMIN: f64 = ICENTER - IWIDTH / 2.0;

// // amount of lines one thread will compute
// const THREADLINES: u32 = HEIGHT / THREADS;

// const RRES: f64 = RWIDTH / WIDTH as f64;
// const IRES: f64 = IWIDTH / HEIGHT as f64;


impl RunMandelbrot {

    pub fn init(output_name: &str, w: u32, h: u32, iter: u64, save: bool, zoom: f64, xmove: f64, ymove: f64) -> Self {
        RunMandelbrot { name: output_name.to_string(),  img_width: w, img_height: h, max_iterations: iter, save_image: save, zoom: zoom, xmove: xmove, ymove: ymove}
    }


    fn get_color_smooth(&self, iteration: u64) -> image::Rgb<u8> {
        if iteration < self.max_iterations && iteration > 0 {
            let i = iteration % 16;
            let vec:[image::Rgb<u8>; 16] = [
                image::Rgb([66, 30, 15]),
                image::Rgb([25, 7, 26]),
                image::Rgb([9, 1, 47]),
                image::Rgb([4, 4, 73]),
                image::Rgb([0, 7, 100]),
                image::Rgb([12, 44, 138]),
                image::Rgb([24, 82, 177]),
                image::Rgb([57, 125, 209]),
                image::Rgb([134, 181, 229]),
                image::Rgb([211, 236, 248]),
                image::Rgb([241, 233, 191]),
                image::Rgb([248, 201, 95]),
                image::Rgb([255, 170, 0]),
                image::Rgb([204, 128, 0]),
                image::Rgb([153, 87, 0]),
                image::Rgb([106, 52, 3]),
            ];
            return vec[i as usize];
        }
        else {
            return image::Rgb([0, 0, 0]);
        }
    }

    // pub fn mandelbrot_native(&self) -> image::RgbImage {
    //     let mut img = image::RgbImage::new(self.img_width as u32, self.img_height as u32);
    //     for(xx, yy, pixel) in img.enumerate_pixels_mut() {

    //         let screen_ratio = self.img_width / self.img_height;

    //         //Get image size depending on zoom
    //         let dx = (2.0 / self.zoom) / self.img_width as f64;
    //         let dy = ((2.0 / screen_ratio as f64) / self.zoom) / self.img_height as f64;
            
    //         let x0 = (((xx as f64 / self.zoom) + self.xmove)/ dx) ;
    //         let y0 = (((yy as f64 / self.zoom) + self.ymove) / dy);


    //         //  let x0 = ((scalex as f64) / (self.width as f64 / 1.5)) * 3.5 - 3.5;
    //         //  let y0 = ((scaley as f64) / (self.height as f64 / 2.0) ) * 1.5 - 1.5;

    //         let mut x = 0.0;
    //         let mut y = 0.0;
    //         let mut iteration: u64 = 0;
    //         let mut xold: f64 = 0.0; 
    //         let mut yold: f64 = 0.0; 
    //         let mut period: u32 = 0;
            

    //         while x * x + y * y <= 2.0 * 2.0 && iteration < self.max_iterations {
    //             let xtemp = x * x - y * y + x0;
    //             y = 2.0 * x * y + y0;
    //             x = xtemp;
    //             iteration = iteration + 1;
               
    //             if x.eq(&xold) && y.eq(&yold) {
    //                 iteration = self.max_iterations;
    //                 break;
    //             }

    //             period = period + 1;
    //             if period > 20 {
    //                 period = 0;
    //                 xold = x;
    //                 yold = y;
    //             }
    //         }

    //         if iteration < self.max_iterations {

    //              let log_zn = ((x * x )+ (y * y) / 2_f64).ln();
    //              let nu = (log_zn / f64::log2(2_f64)) / f64::log2(2_f64);

    //              iteration = iteration + 1 - nu as u64; 
    //         }


    //         let srgb1 = self.get_color_smooth(iteration);
            
    //         *pixel = srgb1;

    //     }
    //     img
    // }

    pub fn new_algo(&self) -> bevy::prelude::Image {
        //Gets screen ratio
        let screen_ratio = self.img_width / self.img_height;

        //Get image size depending on zoom
        let dx = (4.0 / self.zoom) / self.img_width as f64;
        let dy = ((4.0 / screen_ratio as f64) / self.zoom) / self.img_height as f64;
        let M: f64 = 2.0;

        let mut img = image::RgbImage::new(self.img_width as u32, self.img_height as u32);

        for(xx, yy, pixel) in img.enumerate_pixels_mut() {

            //Get complex number posistion by calculationg x/yoove that controlls where on image you are
            // which is multiplyed by area - the written number is where on x and y axis the mandelbrot is
            let cx = self.xmove + xx as f64 * dx - 4.0 / self.zoom;
            let cy = -self.ymove + yy as f64 * dy - (1.0 / screen_ratio as f64) / self.zoom;

            let c = Complex64::new(cx, cy);
            let mut z = Complex64::new(0.0, 0.0);
            let mut prev = Complex64::new(0.0, 0.0);

            let mut iteration: u64 = 0;
            // let mut xold: f64 = 0.0; 
            // let mut yold: f64 = 0.0; 
            // let mut period: u32 = 0;


            let q = (c.re - 0.25) * (c.re - 0.25) + c.im * c.im;
            if q * (q + (c.re - 0.25)) < 0.25 * c.im * c.im { iteration = self.max_iterations; }
            else if (c.re + 1.0) * (c.re + 1.0) + c.im * c.im < 1.0/16.0 { iteration = self.max_iterations; }
            else {
                for t in 0..(self.max_iterations) {
                    if z.norm() > M * M {
                        break
                    }

                    z = z * z + c;

                    if z.eq(&prev) {
                        println!("hit");
                        iteration = self.max_iterations;
                        break
                    }

                    prev = z;
                    iteration = t;
                }
                
            }

            let smooth_iter = if iteration >= (self.max_iterations - 1) {self.max_iterations as f64 }
            else if iteration < 2 { 1f64 }
            else { (iteration as f64) + 1.0f64 - z.norm_sqr().ln().ln() / (2.0f64).ln() };

            let srgb1 = self.get_color_smooth(smooth_iter as u64);
            
            *pixel = srgb1;
        }

        
        let size = Extent3d {
            width: self.img_width,
            height: self.img_height,
            depth_or_array_layers: 1,
        };
    
        let image_descriptor = Image {
            data: img.as_raw().clone(),
            
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
   

  

   

}

