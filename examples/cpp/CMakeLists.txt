cmake_minimum_required(VERSION 3.15)
project(st3215_cpp_example)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Définir les chemins
set(SERVO_CONTROLLER_ROOT "${CMAKE_CURRENT_SOURCE_DIR}/../..")
set(INCLUDE_DIR "${SERVO_CONTROLLER_ROOT}/include")

# Trouver la bibliothèque compilée
if(WIN32)
    set(LIB_NAME "st3215.dll.lib")
    set(DLL_NAME "st3215.dll")
    if(CMAKE_BUILD_TYPE STREQUAL "Release")
        set(LIB_PATH "${SERVO_CONTROLLER_ROOT}/target/release")
    else()
        set(LIB_PATH "${SERVO_CONTROLLER_ROOT}/target/debug")
    endif()
elseif(APPLE)
    set(LIB_NAME "libst3215.dylib")
    if(CMAKE_BUILD_TYPE STREQUAL "Release")
        set(LIB_PATH "${SERVO_CONTROLLER_ROOT}/target/release")
    else()
        set(LIB_PATH "${SERVO_CONTROLLER_ROOT}/target/debug")
    endif()
else()
    set(LIB_NAME "libst3215.so")
    if(CMAKE_BUILD_TYPE STREQUAL "Release")
        set(LIB_PATH "${SERVO_CONTROLLER_ROOT}/target/release")
    else()
        set(LIB_PATH "${SERVO_CONTROLLER_ROOT}/target/debug")
    endif()
endif()

# Créer l'exécutable
add_executable(example example.cpp)

# Ajouter les répertoires d'inclusion
target_include_directories(example PRIVATE ${INCLUDE_DIR})

# Lier la bibliothèque
target_link_libraries(example PRIVATE "${LIB_PATH}/${LIB_NAME}")

# Sur Windows, copier la DLL à côté de l'exécutable
if(WIN32)
    add_custom_command(TARGET example POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy_if_different
        "${LIB_PATH}/${DLL_NAME}"
        $<TARGET_FILE_DIR:example>)
endif()

# Afficher les informations de configuration
message(STATUS "Library path: ${LIB_PATH}/${LIB_NAME}")
message(STATUS "Include directory: ${INCLUDE_DIR}")
