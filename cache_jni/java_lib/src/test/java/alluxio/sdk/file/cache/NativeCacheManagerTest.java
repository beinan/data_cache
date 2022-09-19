package alluxio.sdk.file.cache;

import static org.junit.Assert.assertTrue;

import org.junit.Test;

/**
 * Unit test for simple App.
 */
public class NativeCacheManagerTest
{
    /**
     * Rigorous Test :-)
     */
    @Test
    public void shouldAnswerWithTrue()
    {
        NativeCacheManager manager = new NativeCacheManager();
        assertTrue(manager.put(new byte[5], new byte[5]));
    }
}
