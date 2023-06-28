import numpy as np
import cv2 as cv
from matplotlib import pyplot as plt

import sys

for file in sys.argv[1:]:
    img = cv.imread(file)
    dst = cv.fastNlMeansDenoisingColored(img,None,10,10,7,21)
    export_path = file.split(".")
    export_path[-2] += "_denoised"
    cv.imwrite(".".join(export_path), dst)

#plt.subplot(121),plt.imshow(img)
#plt.subplot(122),plt.imshow(dst)
#plt.show()
