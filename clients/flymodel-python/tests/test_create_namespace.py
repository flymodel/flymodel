from datetime import datetime

import pytest

from flymodel import models

from .fixture import client


@pytest.mark.asyncio
async def test_create_namespace(client):
    vars = models.create_namespace.CreateNamespaceVariables(
        name="test-namespace",
        description="test-namespace description",
    )
    ns = await client.create_namespace(vars)

    assert ns.create_namespace.name == vars.name
    assert ns.create_namespace.description == vars.description
    assert isinstance(ns.create_namespace.created_at, datetime)
    assert isinstance(ns.create_namespace.last_modified, datetime)
