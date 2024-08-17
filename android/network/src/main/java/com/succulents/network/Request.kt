package com.succulents.network

import com.alibaba.fastjson.JSON
import kotlin.reflect.full.memberProperties
import kotlin.time.measureTime

class Request {
    private var action: ((String) -> Unit)? = null

    companion object {
        init {
            System.loadLibrary("ffi")
        }
    }

    fun request(model: RequestModel): String {
        val content = JSON.toJSONString(model)
        val res =
            try {
                requestSync(content)
            } catch (e: Error) {
                println(e)
                ""
            }
        return res
    }

    fun requestAsync(model: RequestModel, action: (String) -> Unit) {
        this.action = action
        val content = JSON.toJSONString(model)
        try {
            requestAsync(content, this)
        } catch (e: Error) {
            println(e)
        }
    }

    /**
     * todo
     * 反射 response model 所有属性
     * 需要考虑耗时问题
     */
    fun <T : Any> requestAsyncWithModel(model: RequestModel, action: (T) -> Unit) {
        val content = JSON.toJSONString(model)
        requestAsync(content, this)
    }

    private external fun requestSync(content: String): String

    private external fun requestAsync(content: String, action: Request)

    fun nativeCallback(result: String) {
        action?.invoke(result)
    }

    private inline fun <reified T : Any> parseResponse(result: String): T? {
        val time = measureTime {
            val response = JSON.parseObject(result, ResponseBody::class.java)
            val jobj = JSON.parseObject(response.body)
            val kClass = T::class
            val properties = kClass.memberProperties
            for (property in properties) {
                val value = jobj[property.name]
                println("Property name: ${value}, Type: ${property.returnType}")
            }
        }
        println(time)
        return null
    }
}