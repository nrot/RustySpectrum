use anyhow::Result;
use clap::{Parser, ValueEnum};
use rustfft::{num_complex::Complex, FftPlanner};
use std::{io::Read, path::PathBuf};

mod colors;
mod display;

#[allow(unused_imports)]
use log::{debug, info, trace, warn};

pub struct App {
    args: Args,
    data: Vec<Complex<f32>>,
    color_fun: colors::ColorFunc,
    display_fun: display::ComplexDisplay,
    sample_size: usize,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "Source cf32 file")]
    source: PathBuf,

    #[arg(
        short,
        long,
        default_value = "spectrum.png",
        help = "Output png file path"
    )]
    output: PathBuf,

    #[arg(short, long, default_value = "1024", help = "FFT size")]
    fft_size: usize,

    #[clap(flatten)]
    verbose: clap_verbosity_flag::Verbosity,

    #[clap(
        short,
        long,
        default_value = "rgb-smooth",
        help = "Color scheme function"
    )]
    colors: colors::ExistsFunc,

    #[clap(
        long,
        default_value = "0.0",
        help = "Value for added to each complex number"
    )]
    fft_clamp_min: f32,

    #[clap(
        long,
        default_value = "1.0",
        help = "Value for color function. Example: Gray color scheme black==0.0, white==max value"
    )]
    fft_clamp_max: f32,

    #[clap(
        short,
        long,
        help = "Searched min and max value for all complex data. Use 'display function' for searching"
    )]
    smart_fft_clamp: bool,

    #[clap(
        short,
        long,
        help = "Normalize fft. Each complex value divide fft-size"
    )]
    normalize_fft: bool,

    #[clap(long, help = "Draws purple central line")]
    central_line: bool,

    #[clap(
        long,
        default_value = "moved-fft",
        help = "Default main freq moved to center. Original draws raw fft"
    )]
    freq_centered: FreqCentered,

    #[clap(
        long,
        default_value = "0",
        help = "Offset bytes from beginning of file"
    )]
    byte_offset: usize,

    #[clap(
        long,
        default_value = "norm",
        help = "Function for display each FFT complex value"
    )]
    display_func: display::DisplayFun,

    #[clap(long, default_value = "0", help = "Sample count limit")]
    sample_limit: usize,
    //TODO: Diff file formats
    //TODO: Diff ouput file formats
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Default)]
pub enum FreqCentered {
    #[default]
    MovedFFT,
    Original,
}

impl App {
    pub fn new() -> Self {
        let args = Args::parse();
        let color_fun = colors::get_function(&args.colors);
        let display_fun = display::get_display_fun(&args.display_func);
        Self {
            args,
            data: Vec::with_capacity(4096),
            color_fun,
            sample_size: 8,
            display_fun,
        }
    }

    pub fn log_level_filter(&self) -> log::LevelFilter {
        self.args.verbose.log_level_filter()
    }

    pub fn run(&mut self) -> Result<()> {
        self.read_file()?;

        self.prepare_data();

        self.fft();

        self.post_fft();

        self.save_fft_to_image()?;

        Ok(())
    }

    fn read_file(&mut self) -> Result<()> {
        let mut file = std::fs::File::open(self.args.source.clone())?;
        if self.args.byte_offset > 0 {
            let mut readed = 0;
            let mut buf = [0u8; 1];
            while let Ok(n) = file.read(&mut buf) {
                readed += n;
                if readed == self.args.byte_offset {
                    break;
                }
            }
        }
        let mut buf = vec![0; self.sample_size];
        while let Ok(n) = file.read(&mut buf) {
            if n != self.sample_size {
                break;
            }
            if self.args.sample_limit > 0 && self.data.len() >= self.args.sample_limit {
                break;
            }
            let re = f32::from_le_bytes(buf[0..(self.sample_size / 2)].try_into().unwrap());
            let im = f32::from_le_bytes(
                buf[(self.sample_size / 2)..self.sample_size]
                    .try_into()
                    .unwrap(),
            );
            self.data.push(Complex { re, im });
        }

        Ok(())
    }

    fn prepare_data(&mut self) {
        trace!("First complex: {}", self.data[0]);

        info!("Read samples: {}", self.data.len());

        let ltmp = self.data.len() - (self.data.len() % self.args.fft_size);
        self.data.truncate(ltmp);

        info!("Truncate to samples: {}", ltmp);

        info!("Rows: {}", ltmp / self.args.fft_size);
    }

    fn fft(&mut self) {
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(self.args.fft_size);
        fft.process(&mut self.data);
    }

    fn post_fft(&mut self) {
        if self.args.smart_fft_clamp {
            info!("Chosen smart FFT clamp; max/min-fft-value ignore");
            let mut mn = f32::MAX;
            let mut mx = -f32::MAX;
            let mut tmp = 0.0;
            self.data.iter().for_each(|c| {
                tmp = (self.display_fun)(c);
                if tmp < mn {
                    mn = tmp;
                }
                if tmp > mx {
                    mx = tmp;
                }
            });
            self.args.fft_clamp_min = mn;
            self.args.fft_clamp_max = mx + mn.abs();
            info!("Min fft: {}; Max fft: {}", mn, mx);
        }
        debug!("Max fft value: {}", self.args.fft_clamp_max);
    }

    fn save_fft_to_image(&mut self) -> Result<()> {
        let height = self.data.len() / self.args.fft_size;
        let mut img =
            image::ImageBuffer::<image::Rgba<u8>, _>::new(self.args.fft_size as u32, height as u32);
        let mut buff = [0u8; 4];
        let size_fft = self.args.fft_size as u32;
        let mut v = 0.0;
        self.data.iter_mut().enumerate().for_each(|(i, c)| {
            c.re -= self.args.fft_clamp_min;
            c.im -= self.args.fft_clamp_min;
            v = (self.display_fun)(c);
            //.clamp(self.args.fft_clamp_min, self.args.fft_clamp_max);
            let mut x = (i % self.args.fft_size) as u32;
            let y = (i / self.args.fft_size) as u32;

            match self.args.freq_centered {
                FreqCentered::MovedFFT => x = (x + size_fft / 2) % size_fft,
                FreqCentered::Original => {}
            }

            (self.color_fun)(v, 0xff, self.args.fft_clamp_max, &mut buff);
            if self.args.central_line && x as usize == self.args.fft_size / 2 {
                colors::blend_color(&[224, 0, 209, 0xff], &mut buff);
            }
            img.put_pixel(x, y, image::Rgba(buff));
        });

        img.save_with_format(self.args.output.clone(), image::ImageFormat::Png)?;
        Ok(())
    }
}
