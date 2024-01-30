from datetime import datetime
from typing import Optional

from .common import Page
from .types import Paginated

__all__ = [
    "Namespace",
    "PaginatedNamespace",
    "QueryNamespaces",
    "QueryNamespacesVariables",
]
__doc__ = None
__spec__ = None

class QueryNamespacesVariables:
    page: Optional[Page]

    def __init__(self, page: Optional[Page] = None): ...

class Namespace:
    id: str
    name: str
    description: str
    created_at: datetime
    last_modified: datetime

class PaginatedNamespace(Paginated[Namespace]): ...

class QueryNamespaces:
    namespace: PaginatedNamespace
