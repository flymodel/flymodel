import pytest

from flymodel import Client


def _client() -> Client:
    return Client(base_url="http://localhost:9009")


@pytest.fixture
def client() -> Client:
    return _client()
