use std::io::Write;
use std::path::Path;
use std::{fs, process};

use anyhow::{Context, Result};
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e:#}");
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: tae-pack <data_dir> [output.tae]");
        process::exit(1);
    }
    let data_dir = Path::new(&args[1]);
    let output = args.get(2).map(|s| s.as_str()).unwrap_or("data.tae");

    anyhow::ensure!(data_dir.is_dir(), "not a directory: {}", data_dir.display());

    let zip_bytes = pack_folder(data_dir)?;
    let encrypted = tae_core::encrypt(&zip_bytes)?;

    fs::write(output, &encrypted).with_context(|| format!("writing {output}"))?;

    println!(
        "Packed {} files → {} ({} bytes)",
        count_files(data_dir),
        output,
        encrypted.len()
    );

    Ok(())
}

fn pack_folder(dir: &Path) -> Result<Vec<u8>> {
    let cursor = std::io::Cursor::new(Vec::new());
    let mut zip = ZipWriter::new(cursor);
    let options = SimpleFileOptions::default();

    add_dir(dir, dir, &mut zip, options)?;

    let cursor = zip.finish().context("finalising zip")?;
    Ok(cursor.into_inner())
}

fn add_dir(
    root: &Path,
    dir: &Path,
    zip: &mut ZipWriter<std::io::Cursor<Vec<u8>>>,
    options: SimpleFileOptions,
) -> Result<()> {
    let mut entries: Vec<_> = fs::read_dir(dir)
        .with_context(|| format!("reading dir {}", dir.display()))?
        .collect::<Result<_, _>>()?;
    entries.sort_by_key(|e| e.path());

    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            add_dir(root, &path, zip, options)?;
        } else {
            let rel = path.strip_prefix(root)?.to_string_lossy().replace('\\', "/");
            let bytes = fs::read(&path).with_context(|| format!("reading {}", path.display()))?;
            zip.start_file(&rel, options)
                .with_context(|| format!("adding {rel} to zip"))?;
            zip.write_all(&bytes)?;
        }
    }
    Ok(())
}

fn count_files(dir: &Path) -> usize {
    fs::read_dir(dir)
        .map(|entries| {
            entries
                .filter_map(|e| e.ok())
                .map(|e| {
                    if e.path().is_dir() { count_files(&e.path()) } else { 1 }
                })
                .sum()
        })
        .unwrap_or(0)
}
