#![cfg_attr(feature = "pedantic", warn(clippy::pedantic))]
#![warn(clippy::use_self)]
#![warn(clippy::map_flatten)]
#![warn(clippy::map_unwrap_or)]
#![warn(deprecated_in_future)]
#![warn(future_incompatible)]
#![warn(noop_method_call)]
#![warn(unreachable_pub)]
#![warn(missing_debug_implementations)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2021_compatibility)]
#![warn(rust_2018_idioms)]
#![warn(unused)]
#![deny(warnings)]

use tracing_subscriber::{fmt, reload::Handle, EnvFilter};

//const LOG_DIR: &str = "/var/log";
//const LOG_PREFIX: &str = "zfstrace";

pub fn init() -> anyhow::Result<Tracer<impl tracing::Subscriber>> {
    // fs::create_dir_all(LOG_DIR).context("Failed to create log dir")?;

    // let file_appender = tracing_appender::rolling::daily(LOG_DIR, LOG_PREFIX);
    // let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // let collector = tracing_subscriber::registry()
    //     .with(EnvFilter::from_default_env().add_directive(tracing::Level::WARN.into()))
    //     //.with(fmt::Subscriber::new().with_writer(io::stdout))
    //     .with(fmt::Subscriber::builder().with_writer(non_blocking).finish());
    // tracing::subscriber::set_global_default(collector)
    //     .expect("Unable to set a global collector");

    //let filter = EnvFilter::new(tracing::Level::WARN.into()).add_directive(from_default_env().into());
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));

    let builder = fmt()
        .with_env_filter(filter)
        .with_timer(fmt::time::ChronoUtc::default())
        .with_filter_reloading();

    let handler = builder.reload_handle();

    builder.init();

    Ok(Tracer { handler })
}

#[derive(Debug)]
pub struct Tracer<S> {
    handler: Handle<EnvFilter, S>,
}

impl<S> Tracer<S>
where
    S: tracing::Subscriber,
{
    pub fn reload(&self, dirs: String) -> anyhow::Result<()> {
        self.handler
            .reload(EnvFilter::new(dirs))
            .map_err(|e| e.into())
    }
}
