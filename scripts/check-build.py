import subprocess
import os


def run_cmd(cmd: str):
    """
    执行 shell 命令，返回 (exit_code, stdout, stderr)

    code, out, err = run_cmd("ls -l /tmp")
    print("退出码:", code)
    print("输出:\n", out)
    print("错误:\n", err)
    """
    try:
        result = subprocess.run(
            cmd,
            shell=True,
            check=False,
            text=True,
            capture_output=True,
        )
        return result.returncode, result.stdout, result.stderr
    except Exception as e:
        return -1, "", f"Exception: {e}"


def check_output_dirs(base_dir: str):

    if not os.path.isdir(base_dir):
        print(f"❌ Base directory '{base_dir}' does not exist.")
        exit(1)

    for entry in os.listdir(base_dir):
        subdir_path = os.path.join(base_dir, entry)
        if not os.path.isdir(subdir_path):
            continue

        print(f"Checking entry: {entry}")
        print(f"Checking directory: {subdir_path}")

        header_path = os.path.join(subdir_path, "ffi_client.h")
        if not os.path.exists(header_path):
            print(f"❌ Missing header file in {header_path}")
            exit(1)

        lib_path = os.path.join(subdir_path, "libmy_lib.a")
        if not os.path.exists(lib_path):
            print(f"❌ Missing library file in {lib_path}")
            exit(1)

        code, out, err = run_cmd(f"strings {lib_path} | grep 'my_lib_version'")
        if entry not in out:
            print(f"❌ target {entry} not match {lib_path} !!!")
            exit(1)

    print("✅ All required files are present in the output directories.")


if __name__ == "__main__":
    check_output_dirs("output")
