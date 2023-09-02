<div align="center">

# DustyDirDetector

[![License](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](LICENSE)
[![GitHub Release](https://img.shields.io/github/release/yourusername/DustyDirDetector-Rust.svg?style=flat-square)](https://github.com/eonm-abes/DustyDirDetector-Rust/releases)

</div>

DustyDirDetector is a Rust-based command-line tool that helps you identify inactive directories within your file system. It's a handy utility for keeping your directory structures clean and organized by detecting directories that haven't been modified for a specified period.

## Table of Contents
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Examples](#examples)
- [License](#license)

## Features

- **Inactive Directory Detection:** DustyDirDetector scans your specified directory structure to identify directories that haven't seen activity for a defined time frame.
- **Command-Line Interface (CLI):** Easily integrate DustyDirDetector into your workflow using its command-line interface.

## Installation

To install DustyDirDetector, follow these steps:

1. Clone this repository or download the latest release from [GitHub](https://github.com/eonm-abes/DustyDirDetector).

2. Navigate to the project directory.

3. Build the Rust project:

   ```shell
   cargo build --release
   ```

    Run the program:

    ```shell
    ./target/release/dusty-dir-detector
    ```

## Usage

DustyDirDetector can be used with the following command-line options:

- `-p, --paths <PATHS>`: Specify one or more directory paths to check for inactive directories. You can provide multiple paths separated by commas.

- `-i, --inactive-for <DURATION>`: Set the inactivity duration threshold to identify inactive directories. Specify the duration in a human-readable format (e.g., "30 days," "2 weeks," "3 months").

### Example Usages:

**Check multiple directories for inactivity:**

```shell
dusty-dir-detector -p /path/to/directory1 -p /path/to/directory2 -i "2 weeks"
```

Use the `--paths` and `--inactive-for` options to customize the directories and inactivity threshold according to your requirements.

```shell
dusty-dir-detector -p /path/to/directory1 -p /path/to/directory2 -i "3 months"
```

## License

DustyDirDetector is licensed under the MIT License. See the LICENSE file for details.
