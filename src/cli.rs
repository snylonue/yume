use clap::App;
use clap::Arg;

pub fn app() -> App<'static, 'static> {
    App::new(env!("CARGO_PKG_NAME")).arg(
        Arg::with_name("image")
            .help("image to open")
            .multiple(true)
            .required(false),
    )
}
