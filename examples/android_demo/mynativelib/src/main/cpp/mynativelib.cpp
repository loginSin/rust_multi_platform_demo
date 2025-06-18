#include <jni.h>
#include <string>

extern "C" JNIEXPORT jstring JNICALL
Java_com_qxb_mynativelib_internal_jni_NativeLib_stringFromJNI(
        JNIEnv* env,
        jobject /* this */) {
    std::string hello = "Hello from C++";
    return env->NewStringUTF(hello.c_str());
}