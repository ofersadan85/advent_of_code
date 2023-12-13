use tracing_subscriber::{fmt, fmt::format::FmtSpan};

pub fn init_tracing() {
    fmt()
        .with_line_number(true)
        .with_span_events(FmtSpan::CLOSE)
        .init();
}
