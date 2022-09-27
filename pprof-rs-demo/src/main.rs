pub mod flamegraph;

fn main() {
    // Flamegraph
    // with dependencies: pprof = { version = "0.10", features = ["flamegraph"] }
    flamegraph::do_something()
}
