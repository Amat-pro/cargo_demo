pub mod flamegraph;
pub mod protobuf;

fn main() {
    /// # Flamegraph
    /// ```with dependencies: pprof = { version = "0.10", features = ["flamegraph"] }```
    println!("==> framegraph start");
    flamegraph::do_something();
    println!("==> framegraph done");

    /// # ProtoBuf
    /// ```with protobuf-codec feature```
    ///
    /// &nbsp;
    /// # Operation
    /// * ```brew install graphviz```
    /// * ```$GOPATH/bin/pprof -svg xxx.pb```
    println!("==> protobuf start");
    protobuf::do_something();
    println!("==> protobuf done");
}
