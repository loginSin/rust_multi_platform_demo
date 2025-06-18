import subprocess
import os

HEADER_NAME = "ffi_client.h"
STATIC_LIB_NAME = "libmy_lib.a"

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

def check_includes_for_header(base_dir: str):
    print(f"🔍 检查 includes 目录下是否有 {HEADER_NAME}:")

    for root, dirs, files in os.walk(base_dir):
        if os.path.basename(root) == "includes":
            header_path = os.path.join(root, HEADER_NAME)
            if os.path.isfile(header_path):
                print(f"✅ 找到: {header_path}")
            else:
                print(f"❌ 缺失: {header_path}")

def check_static_libs(base_dir: str):
    print("🔍 检查所有目录下是的静态库")
    
    for root, dirs, files in os.walk(base_dir):
        if STATIC_LIB_NAME in files:
            full_path = os.path.join(root, STATIC_LIB_NAME)
            dir_name = os.path.basename(root)
            code, out, err = run_cmd(f"strings {full_path} | grep 'my_lib_version'")
            if dir_name in out:
                print(f"✅ 找到静态库: {full_path}，版本信息: {out.strip()}")
            else:
                print(f"❌ target {dir_name} not match {full_path} !!!")
                exit(1)

def check_output_dirs(base_dir: str):

    if not os.path.isdir(base_dir):
        print(f"❌ Base directory '{base_dir}' does not exist.")
        exit(1)

    check_includes_for_header(base_dir)
    check_static_libs(base_dir)


if __name__ == "__main__":
    check_output_dirs("output")
