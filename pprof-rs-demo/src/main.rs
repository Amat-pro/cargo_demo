pub mod flamegraph;

fn main() {
    // Flamegraph
    // with dependencies: pprof = { version = "0.10", features = ["flamegraph"] }
    println!("==> framegraph start");
    flamegraph::do_something();
    println!("==> framegraph done");
}
