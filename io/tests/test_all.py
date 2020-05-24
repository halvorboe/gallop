import pytest

import grpc

from gallop.protos import coordinator_pb2_grpc as coordinator_grpc
from gallop.protos import coordinator_pb2 as coordinator
from gallop.protos import indexer_pb2_grpc as indexer_grpc
from gallop.protos import packer_pb2_grpc as packer_grpc


@pytest.fixture
def coodinator_stub():
    return coordinator_grpc.CoordinatorStub(grpc.insecure_channel("localhost:7070"))

@pytest.fixture
def packer_stub():
    return packer_grpc.PackerStub(grpc.insecure_channel("localhost:70702"))

@pytest.fixture
def indexer_stub():
    return indexer_grpc.IndexerStub(grpc.insecure_channel("localhost:7071"))

def test_insert(coodinator_stub):
    for i in range(10):
        coodinator_stub.Insert(coordinator.InsertRequest())
