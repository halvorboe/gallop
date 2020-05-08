# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
import grpc

import coordinator.protos.common_pb2 as common__pb2
import coordinator.protos.indexer_pb2 as indexer__pb2


class IndexerStub(object):
    """Missing associated documentation comment in .proto file"""

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.Bind = channel.unary_unary(
                '/protos.Indexer/Bind',
                request_serializer=indexer__pb2.BindRequest.SerializeToString,
                response_deserializer=common__pb2.Error.FromString,
                )
        self.UnBind = channel.unary_unary(
                '/protos.Indexer/UnBind',
                request_serializer=indexer__pb2.UnBindRequest.SerializeToString,
                response_deserializer=common__pb2.Error.FromString,
                )


class IndexerServicer(object):
    """Missing associated documentation comment in .proto file"""

    def Bind(self, request, context):
        """Missing associated documentation comment in .proto file"""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')

    def UnBind(self, request, context):
        """Missing associated documentation comment in .proto file"""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')


def add_IndexerServicer_to_server(servicer, server):
    rpc_method_handlers = {
            'Bind': grpc.unary_unary_rpc_method_handler(
                    servicer.Bind,
                    request_deserializer=indexer__pb2.BindRequest.FromString,
                    response_serializer=common__pb2.Error.SerializeToString,
            ),
            'UnBind': grpc.unary_unary_rpc_method_handler(
                    servicer.UnBind,
                    request_deserializer=indexer__pb2.UnBindRequest.FromString,
                    response_serializer=common__pb2.Error.SerializeToString,
            ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
            'protos.Indexer', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))


 # This class is part of an EXPERIMENTAL API.
class Indexer(object):
    """Missing associated documentation comment in .proto file"""

    @staticmethod
    def Bind(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/protos.Indexer/Bind',
            indexer__pb2.BindRequest.SerializeToString,
            common__pb2.Error.FromString,
            options, channel_credentials,
            call_credentials, compression, wait_for_ready, timeout, metadata)

    @staticmethod
    def UnBind(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/protos.Indexer/UnBind',
            indexer__pb2.UnBindRequest.SerializeToString,
            common__pb2.Error.FromString,
            options, channel_credentials,
            call_credentials, compression, wait_for_ready, timeout, metadata)