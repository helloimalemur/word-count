# word-count

## This is a work in progress!

## A writer's Word count application.
#### Supports Windows, MacOS, and Linux
    For keeping track of the word-count in multiple .docx files at the same time.
    While also providing additional information.

## Installation downloads
### [Windows](https://github.com/helloimalemur/word-count/releases/tag/windows)
### [MacOS](https://github.com/helloimalemur/word-count/releases/tag/macos)
### [Linux](https://github.com/helloimalemur/word-count/releases/tag/linux)


## Development
#### you will need to install Rust
    https://www.rust-lang.org/tools/install
#### Then Clone the repo
#### Run the following in a command prompt.
    git clone https://github.com/helloimalemur/word-count.git;
    cd word-count/;
    cargo run;           # build and run
### If on Windows, use "Powershell" via Shift+Right-Click -> Open Command Prompt / Powershell Window Here

#

##### Notes
    Removes unicode em-dashes (u+2014) prior to calculating word count, so "back—then" would correctly calculate as two words.

### Current calculations provided
    word count
    unique words

### Planned calculations
    average sentance count per paragraph
    average character count per paragraph
    top three sentance starter words


## Development and Collaboration
#### Feel free to open a pull request, please run the following prior to your submission please!
    echo "Run clippy"; cargo clippy -- -D clippy::all
    echo "Format source code"; cargo fmt -- --check

### Misc
    Why the dialogs look ugly/blurry on Windows?
    
    Turn on crate features or embed manifests into the .exe to enable visual styling and dpi awareness for your program. Check out examples/windows_manifest and examples/windows_features for example.
    Why the program crashed when opening a dialog on macOS?
    
    The UI framework of macOS (Cocoa) has a limitation that all UI operations must be performed on the main thread.
    Linux dependencies
    
    The Linux implementation of native-dialog requires either Zenity or Kdialog to be installed. Otherwise you'll get a No Implementation error.
