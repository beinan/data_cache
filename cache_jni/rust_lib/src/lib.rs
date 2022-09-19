use jni::JNIEnv;
use jni::sys::{jboolean, jbyteArray, jint, JNI_TRUE, jlong};
use jni::objects::JObject;

#[no_mangle]
pub extern "system" fn Java_alluxio_sdk_file_cache_NativeCacheManager_get(
    _env: JNIEnv, _obj: JObject, key: jbyteArray, page_offset: jint,  bytes_to_read: jint, buffer: jbyteArray, offset_in_buf: jint) -> jint {
    5
}

#[no_mangle]
pub extern "system" fn Java_alluxio_sdk_file_cache_NativeCacheManager_put(env: JNIEnv, obj: JObject, key: jbyteArray, page: jbyteArray) -> jboolean {
    return JNI_TRUE
}

#[no_mangle]
pub extern "system" fn Java_alluxio_sdk_file_cache_NativeCacheManager_init(env: JNIEnv, obj: JObject, cache_size: jlong, seg_size: jint, hash_power: jint) -> jboolean {
    
    return JNI_TRUE
}

