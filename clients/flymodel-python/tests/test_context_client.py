import pytest

from flymodel import Client, client_context, context, current_client

from .fixture import client


@pytest.mark.asyncio
async def test_client_context(client: Client):
    context = client_context(client)
    async with context() as client:
        assert isinstance(client, Client)
        assert isinstance(current_client(), Client)
        assert id(current_client()) == id(client)


@pytest.mark.asyncio
async def test_client_context_provider_noargs(client: Client):
    @context(get_client=lambda: client)
    async def some_main():
        client = current_client()
        assert isinstance(client, Client)

    await some_main()


@pytest.mark.asyncio
async def test_client_context_provider_args(client: Client):
    @context(get_client=lambda: client)
    async def some_main(a: int, b: str):
        assert a == 1
        assert b == "b"
        client = current_client()
        assert isinstance(client, Client)

    await some_main(1, "b")


@pytest.mark.asyncio
async def test_context_nesting(client):
    parent = client_context(client)
    new_client = client_context(Client(base_url="http://localhost:9009"))
    async with parent() as parent:
        assert isinstance(client, Client)
        top = current_client()
        assert isinstance(parent, Client)
        assert id(top) == id(parent)
        async with new_client() as new_client:
            child = current_client()
            assert isinstance(client, Client)
            assert id(parent) != id(new_client)
            assert id(child) == id(new_client)
