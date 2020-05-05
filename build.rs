extern crate protoc_grpcio;

fn main() {
    let proto_input_root = "protos";
    let proto_output_root = "src/protos";
    println!("cargo:rerun-if-changed={}", proto_input_root);
    protoc_grpcio::compile_grpc_protos(&["indexer.proto"], &[proto_input_root], &proto_output_root, None)
        .expect("Failed to compile gRPC definitions!");
}