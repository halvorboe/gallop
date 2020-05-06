# import grpc

# import indexer_pb2
# import indexer_pb2_grpc

# channel = grpc.insecure_channel('localhost:46515')
# stub = indexer_pb2_grpc.IndexerStub(channel)

# print(stub.Eat(indexer_pb2.Order()))

# import graphene
# from fastapi import FastAPI
# from starlette.graphql import GraphQLApp


# class Query(graphene.ObjectType):
#     hello = graphene.String(name=graphene.String(default_value="stranger"))

#     def resolve_hello(self, info, name):
#         return "Hello " + name


# app = FastAPI()
# app.add_route("/", GraphQLApp(schema=graphene.Schema(query=Query)))

from .coordinator import Coordinator
