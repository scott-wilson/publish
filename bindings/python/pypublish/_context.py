from __future__ import annotations

from typing import Dict, List, Union

Value = Union[None, bool, int, float, str, List["Value"], Dict[str, "Value"]]


class Context:
    """The context used for the publish process.

    This handles the data that is passed between each publish stage. Each time
    the context is passed to a publish stage, it should be copied if there are
    changes.
    """

    def __init__(self) -> None:
        self.__data: Dict[str, Value] = {}

    def get(self, key: str) -> Value:
        return self.__data[key]

    def set(self, key: str, value: Value) -> None:
        self.__data[key] = value

    def copy(self) -> Context:
        context = Context()
        context.__data = self.__data.copy()
        return context

    def as_view(self) -> ContextView:
        return ContextView(self)


class ContextView:
    def __init__(self, context: Context) -> None:
        self.__context = context

    def get(self, key: str) -> Value:
        return self.__context.get(key)

    def copy(self) -> Context:
        return self.__context.copy()

    def as_view(self) -> ContextView:
        return self
