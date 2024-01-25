from flymodel_client import FlymodelClient


def test_client_init():
    _ = FlymodelClient(base_url="http://localhost:9009/graphql")
