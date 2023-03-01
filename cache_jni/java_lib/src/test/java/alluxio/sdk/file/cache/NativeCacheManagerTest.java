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
        System.out.println("Done!");
    }
    @Test
    public void ssdTest() {
        NativeCacheManager manager = new NativeCacheManager();
        assertTrue(manager.init(256 * 1024 * 1024, 1024 * 1024 * 4, 20, true, "/tmp/mmap-file"));
        assertTrue(manager.put("key1".getBytes(), "test_value".getBytes()));
        byte[] result = new byte[5];
        int bytesread = manager.get("key1".getBytes(), 0, 5, result, 0);
        assertEquals(5, bytesread);
        assertEquals("test_", new String(result, StandardCharsets.UTF_8));
        System.out.println("#1 Done!");

        String key2 = "key2";
        String value2 = "abcde";
        assertTrue(manager.put(key2.getBytes(), value2.getBytes()));
        bytesread = manager.get(key2.getBytes(), 0, 5, result, 0);
        assertEquals(5, bytesread);
        assertEquals(value2, new String(result, StandardCharsets.UTF_8));
        System.out.println("#2 Done!");

        int n = 16 * 1024;
        int len = 32;
        int cnt = 0;
        String[] key = new String[n];
        String[] value = new String[n];
        for(int i = 0; i < n; i++) {
            key[i] = genRandomString(len);
            value[i] = genRandomString(len);
            //System.out.println(i + " " + key[i] + " " + value[i] + " " + value[i].getBytes().length);
            assertTrue(manager.put(key[i].getBytes(), value[i].getBytes()));
            byte[] res = new byte[len];
            int bytesRead = manager.get(key[i].getBytes(), 0, value[i].getBytes().length, res, 0);
            String resString  = new String(res, StandardCharsets.UTF_8);
            //System.out.println(i + " " + resString);
            //assertEquals(value[i].getBytes().length, bytesRead);
            //assertEquals(value[i], resString);
            if(bytesRead == -1) {
                System.out.println("Error Read -1: " + i + " " + key[i] + " " + value[i]);
                cnt++;
                //retry insert:
                assertTrue(manager.put(key[i].getBytes(), value[i].getBytes()));
                bytesRead = manager.get(key[i].getBytes(), 0, value[i].getBytes().length, res, 0);
                resString  = new String(res, StandardCharsets.UTF_8);
                if(bytesRead == -1) {
                    System.out.println("Retry Error Read -1: " + i + " " + key[i] + " " + value[i]);
                }
                else if(!value[i].equals(resString)) {
                    System.out.println("Retry Error res: " + i + " " + key[i] + " " + value[i] + " " + resString);
                }
                else {
                    System.out.println("Retry Success: " + i + " " + key[i] + " " + value[i] + " " + resString);
                }
            }
            else if(!value[i].equals(resString)) {
                System.out.println("Error res: " + i + " " + key[i] + " " + value[i] + " " + resString);
                cnt++;
            }
        }
        System.out.println("#3 Done! cnt = " + cnt);
        for(int batch = 0; batch < 10; batch++) {
            int batch_cnt = 0;
            for(int i = 0; i < n; i++) {
                byte[] res = new byte[len];
                int bytesRead = manager.get(key[i].getBytes(), 0, value[i].getBytes().length, res, 0);
                String resString  = new String(res, StandardCharsets.UTF_8);
                if(bytesRead == -1) {
                    System.out.println("Error Read -1: " + i + " " + key[i] + " " + value[i]);
                    batch_cnt++;
                }
                else if(!value[i].equals(resString)) {
                    System.out.println("Error res: " + i + " " + key[i] + " " + value[i] + " " + resString);
                    batch_cnt++;
                }
            }
            System.out.println("batch #" + batch + " Done! cnt = " + batch_cnt);
            /*
            try{
                Thread.sleep(30 * 1000);
            } catch(Exception e) {
                e.printStackTrace();
            }
            */
        }
        /*
        
        for(int i = 0; i < n; i++) {
            byte[] res = new byte[16];
            int bytesRead = manager.get(key[i].getBytes(), 0, value[i].getBytes().length, res, 0);
            String resString  = new String(res, StandardCharsets.UTF_8);
            System.out.println(i + " " + resString);
            assertEquals(value[i].getBytes().length, bytesRead);
            assertEquals(value[i], resString);
        }
        */
        
        
    }
}
