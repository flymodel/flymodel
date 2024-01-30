from enum import IntEnum

__all__ = ["Lifecycle"]
__doc__ = None
__spec__ = None

class Lifecycle(IntEnum):
    Prod = 0
    Qa = 1
    Stage = 2
    Test = 3
