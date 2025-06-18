package com.qxb.mynativelib;

import androidx.annotation.NonNull;
import com.qxb.mynativelib.internal.jni.NativeClient;

/**
 * @author rongcloud
 * @date 2025/6/18
 */
public class CoreClient {
    @NonNull private NativeClient nativeClient;

    private CoreClient() {
        nativeClient = new NativeClient();
    }

    private static class SingletonHolder {
        static CoreClient sInstance = new CoreClient();
    }

    public static CoreClient getInstance() {
        return SingletonHolder.sInstance;
    }

    public int addInt(int a, int b) {
        return this.nativeClient.addInt(a, b);
    }

    public String getString() {
        return this.nativeClient.stringFromJNI();
    }
}
