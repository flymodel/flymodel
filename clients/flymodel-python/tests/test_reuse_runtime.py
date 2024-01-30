from asyncio import gather

import pytest

from flymodel import Client, models

from .fixture import client


@pytest.mark.asyncio
async def test_parallelism(client: Client):
    tasks = []
    for i in range(0, 20):
        # this will panic if the tokio runtime is not reused or dropped
        tasks.append(
            client.query_namespaces(
                models.query_namespaces.QueryNamespacesVariables(
                    page=models.common.Page(size=25, page=0)
                )
            )
        )

    for task in await gather(*tasks):
        print(
            f"""name={
            task.namespace.data[0].name
        }"""
        )
