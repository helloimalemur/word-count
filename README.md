# word-count

## This is a work in progress!

### A writer's Word count application.
    For keeping track of the word-count in multiple .docx files at the same time.
    While also providing additional information.

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
