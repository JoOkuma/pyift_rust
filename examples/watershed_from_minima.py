
import time
import scipy.ndimage as ndi
import napari

from skimage import data, filters
from edt import edt

from pyift_rust import watershed_from_minima_u16_3d


def main() -> None:
    cells = data.cells3d()
    nuclei = cells[:, 1]
    print(nuclei.shape)

    # detect foreground and background
    blurred = ndi.gaussian_filter(nuclei, sigma=2)
    # blurred = ndi.zoom(blurred, 4, order=1)
    foreground = blurred > filters.threshold_otsu(blurred)
    dist = edt(foreground, parallel=4)
    dist = (dist.max() - dist).astype("uint16")
    start = time.time()
    labels = watershed_from_minima_u16_3d(dist, foreground, h=5)
    print("Elapsed time: ", time.time() - start)

    viewer = napari.Viewer()
    viewer.add_image(nuclei)
    viewer.add_image(dist)
    viewer.add_labels(foreground)
    viewer.add_labels(labels)

    napari.run()


if __name__ == "__main__":
    main()
