
cd actors/coordinator
pipenv install --dev && pipenv run generate
sed -i "s/import common_pb2/import coordinator.protos.common_pb2/g" coordinator/protos/*
sed -i "s/import packer_pb2/import coordinator.protos.packer_pb2/g" coordinator/protos/*
sed -i "s/import indexer_pb2/import coordinator.protos.indexer_pb2/g" coordinator/protos/*
cd ../..

cargo install protobuf-codegen &&  \
cargo install grpcio-compiler && \
protoc -I./protos --rust_out=./actors/workers/src/protos --grpc_out=./actors/workers/src/protos --plugin=protoc-gen-grpc=`which grpc_rust_plugin` ./protos/indexer.proto ./protos/packer.proto ./protos/common.proto
