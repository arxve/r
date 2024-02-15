# r

This is a simple command line utility written in Rust for performing various file and directory operations. It supports the following commands:

- `echo`: Echoes the input arguments.
- `cat`: Concatenates files and prints their contents.
- `ls`: Lists the contents of a directory.
- `find`: Searches for a file in a directory.
- `grep`: Searches for a pattern in a file.

## Prerequisites

- Have Rust [Installed](https://www.rust-lang.org/tools/install)

## Usage

To use the utility, compile the code and run the executable with the desired command followed by arguments.

```
$ rustc r.rs
$ ./r <command> [arguments]
```

Replace `<command>` with one of the supported commands and `[arguments]` with appropriate arguments based on the chosen command.

## Commands

### `echo`

Echoes the input arguments.

```
$ ./r echo Hello World
Hello World
```

### `cat`

Concatenates files and prints their contents.

```
$ ./r cat file1.txt file2.txt
Contents of file1.txt
Contents of file2.txt
```

### `ls`

Lists the contents of a directory.

```
$ ./r ls /path/to/directory
file1.txt
file2.txt
subdirectory
```

### `find`

Searches for a file in a directory.

```
$ ./r find /path/to/directory filename.txt
/path/to/directory/filename.txt
```

### `grep`

Searches for a pattern in a file.

```
$ ./r grep filename.txt pattern
1: Line containing the pattern
5: Another line with the pattern
```
