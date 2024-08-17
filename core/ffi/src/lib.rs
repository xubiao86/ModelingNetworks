#[macro_use]
extern crate log;
extern crate android_logger;

use std::thread;

use android_logger::Config;
use jni::{
    objects::{JObject, JString, JValueGen},
    JNIEnv,
};
use log::{debug, LevelFilter};
use spider_man::{engine::request, model::request_model::RequestModel};
use tokio::runtime::Runtime;

#[no_mangle]
pub unsafe extern "C" fn Java_com_succulents_network_Request_requestSync<'local>(
    mut env: JNIEnv<'local>,
    _: JObject,
    content: JString<'local>,
) -> JString<'local> {
    android_logger::init_once(Config::default().with_max_level(LevelFilter::Trace));

    let input: String = env
        .get_string(&content)
        .expect("get jvm string error")
        .into();
    info!("input is {}", input);
    let model: RequestModel = serde_json::from_str(&input).unwrap();
    info!("model is {:?}", model);

    let rt = Runtime::new().unwrap();
    let r = rt.block_on(async move {
        let r = request(model).await;
        debug!("res is {r:?}");
        r
    });

    let output = env
        .new_string(serde_json::to_string(&r.unwrap()).unwrap())
        .expect("Couldn't create java string!");
    output
}

#[no_mangle]
pub unsafe extern "C" fn Java_com_succulents_network_Request_requestAsync<'local>(
    mut env: JNIEnv<'local>,
    _: JObject,
    content: JString<'local>,
    callback: JObject,
) {
    android_logger::init_once(Config::default().with_max_level(LevelFilter::Trace));

    let jvm = env.get_java_vm().unwrap();
    let callback = env.new_global_ref(callback).unwrap();

    let input: String = env
        .get_string(&content)
        .expect("get jvm string error")
        .into();
    info!("input is {}", input);
    let model: RequestModel = serde_json::from_str(&input).unwrap();

    thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        let r = rt.block_on(async move {
            let r = request(model).await;
            debug!("res is {r:?}");
            r
        });

        let jvm = jvm;
        let callback = callback;
        let mut env = jvm.attach_current_thread().unwrap();
        // 网络结果转换成 JString
        let reuslt = env
            .new_string(serde_json::to_string(&r.unwrap()).unwrap())
            .expect("Couldn't create java string!");
        // convert to JObject
        let value: JObject = reuslt.into();

        env.call_method(
            &callback,
            "nativeCallback",
            "(Ljava/lang/String;)V",
            &[JValueGen::Object(&value)],
        )
        .expect("callback app crash");
    });
}
