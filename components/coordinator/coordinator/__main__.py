# import grpc

# import indexer_pb2
# import indexer_pb2_grpc

# channel = grpc.insecure_channel('localhost:46515')
# stub = indexer_pb2_grpc.IndexerStub(channel)

# print(stub.Eat(indexer_pb2.Order()))

from coordinator import Coordinator
from coordinator.data import Row

coordinator = Coordinator()

coordinator.insert(Row(timestamp=1, data={"title": "Hello, world!"}))
coordinator.insert(Row(timestamp=2, data={"title": "Hello, goose!"}))
coordinator.insert(Row(timestamp=3, data={"title": "Hello, test!"}))

coordinator.sync()

print(coordinator.query("test"))

print(Row.random().time())
