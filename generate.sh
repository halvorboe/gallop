
cd components/coordinator
pipenv install --dev && pipenv run generate
cd ../..

cargo install protobuf-codegen &&  \
cargo install grpcio-compiler && \
protoc -I./protos --rust_out=./components/workers/src/protos --grpc_out=./components/workers/src/protos --plugin=protoc-gen-grpc=`which grpc_rust_plugin` ./protos/indexer.proto ./protos/storage.proto ./protos/common.proto
