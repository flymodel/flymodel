import pytest

from flymodel import Client, models

from .fixture import client


@pytest.mark.asyncio
async def test_create_experiment(client: Client):
    model = models.create_model.CreateModelVariables(name="test_model", namespace=1)
    model = await client.create_model(model)
    version = models.create_model_version.CreateModelVersionVariables(
        model_id=model.create_model.id, version_tag="v1.0.0rc1"
    )
    version = await client.create_model_version(version)
    new = models.create_experiment.CreateExperimentVariables(
        experiment_name="my-experiment",
        model_version_id=version.create_model_version.id,
    )
    resp = await client.create_experiment(new)

    exp = resp.create_experiment

    assert exp.name == new.experiment_name
    assert exp.version_id == new.model_version_id

    return exp
