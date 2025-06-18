package com.qxb.mynativelib.internal.jni;

public class NativeLib {

    // Used to load the 'mynativelib' library on application startup.
    static {
        System.loadLibrary("mynativelib");
    }

    /**
     * A native method that is implemented by the 'mynativelib' native library, which is packaged
     * with this application.
     */
    public native String stringFromJNI();
}
