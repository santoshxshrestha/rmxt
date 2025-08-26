# rmxt

`rmxt` is a safer, recoverable alternative to the traditional `rm` command. Instead of permanently deleting files, `rmxt` moves them to a trash directory, allowing you to recover them later if needed.

> **Note:** We do not move empty directories to the trash directory because they serve no purpose and can be recreated easily if needed.

## Features

- Prevents accidental permanent deletion of files.
- Moves deleted files to a designated trash directory.

## Warning

The current implementation of `rmxt` relies heavily on the use of `unwrap()` for error handling. This means:

- If any operation (e.g., file I/O, directory creation) fails, the program will panic and terminate abruptly.
- There is no graceful recovery or fallback mechanism in place for unexpected errors.

However, this is a work in progress, and the logic will be refactored to convert the code into a safer implementation. Future updates will:

- Replace `unwrap()` with proper error propagation using `Result` and the `?` operator.
- Introduce robust error handling to ensure the program can recover gracefully from unexpected failures.

## License

This project is licensed under the [MIT License](LICENSE).
