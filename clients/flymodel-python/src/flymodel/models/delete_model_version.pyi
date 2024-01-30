from typing import Optional

__all__ = ["DeleteModelVersion", "DeleteModelVersionVariables"]
__doc__ = None
__spec__ = None

class DeleteModelVersionVariables:
    id: int
    hard: Optional[bool] = False

    def __init__(self, id: int, hard: Optional[bool] = False): ...

class DeleteModelVersion:
    delete_model_version: bool
