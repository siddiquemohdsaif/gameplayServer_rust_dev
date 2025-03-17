#include <jni.h>


#ifndef _Included_com_carromclash_simulatorapi_HelloWorldNative
#define _Included_com_carromclash_simulatorapi_HelloWorldNative
#ifdef __cplusplus
extern "C" {
#endif

JNIEXPORT jstring JNICALL Java_com_alfa_carromclash_NativeBridge_helloWorld(JNIEnv *, jobject);

JNIEXPORT void JNICALL Java_com_alfa_carromclash_NativeBridge_startSimulation(JNIEnv *env, jclass clazz, jobject simulatorJObject);

JNIEXPORT void JNICALL Java_com_alfa_carromclash_NativeBridge_stimulateToCurrentTime(JNIEnv *env, jclass clazz, jobject simulatorJObject);

JNIEXPORT jobjectArray JNICALL Java_com_alfa_carromclash_NativeBridge_insertNewCarrom(JNIEnv *env, jclass clazz, jobjectArray carrom_array, jdoubleArray carrom);
 

#ifdef __cplusplus
}
#endif
#endif
