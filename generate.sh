protoc -I./protos \
    --rust_out=./src/protos \
    --grpc_out=./src/protos \
    --plugin=protoc-gen-grpc=`which grpc_rust_plugin` \
    ./protos/indexer.proto ./protos/packer.proto  ./protos/coordinator.proto ./protos/common.proto

cat > src/protos/mod.rs << EOF
pub mod common;

pub mod coordinator_grpc;
pub mod coordinator;

pub mod indexer_grpc;
pub mod indexer;

pub mod packer_grpc;
pub mod packer;
EOF
