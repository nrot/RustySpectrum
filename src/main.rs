use clap::Parser;
use log::trace;

use std::path::PathBuf;

use crate::fft::do_fft;

#[allow(unused_imports)]
use log::{debug, info};

mod colors;
mod fft;
mod files;

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
    freq_centered: files::FreqCentered,

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
    display_func: files::DisplayFun,
    
    //TODO: Limit by samples
    //TODO: Diff file formats
    //TODO: Diff ouput file formats
}

fn main() {
    let args = Args::parse();
    init_logs(args.verbose.log_level_filter());
    let time = std::time::Instant::now();

    let mut data = files::read_file(&args.source, args.byte_offset)
        .expect(&format!("Can`t read source file: {:?}", args.source));
    trace!("First complex: {}", data[0]);

    info!("Read samples: {}", data.len());

    let ltmp = data.len() - (data.len() % args.fft_size);
    unsafe {
        data.set_len(ltmp);
    }
    info!("Truncate to samples: {}", ltmp);

    info!("Row: {}", ltmp / args.fft_size);

    do_fft(&mut data, args.fft_size);
    let color_fun = colors::get_function(args.colors);

    if args.normalize_fft {
        data.iter_mut().for_each(|c| {
            *c = *c / args.fft_size as f32;
        });
    }

    let disp_f = files::get_display_fun(args.display_func);

    let mut max_fft_value = args.fft_clamp_max;
    let mut min_fft_value = args.fft_clamp_min;
    if args.smart_fft_clamp {
        info!("Chosen smart FFT clamp; max/min-fft-value ignore");
        let mut mn = f32::MAX;
        let mut mx = -f32::MAX;
        data.iter().for_each(|c| {
            if disp_f(c) < mn {
                mn = disp_f(c);
            }
            if disp_f(c) > mx {
                mx = disp_f(c);
            }
        });
        min_fft_value = mn;
        max_fft_value = mx + mn.abs();
        info!("Min fft: {}; Max fft: {}", mn, mx);
    }
    data.iter_mut().for_each(|c| {
        c.re = c.re - min_fft_value;
        c.im = c.im - min_fft_value;
    });
    debug!("Max fft value: {}", max_fft_value);

    let data_f32 = data.iter().map(disp_f).collect();

    let proccess = files::FFTImage {
        src: &data_f32,
        size_fft: args.fft_size,
        output: args.output,
        color: color_fun,
        max: max_fft_value,
        central_line: args.central_line,
        freq_centered: args.freq_centered,
    };
    proccess
        .save_fft_to_image()
        .expect("Can`t write data to image");

    info!("Take time: {:.2}s", time.elapsed().as_secs_f32());
}

fn init_logs(log_level: log::LevelFilter) {
    let colors = fern::colors::ColoredLevelConfig::default()
        .info(fern::colors::Color::Blue)
        .debug(fern::colors::Color::Yellow)
        .trace(fern::colors::Color::Magenta);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{color_line}{}[{}][{}{color_line}]\x1B[0m {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                colors.color(record.level()),
                message,
                color_line =
                    format_args!("\x1B[{}m", colors.get_color(&record.level()).to_fg_str())
            ))
        })
        .level(log_level)
        .chain(std::io::stdout())
        .apply()
        .unwrap();
}
