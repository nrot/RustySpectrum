use std::{fmt::Display, error::Error, mem::size_of};

use clap::ValueEnum;
use rustfft::num_complex::Complex;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Default)]
pub enum FileFormat{
    #[default]
    Cf32,
    Cf64,
    Cs32,
    Cs16,
    Cs8,
    Cu8,
    F32,
    F64,
    S16,
    S8,
    U8
}

impl FileFormat{
    pub fn sample_size(&self)->usize{
        match self {
            FileFormat::Cf32 => 2 * size_of::<f32>(),
            FileFormat::Cf64 => 2 * size_of::<f64>(),
            FileFormat::Cs32 => 2 * size_of::<i16>(),
            FileFormat::Cs16 => 2 * size_of::<i16>(),
            FileFormat::Cs8 => 2 * size_of::<i8>(),
            FileFormat::Cu8 => 2 * size_of::<u8>(),
            FileFormat::F32 => size_of::<f32>(),
            FileFormat::F64 => size_of::<f64>(),
            FileFormat::S16 => size_of::<i16>(),
            FileFormat::S8 => size_of::<i8>(),
            FileFormat::U8 => size_of::<u8>(),
        }
    }
    pub fn covert(&self, src: &[u8])->Complex<f64>{
        match self {
            FileFormat::Cf32 => {
                Complex{
                    re: f32::from_le_bytes(src[0..self.sample_size()/2].try_into().unwrap()) as f64,
                    im: f32::from_le_bytes(src[self.sample_size()/2..self.sample_size()].try_into().unwrap()) as f64,
                }
            },
            FileFormat::Cf64 => {
                Complex{
                    re: f64::from_le_bytes(src[0..self.sample_size()/2].try_into().unwrap()),
                    im: f64::from_le_bytes(src[self.sample_size()/2..self.sample_size()].try_into().unwrap()),
                }
            }
            FileFormat::Cs32 | FileFormat::Cs16 => {
                Complex{
                    re: i16::from_le_bytes(src[0..self.sample_size()/2].try_into().unwrap()) as f64,
                    im: i16::from_le_bytes(src[self.sample_size()/2..self.sample_size()].try_into().unwrap()) as f64,
                }
            },
            FileFormat::Cs8 => {
                Complex{
                    re: i8::from_le_bytes(src[0..self.sample_size()/2].try_into().unwrap()) as f64,
                    im: i8::from_le_bytes(src[self.sample_size()/2..self.sample_size()].try_into().unwrap()) as f64,
                }
            },
            FileFormat::Cu8 => {
                Complex{
                    re: u8::from_le_bytes(src[0..self.sample_size()/2].try_into().unwrap()) as f64,
                    im: u8::from_le_bytes(src[self.sample_size()/2..self.sample_size()].try_into().unwrap()) as f64,
                }
            },
            FileFormat::F32 => {
                Complex{
                    re: f32::from_le_bytes(src[0..self.sample_size()].try_into().unwrap()) as f64,
                    im: 0.0,
                }
            },
            FileFormat::F64 => {
                Complex{
                    re: f64::from_le_bytes(src[0..self.sample_size()/2].try_into().unwrap()) ,
                    im: 0.0,
                }
            },
            FileFormat::S16 => {
                Complex{
                    re: i16::from_le_bytes(src[0..self.sample_size()/2].try_into().unwrap()) as f64,
                    im: 0.0,
                }
            },
            FileFormat::S8 => {
                Complex{
                    re: i8::from_le_bytes(src[0..self.sample_size()/2].try_into().unwrap()) as f64,
                    im: 0.0,
                }
            },
            FileFormat::U8 => {
                Complex{
                    re: u8::from_le_bytes(src[0..self.sample_size()/2].try_into().unwrap()) as f64,
                    im: 0.0,
                }
            },
        }
    }
}


#[derive(Debug)]
pub struct FormatError{
    pub find_format: String
}

impl Display for FormatError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Not know format: {}", self.find_format)
    }
}

impl Error for FormatError{

}
