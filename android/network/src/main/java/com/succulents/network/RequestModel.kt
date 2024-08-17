package com.succulents.network

abstract class RequestModel {
    abstract val url: String
    abstract val method: String

    open val path: String? = null
    open val header: Map<String, String> = emptyMap()
    open val query: Map<String, String> = emptyMap()
    open val body: Map<String, String> = emptyMap()
}
