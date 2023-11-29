# MediaMender
MediaMender is a Rust-based command-line tool designed to help organize and tidy up media libraries. It allows users to browse files in their directories, sorted and grouped by file extension, and assists in cleaning up unnecessary files such as text documents within media folders.
<img src="doc/icon.png" width="200" />
## Features
Browse files in any directory and its subdirectories.
- Group and list files by their extensions.
- Interactive browsing with options to skip or break out of the file list.
- Ideal for organizing media libraries, specifically tailored for Plex media library management.

## Getting Started
### Prerequisites
Rust: Ensure you have Rust installed on your system.
### Installation
Clone the repository:

```sh
git clone https://github.com/christophacham/MediaMender.git
cd MediaMender
```

### Building
Build the application using Cargo:

```sh
cargo build --release
```

### Running the Application
Run MediaMender from the command line:

```sh
cargo run
```

## Usage
After starting MediaMender, follow the on-screen instructions to browse files:

Select the file extension from the list to view files of that type.
Use the 'Next' and 'Back' options to navigate through files.
Press 'Quit' to exit the application.

## Contributing
Contributions to MediaMender are welcome. Please feel free to fork the repository, make changes, and submit pull requests.

## License
This project is licensed under the MIT License - see the LICENSE.md file for details.
