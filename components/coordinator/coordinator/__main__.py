
import grpc

import coordinator.protos.packer_pb2 as packer
import coordinator.protos.packer_pb2_grpc as packer_grpc

import coordinator.protos.common_pb2 as common

channel = grpc.insecure_channel('localhost:8081')
stub = packer_grpc.PackerStub(channel)

result = stub.Insert(common.Row())
print(result)

result = stub.Segments(packer.SegmentsRequest())
print(result)

result = stub.Segment(packer.SegmentRequest())
print(result)