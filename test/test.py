import glob
import os

outs = []

os.system("cargo install --git https://github.com/DhruvDh/upscaler")

for ext in ["png", "jpg", "jpeg"]:
    outs.extend(glob.glob("./*_*x." + ext))
    outs.extend(glob.glob("./**/*_*x." + ext))

for out in outs:
    # os.system("rm " + out)
    if "_2x" in out:
        s = 2
        _in = out.replace("_2x", "")
        cmd = f"upscaler {_in} {out} -s {s}"
        print(f"Running {cmd}")
        os.system(cmd)

    if "_4x" in out:
        s = 4
        _in = out.replace("_4x", "")
        cmd = f"upscaler {_in} {out} -s {s}"
        print(f"Running {cmd}")
        os.system(cmd)
    