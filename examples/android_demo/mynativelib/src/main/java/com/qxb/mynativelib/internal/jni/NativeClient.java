package com.qxb.mynativelib.internal.jni;

import androidx.annotation.NonNull;

/**
 * @author rongcloud
 * @date 2025/6/18
 */
public class NativeClient {
    static {
        System.loadLibrary("mynativelib");
    }

    @NonNull private long nativePtr;

    private native long nativeInit();

    public NativeClient() {
        nativePtr = nativeInit();
    }

    private native void nativeRelease();

    public void release() {
        nativeRelease();
    }

    private native int nativeAddInt(int a, int b);

    public int addInt(int a, int b) {
        return nativeAddInt(a, b);
    }

    private native String nativeStringFromJNI();

    public String stringFromJNI() {
        return nativeStringFromJNI();
    }
}
