#include "NativeClient.h"

#include <jni.h>
#include <string>

namespace rcim {

extern "C" JNIEXPORT jlong JNICALL
Java_com_qxb_mynativelib_internal_jni_NativeClient_nativeInit(JNIEnv* env, jobject jThis) {
    auto* instance = new rcim::NativeClient();
    return reinterpret_cast<jlong>(instance);
}

extern "C" JNIEXPORT void JNICALL
Java_com_qxb_mynativelib_internal_jni_NativeClient_nativeRelease(JNIEnv* env, jobject jThis) {
    jclass cls = env->GetObjectClass(jThis);
    jfieldID fieldId = env->GetFieldID(cls, "nativePtr", "J");
    jlong ptr = env->GetLongField(jThis, fieldId);
    delete reinterpret_cast<rcim::NativeClient*>(ptr);
}

extern "C" JNIEXPORT jstring JNICALL
Java_com_qxb_mynativelib_internal_jni_NativeClient_nativeStringFromJNI(JNIEnv* env, jobject jThis) {
    jclass cls = env->GetObjectClass(jThis);
    jfieldID fieldId = env->GetFieldID(cls, "nativePtr", "J");
    jlong ptr = env->GetLongField(jThis, fieldId);
    rcim::NativeClient* instance = reinterpret_cast<rcim::NativeClient*>(ptr);
    return env->NewStringUTF(instance->stringFromJNI().c_str());
}

extern "C" JNIEXPORT jint JNICALL
Java_com_qxb_mynativelib_internal_jni_NativeClient_nativeAddInt(JNIEnv* env,
                                                                jobject jThis,
                                                                jint a,
                                                                jint b) {
    return a + b;
}

}  // namespace rcim