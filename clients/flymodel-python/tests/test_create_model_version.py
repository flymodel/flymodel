import pytest

from flymodel import Client, models

from .fixture import client


@pytest.mark.asyncio
async def test_create_model_version(client: Client):
    model = models.create_model.CreateModelVariables(name="test_model", namespace=1)
    model = await client.create_model(model)
    version = models.create_model_version.CreateModelVersionVariables(
        model_id=model.create_model.id, version_tag="v1.0.0rc1"
    )
    version = await client.create_model_version(version)
    assert isinstance(version, models.create_model_version.CreateModelVersion)

    version = version.create_model_version
    assert version.model_id == model.create_model.id
    assert version.version == "v1.0.0rc1"
