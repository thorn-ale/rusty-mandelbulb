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

use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

const STEP: u32 = 100;
const EXPONENT: u32 = 8;
const MAX_ITER: u32 = 10;

fn linspace<T>(start: T, stop: T, nstep: u32) -> Vec<T>
where
    T: Float + FromPrimitive,
{
    let delta: T = (stop - start) / T::from_u32(nstep - 1).expect("out of range");
    return (0..(nstep))
        .map(|i| start + T::from_u32(i).expect("out of range") * delta)
        .collect();
}

fn mandelbulb(x: f64, y: f64, z: f64, exponent: u32, max_iter: u32) -> bool {
	let mut x0 = x;
	let mut y0 = y;
	let mut z0 = z;

	let mut r: f64;
	let mut theta: f64;
	let mut phi: f64;

	for _ in 0..max_iter {
		r = (x0*x0 + y0*y0 + z0*z0).sqrt();
		theta = (x0*x0 + y0*y0).sqrt().atan2(z0);
		phi = y0.atan2(x0);
		
		if r > 24.0 {
			return false;
		}
		let temp = r.powi(exponent as i32);
		x0 += (theta * exponent as f64).sin() * (phi * exponent as f64).cos() * temp;
		y0 += (theta * exponent as f64).sin() * (phi * exponent as f64).sin() * temp;
		z0 += (theta * exponent as f64).cos() * temp;

	}

	true
}

fn main() {
	let x_range = linspace(-1.5 as f64, 1.5 as f64, STEP);
	let y_range = linspace(-1.5 as f64, 1.5 as f64, STEP);
	let z_range = linspace(-1.5 as f64, 1.5 as f64, STEP);
	let mut name = 1;
	for x in x_range.iter(){
		let str_name = "temp/".to_owned() + &name.to_string() + ".png";
		let path = Path::new(&str_name);
		let file = match File::create(&path) {
			Err(why) => panic!("couldn't open: {}", why.description()),
			Ok(file) => file
		};
		let ref mut w = BufWriter::new(file);
		let mut encoder = png::Encoder::new(w, STEP, STEP);
		encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);
		let mut writer = encoder.write_header().unwrap();
		let mut data = vec![];
		for y in y_range.iter(){
			for z in z_range.iter(){
				if mandelbulb(*x, *y, *z, EXPONENT, MAX_ITER) {
					data.push(255);
					data.push(255);
					data.push(255);
				} else {
					data.push(0);
					data.push(0);
					data.push(0);
				}
			}
		}
		writer.write_image_data(&data).unwrap();
		name += 1;
	}
}
