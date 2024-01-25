from flymodel_client import models


def test_query_namespaces():
    vars = models.query_namespaces.QueryNamespacesVariables(
        page=models.common.Page(size=25, page=1)
    )

    print(vars, vars.page.size, vars.page.page)
