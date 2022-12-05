package alluxio.sdk.file.cache;

import java.io.File;
import java.io.IOException;
import java.io.InputStream;
import java.net.URL;
import java.nio.file.Files;
import java.nio.file.StandardCopyOption;

/**
 * Hello world!
 *
 */
public class NativeCacheManager
{
  static {
    loadLibrary("alluxio_jni_rust_lib");
  }

  private long nativeCacheHandle;

  public native boolean init(long cacheSize, int segSize, int hashPower);

  public native int get(byte[] key, int pageOffset, int bytesToRead, byte[] buffer,
      int offsetInBuffer);

  public native boolean put(byte[] key, byte[] page);

  private static String getLibraryPath(String name)
  {
    return "/nativelib/" + getPlatform() + "/" + System.mapLibraryName(name);
  }

  private static void loadLibrary(String name)
  {
    String libraryPath = getLibraryPath(name);
    URL url = NativeCacheManager.class.getResource(libraryPath);
    if (url == null) {
      throw new RuntimeException("library not found: " + libraryPath);
    }

    File file = null;
    try {
      file = File.createTempFile(name, null);
      file.deleteOnExit();
      try (InputStream in = url.openStream()) {
        Files.copy(in, file.toPath(), StandardCopyOption.REPLACE_EXISTING);
      }
    } catch (IOException e) {
      throw new RuntimeException(e);
    }

    System.load(file.getAbsolutePath());
  }
  private static String getPlatform()
  {
    String name = System.getProperty("os.name");
    String arch = System.getProperty("os.arch");
    return (name + "-" + arch).replace(' ', '_');
  }
}
