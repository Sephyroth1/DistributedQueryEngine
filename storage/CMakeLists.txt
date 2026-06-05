cmake_minimum_required(VERSION 3.14)
include(FetchContent)

project(storage)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

FetchContent_Declare(
    nlohmann_json
    GIT_REPOSITORY https://github.com/nlohmann/json.git
    GIT_TAG v3.11.3
)

FetchContent_MakeAvailable(nlohmann_json)
add_executable(storage src/main.cpp)

target_include_directories(storage PRIVATE ${nlohmann_json_SOURCE_DIR}/include)
