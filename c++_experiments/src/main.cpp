#include <iostream>
#include <map>
#include <unordered_set>
#include <unistd.h>
#include <vector>
#include <boost/filesystem.hpp>
#include "fmt/format.h"

//template<typename... T> void ignore(T &&...) {};

int main(int, const char**) {
  //  ignore(argc, argv);

    const std::vector<boost::filesystem::path> paths {"/bin", "/sbin", "/usr/bin"};

    std::map<std::string, std::string> hashed_executables {};

    // scan directories for executables

    for(auto& path : paths) {
        if(boost::filesystem::is_directory(path)) {
            for(auto& file_entry: boost::filesystem::directory_iterator(path)) {
                if(boost::filesystem::is_regular_file(file_entry)) {
                    auto& file_path = file_entry.path();
                    auto& file_name = file_path.filename().string();

                    if(hashed_executables.count(file_name) == 0) {
                        hashed_executables[file_name] = file_path.string();
                    }
                }
            }
        } else {
            fmt::print("Not a directory: {}\n", path.string());
        }
    }

    for(auto& kv : hashed_executables) {
        fmt::print("key: {}, value: {}\n", kv.first, kv.second);
    }

    // builtins temp
    const std::unordered_set<std::string> exit_cmds {"exit", "quit"};

    while(true) {
        fmt::print("> ");
        std::string input;
        std::getline(std::cin, input);

        // process_builtins();
        if((exit_cmds.count(input) > 0) || std::cin.eof()) {
            exit(EXIT_SUCCESS);
        }

        if(hashed_executables.count(input) != 1) {
            fmt::print("No such executable: {}\n", input);
            continue;
        }

        fmt::print("found {}\n", input);

        // process_exec()
        auto pid = fork();

        if(pid == 0) {
            auto cinput = input.c_str();
            auto fullpath = hashed_executables.at(input);

            fmt::print("executing {}", fullpath);
            execl(fullpath.c_str(), cinput, nullptr);
            fmt::print("exec failed\n");

            exit(EXIT_FAILURE);
        } else if(pid < 0) {
            //error
            fmt::print("Something went wrong. Exiting!\n");
            exit(EXIT_FAILURE);
        }

        int exit_status;
        do {
            waitpid(pid, &exit_status, WUNTRACED);
        } while(!WIFEXITED(exit_status) && !WIFSIGNALED(exit_status));
    }


    return EXIT_SUCCESS;
}
