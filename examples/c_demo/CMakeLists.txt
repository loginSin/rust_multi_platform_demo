cmake_minimum_required(VERSION 3.18)
project(c_demo)

set(CMAKE_CXX_STANDARD 17)

# 仅此路径支持 Win & Mac
link_directories("../../target/debug/")

add_executable(${PROJECT_NAME} main.cpp)
target_link_libraries(${PROJECT_NAME} PRIVATE my_lib)

if (APPLE)
    target_link_libraries(${PROJECT_NAME} PRIVATE
            "-framework CoreFoundation"
            "-framework Foundation"
            "-framework CFNetwork"
            "-framework SystemConfiguration"
            "-framework Security"
            "-framework Carbon"
    )
elseif (WIN32 AND NOT LINK_DYNAMIC)
    target_link_libraries(${PROJECT_NAME} PRIVATE
            Ws2_32
            Secur32.lib
            Crypt32.lib
            Userenv.lib
            ntdll.lib
            bcrypt.lib
            ncrypt.lib
            runtimeobject.lib
    )
endif ()

