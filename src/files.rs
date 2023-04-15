use std::{io::Read, path::PathBuf};

use clap::ValueEnum;
use rustfft::num_complex::Complex;

use anyhow::Result;

#[allow(unused_imports)]
use log::trace;

use crate::colors;

pub fn read_file(src: &PathBuf, bytes_offset: usize) -> Result<Vec<Complex<f32>>> {
    let mut file = std::fs::File::open(src)?;
    let mut result = Vec::with_capacity(1024);
    if bytes_offset > 0 {
        let mut readed = 0;
        let mut buf = [0u8; 1];
        while let Ok(n) = file.read(&mut buf){
            readed += n;
            if readed == bytes_offset{
                break;
            }
        }
        
    }
    let mut buf = [0u8; 8];
    while let Ok(n) = file.read(&mut buf) {
        if n != 8 {
            break;
        }
        let re = f32::from_le_bytes(buf[0..4].try_into().unwrap());
        let im = f32::from_le_bytes(buf[4..8].try_into().unwrap());
        result.push(Complex { re, im });
    }
    Ok(result)
}

pub struct FFTImage<'a> {
    pub src: &'a Vec<f32>,
    pub size_fft: usize,
    pub output: PathBuf,
    pub color: colors::ColorFunc,
    pub max: f32,
    pub central_line: bool,
    pub freq_centered: FreqCentered
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Default)]
pub enum FreqCentered{
    #[default]
    MovedFFT,
    Original,
}


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Default)]
pub enum DisplayFun{
    #[default]
    Norm, 
    Real,
    Image,
}

impl<'a> FFTImage<'a> {
    pub fn save_fft_to_image(self) -> Result<()> {
        let height = self.src.len() / self.size_fft;
        let mut img =
            image::ImageBuffer::<image::Rgba<u8>, _>::new(self.size_fft as u32, height as u32);
        let mut buff = [0u8; 4];
        let size_fft = self.size_fft as u32;
        self.src.iter().enumerate().for_each(|(i, c)| {
            let mut x = (i % self.size_fft) as u32;
            let y = (i / self.size_fft) as u32;

            match self.freq_centered {
                FreqCentered::MovedFFT => {
                    x = (x + size_fft / 2) % size_fft
                },
                FreqCentered::Original => {},
            }
            
            (self.color)(*c, 0xff, self.max, &mut buff);
            if self.central_line{
                if x as usize == self.size_fft / 2{
                    // buff[3] = 0x0f;
                    blend_color(&[224, 0, 209, 0xff], &mut buff);
                }
            }
            img.put_pixel(x as u32, y as u32, image::Rgba(buff));
        });

        img.save_with_format(self.output, image::ImageFormat::Png)?;
        Ok(())
    }
}

fn blend_color(color: &[u8; 4], dst: &mut [u8; 4]){
    *dst = [
        color[0] / 2 + dst[0] / 2,
        color[1] / 2 + dst[1] / 2,
        color[2] / 2 + dst[2] / 2,
        color[3] / 2 + dst[3] / 2,
    ]
}

#[allow(dead_code)]
fn blend_color_break(color_rgba1: &[u8; 4], dst: &mut [u8; 4]){
    let alpha = 255 - ((255 - color_rgba1[3]) as u16 * (255 - dst[3]) as u16);
    let red   = (color_rgba1[0] as u16 * (255 - dst[3] as u16) + dst[0] as u16 * dst[3] as u16) / 255;
    let green = (color_rgba1[1] as u16 * (255 - dst[3] as u16) + dst[1] as u16 * dst[3] as u16) / 255;
    let blue  = (color_rgba1[2] as u16 * (255 - dst[3] as u16) + dst[2] as u16 * dst[3] as u16) / 255;
    *dst = [
        red as u8,
        green as u8,
        blue as u8,
        alpha as u8
    ]
}


pub type ComplexDisplay = fn(&Complex<f32>)->f32;

pub fn get_display_fun(s: DisplayFun)->ComplexDisplay{
    match s {
        DisplayFun::Norm => norm_display,
        DisplayFun::Real => real_display,
        DisplayFun::Image => image_display,
    }
}

fn norm_display(s: &Complex<f32>)->f32{
    s.norm()
}

fn real_display(s: &Complex<f32>)->f32{
    s.re
}

fn image_display(s: &Complex<f32>)->f32{
    s.im
}