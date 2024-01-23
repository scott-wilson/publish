cmake_minimum_required(VERSION 3.15.0 FATAL_ERROR)

set(CPUBLISH_VERSION_MAJOR "0")
set(CPUBLISH_VERSION_MINOR "1")
set(CPUBLISH_VERSION_PATCH "0")
set(CPUBLISH_VERSION ${CPUBLISH_VERSION_MAJOR}.${CPUBLISH_VERSION_MINOR}.${CPUBLISH_VERSION_PATCH})

project(cpubilsh VERSION ${CPUBLISH_VERSION} LANGUAGES C)

# --------------------------------------------------------------------------------
# Build Options
# --------------------------------------------------------------------------------
option(CODE_COVERAGE "Enable coverage reporting" OFF)

# --------------------------------------------------------------------------------
# Do the build
# --------------------------------------------------------------------------------
set(UNIT_TESTING OFF)

add_subdirectory(corrosion)

if (NOT DEFINED CPUBLISH_SOURCE_DIR OR CPUBLISH_SOURCE_DIR MATCHES "")
set(CPUBLISH_SOURCE_DIR "${PROJECT_SOURCE_DIR}")
endif()

corrosion_import_crate(MANIFEST_PATH "${CPUBLISH_SOURCE_DIR}/Cargo.toml")

if (BUILD_CPUBLISH_TESTS)
    enable_testing()
    add_subdirectory(cmocka)
    add_subdirectory(tests)
endif()
