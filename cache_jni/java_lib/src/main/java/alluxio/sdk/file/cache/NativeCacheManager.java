package alluxio.sdk.file.cache;

/**
 * Hello world!
 *
 */
public class NativeCacheManager
{
  static {
    System.load("/Users/beinanwang/w/data_cache/target/debug/liballuxio_jni_rust_lib.dylib");
  }

  private long nativeCacheHandle;

  public native boolean init(long cacheSize, int segSize, int hashPower);

  public native int get(byte[] key, int pageOffset, int bytesToRead, byte[] buffer,
      int offsetInBuffer);

  public native boolean put(byte[] key, byte[] page);
}
