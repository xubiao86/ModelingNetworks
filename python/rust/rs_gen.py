import os

from jinja2 import Template

from request_data import RequestData
from constant import templates_directory, output_base_directory


class RsGen:
    def __init__(self, file_name, data: RequestData) -> None:
        self.file_name = file_name
        self.data = data

    def gen(self):
        template_file_path = os.path.join(templates_directory, "rust.rs")

        # 读取模板文件
        with open(template_file_path, "r") as file:
            template_content = file.read()

        template = Template(template_content, trim_blocks=False, lstrip_blocks=True)

        # 渲染模板
        output = template.render(
            struct_name=f"{self.file_name}RequestModel",
            url=self.data.url,
            path=self.data.path,
            method=self.data.method,
            header=self.data.headers,
            query=self.data.query,
            body=self.data.body,
        )

        platform = "rust" 
        output_directory = os.path.join(output_base_directory, platform)
        os.makedirs(output_directory, exist_ok=True)

        # 确定输出文件路径
        output_name = f"{self.file_name.lower()}_request_model.rs"
        output_file_path = os.path.join(output_directory, output_name)

        # 保存生成的 Rust 代码到文件
        with open(output_file_path, "w") as file:
            file.write(output)

        # 打印生成的文件路径
        print(f"Generated {output_file_path}")
