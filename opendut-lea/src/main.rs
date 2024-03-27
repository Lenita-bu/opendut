use std::str::FromStr;
use leptos::*;
use opentelemetry::{global, KeyValue};
use opentelemetry::trace::TracerProvider as _;
use opentelemetry_stdout as stdout;
use tracing::info;
use opentelemetry_sdk::{
    export::trace::{SpanExporter},
    trace::{Tracer, TracerProvider},
};
use opentelemetry_sdk::propagation::TraceContextPropagator;
/*
use std::str::FromStr;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry::{global, KeyValue};
use opentelemetry_sdk::propagation::TraceContextPropagator;
use opentelemetry_sdk::trace::Tracer;
use opentelemetry_sdk::{Resource, runtime};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::filter::Directive;
*/

use tracing_subscriber::fmt::format::{FmtSpan, Pretty};
use tracing_subscriber::layer::SubscriberExt;
//use tracing_subscriber::{EnvFilter, Registry};
//use tracing_subscriber::filter::Directive;
use tracing_subscriber::util::SubscriberInitExt;

use crate::app::App;

mod app;
mod peers;
mod api;
mod dashboard;
mod components;
mod clusters;
mod error;
mod routing;
mod util;
mod licenses;
mod nav;
mod user;

fn main() {

    /*
    console_error_panic_hook::set_once();
    let provider = TracerProvider::builder()
         .with_simple_exporter(stdout::SpanExporter::default())
        .build();
    let tracer = provider.tracer("readme_example");
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    let subscriber = Registry::default().with(telemetry);
*/
    /*
        global::set_text_map_propagator(TraceContextPropagator::new());
        let tracing_filter = EnvFilter::builder()
            .with_default_directive(Directive::from_str("opendut=trace").ok().expect("Could not read default directive"))
            .with_env_var("OPENDUT_LOG")
            .from_env().ok().expect("Could not read the env var");

        let service_name: String = "opendut-lea".to_owned();
        let service_instance_id = "lea_instance".to_owned();


        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_trace_config(
                opentelemetry_sdk::trace::config().with_resource(opentelemetry_sdk::resource::Resource::new(vec![KeyValue::new::<&str, String>(
                    opentelemetry_semantic_conventions::resource::SERVICE_NAME,
                    service_name.into()), KeyValue::new::<&str, String>(
                    opentelemetry_semantic_conventions::resource::SERVICE_INSTANCE_ID,
                    service_instance_id.into())
                ])),
            )
            .install_simple().expect("Failed to initialize tracer.");

*/
        let fmt_layer = tracing_subscriber::fmt::layer()
            .with_ansi(false)
            .without_time()
            .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
            .with_writer(tracing_web::MakeConsoleWriter)
            .pretty();
        let perf_layer = tracing_web::performance_layer()
            .with_details_from_fields(Pretty::default());


        tracing_subscriber::registry()
            .with(fmt_layer)
            .with(perf_layer)
            //.with(tracing_opentelemetry::layer().with_tracer(tracer))
            .init();

        info!("LEA started.");

        mount_to_body(|| view! { <App /> })
    }
