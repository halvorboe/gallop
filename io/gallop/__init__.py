from starlette.applications import Starlette
from starlette.routing import Route
from starlette.graphql import GraphQLApp
from starlette.responses import JSONResponse
import graphene
import random


async def insert(request):
    print(f"Inserting: {request}")
    return JSONResponse({'message': 'ok'})


class Document(graphene.ObjectType):
    score = graphene.Float()
    title = graphene.String()
    body = graphene.String()


class Documents(graphene.ObjectType):
    number_of_hits = graphene.Int()
    hits = graphene.List(Document)


class Query(graphene.ObjectType):
    search = graphene.Field(Documents, query=graphene.String())

    def resolve_search(self, info, query):
        return {
            "number_of_hits": random.randint(0, 1000),
            "hits": [
                {
                    "score": 0.123,
                    "title": "Hello, world!",
                    "body": "Is there anybody out there?!",
                }
            ]

        }

routes = [
    Route('/api/v1/insert', insert),
    Route('/', GraphQLApp(schema=graphene.Schema(query=Query)))
]


app = Starlette(routes=routes, debug=True)