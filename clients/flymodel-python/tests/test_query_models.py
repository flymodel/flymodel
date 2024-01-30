from asyncio import gather

import pytest

from flymodel import Client, models

from .fixture import client


@pytest.mark.asyncio
async def test_query_models(client: Client):
    found = await client.query_namespace_models(
        models.query_models.NamespaceModelsVariables(
            model_namespace=1, model_id=None, model_name=None, page=None
        )
    )

    assert isinstance(found.model.data, list)
