from contextlib import asynccontextmanager
from contextvars import ContextVar
from typing import Callable, Optional, ParamSpec, TypeVar

from flymodel import Client

P = ParamSpec("P")
T = TypeVar("T")

Context: ContextVar[Optional[Client]] = ContextVar("flymodel_client")


def client_context(client: Client):
    @asynccontextmanager
    async def context():
        token = Context.set(client)
        try:
            yield client
        finally:
            Context.reset(token)

    return context


class ContextError(BaseException):
    def __init__(self, message: str):
        self.message = message
        super().__init__(self.message)


def current_client() -> Optional[Client]:
    client = Context.get()
    if client:
        return client
    else:
        raise ContextError("No flymodel client in context scope")


def context(
    func: Optional[Callable[P, T]] = None, get_client: Callable[[], Client] = None
):
    if get_client is None:
        __err__ = "get_client is required"
        raise ValueError(__err__)

    def wrapper(func: Callable[P, T]):
        client = get_client()
        ctx = client_context(client)

        async def inner(*args, **kwargs):
            async with ctx():
                return await func(*args, **kwargs)

        return inner

    return wrapper if func is None else wrapper(func)
