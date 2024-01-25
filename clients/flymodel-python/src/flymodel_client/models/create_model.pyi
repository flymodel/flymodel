__all__ = ["CreateModel", "CreateModelVariables", "Model"]
__doc__ = None
__spec__ = None

class CreateModelVariables:
    name: str
    namespace: int

    def __init__(self, name: str, namespace: int): ...

class Model:
    id: int
    name: str
    namespace_id: int

class CreateModel:
    create_model: Model
