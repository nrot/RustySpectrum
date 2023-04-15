use log::info;

mod app;

fn main() {
    let mut application = app::App::new();
    init_logs(application.log_level_filter());
    let time = std::time::Instant::now();
    application.run().unwrap();
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
