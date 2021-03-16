from pathlib import Path
from typing import Union

import numpy as np

from . import pyclops


def sift_descriptors(image: Union[str, Path]) -> np.ndarray:
    return pyclops.sift_descriptors(str(image))
