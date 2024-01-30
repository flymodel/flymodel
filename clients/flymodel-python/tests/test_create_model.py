import pytest

from flymodel import Client, models

from .fixture import client


@pytest.mark.asyncio
async def test_create_model(client: Client):
    vars = models.create_model.CreateModelVariables(
        name="llm.sm.test.python", namespace=1
    )

    ret = await client.create_model(vars)

    assert ret.create_model.name == vars.name
    assert ret.create_model.namespace_id == vars.namespace
