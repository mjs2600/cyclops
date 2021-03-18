from pathlib import Path
from typing import Union

import numpy as np

from . import pyclops


def descriptors(image: Union[str, Path]) -> np.ndarray:
    return pyclops.descriptors(str(image))
