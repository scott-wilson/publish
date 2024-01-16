# ruff: noqa: D103,D100,S101

import hypothesis
from hypothesis import strategies

import pypublish


@hypothesis.given(
    key=strategies.text(),
    value=strategies.recursive(
        strategies.one_of(
            strategies.none(),
            strategies.booleans(),
            # By default, integers can be arbitrarily large, while the context
            # can only support signed 64-bit integers.
            strategies.integers(min_value=-1000, max_value=1000),
            strategies.floats(allow_nan=False),
            strategies.text(max_size=10),
        ),
        lambda children: strategies.one_of(
            strategies.lists(children, max_size=10),
            strategies.dictionaries(
                strategies.text(max_size=10), children, max_size=10
            ),
        ),
        max_leaves=10,
    ),
)
def test_context_get_success(key: str, value: pypublish.Value) -> None:
    ctx = pypublish.Context()
    ctx.set(key, value)

    assert ctx.get(key) == value


@hypothesis.given(
    key=strategies.text(),
    value=strategies.recursive(
        strategies.one_of(
            strategies.none(),
            strategies.booleans(),
            # By default, integers can be arbitrarily large, while the context
            # can only support signed 64-bit integers.
            strategies.integers(min_value=-1000, max_value=1000),
            strategies.floats(allow_nan=False),
            strategies.text(max_size=10),
        ),
        lambda children: strategies.one_of(
            strategies.lists(children, max_size=10),
            strategies.dictionaries(
                strategies.text(max_size=10), children, max_size=10
            ),
        ),
        max_leaves=10,
    ),
)
def test_contextview_get_success(key: str, value: pypublish.Value) -> None:
    ctx = pypublish.Context()
    ctx.set(key, value)
    ctx_view = ctx.to_view()

    assert ctx_view.get(key) == value
