//
// Created by Qi on 2025/6/18.
//

#ifndef ANDROID_DEMO_NATIVECLIENT_H
#define ANDROID_DEMO_NATIVECLIENT_H
#include <iostream>
#include <string>

namespace rcim {

class NativeClient {
   public:
    void createEngine();
    void release();

   public:
    std::string stringFromJNI();
};

}  // namespace rcim

#endif  // ANDROID_DEMO_NATIVECLIENT_H
