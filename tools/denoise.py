import numpy as np
import cv2 as cv
from matplotlib import pyplot as plt
img = cv.imread('/tmp/test0.ppm')
dst = cv.fastNlMeansDenoisingColored(img,None,10,10,7,21)
cv.imwrite("/tmp/test.denoised.ppm", dst)

#plt.subplot(121),plt.imshow(img)
# plt.subplot(122),plt.imshow(dst)
plt.show()