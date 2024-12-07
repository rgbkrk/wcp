#![feature(portable_simd)]

use clap::Parser;
use memmap2::Mmap;
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::simd::cmp::*;
use std::simd::*;

const CHUNK_SIZE: usize = 1024 * 1024; // 1MB chunks

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File to process
    #[arg(name = "FILE")]
    file: String,

    /// Count lines
    #[arg(short = 'l', long)]
    lines: bool,

    /// Count words
    #[arg(short = 'w', long)]
    words: bool,

    /// Count bytes
    #[arg(short = 'c', long)]
    bytes: bool,
}

#[derive(Default)]
struct Counts {
    lines: usize,
    words: usize,
    bytes: usize,
}

#[inline(always)]
unsafe fn count_lines_words_bytes_simd(chunk: &[u8]) -> Counts {
    let mut counts = Counts::default();
    let mut word_count = 0;
    let mut in_word = false;

    let newline_mask = Simd::<u8, 64>::splat(b'\n');
    let whitespace_mask = Simd::<u8, 64>::splat(b' ') | Simd::<u8, 64>::splat(b'\t') | newline_mask;

    let mut i = 0;
    while i + 64 <= chunk.len() {
        let v = Simd::<u8, 64>::from_slice(&chunk[i..i + 64]);

        let newlines = v.simd_eq(newline_mask).to_bitmask();
        let spaces = v.simd_eq(whitespace_mask).to_bitmask();

        counts.lines += newlines.count_ones() as usize;
        word_count += spaces.count_ones() as usize;

        if in_word && spaces & 1 != 0 {
            word_count += 1;
        }
        in_word = spaces & (1 << 63) == 0;

        i += 64;
    }

    // Handle remaining bytes
    for &b in chunk.get_unchecked(i..) {
        if b == b'\n' {
            counts.lines += 1;
        }
        if b.is_ascii_whitespace() {
            if in_word {
                word_count += 1;
            }
            in_word = false;
        } else {
            in_word = true;
        }
    }

    counts.words = word_count;
    counts.bytes = chunk.len();
    counts
}

#[inline(always)]
fn count_lines_words_bytes(chunk: &[u8]) -> Counts {
    unsafe { count_lines_words_bytes_simd(chunk) }
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let file = File::open(&args.file)?;
    let mmap = unsafe { Mmap::map(&file)? };

    let total_counts: Counts = mmap
        .par_chunks(CHUNK_SIZE)
        .map(count_lines_words_bytes)
        .reduce(
            || Counts::default(),
            |a, b| Counts {
                lines: a.lines + b.lines,
                words: a.words + b.words,
                bytes: a.bytes + b.bytes,
            },
        );

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    if args.lines {
        write!(writer, "{} ", total_counts.lines)?;
    }
    if args.words {
        write!(writer, "{} ", total_counts.words)?;
    }
    if args.bytes {
        write!(writer, "{} ", total_counts.bytes)?;
    }
    writeln!(writer, "{}", args.file)?;
    writer.flush()?;

    Ok(())
}
