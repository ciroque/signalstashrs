fn main() {
    prost_build::Config::new()
        .out_dir("src/")
        .compile_protos(&["proto/sensor.proto"], &["proto"])
        .unwrap();
}
