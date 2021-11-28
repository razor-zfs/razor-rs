// Copyright (c) 2021 RepliXio Ltd. All rights reserved.
//
// Use is subject to license terms.
//

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

const DEFAULT_TRACE_LEVEL: &str = "info";

pub fn init() -> anyhow::Result<Tracer<impl tracing::Subscriber>> {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(DEFAULT_TRACE_LEVEL));

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
