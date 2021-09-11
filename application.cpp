// Runs in jail; should not have access to anything.

#include <filesystem>
#include <iostream>

//#include <fmt/core.h>

int main() {
    std::cout << "I should be running in jail, here is what I still have access to:\n";
    //fmt::print("I should be running in jail, here is what I still have access to:\n");
    for (auto& entry : std::filesystem::recursive_directory_iterator{"/"}) {
        std::cout << "  " << entry.path().native() << '\n';
        //fmt::print("  {}\n", entry.path().native());
    }
}
