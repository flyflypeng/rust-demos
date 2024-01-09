mod a {
    pub mod b {
        pub mod c {
            pub fn hello() {
                tracing::info!("hello JPF!");
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_target(true)
        .with_file(true)
        .with_line_number(true)
        .with_level(true)
        // .with_thread_ids(true)
        // .with_thread_names(true)
        // .without_time()
        .init();

    a::b::c::hello();
}
