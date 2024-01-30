from datetime import datetime
from typing import Optional

from .common import Page
from .types import Paginated

__all__ = [
    "Experiment",
    "PaginatedExperiment",
    "QueryExperiment",
    "QueryExperimentVariables",
]
__doc__ = None
__spec__ = None

class QueryExperimentVariables:
    id: Optional[int] = None
    model_id: Optional[int] = None
    name: Optional[str] = None
    page: Optional[Page] = None

    def __init__(
        self,
        id: Optional[int] = None,
        model_id: Optional[int] = None,
        name: Optional[str] = None,
        page: Optional[Page] = None,
    ): ...

class Experiment:
    id: int
    name: str
    version_id: int
    created_at: datetime

class PaginatedExperiment(Paginated[Experiment]): ...

class QueryExperiment:
    experiment: Experiment
