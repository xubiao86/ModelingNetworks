package com.succulents.request.models

import com.succulents.network.RequestModel

class GetRequestModel : RequestModel() {
    override val url: String
        get() = "https://jsonplaceholder.typicode.com"

    override val path: String
        get() = "/posts/1"

    override val method: String
        get() = "GET"
}