# RustySpectrum

A simple app for draws spectrum on image. Each pixel on image is FFT value . Each row is FFT set. On default: swaps FIFTH part to move zero frequency to center (see freq-centered).
# Support only cf32 input format and png output format.

```
spectrum_analyzer file.cf32 -n -f 128 --sample-limit 16000 --fft-clip-max 30
```
 
![Color preview](https://github.com/nrot/RustySpectrum/raw/main/images/spectrum-color.png)
```
spectrum_analyzer file.cf32 -n -f 128 --sample-limit 16000 --fft-clip-max 30 -c gray-smooth
``` 
![Gray preview](https://github.com/nrot/RustySpectrum/raw/main/images/spectrum-gray.png)


```
Usage: spectrum_analyzer [OPTIONS] <SOURCE>

Arguments:
  <SOURCE>  Source cf32 file

Options:
  -o, --output <OUTPUT>                Output png file path [default: spectrum.png]
  -f, --fft-size <FFT_SIZE>            FFT size [default: 1024]
  -v, --verbose...                     More output per occurrence
  -q, --quiet...                       Less output per occurrence
  -c, --colors <COLORS>                Color scheme function [default: rgb-smooth] [possible values: rgb-smooth, gray-smooth]
      --fft-clamp-min <FFT_CLAMP_MIN>  Value for added to each complex number [default: 0.0]
      --fft-clamp-max <FFT_CLAMP_MAX>  Value for color function. Example: Gray color scheme black==0.0, white==max value [default: 1.0]
  -s, --smart-fft-clamp                Searched min and max value for all complex data. Use 'display function' for searching
  -n, --normalize-fft                  Normalize fft. Each complex value divide fft-size
      --central-line                   Draws purple central line
      --freq-centered <FREQ_CENTERED>  Default main freq moved to center. Original draws raw fft [default: moved-fft] [possible values: moved-fft, original]
      --byte-offset <BYTE_OFFSET>      Offset bytes from beginning of file [default: 0]
      --display-func <DISPLAY_FUNC>    Function for display each FFT complex value [default: norm] [possible values: norm, real, image]
      --sample-limit <SAMPLE_LIMIT>    Sample count limit [default: 0]
  -h, --help                           Print help
  -V, --version                        Print version
```

## Build from source
```
  git clone https://github.com/nrot/RustySpectrum.git
  cd RustySpectrum
  cargo build --release
```
