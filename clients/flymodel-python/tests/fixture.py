import pytest
from flymodel import Client


@pytest.fixture
def client() -> Client:
    return Client(base_url="http://localhost:9009")
