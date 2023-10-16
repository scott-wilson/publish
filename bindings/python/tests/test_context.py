import hypothesis
import pytest
from hypothesis import strategies

import pypublish


@hypothesis.given(
    key=strategies.text(),
    value=strategies.recursive(
        strategies.one_of(
            strategies.none(),
            strategies.booleans(),
            strategies.integers(),
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
def test_context_get_success(key: str, value: pypublish.Value):
    ctx = pypublish.Context()
    ctx.set(key, value)

    assert ctx.get(key) == value


@hypothesis.given(
    key=strategies.text(),
    value=strategies.recursive(
        strategies.one_of(
            strategies.none(),
            strategies.booleans(),
            strategies.integers(),
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
def test_contextview_get_success(key: str, value: pypublish.Value):
    ctx = pypublish.Context()
    ctx.set(key, value)
    ctx_view = ctx.as_view()

    assert ctx_view.get(key) == value
