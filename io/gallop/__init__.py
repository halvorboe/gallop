# from starlette.applications import Starlette
# from starlette.routing import Route
# from starlette.graphql import GraphQLApp
# from starlette.responses import JSONResponse
# import graphene
# import random

# import json

# # async def insert(request):
# #     print(f"Inserting: {request}")
# #     return JSONResponse({'message': 'ok'})


# class Query(graphene.ObjectType):
#     search = graphene.String()
#     # search = graphene.Field(Rows) #  , table=graphene.String(), query=graphene.String())

#     def resolve_search(self, info, table, query):
#         # print(table, query)
#         # return {
#         #     "number_of_hits": random.randint(0, 1000),
#         #     "hits": [
#         #         {
#         #             "score": 0.123,
#         #             "timestamp": 123,
#         #             "data": "Hello, world!",
#         #         }
#         #     ]

#         # }
#         # return {
#         #     "number_of_hits": 0,
#         #     # "rows": [],
#         # }
#         return "Hello, world!"


# routes = [
#     # Route('/api/v1/insert', insert),
#     Route('/', GraphQLApp(schema=graphene.Schema(query=Query)))
# ]


# app = Starlette(routes=routes, debug=True)


from starlette.applications import Starlette
from starlette.routing import Route
from starlette.graphql import GraphQLApp
import graphene


class DataType(graphene.Enum):
    STRING = 0


class Data(graphene.ObjectType):
    key = graphene.String()
    value = graphene.String()


class Row(graphene.ObjectType):
    timestamp = graphene.Int()
    data = graphene.List(Data)

class Column(graphene.ObjectType):
    name = graphene.String()
    data_type = graphene.Field(DataType)

class Table(graphene.ObjectType):
    name = graphene.String()

    columns = graphene.List(Column)
    rows = graphene.List(Row)

    def resolve_columns(self, info):
        return [{
            "name": "message",
            "data_type": 0,
        }]

    def resolve_rows(self, info):
        return [{
            "timestamp": 123,
            "data": [{
                "key": "message",
                "value": "Hello, world!"
            }],
        }]
    


class Query(graphene.ObjectType):
    tables = graphene.List(graphene.String)
    table = graphene.Field(Table, name=graphene.String())

    def resolve_tables(self, info):
        return ["a", "b", "c"]
    
    def resolve_table(self, info, name):
        return {"name": name}


class Mutations(graphene.ObjectType):
    insert_row = graphene.String()


routes = [
    Route('/', GraphQLApp(schema=graphene.Schema(query=Query, mutation=Mutations)))
]

app = Starlette(routes=routes)