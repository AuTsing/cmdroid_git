import os

build_cmd = "cargo build --target=aarch64-linux-android"

exit_code = os.system(build_cmd)
if exit_code != 0:
    raise Exception(f"Build failed, exit with code: {exit_code}.")
