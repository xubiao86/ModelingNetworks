import os
import yaml

from android.kt_gen import KtGen
from rust.rs_gen import RsGen
from request_data import RequestData
from constant import yaml_directory, output_base_directory


# 创建输出目录，如果不存在
os.makedirs(output_base_directory, exist_ok=True)

# 遍历 YAML 文件目录
for yaml_file_name in os.listdir(yaml_directory):
    if yaml_file_name.endswith(".yaml"):
        yaml_file_path = os.path.join(yaml_directory, yaml_file_name)

        # 读取 YAML 文件
        with open(yaml_file_path, "r") as file:
            data = yaml.safe_load(file)

        # 提取文件名
        file_name = os.path.splitext(yaml_file_name)[0].capitalize()

        # 获取数据
        url = data.get("url")
        path = data.get("path")
        method = data.get("method", "GET").upper()
        header = data.get("header")
        query = data.get("query")
        body = data.get("body")
        data = RequestData(url, path, method, header, query, body)

        # 生成 KT 模板代码
        kt = KtGen(file_name, data)
        kt.gen()

        # 生成 Rust 模板代码
        rs = RsGen(file_name, data)
        rs.gen()
