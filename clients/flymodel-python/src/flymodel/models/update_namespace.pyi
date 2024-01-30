from datetime import datetime
from typing import Optional

__all__ = ["UpdateNamespace", "UpdateNamespaceVariables", "Namespace"]
__doc__ = None
__spec__ = None

class UpdateNamespaceVariables:
    id: int
    name: Optional[str] = None
    description: Optional[str] = None

class Namespace:
    id: int
    name: str
    description: str
    last_modified: datetime

class UpdateNamespace:
    update_namespace: Namespace
