
cd io
pipenv install --dev && pipenv run generate
sed -i "s/import common_pb2/import gallop.protos.common_pb2/g" gallop/protos/*
sed -i "s/import packer_pb2/import gallop.protos.packer_pb2/g" gallop/protos/*
sed -i "s/import indexer_pb2/import gallop.protos.indexer_pb2/g" gallop/protos/*
cd ..

cargo install protobuf-codegen &&  \
cargo install grpcio-compiler && \
protoc -I./protos --rust_out=./src/protos --grpc_out=./src/protos --plugin=protoc-gen-grpc=`which grpc_rust_plugin` ./protos/indexer.proto ./protos/packer.proto ./protos/common.proto
