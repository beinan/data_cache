package alluxio.sdk.file.cache;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertTrue;

import org.junit.Test;

import java.nio.charset.StandardCharsets;
import java.util.Random;

/**
 * Unit test for simple App.
 */
public class NativeCacheManagerTest
{
    public String genRandomString(int len) {
        String s = "abcdefghijklmnopqrstuvwxyz0123456789";
        StringBuilder sb = new StringBuilder();
        Random r = new Random();
        for(int i = 0; i < len; i++) {
            sb.append(s.charAt(r.nextInt(s.length())));
        }
        return sb.toString();
    }

    @Test
    public void putAndGetTest()
    {
        NativeCacheManager manager = new NativeCacheManager();
        assertTrue(manager.init(256 * 1024 * 1024, 1024 * 1024 * 4, 20, false, ""));
        assertTrue(manager.put("key1".getBytes(), "test_value".getBytes()));
        byte[] result = new byte[5];
        int bytesread = manager.get("key1".getBytes(), 0, 5, result, 0);
        assertEquals(5, bytesread);
        assertEquals("test_", new String(result, StandardCharsets.UTF_8));
    }
    @Test
    public void ssdTest() {
        NativeCacheManagerOptions options = new NativeCacheManagerOptions();
        options.setCacheSize(256 * 1024 * 1024)
            .setSegSize(1024 * 1024 * 4)
            .setHashPower(20)
            .setEnableSsd(true)
            .setFileName("/tmp/mmap-file");
        NativeCacheManager manager = new NativeCacheManager();
        assertTrue(manager.init(options));
        assertTrue(manager.put("key1".getBytes(), "test_value".getBytes()));
        byte[] result = new byte[5];
        int bytesread = manager.get("key1".getBytes(), 0, 5, result, 0);
        assertEquals(5, bytesread);
        assertEquals("test_", new String(result, StandardCharsets.UTF_8));

        String key2 = "key2";
        String value2 = "abcde";
        assertTrue(manager.put(key2.getBytes(), value2.getBytes()));
        bytesread = manager.get(key2.getBytes(), 0, 5, result, 0);
        assertEquals(5, bytesread);
        assertEquals(value2, new String(result, StandardCharsets.UTF_8));

        int n = 16 * 1024;
        int len = 32;
        String[] key = new String[n];
        String[] value = new String[n];
        for(int i = 0; i < n; i++) {
            key[i] = genRandomString(len);
            value[i] = genRandomString(len);
            assertTrue(manager.put(key[i].getBytes(), value[i].getBytes()));
            byte[] res = new byte[len];
            int bytesRead = manager.get(key[i].getBytes(), 0, value[i].getBytes().length, res, 0);
            String resString  = new String(res, StandardCharsets.UTF_8);
            assertEquals(value[i].getBytes().length, bytesRead);
            assertEquals(value[i], resString);
        }
    }
}
