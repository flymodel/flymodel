from .common import Lifecycle

__all__ = ["UpdateModelVersionState", "UpdateModelVersionStateVariables", "ModelState"]
__doc__ = None
__spec__ = None

class UpdateModelVersionStateVariables:
    id: int
    state: Lifecycle

class ModelState:
    id: int
    version_id: int
    state: Lifecycle

class UpdateModelVersionState:
    update_model_version_state: ModelState
