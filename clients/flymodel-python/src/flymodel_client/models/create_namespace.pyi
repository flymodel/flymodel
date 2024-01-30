from datetime import datetime

__all__ = ["CreateNamespace", "CreateNamespaceVariables", "Namespace"]
__doc__ = None
__spec__ = None

class CreateNamespaceVariables:
    name: str
    description: str

    def __init__(self, name: str, description: str): ...

class Namespace:
    id: int
    name: str
    description: str
    created_at: datetime
    last_modified: datetime

class CreateNamespace:
    create_namespace: Namespace
