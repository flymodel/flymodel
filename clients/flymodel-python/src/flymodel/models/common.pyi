__all__ = ["CurrentPage", "Page"]
__doc__ = None
__spec__ = None

class Page:
    page: int
    size: int

    def __init__(self, page: int, size: int): ...

class CurrentPage:
    page: int
    size: int
