import os
import os.path as osp
import shutil
import platform
from glob import glob

plat = platform.system()

def convert_upath(p):
    drive, p = p.split(":")
    return '/' + drive + p.replace('\\', '/')

def find_exe_path(cmd):
    return osp.dirname(shutil.which(cmd))

prefix = os.environ.get("PREFIX", os.environ["CONDA_PREFIX"])

clang_path = find_exe_path("clang")
if plat.startswith("MINGW") or plat == "Windows":
    clang_dlls = glob(osp.join(clang_path, 'libclang*.dll'))
    found = any("libclang.dll" in i for i in clang_dlls)
    if not found:
        shutil.copy2(clang_dlls[0], osp.join(clang_path, "libclang.dll"))
    prefix = osp.join(prefix, 'Library')
elif plat == "Linux":
    clang_path = osp.join(clang_path, "..", "lib")

env = {
    **os.environ,
    "LIBCLANG_PATH": clang_path,
    "PREFIX": prefix,
}

env = Environment(
    ENV=env,
)

env.Command("build", [], "cargo build")
env.Command("test", [], "cargo test --lib -- --nocapture")
env.Default("test")
