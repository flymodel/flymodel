__all__ = ["CreateModelVersion", "CreateModelVersionVariables", "ModelVersion"]
__doc__ = None
__spec__ = None

class CreateModelVersionVariables:
    model_id: int
    version_tag: str

    def __init__(self, version_tag: str, model_id: int): ...

class ModelVersion:
    id: int
    version: str
    model_id: int

class CreateModelVersion:
    create_model_version: ModelVersion
