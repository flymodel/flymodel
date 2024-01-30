from typing import Generic, List, TypeVar

from .common import CurrentPage

T = TypeVar("T")

class Paginated(Generic[T]):
    data: List[T]
    page: CurrentPage
    total_items: int
    total_pages: int
