use clap::ValueEnum;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ExistsFunc{
    RGBSmooth,
    GraySmooth
}

pub fn get_function(s: &ExistsFunc)->ColorFunc{
    match s {
        ExistsFunc::RGBSmooth => smooth_gradient,
        ExistsFunc::GraySmooth => smooth_gradient_gray,
    }
}

pub type ColorFunc = fn( f64, u8, f64, &mut [u8]);

#[inline(always)]
pub fn smooth_gradient_gray(v: f64, alpha: u8, max: f64, dst: &mut [u8]) {
    let c = (v / max * 255.0) as u8;
    unsafe{
        std::ptr::write_bytes(dst.as_mut_ptr(), c, 3);
    }
    dst[3] = alpha;
}

#[rustfmt::skip]
#[inline(always)]
pub fn smooth_gradient(v: f64, alpha: u8, max: f64, dst: &mut [u8]) {
    let x = v / max;
    let h = (-150.0 * x + 237.0).clamp(0.0, 240.0);
    // let h = (88.0 * x.powi(2) - 302.0 * x + 239.0).clamp(0.0, 240.0);
    let hi = (h / 60.0) as u8;
    let vmin = 0.0;
    let v = 100.0;
    let a = (100.0 - vmin) * (h % 60.0) / 60.0;
    let vinc = ((vmin + a) * 255.0 / 100.0) as u8;
    let vdec = ((v - a) * 255.0 / 100.0) as u8;

    let vmin = 0;
    let v = 255;
    match hi {
        0 => {dst[0] = v;    dst[1]= vinc; dst[2] = vmin; dst[3] = alpha;},
        1 => {dst[0] = vdec; dst[1]= v;    dst[2] = vmin; dst[3] = alpha;},
        2 => {dst[0] = vmin; dst[1]= v;    dst[2] = vinc; dst[3] = alpha;},
        3 => {dst[0] = vmin; dst[1]= vdec; dst[2] = v;    dst[3] = alpha;},
        4 => {dst[0] = vinc; dst[1]= vmin; dst[2] = v;    dst[3] = alpha;},
        _ => {dst[0] = v;    dst[1]= vmin; dst[2] = vdec; dst[3] = alpha;},
        // _ => panic!("Out of range color"),
    };
}

#[inline(always)]
pub fn blend_color(color: &[u8; 4], dst: &mut [u8; 4]){
    *dst = [
        color[0] / 2 + dst[0] / 2,
        color[1] / 2 + dst[1] / 2,
        color[2] / 2 + dst[2] / 2,
        color[3] / 2 + dst[3] / 2,
    ]
}