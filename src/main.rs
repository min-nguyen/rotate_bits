use std::fs;
use std::io::{self, Error, ErrorKind};

fn rotate_left(bytes: &mut [u8]) {
    if bytes.is_empty() {
        return;
    }

    let mut msb = bytes[0] >> 7 & 1;
    bytes.iter_mut().rev().for_each(|byte| {
        let lbyte = (*byte << 1) | msb;
        msb = (*byte >> 7) & 1;
        *byte = lbyte
    });
}

fn rotate_right(bytes: &mut [u8]) {
    if bytes.is_empty() {
        return;
    }

    let mut lsb = bytes.last().unwrap() & 1;
    bytes.iter_mut().for_each(|byte| {
        let rbyte = (lsb << 7) | (*byte >> 1);
        lsb = *byte & 1;
        *byte = rbyte
    });
}

fn rotate_from_file(buff: &mut String) -> io::Result<()> {
    println!("Usage: <right|left> <input_file_name> <output_file_name>");

    io::stdin().read_line(buff)?;

    let words: Vec<&str> = buff.split_whitespace().collect();

    if words.len() != 3 {
        return Err(error_other("Expected exactly three arguments.".to_string()));
    }

    let (cmd, ifile, ofile) = (words[0], words[1], words[2]);

    let mut bytes = fs::read(ifile)
        .map_err(|e: Error| error_other(format!("Input file error on \"{}\": {}.", ifile, e)))?;

    println!(
        "Read input bit stream from \"{}\":\n {}",
        ifile,
        format_bits(&bytes)
    );

    match cmd {
        "left" => {
            rotate_left(&mut bytes);
            Ok(())
        }
        "right" => {
            rotate_right(&mut bytes);
            Ok(())
        }
        _ => Err(error_other(
            "Unexpected first argument. Should be \"left\" or \"right\".".to_string(),
        )),
    }?;

    fs::write(ofile, &bytes)
        .map_err(|e| error_other(format!("Output file error on \"{}\":  {}", ofile, e)))?;

    println!(
        "Wrote output bit stream to \"{}\":\n {}",
        ofile,
        format_bits(&bytes)
    );

    Ok(())
}

fn main() {
    let mut buff: String = String::new();
    while let Err(e) = rotate_from_file(&mut buff) {
        println!("{}", e);
        buff.clear();
    }
}

fn error_other(str: String) -> Error {
    Error::new(ErrorKind::Other, str)
}

fn format_bits(bytes: &[u8]) -> String {
    bytes
        .iter()
        .fold(String::with_capacity(bytes.len() * 8), |s, byte| {
            s + &format!("{:08b}", byte)
        })
}
