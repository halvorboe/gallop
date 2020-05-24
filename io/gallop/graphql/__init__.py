

routes = [
    Route("/", GraphQLApp(schema=graphene.Schema(query=Query, mutation=Mutations)))
]

app = Starlette(routes=routes)
