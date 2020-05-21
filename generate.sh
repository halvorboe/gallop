
cd io
pipenv run generate
find . | grep -E "(__pycache__|\.pyc|\.pyo$)" | xargs rm -rf
sed -i "s/import common_pb2/import gallop.protos.common_pb2/g" gallop/protos/*
sed -i "s/import packer_pb2/import gallop.protos.packer_pb2/g" gallop/protos/*
sed -i "s/import indexer_pb2/import gallop.protos.indexer_pb2/g" gallop/protos/*
sed -i "s/import coordinator_pb2/import gallop.protos.coordinator_pb2/g" gallop/protos/*
cd ..

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
