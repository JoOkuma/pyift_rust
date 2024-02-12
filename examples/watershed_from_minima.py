import time
import scipy.ndimage as ndi
import napari

from skimage import data, filters, segmentation
from edt import edt

from pyift_rust import watershed_from_minima_u8_3d


def main() -> None:
    cells = data.cells3d()
    nuclei = cells[:, 1]

    # detect foreground and background
    blurred = ndi.gaussian_filter(nuclei, sigma=2)
    # blurred = ndi.zoom(blurred, 2, order=1)
    print(blurred.shape)

    foreground = blurred > filters.threshold_otsu(blurred)
    dist = edt(foreground, parallel=4)
    dist = (dist.max() - dist).astype("uint8")

    start = time.time()
    labels = segmentation.watershed(dist, mask=foreground)
    print("Scikit-image watershed time: ", time.time() - start)

    start = time.time()
    labels = watershed_from_minima_u8_3d(dist, mask=foreground, h=5)
    print("IFT watershed time: ", time.time() - start)
    labels, _, _ = segmentation.relabel_sequential(labels)

    viewer = napari.Viewer()
    viewer.add_image(nuclei)
    viewer.add_image(dist)
    viewer.add_labels(foreground)
    viewer.add_labels(labels)
    napari.run()


if __name__ == "__main__":
    main()
