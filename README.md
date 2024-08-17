# 模型化网络请求

通过定义统一格式的 YAML 文件，我们可以自动生成 Kotlin、Swift、Rust 和 Dart 等多种语言的网络请求模型。项目利用 Rust 模块来执行实际的网络请求，确保不同平台间的请求和响应保持一致。一次编写 YAML 文件，即可生成一致的多端网络模型，从而简化跨语言开发中的数据处理和网络交互。

## 项目结构

```
.
├── android
│   ├── app
│   └── network
├── core
│   ├── ffi
│   ├── spider-man
├── model
└── python
    ├── android
    ├── rust
    └── template
```

- **[`android`](#android)**: Android example dir.
- **[`core`](#core)**: Network request model dir.
- **[`model`](#model)**: Request model dir.
- **[`python`](#python)**: Generate model py dir.

## 使用方式
**1. 生成模型**

我们将在 model 目录中定义所需生成的数据模型。运行 python/gen.sh 脚本后，将自动生成适用于 Rust 和 Kotlin 的数据模型。模型的定义示例如下：

Kotlin 模型 🌰 
``` kotlin
package com.succulents.request.models

import com.succulents.network.RequestModel

class GetRequestModel : RequestModel() {
    override val url: String
        get() = "https://example.com"

    override val path: String
        get() = "/posts/1"

    override val method: String
        get() = "GET"
}
```

Rust 模型 🌰 
``` rust
struct GetRequestModel;

impl RequestTrait for GetRequestModel {
    fn http_url(&self) -> String {
        "https://example.com".to_string()
    }

    fn http_method(&self) -> String {
        "GET".to_string()
    }

    fn http_path(&self) -> Option<String> {
        Some("/posts/1".to_string())
    }
}
```

**2. 如何使用**

Android 端 🌰
```kotlin
val model = GetRequestModel()
// 同步请求
Request().request(model)
// 异步请求
Request().requestAsync(model) {
    Log.e("TAG", "net request : $it")
}
```

Rust 端 🌰
```rust
let model = GetRequestModel {};
let res = request(model).await;
```

## 配置文件

配置文件是生成统一模型的核心，正确编写 YAML 文件至关重要。以下是文件格式的示例：

``` yaml
method: get
url: https://example.com
path: /posts/1
body:
    id: 1
    name: Jack
```