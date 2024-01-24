# DUNEPAK - DUNE II Pak file packer/unpacker

## About
I'm currently trying to learn [Rust programming language](https://www.rust-lang.org/), and this project is my first try at making something useful using it. This should be a replacement for another tool called WWPAK, which is mentioned on [Dune2K Forums - List of Dune II Editing Tools](https://forum.dune2k.com/topic/19752-list-of-dune-ii-editing-tools/). WWPAK is a great easy-to-use tool, but it is a 16-bit DOS executable and has some weird quirks.
Dunepak should be buildable everywhere the Rust toolchain works, this means Windows (32bit and 64bit), as well as Linux and Mac.

## Usage

###### TLDR:

Example using `SCENARIO.PAK`, when dunepak.exe is in the same folder:

Issuing `dunepak.exe unpak SCENARIO.PAK` or `dunepak.exe unpak SCENARIO` would unpack `SCENARIO.PAK` file into `SCENARIO` folder (`SCENA001.INI, ...` files will be created/overwritten there).

While issuing (possibly after doing edits to files you want):
`dunepak.exe pak SCENARIO` would pack all files (like `SCENA001.INI, ...`) in `SCENARIO` folder to a file `SCENARIO.PAK` (possibly overwriting it, checks, so be careful).

###### More detailed: 

Use --help to get help:
```
dunepak.exe --help
DUNE II Pak file extractor/packer

Usage: dunepak.exe [OPTIONS] <COMMAND>

Commands:
  pak    Pack folder
  unpak  Unpacks PAK file
  help   Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose  Display more info when packing/unpacking
  -h, --help     Print help
```
  
For packing use `pak` keyword, pass the folder name to pack. and optionally - file name to pack to. The resulting file name  (if not given) will be inferred from the folder name like this: 

`{CWD}\{FOLDER NAME}.PAK` if the folder has no Dots in the name, otherwise `{CWD}\{FOLDER NAME}` will be used. `{CWD}` stands for "Current Working Directory".

**NOTE:** Folder should contain all files you want to pack, it should be *FLAT*, meaning all subfolders and files in them will be ignored. 

Also file names should follow 8.3 naming scheme (used in the DOS FAT filesystem), which means - to 8 chars for the file name, one DOT, and up to Three character extension (12 chars in total). 

Use only valid ASCII characters in file names.

**IMPORTANT:** Resulting PAK file will be overwritten without any checks if it already exists, even if the name was inferred, so please be careful.

**Help section:**
```
dunepak.exe pak --help
Pack folder

Usage: dunepak.exe pak <FOLDER> [OUTFILE]

Arguments:
  <FOLDER>   Folder with loose files to pack
  [OUTFILE]  File to pack to

Options:
  -h, --help  Print help
```

For unpacking use `unpak` keyword, pass a file to unpack, and optionally - folder to unpack to.
Folder to unpack to (if not given( will be inferred from the file name like this:

`{CWD}\{FILE NAME W/O EXTENSION}`, `{CWD}` is "Current Working Directory".

**IMPORTANT:** Unpacked files in the target folder will be overwritten without any checks if they exist, so please be careful.

**Help section:**
```
dunepak.exe unpak --help
Unpacks PAK file

Usage: dunepak.exe unpak <FILE> [OUTFOLDER]

Arguments:
  <FILE>       File to unpack
  [OUTFOLDER]  Folder to put loose files to

Options:
  -h, --help  Print help
```

The file and folder names you pass can be absolute or relative to the current working directory.

## How to get
### Download binary release
Look for the zipped executable in the Releases section here on GitHub

### Build from sources
You can always build your own, like if you cannot find binary release for your system:

- [Install Rust toolchain](https://www.rust-lang.org/tools/install) for your system
- Clone this repository
- `cd dunepak`
- `cargo build --release`
- look for built *dunepak* executable in `.\target\release` folder
- profit

## Disclaimer/Licensing
Sources are distributed under [MIT License](https://opensource.org/license/mit/). This means you don't have to ask my permission to do whatever you want with the sources, as long as you keep this license.

I'm not responsible if this software damages your system, overwrites important files, is not fit for the purpose you want it used for, your computer catches fire or anything else in that matter.

Use it at your discretion.

## Thanks
Thank you for taking interest, and have fun...

