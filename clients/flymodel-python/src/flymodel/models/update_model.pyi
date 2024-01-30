from datetime import datetime

__all__ = ["UpdateModel", "UpdateModelVariables", "Model"]
__doc__ = None
__spec__ = None

class UpdateModelVariables:
    id: int
    name: str

    def __init__(self, id: int, name: str): ...

class Model:
    id: int
    name: str
    last_modified: datetime

class UpdateModel:
    update_model: Model
