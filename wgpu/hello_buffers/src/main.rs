use hello_buffers::run;

fn main() {
    //! todo: learn how to compile this to wasm too
    pollster::block_on(run());
}
