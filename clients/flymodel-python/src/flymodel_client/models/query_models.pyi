from typing import Optional

from .common import Page
from .types import Paginated

__all__ = ["Model", "NamespaceModels", "NamespaceModelsVariables", "PaginatedModel"]
__doc__ = None
__spec__ = None

class NamespaceModelsVariables:
    model_id: Optional[int]
    model_name: Optional[str]
    model_namespace: Optional[int]
    page: Optional[Page]

    def __init__(
        self,
        model_id: Optional[int] = None,
        model_name: Optional[str] = None,
        model_namespace: Optional[int] = None,
        page: Optional[Page] = None,
    ): ...

class Model:
    id: int
    name: str
    namespace_id: int
    created_at: str
    last_modified: str

class PaginatedModel(Paginated[Model]): ...

class NamespaceModels:
    model: PaginatedModel
