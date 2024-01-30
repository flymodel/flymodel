from typing import Optional

from .common import Page
from .enums import Lifecycle
from .types import Paginated

__all__ = ["Bucket", "PaginatedBucket", "QueryBuckets", "QueryBucketsVariables"]
__doc__ = None
__spec__ = None

class QueryBucketsVariables:
    id: Optional[int]
    namespace: Optional[int]
    page: Optional[Page]
    role: Optional[Lifecycle]

    def __init__(
        self,
        id: Optional[int] = None,
        namespace: Optional[int] = None,
        page: Optional[Page] = None,
        role: Optional[Lifecycle] = None,
    ): ...

class Bucket:
    id: int
    name: str
    namespace: int
    role: Lifecycle
    created_at: str
    last_modified: str

class PaginatedBucket(Paginated[Bucket]): ...

class QueryBuckets:
    bucket: PaginatedBucket
