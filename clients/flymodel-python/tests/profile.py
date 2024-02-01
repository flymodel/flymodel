from dataclasses import dataclass
from typing import Callable


@dataclass
class Profile:
    name: str
    n_its: int
    bench: Callable


def profile(func=None, n_its: int = 10):
    def wrapper(func):
        b = Profile(name=func.__qualname__, n_its=n_its, bench=func)
        b.__name__ = b.name
        b.__qualname__ = b.name
        return b

    return wrapper(func) if func else wrapper
