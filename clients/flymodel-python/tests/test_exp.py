import zlib
from io import BytesIO

import pytest
from polars import DataFrame

from flymodel.client import UploadExperimentArgs, UploadRequestParams
from flymodel.experiment import Experiment
from flymodel.models.enums import ArchiveCompression, ArchiveFormat

from .fixture import client
from .test_create_experiment import test_create_experiment as initial_experiment


def get_some_df():
    return DataFrame(
        {
            "a": [1, 2, 3],
            "b": [4, 5, 6],
        }
    )


@pytest.mark.asyncio
async def test_it(client):
    exp = await initial_experiment(client)

    async with Experiment(client) as test:
        args = UploadExperimentArgs(
            experiment=exp.id,
            params=UploadRequestParams(
                artifact_name="abctest",
                format=None,
                encode=None,
            ),
        )

        resp = await test.save_artifact(args, b"okok")

        assert resp.name == "abctest"
        assert resp.experiment_id == exp.id
        assert resp.version_id == exp.version_id

        fits = get_some_df()
        bts = BytesIO()
        fits.write_ipc(bts)

        args = UploadExperimentArgs(
            experiment=exp.id,
            params=UploadRequestParams(
                artifact_name="abctest",
                format=ArchiveFormat("arrow"),
                encode=ArchiveCompression("gzip"),
            ),
        )
        resp = await test.save_artifact(args, zlib.compress(bts.getvalue()))

    raise ValueError("err")
