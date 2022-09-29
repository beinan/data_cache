package alluxio.sdk.file.cache;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;

import org.junit.Test;

import java.nio.charset.Charset;
import java.nio.charset.StandardCharsets;

/**
 * Unit test for simple App.
 */
public class NativeCacheManagerTest
{

    @Test
    public void putAndGetTest()
    {
        NativeCacheManager manager = new NativeCacheManager();
        assertTrue(manager.init(128 * 1024 * 1024, 1024 * 1024, 16));
        assertTrue(manager.put("key1".getBytes(), "test_value".getBytes()));
        byte[] result = new byte[5];
        int bytesread = manager.get("key1".getBytes(), 0, 5, result, 0);
        assertEquals(5, bytesread);
        assertEquals("test_", new String(result, StandardCharsets.UTF_8));
    }
}
