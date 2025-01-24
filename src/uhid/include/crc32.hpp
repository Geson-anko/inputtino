#pragma once

#include <cstdint>
#include <zlib.h>

inline uint32_t CRC32(const unsigned char *buffer, size_t length, uint32_t seed = 0) {
  return crc32_z(seed, buffer, length);
}