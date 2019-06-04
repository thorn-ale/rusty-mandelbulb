/*
Copyright (c) 2019 Léo Ruusseau

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

extern crate num;
use num::{Float, FromPrimitive};

extern crate png;
use png::HasParameters;

extern crate rayon;
use rayon::prelude::*;

use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;


const EXPONENT: u32 = 8;
const MAX_ITER: u32 = 10;

const X_MIN: f64 = -1.5;
const X_MAX: f64 = 1.0;
const X_STEP: u32 = 1000;

const Y_MIN: f64 = -1.5;
const Y_MAX: f64 = 1.5;
const Y_STEP: u32 = 1000;

const Z_MIN: f64 = -1.5;
const Z_MAX: f64 = 1.5;
const Z_STEP: u32 = 1000;

fn linspace<T>(start: T, stop: T, nstep: u32) -> Vec<T>
where
    T: Float + FromPrimitive,
{
    let delta: T = (stop - start) / T::from_u32(nstep - 1).expect("out of range");
    return (0..(nstep))
        .map(|i| start + T::from_u32(i)
        .expect("out of range") * delta)
        .collect();
}

fn mandelbulb(x: f64, y: f64, z: f64) -> bool {
	let mut x0 = x;
	let mut y0 = y;
	let mut z0 = z;

	let mut r: f64;
	let mut theta: f64;
	let mut phi: f64;

	for _ in 0..MAX_ITER {
		r = (x0*x0 + y0*y0 + z0*z0).sqrt();
		theta = (x0*x0 + y0*y0).sqrt().atan2(z0);
		phi = y0.atan2(x0);
		
		if r > 24.0 {
			return false;
		}
		let temp = r.powi(EXPONENT as i32);
		x0 += (theta * EXPONENT as f64).sin() * (phi * EXPONENT as f64).cos() * temp;
		y0 += (theta * EXPONENT as f64).sin() * (phi * EXPONENT as f64).sin() * temp;
		z0 += (theta * EXPONENT as f64).cos() * temp;

	}

	true
}



fn compute_slice(x: f64, id: usize){
	let y_range = linspace(Y_MIN, Y_MAX, Y_STEP);
	let z_range = linspace(Z_MIN, Z_MAX, Z_STEP);
	let slice_name = "temp/".to_owned() + &id.to_string() + ".png";
	let path = Path::new(&slice_name);
	let file = match File::create(&path) {
		Err(why) => panic!("couldn't open: {}", why.description()),
		Ok(file) => file
	};
	let ref mut w = BufWriter::new(file);
	let mut encoder = png::Encoder::new(w, Y_STEP, Z_STEP);
	encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
	let mut writer = encoder.write_header().unwrap();
	let mut data = vec![];
	for y in y_range.iter(){
		for z in z_range.iter(){
			match mandelbulb(x, *y, *z) {
				true => data.append(&mut vec![255, 255, 255]),
				false => data.append(&mut vec![0, 0, 0])
			}
		}
	}
	writer.write_image_data(&data).unwrap();
}

fn main() {
	let x_range = linspace(X_MIN, X_MAX, X_STEP);
	x_range
		.par_iter()
		.enumerate()
		.for_each(|(id, x)| compute_slice(*x, id));
}
