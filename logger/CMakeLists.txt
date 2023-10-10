# SPDX-FileCopyrightText: 2023 Joshua Goins <josh@redstrate.com>
# SPDX-License-Identifier: CC0-1.0

add_library(physis-logger STATIC)
target_sources(physis-logger PRIVATE
        src/physis_logger.cpp)
target_link_libraries(physis-logger PRIVATE physis Qt6::Core)
target_include_directories(physis-logger PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)