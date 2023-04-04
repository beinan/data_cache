package alluxio.sdk.file.cache;

public class NativeCacheManagerOptions {
    private long cacheSize;
    private int segSize;
    private int hashPower;
    private boolean enableSsd;
    private String fileName;

    public long getCacheSize() {
        return cacheSize;
    }

    public NativeCacheManagerOptions setCacheSize(long _cacheSize) {
        cacheSize = _cacheSize;
        return this;
    }

    public int getSegSize() {
        return segSize;
    }

    public NativeCacheManagerOptions setSegSize(int _segSize) {
        segSize = _segSize;
        return this;
    }

    public int getHashPower() {
        return hashPower;
    }

    public NativeCacheManagerOptions setHashPower(int _hashPower) {
        hashPower = _hashPower;
        return this;
    }

    public boolean getEnableSsd() {
        return enableSsd;
    }

    public NativeCacheManagerOptions setEnableSsd(boolean _enableSsd) {
        enableSsd = _enableSsd;
        return this;
    }

    public String getFileName() {
        return fileName;
    }

    public NativeCacheManagerOptions setFileName(String _fileName) {
        fileName = _fileName;
        return this;
    }
}
