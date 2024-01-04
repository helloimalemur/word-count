# word-count


### Misc
    Why the dialogs look ugly/blurry on Windows?
    
    Turn on crate features or embed manifests into the .exe to enable visual styling and dpi awareness for your program. Check out examples/windows_manifest and examples/windows_features for example.
    Why the program crashed when opening a dialog on macOS?
    
    The UI framework of macOS (Cocoa) has a limitation that all UI operations must be performed on the main thread.
    Linux dependencies
    
    The Linux implementation of native-dialog requires either Zenity or Kdialog to be installed. Otherwise you'll get a No Implementation error.
