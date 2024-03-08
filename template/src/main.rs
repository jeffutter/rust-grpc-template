mod {{project-name | snake_case}}_service;

use crate::proto::{{project-name | snake_case}}_server::{{project-name | upper_camel_case}}Server;
use crate::{{project-name | snake_case}}_service::{{project-name | upper_camel_case}}Service;
use clap::Parser;
use std::time::Duration;
use tonic::transport::Server;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub mod proto {
    tonic::include_proto!("{{project-name | snake_case }}");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("{{project-name | snake_case }}_descriptor");
}

/// Commandline Args
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    pretty_traces: bool,

    #[arg(long, env = "PORT", default_value_t = 8080)]
    port: usize,

    #[arg(long, default_value_t = 2_000)]
    concurrency_limit: usize,

    #[arg(long, default_value_t = 1_000)]
    request_timeout: u64,
}

/// Setup tracing and logging
fn setup_tracing(args: &Args) {
    let default_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
    if args.pretty_traces {
        tracing_subscriber::registry()
            .with(tracing_forest::ForestLayer::default().with_filter(default_filter))
            .init();
    } else {
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer().with_filter(default_filter))
            .init();
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    setup_tracing(&args);

    let addr = format!("0.0.0.0:{}", args.port).parse()?;

    let {{project-name | snake_case}} = {{project-name | upper_camel_case}}Service {};

    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<{{project-name | upper_camel_case}}Server<{{project-name | upper_camel_case}}Service>>()
        .await;

    println!("Running on {}", addr);

    let layer = tower::ServiceBuilder::new()
        .concurrency_limit(args.concurrency_limit)
        .timeout(Duration::from_millis(args.request_timeout))
        .into_inner();

    Server::builder()
        .layer(layer)
        .trace_fn(|_| tracing::info_span!("server"))
        .add_service(health_service)
        .add_service(reflection)
        .add_service({{project-name | upper_camel_case}}Server::new({{project-name | snake_case}}))
        .serve(addr)
        .await?;

    Ok(())
}
