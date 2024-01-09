use tracing_core::Level;
use tracing_subscriber::fmt::writer::MakeWriterExt;

fn main() {
    tracing_subscriber::fmt()
        .with_writer(
            std::io::stdout
                .with_filter(|meta| meta.level() > &Level::ERROR)
                .or_else(std::io::stderr),
        )
        .init();

    tracing::info!("all is well!");
    tracing::error!("oh no!");
}
