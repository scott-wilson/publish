# ruff: noqa: D104,F403
from __future__ import annotations

from typing import Dict, List, Union

import pypublish.pypublish
from pypublish.pypublish import *

Value = Union[None, bool, int, float, str, List["Value"], Dict[str, "Value"]]

__doc__ = pypublish.pypublish.__doc__
