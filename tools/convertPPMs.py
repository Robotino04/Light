import os
import sys

files = sorted(os.listdir(sys.argv[1]))
files = [file for file in files if ".ppm" in file]
pngs = [".".join(file.split(".")[:-1]) + ".png" for file in files]

for i, (ppm, png) in enumerate(zip(files, pngs)):
    print(f'[{i+1:>3}/{len(files)}] "{ppm}" -> "{png}"')
    os.system(f"convert {ppm} {png}")