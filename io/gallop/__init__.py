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
