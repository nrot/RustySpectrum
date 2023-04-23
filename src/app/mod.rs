use anyhow::Result;
use clap::{Parser, ValueEnum};
use rustfft::{num_complex::Complex, FftPlanner};
use std::{io::Read, path::PathBuf};

mod colors;
mod display;
mod file_format;
mod emath;

#[allow(unused_imports)]
use log::{debug, info, trace, warn};

use crate::app::file_format::FormatError;

use self::{display::DISPLAY_FN_NAME, file_format::FileFormat};

pub struct App {
    args: Args,
    data: Vec<Complex<f64>>,
    color_fun: colors::ColorFunc,
    display_fun: rhai::AST,
    rhai_engine: rhai::Engine,
    rhai_scope: rhai::Scope<'static>
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
    fft_clamp_min: f64,

    #[clap(
        long,
        default_value = "1.0",
        help = "Value for color function. Example: Gray color scheme black==0.0, white==max value"
    )]
    fft_clamp_max: f64,

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
        help = "Function for display each FFT complex value; Custom must set custom-function"
    )]
    display_func: display::DisplayFun,

    #[clap(
        long,
        default_value = "",
        help = "Custom eval function for display function. Have one value complex<f64>{re, im}; Requered f64 output; Have some math functions"
    )]
    custom_function: String,

    #[clap(long, default_value = "0", help = "Sample count limit")]
    sample_limit: usize,

    #[clap(long, help = "File format; Default fc32")]
    format: Option<FileFormat>, //TODO: Diff file formats
                                //TODO: Diff ouput file formats
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Default)]
pub enum FreqCentered {
    #[default]
    MovedFFT,
    Original,
}

impl App {
    pub fn new() -> Result<Self> {
        let args = Args::parse();
        let color_fun = colors::get_function(&args.colors);
        let mut rhai_engine = rhai::Engine::new();
        emath::add_math(&mut rhai_engine);
        let display_fun = rhai_engine.compile(display::get_display_fun(
            &args.display_func,
            args.custom_function.clone(),
        ))?;
        let mut scope = rhai::Scope::new();
        let _ = rhai_engine.call_fn::<f64>(&mut scope, &display_fun, DISPLAY_FN_NAME, (0.0, 0.0)).unwrap();
        
        Ok(Self {
            args,
            data: Vec::with_capacity(4096),
            color_fun,
            display_fun,
            rhai_engine,
            rhai_scope: scope
        })
    }

    pub fn log_level_filter(&self) -> log::LevelFilter {
        self.args.verbose.log_level_filter()
    }

    pub fn run(&mut self) -> Result<()> {
        self.read_file()?;

        self.prepare_data();

        self.fft();

        self.post_fft()?;

        self.save_fft_to_image()?;

        Ok(())
    }

    fn read_file(&mut self) -> Result<()> {
        let mut file = std::fs::File::open(self.args.source.clone())?;
        let file_format = if let Some(format) = self.args.format {
            format
        } else if let Some(ff) = self.args.source.extension() {
            match ff.to_ascii_lowercase().to_str() {
                Some(ff) => match ff {
                    "cf32" | "cfile" => FileFormat::Cf32,
                    "cf64" => FileFormat::Cf64,
                    "cs32" => FileFormat::Cs32,
                    "cs16" => FileFormat::Cs16,
                    "cs8" => FileFormat::Cs8,
                    "cu8" => FileFormat::Cu8,
                    "f32" => FileFormat::F32,
                    "f64" => FileFormat::F64,
                    "s16" => FileFormat::S16,
                    "s8" => FileFormat::S16,
                    "u8" => FileFormat::U8,
                    f => {
                        return Err(FormatError {
                            find_format: String::from(f),
                        }
                        .into())
                    }
                },
                None => {
                    return Err(FormatError {
                        find_format: String::from(ff.to_string_lossy()),
                    }
                    .into())
                }
            }
        } else {
            FileFormat::default()
        };
        info!("Used file format: {:?}", file_format);
        let sample_size = file_format.sample_size();
        debug!("Sample size: {}", sample_size);
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
        let mut buf = vec![0; sample_size];
        while let Ok(n) = file.read(&mut buf) {
            if n != sample_size {
                break;
            }
            if self.args.sample_limit > 0 && self.data.len() >= self.args.sample_limit {
                break;
            }
            self.data.push(file_format.covert(&buf));
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

    fn post_fft(&mut self) -> Result<()> {
        if self.args.smart_fft_clamp {
            info!("Chosen smart FFT clamp; max/min-fft-value ignore");
            let mut mn = f64::MAX;
            let mut mx = -f64::MAX;
            #[allow(unused_assignments)]
            let mut tmp = 0.0;
            for c in self.data.iter() {
                // tmp = App::eval_func(self.display_fun.clone(), c)?;
                tmp = self.rhai_engine.call_fn(&mut self.rhai_scope, &self.display_fun, DISPLAY_FN_NAME, (c.re, c.im)).unwrap();
                if tmp < mn {
                    mn = tmp;
                }
                if tmp > mx {
                    mx = tmp;
                }
            }
            self.args.fft_clamp_min = mn;
            self.args.fft_clamp_max = mx + mn.abs();
            info!("Min fft: {}; Max fft: {}", mn, mx);
        }
        debug!("Max fft value: {}", self.args.fft_clamp_max);
        Ok(())
    }

    fn save_fft_to_image(&mut self) -> Result<()> {
        let height = self.data.len() / self.args.fft_size;
        let mut img =
            image::ImageBuffer::<image::Rgba<u8>, _>::new(self.args.fft_size as u32, height as u32);
        let mut buff = [0u8; 4];
        let size_fft = self.args.fft_size as u32;
        #[allow(unused_assignments)]
        let mut v = 0.0;
        for (i, c) in self.data.iter_mut().enumerate() {
            c.re -= self.args.fft_clamp_min;
            c.im -= self.args.fft_clamp_min;
            // v = App::eval_func(self.display_fun.clone(), c)?;
            v = self.rhai_engine.call_fn(&mut self.rhai_scope, &self.display_fun, DISPLAY_FN_NAME, (c.re, c.im)).unwrap();
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
        }

        img.save_with_format(self.args.output.clone(), image::ImageFormat::Png)?;
        Ok(())
    }
}
