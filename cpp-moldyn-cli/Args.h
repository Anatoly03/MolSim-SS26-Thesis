/**
 * @file Args.h
 * @author Anatoly Weinstein
 * 
 * @brief Definition of the Args class for CLI argument parsing,
 * wrapping around [getopt](https://www.man7.org/linux/man-pages/man3/getopt.3.html)
 * and `getopt_long` for parsing.
 */

#pragma once

#include <cxxabi.h>
#include <cstdlib>
#include <filesystem>
#include <fstream>
#include <getopt.h>
#include <iostream>
#include <map>
#include <memory>
#include <optional>
#include <set>
#include <string>
#include <typeinfo>
#include <vector>

#ifndef PROGRAM_VERSION
#define PROGRAM_VERSION "unknown"
#endif

/**
 * @brief A CLI argument parser inspired by the [argparse](https://github.com/p-ranav/argparse)
 * library for C++.
 *
 * # Example
 *
 * ```cpp
 * int main(int argc, char* argv[]) {
 *     int int_arg;
 *     double double_arg;
 *     std::string string_arg;
 *
 *     Args()
 *         .required<int>('i', &int_arg)
 *         .required<double>('d', &double_arg)
 *         .required<std::string>('s', "string", &string_arg)
 *         .help("This is a help message for the CLI application.")
 *         .parse(argc, argv);
 * }
 * ```
 */
struct Args
{
private:
    /**
     * Helper construct to store reference to registered arguments along their
     * type information for parsing and error handling.
     */
    struct ArgsRef
    {
    public:
        const size_t type_hint;
        const std::string brief = "";

    private:
        const std::string type_name;
        void *value;
        // @brief Did the parser ask for this argument to be read?
        bool is_set = false;

    public:
        ArgsRef(size_t type_hint, std::string type_name, void *value)
            : type_hint(type_hint), type_name(type_name), value(value) {}
        ArgsRef(size_t type_hint, std::string type_name, void *value, const char *brief)
            : type_hint(type_hint), type_name(type_name), value(value), brief(brief) {}

        // type_hint(typeid(P).hash_code()), type_name(typeid(P).name()), value(value)

        template <typename P>
        static ArgsRef with(P *value)
        {
            return ArgsRef(typeid(P).hash_code(), typeid(P).name(), value);
        }

        template <typename P>
        static ArgsRef with_brief(P *value, const char *brief)
        {
            return ArgsRef(typeid(P).hash_code(), typeid(P).name(), value, brief);
        }

        template <typename P>
        P *try_get() const
        {
            if (typeid(P).hash_code() == type_hint)
            {
                return static_cast<P *>(value);
            }

            return nullptr;
        }

        const std::string type_human_readable() const
        {
            if (type_hint == typeid(bool).hash_code())
                return "bool";
            if (type_hint == typeid(int).hash_code())
                return "int";
            if (type_hint == typeid(double).hash_code())
                return "double";
            if (type_hint == typeid(std::string).hash_code())
                return "string";
            if (type_hint == typeid(std::optional<std::string>).hash_code())
                return "[string]";
            if (type_hint == typeid(std::unique_ptr<std::ifstream>).hash_code())
                return "file path";
            if (type_hint == typeid(std::filesystem::path).hash_code())
                return "file path";

            // https://stackoverflow.com/questions/12877521/human-readable-type-info-name
            // caller is responsible for freeing the demangled name
            int status = 0;
            std::string demangled_name;
            char *demangled = abi::__cxa_demangle(&type_name[0], nullptr, nullptr, &status);
            demangled_name = (status == 0 && demangled) ? demangled : type_name;
            std::free(demangled);
            return demangled_name;
        }
    };

    /**
     * @brief The string of short options for getopt_long. The string format is
     * a sequence of characters, followed by a colon if the option requires an
     * argument, or two colons if the option takes an optional argument.
     *
     * @see [getopt](https://www.man7.org/linux/man-pages/man3/getopt.3.html)
     */
    std::string optstring;

    /**
     * @brief The vector of long options for command parsing.
     *
     * @see [getopt](https://www.man7.org/linux/man-pages/man3/getopt.3.html)
     */
    std::vector<option> options;

    /**
     * @brief An ordered map of option arguments mapped to their outer references.
     */
    std::map<char, ArgsRef> references;

    /**
     * @brief An vector of positional argument mapped to their outer references.
     */
    std::vector<ArgsRef> positional_references;

    // @brief Amount of, registered as required, positional arguments.
    int required_positional_arguments = 0;

    // @brief Amount of, required and optional, positional arguments.
    int max_positional_arguments = 0;

    // @brief Internal help argument tag: `--help` or `-h`.
    std::optional<std::string> help_flag = std::nullopt;

    // @brief Brief description of the CLI application for the help and usage message.
    std::optional<std::string> help_brief = std::nullopt;

    // @brief Internal help argument tag: `--version` or `-v`.
    std::optional<std::string> version_flag = std::nullopt;

    /**
     * @brief Prints the help message and exits the application.
     */
    [[noreturn]]
    void print_help(const char *progname)
    {
        if (help_brief.has_value())
        {
            std::cout << help_brief.value() << "\n\n";
        }

        std::cout << "Usage: " << progname << " [OPTIONS]";
        for (int i = 0; i < required_positional_arguments; i++)
        {
            std::cout << " <arg" << i << ">";
        }
        std::cout << "\n";

        if (max_positional_arguments)
        {
            std::cout << "\nArguments:\n";

            int arg_index = 0;
            for (const auto refs : positional_references)
            {
                std::cout << "  arg" << arg_index++
                          << "\t\t" << refs.type_human_readable()
                          << "\t" << refs.brief
                          << "\n";
            }
        }

        if (!options.empty())
        {
            std::cout << "\nOptions:\n";

            for (const auto &option : options)
            {
                const auto ref = references.find(option.val)->second;
                std::cout << "  -" << (char)option.val
                          << ", --" << option.name
                          << "\t" << ref.type_human_readable()
                          << "\t" << ref.brief
                          << "\n";
            }
        }

        exit(0);
    }

    /**
     * @brief Prints the help message and exits the application.
     */
    [[noreturn]]
    void print_usage(const char *progname)
    {
        std::cout << "Usage:   " << progname << " [OPTIONS]";
        for (int i = 0; i < required_positional_arguments; i++)
        {
            std::cout << " <arg" << i << ">";
        }
        std::cout << "\n";
        std::cout << "Help:    " << progname << " --help\n";
        std::cout << "Version: " << progname << " --help\n";
        exit(0);
    }

    /**
     * @brief Parse single argument with the appropriate type and store in outer
     * reference.
     */
    bool parse_into_ref(char *optarg, const ArgsRef &ref)
    {
        if (const auto v = ref.try_get<bool>())
        {
            *v = optarg ? (std::string(optarg) != "0") : true;
            return true;
        }

        if (const auto v = ref.try_get<double>())
        {
            *v = atof(optarg);
            return true;
        }

        if (const auto v = ref.try_get<int>())
        {
            *v = atoi(optarg);
            return true;
        }

        if (const auto v = ref.try_get<std::string>())
        {
            *v = std::string(optarg);
            return true;
        }

        if (const auto v = ref.try_get<std::unique_ptr<std::ifstream>>())
        {
            std::string file_path(optarg);
            std::unique_ptr<std::ifstream> pointer = std::make_unique<std::ifstream>(file_path);

            if (!pointer->is_open())
            {
                std::cerr << "Error: Failed to open file at path `" << file_path << "`\n";
                exit(1);
            }

            *v = std::move(pointer);
            return true;
        }

        if (const auto v = ref.try_get<std::filesystem::path>())
        {
            *v = std::filesystem::path(optarg);
            return true;
        }

        if (const auto v = ref.try_get<std::optional<std::string>>())
        {
            *v = std::optional<std::string>(optarg ? optarg : "");
            return true;
        }

        return false;
    }

public:
    /**
     * @brief Initializes an empty argument parser.
     */
    Args() = default;

    // remove implicit copy constructors
    // https://stackoverflow.com/questions/33776697/deleting-copy-constructors-and-copy-assignment-operators-which-of-them-are-esse
    Args(const Args &) = delete;
    Args &operator=(const Args &) = delete;

    /**
     * @brief Registers a required positional argument with a reference.
     * @tparam T The type of the argument value.
     *
     * # Example
     *
     * ```cpp
     * // @brief Accepts the command `./app <int_arg>`
     * int main(int argc, char* argv[]) {
     *     int int_arg;
     *
     *     Args()
     *         .required<int>(&int_arg)
     *         .parse(argc, argv);
     * }
     * ```
     */
    template <typename T>
    Args &required(T *value)
    {
        if (required_positional_arguments != positional_references.size())
        {
            std::cerr << "Error: Required positional arguments must be registered before optional positional arguments.\n";
            exit(1);
        }

        required_positional_arguments += 1;
        max_positional_arguments += 1;

        positional_references.push_back(ArgsRef::with<T>(value));
        return *this;
    }

    /**
     * @brief Registers a required positional argument with a brief details message.
     * @tparam T The type of the argument value.
     *
     * # Example
     *
     * ```cpp
     * // @brief Accepts the command `./app <int_arg>`
     * int main(int argc, char* argv[]) {
     *     int int_arg;
     *
     *     Args()
     *         .required_details<int>(&int_arg, "This is an integer argument.")
     *         .parse(argc, argv);
     * }
     * ```
     */
    template <typename T>
    Args &required_details(T *value, const char *details)
    {
        if (required_positional_arguments != positional_references.size())
        {
            std::cerr << "Error: Required positional arguments must be registered before optional positional arguments.\n";
            exit(1);
        }

        required_positional_arguments += 1;
        max_positional_arguments += 1;

        positional_references.push_back(ArgsRef::with_brief<T>(value, details));
        return *this;
    }

    /**
     * @brief Registers a required argument with a short name and a reference.
     * @tparam T The type of the argument value.
     *
     * # Example
     *
     * ```cpp
     * // @brief Accepts the command `./app -i <int_arg>`
     * int main(int argc, char* argv[]) {
     *     int int_arg;
     *
     *     Args()
     *         .required<int>('i', &int_arg)
     *         .parse(argc, argv);
     * }
     * ```
     */
    template <typename T>
    Args &required(char short_name, T *value)
    {
        // https://stackoverflow.com/questions/1472048/how-to-append-a-char-to-a-stdstring
        optstring += short_name;
        optstring += ':';

        options.push_back(option{nullptr, required_argument, nullptr, short_name});
        references.insert({short_name, ArgsRef::with<T>(value)});
        return *this;
    }

    /**
     * @brief Registers a required argument with a short name, a reference and a
     * brief details message.
     * @tparam T The type of the argument value.
     *
     * # Example
     *
     * ```cpp
     * // @brief Accepts the command `./app -i <int_arg>`
     * int main(int argc, char* argv[]) {
     *     int int_arg;
     *
     *     Args()
     *         .required_details<int>('i', &int_arg, "This is an integer argument.")
     *         .parse(argc, argv);
     * }
     * ```
     */
    template <typename T>
    Args &required_details(char short_name, T *value, const char *details)
    {
        optstring += short_name;
        optstring += ':';

        options.push_back(option{nullptr, required_argument, nullptr, short_name});
        references.insert({short_name, ArgsRef::with_brief<T>(value, details)});
        return *this;
    }

    /**
     * @brief Registers a required argument with a short and long name,
     * and a reference.
     * @tparam T The type of the argument value.
     *
     * # Example
     *
     * ```cpp
     * // @brief Accepts the commands `./app -i <int_arg>`
     * // and `./app --integer <int_arg>`
     * int main(int argc, char* argv[]) {
     *     int int_arg;
     *
     *     Args()
     *         .required<int>('i', "integer", &int_arg)
     *         .parse(argc, argv);
     * }
     * ```
     */
    template <typename T>
    Args &required(char short_name, const char *long_name, T *value)
    {
        optstring += short_name;
        optstring += ':';

        options.push_back(option{long_name, required_argument, nullptr, short_name});
        references.insert({short_name, ArgsRef::with<T>(value)});
        return *this;
    }

    /**
     * @brief Registers a required argument with a short name, long name, a
     * reference and a brief details message.
     * @tparam T The type of the argument value.
     *
     * # Example
     *
     * ```cpp
     * // @brief Accepts the commands `./app -i <int_arg>`
     * // and `./app --integer <int_arg>`
     * int main(int argc, char* argv[]) {
     *     int int_arg;
     *
     *     Args()
     *         .required_details<int>('i', "integer", &int_arg, "This is an integer argument.")
     *         .parse(argc, argv);
     * }
     * ```
     */
    template <typename T>
    Args &required_details(char short_name, const char *long_name, T *value, const char *details)
    {
        optstring += short_name;
        optstring += ':';

        options.push_back(option{long_name, required_argument, nullptr, short_name});
        references.insert({short_name, ArgsRef::with_brief<T>(value, details)});
        return *this;
    }

    /**
     * @brief Registers an optional positional argument with a reference.
     * @tparam T The type of the argument value.
     *
     * # Example
     *
     * ```cpp
     * #include <optional>
     *
     * // @brief Accepts the command `./app [<int_arg>]`
     * int main(int argc, char* argv[]) {
     *     std::optional<int> int_arg;
     *
     *     Args()
     *         .optional<int>(&int_arg)
     *         .parse(argc, argv);
     * }
     * ```
     */
    template <typename T>
    Args &optional(std::optional<T> *value)
    {
        max_positional_arguments += 1;
        positional_references.push_back(ArgsRef::with<std::optional<T>>(value));
        return *this;
    }

    /**
     * @brief Registers an optional positional argument with a brief details message.
     * @tparam T The type of the argument value.
     *
     * # Example
     *
     * ```cpp
     * #include <optional>
     *
     * // @brief Accepts the command `./app [<int_arg>]`
     * int main(int argc, char* argv[]) {
     *     std::optional<int> int_arg;
     *
     *     Args()
     *         .optional_details<int>(&int_arg, "This is an integer argument.")
     *         .parse(argc, argv);
     * }
     * ```
     */
    template <typename T>
    Args &optional_details(std::optional<T> *value, const char *details)
    {
        max_positional_arguments += 1;
        positional_references.push_back(ArgsRef::with_brief<std::optional<T>>(value, details));
        return *this;
    }

    /**
     * @brief Registers an optional option with a short name and a reference.
     * @tparam T The type of the argument value.
     *
     * # Example
     *
     * ```cpp
     * // @brief Accepts the command `./app [-i <int_arg>]`
     * int main(int argc, char* argv[]) {
     *     int int_arg;
     *
     *     Args()
     *         .optional<int>('i', &int_arg)
     *         .parse(argc, argv);
     * }
     * ```
     */
    template <typename T>
    Args &optional(char short_name, std::optional<T> *value)
    {
        optstring += short_name;
        optstring += "::";

        options.push_back(option{nullptr, optional_argument, nullptr, short_name});
        references.insert({short_name, ArgsRef::with<std::optional<T>>(value)});
        return *this;
    }

    /**
     * @brief Registers an optional option with a short name, a reference and a
     * brief details message.
     * @tparam T The type of the argument value.
     *
     * # Example
     *
     * ```cpp
     * // @brief Accepts the command `./app [-i <int_arg>]`
     * int main(int argc, char* argv[]) {
     *     int int_arg;
     *
     *     Args()
     *         .optional_details<int>('i', &int_arg, "This is an integer argument.")
     *         .parse(argc, argv);
     * }
     * ```
     */
    template <typename T>
    Args &optional_details(char short_name, std::optional<T> *value, const char *details)
    {
        optstring += short_name;
        optstring += "::";

        options.push_back(option{nullptr, optional_argument, nullptr, short_name});
        references.insert({short_name, ArgsRef::with_brief<std::optional<T>>(value, details)});
        return *this;
    }

    /**
     * @brief Registers an optional option with a short and long name,
     * and a reference.
     * @tparam T The type of the argument value.
     *
     * # Example
     *
     * ```cpp
     * // @brief Accepts the commands `./app [-i <int_arg>]`
     * // and `./app [--integer <int_arg>]`
     * int main(int argc, char* argv[]) {
     *     int int_arg;
     *
     *     Args()
     *         .optional<int>('i', "integer", &int_arg)
     *         .parse(argc, argv);
     * }
     * ```
     */
    template <typename T>
    Args &optional(char short_name, const char *long_name, std::optional<T> *value)
    {
        optstring += short_name;
        optstring += "::";

        options.push_back(option{long_name, optional_argument, nullptr, short_name});
        references.insert({short_name, ArgsRef::with<std::optional<T>>(value)});
        return *this;
    }

    /**
     * @brief Registers an optional option with a short name, long name, a
     * reference and a brief details message.
     * @tparam T The type of the argument value.
     *
     * # Example
     *
     * ```cpp
     * // @brief Accepts the commands `./app [-i <int_arg>]`
     * // and `./app [--integer <int_arg>]`
     * int main(int argc, char* argv[]) {
     *     int int_arg;
     *
     *     Args()
     *         .optional_details<int>('i', "integer", &int_arg, "This is an integer argument.")
     *         .parse(argc, argv);
     * }
     * ```
     */
    template <typename T>
    Args &optional_details(char short_name, const char *long_name, std::optional<T> *value, const char *details)
    {
        optstring += short_name;
        optstring += "::";

        options.push_back(option{long_name, optional_argument, nullptr, short_name});
        references.insert({short_name, ArgsRef::with_brief<std::optional<T>>(value, details)});
        return *this;
    }

    /**
     * @brief Registers the help message for the CLI application.
     *
     * # Example
     *
     * ```cpp
     * // @brief Accepts the command `./app --help` or `./app -h`
     * int main(int argc, char* argv[]) {
     *     int int_arg;
     *
     *     Args()
     *         .help("This is a help message for the CLI application.")
     *         .parse(argc, argv);
     * }
     * ```
     */
    Args &help(const char *message)
    {
        // optstring.append("h::");
        // options.push_back(option{"help", optional_argument, nullptr, 'h'});
        // references.insert({'h', ArgsRef::with<std::optional<std::string>>(&help_flag)});
        // return *this;
        help_brief = std::string(message);
        return optional_details('h', "help", &help_flag, "Prints this help message and exits.");
    }

    /**
     * @brief Registers the version message for the CLI application.
     *
     * # Example
     *
     * ```cpp
     * // @brief Accepts the command `./app --help` or `./app -h`
     * int main(int argc, char* argv[]) {
     *     int int_arg;
     *
     *     Args()
     *         .version()
     *         .parse(argc, argv);
     * }
     * ```
     */
    Args &version()
    {
        // optstring.append("v");
        // options.push_back(option{"version", optional_argument, nullptr, 'v'});
        // references.insert({'v', ArgsRef::with<std::optional<std::string>>(&version_flag)});
        // return *this;
        return optional_details('v', "version", &version_flag, "Prints the application version and exits.");
    }

    /**
     * @brief Registers the help message for the CLI application.
     *
     * # Example
     *
     * ```cpp
     * int main(int argc, char* argv[]) {
     *     int int_arg;
     *
     *     Args()
     *         .help("This is a help message for the CLI application.")
     *         .parse(argc, argv);
     * }
     * ```
     */
    void parse(int argc, char *argv[])
    {
        const char *progname = argv[0];

        int opt;
        while ((opt = getopt_long(argc, argv, optstring.c_str(), &options[0], nullptr)) != -1)
        {
            for (const auto &option : options)
            {
                if (opt == '?')
                {
                    std::cerr << "Error: Unrecognized option.\n";
                    print_usage(progname);
                }

                if (option.val != opt)
                    continue;

                // parse argument into appropriate type and store in outer reference
                const auto ref = references.find(opt)->second;

                if (parse_into_ref(optarg, ref))
                    ;
                break;

                // We land here if the option type could not be parsed.
                std::cerr << "Error: Failed to parse argument for option -"
                          << (char)opt << ". expected type: `" << ref.type_human_readable()
                          << "`. received value: `" << (optarg ? optarg : "null") << "`\n";
                exit(1);
            }
        }

        // if help flag was set, print the help message and exit
        if (help_flag.has_value())
        {
            print_help(progname);
        }

        if (version_flag.has_value())
        {
            std::cout << progname << " v" << PROGRAM_VERSION << "\n";
            exit(0);
        }

        // parse positional arguments
        int pos_index = 0;
        while (optind < argc && pos_index < max_positional_arguments)
        {
            // parse positional argument into appropriate type and store in outer reference
            const auto ref = positional_references[pos_index];

            if (parse_into_ref(argv[optind], ref))
            {
                pos_index += 1;
                optind += 1;
            }
            break;

            // We land here if the option type could not be parsed.
            std::cerr << "Error: Failed to parse argument at position " << pos_index + 1
                      << ". expected type: `" << ref.type_human_readable()
                      << "`. received value: `" << (argv[optind] ? argv[optind] : "null") << "`\n";
            exit(1);
        }

        if (pos_index < required_positional_arguments)
        {
            const auto ref = positional_references[pos_index];
            std::cerr << "Error: Missing required positional argument at position " << pos_index + 1
                      << ". Expected type: " << ref.type_human_readable() << "\n";
            print_usage(progname);
        }
    }
};
