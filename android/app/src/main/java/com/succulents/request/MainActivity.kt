package com.succulents.request

import android.os.Bundle
import android.util.Log
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.activity.enableEdgeToEdge
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.padding
import androidx.compose.material3.Button
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.ui.Modifier
import androidx.compose.ui.tooling.preview.Preview
import com.succulents.request.models.GetRequestModel
import com.succulents.request.models.PostRequestModel
import com.succulents.request.ui.theme.RequestTheme
import com.succulents.network.Request

class MainActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        enableEdgeToEdge()
        setContent {
            RequestTheme {
                Scaffold(modifier = Modifier.fillMaxSize()) { innerPadding ->
                    RequestCompose(
                        name = "Android", modifier = Modifier.padding(innerPadding)
                    )
                }
            }
        }


    }
}

@Composable
fun RequestCompose(name: String, modifier: Modifier = Modifier) {
    Column(modifier) {
        Button(onClick = {
            val model = GetRequestModel()
            Request().requestAsync(model) {
                Log.e("TAG", "net request : $it")
            }
        }) {
            Text(text = "Get Request")
        }
        Button(onClick = {
            val model = PostRequestModel()
            Request().requestAsync(model) {
                Log.e("TAG", "net request : $it")
            }
        }) {
            Text(text = "Post Request")
        }
    }
}

@Preview(showBackground = true)
@Composable
fun GreetingPreview() {
    RequestTheme {
        RequestCompose("Android")
    }
}