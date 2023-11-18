use hello_wgsl::run;

fn main() {
    //! todo: learn how to compile this to wasm too
    pollster::block_on(run());
}
