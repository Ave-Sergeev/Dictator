fn main() {
    let proto_gen_dir = "./src/pb";
    let proto_file = "./proto/transcribe.proto";
    let arg = "--experimental_allow_proto3_optional";

    tonic_build::configure()
        .protoc_arg(arg)
        .out_dir(proto_gen_dir)
        .build_client(false)
        .build_server(true)
        .compile_protos(&[proto_file], &["proto"])
        .unwrap_or_else(|err| panic!("Failed to compile protos {err}"));

    let lib_path = "./model/vosk";

    println!("cargo:rustc-link-search={lib_path}");
}
