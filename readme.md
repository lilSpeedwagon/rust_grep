# Rust Grep

Simple tool repeating the well-known `grep` tool features: simple and fast text search among local files.

This is just an educational project to learn Rust basics.

## Prerequisites

- Rust compiler

## Build and run

1. Build the project with:

    ```shell
    cargo build
    ```

2. Run the project with:

    ```shell
    cargo run <pattern> <path>
    ```

## Usage

To search for a specific text pattern among the local files use the following command:

    ```shell
    rust_grep <pattern> <path>
    ```

Where:
    - `<pattern>` - regular expression text pattern to search. 
    Learn more about [regular expressions](https://en.wikipedia.org/wiki/Regular_expression).
    - `<path>` local file path to search the pattern specified. May be a file or a directory.

For example:

    ```shell
    rust_grep "hello*" "./**/*"
    ```

to find all occurences of any string starting from characters sequence "hello" in the current directory and all nested directories.
