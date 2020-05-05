# Generated by the gRPC Python protocol compiler plugin. DO NOT EDIT!
import grpc

import indexer_pb2 as indexer__pb2


class IndexerStub(object):
    """Missing associated documentation comment in .proto file"""

    def __init__(self, channel):
        """Constructor.

        Args:
            channel: A grpc.Channel.
        """
        self.Eat = channel.unary_unary(
                '/protos.Indexer/Eat',
                request_serializer=indexer__pb2.Order.SerializeToString,
                response_deserializer=indexer__pb2.Check.FromString,
                )


class IndexerServicer(object):
    """Missing associated documentation comment in .proto file"""

    def Eat(self, request, context):
        """Missing associated documentation comment in .proto file"""
        context.set_code(grpc.StatusCode.UNIMPLEMENTED)
        context.set_details('Method not implemented!')
        raise NotImplementedError('Method not implemented!')


def add_IndexerServicer_to_server(servicer, server):
    rpc_method_handlers = {
            'Eat': grpc.unary_unary_rpc_method_handler(
                    servicer.Eat,
                    request_deserializer=indexer__pb2.Order.FromString,
                    response_serializer=indexer__pb2.Check.SerializeToString,
            ),
    }
    generic_handler = grpc.method_handlers_generic_handler(
            'protos.Indexer', rpc_method_handlers)
    server.add_generic_rpc_handlers((generic_handler,))


 # This class is part of an EXPERIMENTAL API.
class Indexer(object):
    """Missing associated documentation comment in .proto file"""

    @staticmethod
    def Eat(request,
            target,
            options=(),
            channel_credentials=None,
            call_credentials=None,
            compression=None,
            wait_for_ready=None,
            timeout=None,
            metadata=None):
        return grpc.experimental.unary_unary(request, target, '/protos.Indexer/Eat',
            indexer__pb2.Order.SerializeToString,
            indexer__pb2.Check.FromString,
            options, channel_credentials,
            call_credentials, compression, wait_for_ready, timeout, metadata)