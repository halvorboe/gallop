import grpc

import indexer_pb2
import indexer_pb2_grpc

channel = grpc.insecure_channel('localhost:46515')
stub = indexer_pb2_grpc.IndexerStub(channel)

print(stub.Eat(indexer_pb2.Order()))