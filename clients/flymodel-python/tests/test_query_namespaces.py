import pytest
from flymodel_client import Client, models

from .fixture import client


@pytest.mark.asyncio
async def test_query_namespaces(client: Client):
    vars = models.query_namespaces.QueryNamespacesVariables(
        page=models.common.Page(size=25, page=0)
    )
    res = await client.query_namespaces(vars)
    namespaces = res.namespace.data
    assert namespaces[0].id == 1
    assert namespaces[0].name == "canada"
    assert namespaces[0].description == "Flymodel Canada"
