// C++ convenience wrapper over the Uldren Loom C ABI (include/loom.h).
// Header-only RAII helpers that handle the "core allocates, caller frees" ownership rule.
// Licensed under BUSL-1.1. (c) Uldren Technologies LLC.
#pragma once

#include <cstdint>
#include <string>
#include <vector>

#include "loom.h"

namespace uldren::loom {

/// Library version.
inline std::string version() {
    char *raw = ::loom_version();
    std::string out = raw ? std::string(raw) : std::string();
    ::loom_string_free(raw);
    return out;
}

/// Blob content address ("algo:hex") of the given bytes.
inline std::string blob_digest(const std::vector<std::uint8_t> &data) {
    char *raw = ::loom_blob_digest(data.data(), data.size());
    std::string out = raw ? std::string(raw) : std::string();
    ::loom_string_free(raw);
    return out;
}

}  // namespace uldren::loom
