
import grpc

import coordinator.protos.packer_pb2 as packer
import coordinator.protos.packer_pb2_grpc as packer_grpc

import coordinator.protos.common_pb2 as common

channel = grpc.insecure_channel('localhost:40091')
stub = packer_grpc.PackerStub(channel)

stub.Insert(common.Row())

