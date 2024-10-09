fn main() {
    let net_out_dir = format!("{}/net_protos", std::env::var("OUT_DIR").unwrap());
    std::fs::create_dir_all(&net_out_dir).unwrap();
    let _ = tonic_build::compile_protos("net_protos/lan_net.proto");
    let out_dir = "../../target".to_string();
    let builder = tonic_build::configure();
    builder
        .out_dir(out_dir)
        .compile(&["net_protos/lan_net.proto"], &["net_protos"])
        .unwrap();
}
