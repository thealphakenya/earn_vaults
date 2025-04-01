#include <iostream>

// Ensure the function uses C linkage for compatibility with Rust
extern "C" {
    void hello_world() {
        std::cout << "Hello from C++!" << std::endl;
    }
}
