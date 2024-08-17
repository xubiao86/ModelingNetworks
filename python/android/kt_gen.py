import os
import shutil

from jinja2 import Template

from request_data import RequestData
from constant import templates_directory, output_base_directory

android_target_dir = "./android/app/src/main/java/com/succulents/request/models"


class KtGen:
    def __init__(self, file_name: str, data: RequestData) -> None:
        self.file_name = file_name
        self.data = data

    def gen(self):
        template_file_path = os.path.join(templates_directory, "android.kt")

        # 读取模板文件
        with open(template_file_path, "r") as file:
            template_content = file.read()

        # 使用 Jinja2 解析模板并填充数据
        template = Template(template_content, trim_blocks=False, lstrip_blocks=True)

        package_name = get_package_name(android_target_dir)
        output = template.render(
            package_name=package_name,
            class_name=f"{self.file_name}RequestModel",
            url=self.data.url,
            path=self.data.path,
            method=self.data.method,
            header=self.data.headers,
            query=self.data.query,
            body=self.data.body,
        )

        # 确定输出目录和文件名
        platform = "android"
        output_directory = os.path.join(output_base_directory, platform)
        os.makedirs(output_directory, exist_ok=True)

        output_name = f"{self.file_name}RequestModel.kt"
        output_file_path = os.path.join(output_directory, output_name)

        # 保存生成的 Kotlin 代码到文件
        with open(output_file_path, "w") as file:
            file.write(output)

        # 打印生成的文件路径
        print(f"Generated {output_file_path}")

        # 拷贝到 Android 项目中
        # 确保目标目录存在
        os.makedirs(android_target_dir, exist_ok=True)

        # 拷贝文件
        shutil.copy(output_file_path, android_target_dir)


def get_package_name(file_path):
    # 确保路径是绝对路径
    file_path = os.path.abspath(file_path)

    # 查找 'java' 目录的起始位置
    try:
        java_index = file_path.index("java")
    except ValueError:
        raise ValueError("'java' not found in the file path")

    # 提取 java 目录之后的部分路径
    package_path = file_path[java_index + len("java/") :]

    # 将路径分隔符替换为点
    package_name = package_path.replace(os.path.sep, ".")

    return package_name
