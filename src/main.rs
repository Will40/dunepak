use std::ffi::CString;
use std::error::Error;
use std::path::PathBuf;
use std::fs::{self, OpenOptions};
use std::io::Write;

use byteorder::{LittleEndian, ReadBytesExt};
use clap::{Parser, Subcommand};
use path_absolutize::*;

#[macro_use]
extern crate unwrap;

/// DUNE II Pak file extractor/packer
#[derive(Debug, Parser)] // requires `derive` feature
#[command(name = "dunepak")]
#[command(author, version, about = "DUNE II Pak extractor/packer", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    /// Display more info when packing/unpacking
    #[clap(long, short, action)]
    verbose: bool
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Pack folder
    #[command(arg_required_else_help = true)]
    Pak {
        /// Folder with loose files to pack
        folder: PathBuf,
        /// File to pack to
        outfile: Option<PathBuf>,
    },

    /// Unpacks PAK file
    #[command(arg_required_else_help = true)]
    Unpak {
        /// File to unpack
        file: PathBuf,
        /// Folder to put loose files to
        outfolder: Option<PathBuf>,
    },
}

fn main() -> Result<(), Box<dyn Error>>{
    //let mut pargs = pico_args::Arguments::from_env();
    let args = Cli::parse();
    match args.command {
        Commands::Pak { folder, outfile } => {
            let folder: PathBuf = folder.absolutize().unwrap().to_path_buf();
            let mut outfile: PathBuf = outfile.unwrap_or(PathBuf::from(folder.file_name().unwrap().to_ascii_uppercase())).absolutize().unwrap().to_path_buf();
            if outfile.extension().is_none() { outfile.set_extension("PAK"); }
            println!("Packing {} to {}", folder.display(), outfile.display());
            pak(&folder, &outfile, args.verbose)?;
        },
        
        Commands::Unpak { file, outfolder} => {
            let mut file: PathBuf = file.absolutize().unwrap().to_path_buf(); // NO UNWRAPPING is Recommended
            let outfolder: PathBuf = outfolder.unwrap_or(PathBuf::from(file.file_stem().unwrap().to_ascii_uppercase())).absolutize().unwrap().to_path_buf();
            if !file.is_file() { add_extension( &mut file, "PAK"); }
            if file.is_file() {
                println!("Unpacking {} to {}", file.display(), outfolder.display());
                unpak(&file, &outfolder, args.verbose)?;
            } else { 
                panic!("PAK File {} not found!", file.display()); 
            }
        }
    }
    Ok(())
}

#[derive(Debug)]
struct HeaderEntry {
    starts: u32,
    ends: u32,
    filename: String
}

fn pak(pakdir: &PathBuf, tofile: &PathBuf, verbose: bool) -> Result<(), Box<dyn Error>> {
    let mut paths: Vec<_> = unwrap!(fs::read_dir(pakdir), "{} Is not a directory!", pakdir.display())
    .map(|r| r.unwrap())
    .collect();
    let mut headers:Vec<HeaderEntry> = vec![];
    paths.sort_by_key(|dir| dir.path());
    let mut hdrtablesize = 0;
    for path in paths {
        let path = path.path();
        let metadata = fs::metadata(&path)?;
        if metadata.is_file() { // Pack only files
            let hdr = HeaderEntry {
                starts: 0,
                ends: 0,
                filename: String::from(path.file_name().unwrap().to_str().unwrap())
            };
            hdrtablesize += 4 + hdr.filename.len() + 1;
            headers.push(hdr);
        }
    }
    hdrtablesize += 4;
    let mut buf: Vec<u8> = vec![];
    let mut hdrbuf: Vec<u8> = vec![];
    let file_count = headers.len();
    for mut hdr in headers {
        let filename = pakdir.join(&hdr.filename);
        if verbose { print!("{}", filename.display()); }
        assert!(hdr.filename.len() <= 12, "File name \"{}\" is longer than 12 characters, max allowed in Dune II PAK files", &hdr.filename);
        hdr.starts = u32::try_from(hdrtablesize + buf.len()).unwrap();
        let starts_le_bytes = hdr.starts.to_le_bytes();
        hdrbuf.extend_from_slice(&starts_le_bytes);
        let fname = CString::new(hdr.filename.to_ascii_uppercase()).unwrap(); // UPPERCASE it in PAK file
        hdrbuf.extend_from_slice(fname.to_bytes_with_nul());
        let mut contents = unwrap!(std::fs::read(&filename), "Could not read the file {}.", filename.display());
        buf.append(&mut contents);
        if verbose { print!(" - OK\n"); }
    }
    hdrbuf.extend_from_slice(b"\0\0\0\0");
    let mut fulldata = hdrbuf;
    fulldata.append(&mut buf);
    unwrap!(std::fs::write(&tofile, fulldata), "Error saving PAK file {}.", tofile.display());
    println!("{file_count} files packed successfuly.");
    Ok(())
}

fn unpak(pakfile: &PathBuf, tofolder: &PathBuf, verbose: bool) -> Result<(), Box<dyn Error>> {
    let data = unwrap!(fs::read(&pakfile), "Error reading PAK file {}.", pakfile.display());
    let mut slice = &data[..];
    let mut headers : Vec<HeaderEntry> = vec![];
    loop {
        if let Ok((slice1, starts)) = read_int32(slice) {
            if starts == 0 { 
                if headers.len() > 0 { headers.last_mut().unwrap().ends = u32::try_from(data.len()).unwrap() - 1; }
                break; 
            }
            if let Ok((slice2, filename)) = read_filename_with_nul(slice1) {
                if headers.len() > 0 { headers.last_mut().unwrap().ends = starts - 1; }
                headers.push(HeaderEntry {
                    starts,
                    ends: 0,
                    filename,
                });
                slice = slice2;
            }
        }        
    }

    let filecount = headers.len();
    for hdr in headers {
        if !tofolder.exists() { unwrap!(std::fs::create_dir(&tofolder), "Failed to create output folder {}.", tofolder.display()); }
        if tofolder.is_dir() {
            if verbose { print!("{}", tofolder.join(&hdr.filename).display()); }
            let tofile = tofolder.clone().join(hdr.filename);
            let mut file = unwrap!(OpenOptions::new().write(true).create(true).truncate(true).open(&tofile), "Error opening file {} for writing.", tofile.display());
            unwrap!(file.write_all(&data[usize::try_from(hdr.starts).unwrap()..=usize::try_from(hdr.ends).unwrap()]), "Error writing data to file {}.", tofile.display());
            if verbose { print!(" - OK\n"); };
        } else { panic!("Failure, {} is not a folder.", tofolder.display()); }
    }
    print!("{filecount} files unpacked successfuly.");
    Ok(())
}

fn read_int32(s: &[u8])-> Result<(&[u8], u32), Box<dyn Error>> {
    let mut rdr = std::io::Cursor::new(s);
    let int = unwrap!(rdr.read_u32::<LittleEndian>(), "Error reading integer from binary buffer.");
    Ok((&s[4..], int))
}

fn read_filename_with_nul(s: &[u8]) -> Result<(&[u8], String), Box<dyn Error>> {
    let mut i = 0;
    while s[i] > 0 && i < 14 { i+=1; };
    assert!(i <= 12, "PAK is corrupted, Found file name longer than 12 characters");
    Ok((&s[i+1..], String::from(std::str::from_utf8(&s[..i]).expect("Non ASCII FileName in PAK"))))
}

fn add_extension(path: &mut std::path::PathBuf, extension: impl AsRef<std::path::Path>) {
    match path.extension() {
        Some(ext) => {
            let mut ext = ext.to_os_string();
            ext.push(".");
            ext.push(extension.as_ref());
            path.set_extension(ext)
        }
        None => path.set_extension(extension.as_ref()),
    };
}