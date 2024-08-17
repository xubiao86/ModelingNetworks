package com.succulents.request.models

import com.succulents.network.RequestModel

class PostRequestModel : RequestModel() {
    override val url: String
        get() = "https://jsonplaceholder.typicode.com"

    override val path: String
        get() = "/posts"

    override val method: String
        get() = "POST"

    override val header: Map<String, String>
        get() = mapOf(
            "Content-type" to "application/json; charset=UTF-8"
        )

    override val query: Map<String, String>
        get() = mapOf(
            "title" to "foo",
            "body" to "bar",
            "userId" to "1"
        )
}