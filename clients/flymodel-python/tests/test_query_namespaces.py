from asyncio import run

import pytest
import yappi

from flymodel import Client, models

from .fixture import client
from .profile import profile

QUERY = models.query_namespaces.QueryNamespacesVariables(
    page=models.common.Page(size=25, page=0)
)


async def run(client: Client):
    return await client.query_namespaces(QUERY)


@pytest.mark.asyncio
async def test_query_namespaces(client: Client):
    res = await run(client)
    namespaces = res.namespace.data
    assert namespaces[0].id == 1
    assert namespaces[0].name == "canada"
    assert namespaces[0].description == "Flymodel Canada"


vprof_namespace = profile(run, n_its=100)
