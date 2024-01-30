from typing import Optional

__all__ = ["DeleteExperiment", "DeleteExperimentVariables"]
__doc__ = None
__spec__ = None

class DeleteExperimentVariables:
    id: int
    hard: Optional[bool] = False

    def __init__(self, id: int, hard: Optional[bool] = False): ...

class DeleteExperiment:
    delete_experiment: bool
