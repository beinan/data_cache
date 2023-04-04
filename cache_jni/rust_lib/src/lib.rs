use std::ops::{Deref, DerefMut};
use std::sync::MutexGuard;
use std::time::Duration;
use std::path::{Path, PathBuf};

use jni::JNIEnv;
use jni::sys::{jboolean, jbyteArray, jint, JNI_TRUE, jlong, JNI_FALSE};
use jni::objects::{JObject, JString};

use segcache::{Policy, Seg, Value};

#[no_mangle]
pub extern "system" fn Java_alluxio_sdk_file_cache_NativeCacheManager_get(
    env: JNIEnv, obj: JObject, key: jbyteArray, page_offset: jint,  bytes_to_read: jint, buffer: jbyteArray, offset_in_buf: jint) -> jint {
        let cache = env.get_rust_field::<_, _, Seg>(obj, "nativeCacheHandle");
        match cache {
            Ok(mut guard) => {
                let seg = guard.deref_mut();
                let rust_key = &env.convert_byte_array(key).unwrap()[..];
                let item = seg.get(rust_key);
                if item.is_none() {
                    -1
                } else {
                    match item.unwrap().value() {
                        Value::Bytes(v) => {
                            let mut slice = &v[page_offset as usize .. (page_offset + bytes_to_read) as usize];
                            let slice = unsafe{ &*(slice as *const [u8] as *const [i8])};
                            env.set_byte_array_region(buffer, offset_in_buf, slice).unwrap();
                            bytes_to_read as i32
                        },
                        Value::U64(l) => core::mem::size_of::<u64>() as i32,       
                    }
                }
            },
            Err(_) => -1   
        }    
}

#[no_mangle]
pub extern "system" fn Java_alluxio_sdk_file_cache_NativeCacheManager_put(env: JNIEnv, obj: JObject, key: jbyteArray, page: jbyteArray) -> jboolean {
    let cache = env.get_rust_field::<_, _, Seg>(obj, "nativeCacheHandle");
    match cache {
        Ok(mut guard) => {
            let seg = guard.deref_mut();
            let rust_key = &env.convert_byte_array(key).unwrap()[..];
            let rust_value = &env.convert_byte_array(page).unwrap();
            seg.insert(rust_key, rust_value, None, Duration::ZERO);
            JNI_TRUE
        },
        Err(_) => JNI_FALSE
    }
}

#[no_mangle]
pub extern "system" fn Java_alluxio_sdk_file_cache_NativeCacheManager_init__JIIZLjava_lang_String_2(env: JNIEnv, obj: JObject, cache_size: jlong, seg_size: jint, hash_power: jint, enable_ssd: jboolean, file_name: JString) -> jboolean {
    let file_name : String = env.get_string(file_name).expect("couldn't get file name!").into();
    let enable_ssd = enable_ssd != 0;
    let mut cache_builder = Seg::builder()
            .heap_size(cache_size as usize)
            .segment_size(seg_size)
            .hash_power(hash_power as u8)
            .eviction(Policy::Random);
    if enable_ssd {
        cache_builder = cache_builder.datapool_path(Some(PathBuf::from(file_name)));
    }
    let cache = cache_builder.build().expect("failed to create cache");
    match env.set_rust_field(obj, "nativeCacheHandle", cache) {
        Ok(_) => JNI_TRUE,
        Err(_) => JNI_FALSE,
    }
}


#[no_mangle]
pub extern "system" fn Java_alluxio_sdk_file_cache_NativeCacheManager_init__Lalluxio_sdk_file_cache_NativeCacheManagerOptions_2(env: JNIEnv, obj: JObject, options: JObject) -> jboolean {
    let cache_size = env.get_field(options, "cacheSize", "J").unwrap().j().unwrap();
    let seg_size = env.get_field(options, "segSize", "I").unwrap().i().unwrap();
    let hash_power = env.get_field(options, "hashPower", "I").unwrap().i().unwrap();
    let enable_ssd = env.get_field(options, "enableSsd", "Z").unwrap().z().unwrap();
    let file_name = env.get_field(options, "fileName", "Ljava/lang/String;").unwrap().l().unwrap();
    let file_name = JString::from(file_name);
    let file_name : String = env.get_string(file_name).expect("couldn't get file name!").into();
    //let enable_ssd = enable_ssd != 0;
    let mut cache_builder = Seg::builder()
            .heap_size(cache_size as usize)
            .segment_size(seg_size)
            .hash_power(hash_power as u8)
            .eviction(Policy::Random);
    if enable_ssd {
        cache_builder = cache_builder.datapool_path(Some(PathBuf::from(file_name)));
    }
    let cache = cache_builder.build().expect("failed to create cache");
    match env.set_rust_field(obj, "nativeCacheHandle", cache) {
        Ok(_) => JNI_TRUE,
        Err(_) => JNI_FALSE,
    }
}

