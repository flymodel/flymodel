__all__ = ["CreateExperiment", "CreateExperimentVariables", "Experiment"]
__doc__ = None
__spec__ = None

class CreateExperimentVariables:
    experiment_name: str
    model_version_id: int

    def __init__(self, experiment_name: str, model_version_id: int): ...

class Experiment:
    id: int
    name: str
    version_id: int

class CreateExperiment:
    create_experiment: CreateExperiment
