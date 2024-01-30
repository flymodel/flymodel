from datetime import datetime
from typing import Optional

from .common import Lifecycle

__all__ = ["Bucket", "CreateBucket", "CreateBucketVariables"]
__doc__ = None
__spec__ = None

class CreateBucketVariables:
    name: str
    namespace_id: int
    role: Lifecycle
    region: Optional[str] = None

    def __init__(
        self,
        name: str,
        namespace_id: int,
        role: Lifecycle,
        region: Optional[str] = None,
    ): ...

class Bucket:
    name: str
    namespace: int
    role: Lifecycle
    created_at: datetime
    last_modified: datetime
    region: Optional[str] = None

class CreateBucket:
    create_bucket: Bucket
