import numpy as np
from numpy.typing import ArrayLike
from pyift import _rust_ift

WS_FROM_MINIMA = {
    ("uint8", 2): _rust_ift.watershed_from_minima_u8_2d,
    ("uint8", 3): _rust_ift.watershed_from_minima_u8_3d,
    ("uint16", 2): _rust_ift.watershed_from_minima_u16_2d,
    ("uint16", 3): _rust_ift.watershed_from_minima_u16_3d,
    ("uint32", 2): _rust_ift.watershed_from_minima_u32_2d,
    ("uint32", 3): _rust_ift.watershed_from_minima_u32_3d,
    ("int16", 2): _rust_ift.watershed_from_minima_i16_2d,
    ("int16", 3): _rust_ift.watershed_from_minima_i16_3d,
    ("int32", 2): _rust_ift.watershed_from_minima_i32_2d,
    ("int32", 3): _rust_ift.watershed_from_minima_i32_3d,
    ("int64", 2): _rust_ift.watershed_from_minima_i64_2d,
    ("int64", 3): _rust_ift.watershed_from_minima_i64_3d,
    ("float32", 2): _rust_ift.watershed_from_minima_f32_2d,
    ("float32", 3): _rust_ift.watershed_from_minima_f32_3d,
    ("float64", 2): _rust_ift.watershed_from_minima_f64_2d,
    ("float64", 3): _rust_ift.watershed_from_minima_f64_3d,
}


def watershed_from_minima(
    topology: ArrayLike,
    mask: ArrayLike | None,
    h: int | float,
) -> ArrayLike:
    """
    Compute the watershed transform from minima of a grayscale image (topology).

    Parameters
    ----------
    topology : ArrayLike
        2 or 3-D grayscale image.
    mask : ArrayLike, optional
        Binary mask of the same shape as `topology`. If `None`, the whole image is considered.
    h : int or float
        Minimum height of the catchment basins.

    Returns
    -------
    ArrayLike
        Labels from the watershed transform of the image.
    """
    if mask is None:
        mask = np.ones_like(topology, dtype=bool)

    if mask.shape != topology.shape:
        raise ValueError(
            f"Mask and topology must have the same shape. Found {mask.shape} and {topology.shape} instead."
        )

    ws_func = WS_FROM_MINIMA.get((str(topology.dtype), topology.ndim))
    if ws_func is None:
        if topology.ndim != 2 and topology.ndim != 3:
            raise ValueError(f"Unsupported dimension {topology.ndim}, must be 2 or 3.")
        else:
            raise ValueError(f"Unsupported dtype {topology.dtype}.")

    return ws_func(topology, mask, h)
