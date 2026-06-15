// Minimal example exercising the Uldren Loom C++ wrapper.
// Build: see bindings/cpp/README.md (links against libuldren_loom from loom-ffi).
#include <iostream>
#include <vector>

#include "loom.hpp"

int main() {
    std::cout << "loom " << uldren::loom::version() << "\n";
    std::vector<std::uint8_t> abc = {'a', 'b', 'c'};
    std::cout << uldren::loom::blob_digest(abc) << "\n";
    return 0;
}
