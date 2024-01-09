# word-count

## This is a work in progress!

## A writer's Word count application.
#### Supports Windows, MacOS, and Linux
    For keeping track of the word-count in multiple .docx files at the same time.
    While also providing additional information.

## Running
#### For the time-being, to Run this and try it out you will need to install Rust
    https://www.rust-lang.org/tools/install
#### Then [download+unzip](https://codeload.github.com/helloimalemur/word-count/zip/refs/heads/master) OR Clone the repo
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


    ┌───────────────────────────────────────────────────────────────────────────────────────┐
    | Language                        files        size       blank     comment        code |
    ├───────────────────────────────────────────────────────────────────────────────────────┤
    | Markdown                            1     1.96 KB          13           0          36 |
    | Rust                               11    14.21 KB          57          29         352 |
    | Toml                                1    335.00 B           2           1          13 |
    | XML                                 5    23.61 KB           0           0         568 |
    ├───────────────────────────────────────────────────────────────────────────────────────┤
    | Sum                                18    40.10 KB          72          30         969 |
    └───────────────────────────────────────────────────────────────────────────────────────┘
