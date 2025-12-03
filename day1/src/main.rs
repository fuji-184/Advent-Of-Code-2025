use std::env;
use std::ffi::OsStr;
use std::fs::read;

fn rotate(path: &OsStr) {
    let bytes = read(path).expect("failed to read input file");

    let mut position = 50;
    let mut password = 0;

    // this heap vector is not neccerely
    // just to see the list of all positions
    //let mut positions = vec![];

    let mut i = 0;
    let len = bytes.len();

    while i < len {
        match &bytes[i] {
            b'L' => {
                i += 1;
                let number_start = i;
                while bytes[i] != b'\n' {
                    i += 1;
                }
                let number = byte_to_number(&bytes[number_start..i]);
                let new_position = (position + 100 - number) % 100;
                if new_position == 0 {
                    password += 1;
                }
                position = new_position;
                //positions.push(position);
            }
            b'R' => {
                i += 1;
                let number_start = i;
                while bytes[i] != b'\n' {
                    i += 1;
                }
                let number = byte_to_number(&bytes[number_start..i]);
                let new_position = (position + number) % 100;
                if new_position == 0 {
                    password += 1;
                }
                position = new_position;
                //positions.push(new_position);
            }
            _ => i += 1,
        }
    }

    println!("password: {}", password);
    //println!("positions: {:?}", positions);
}

fn byte_to_number(bytes_view: &[u8]) -> i32 {
    let mut n = 0i32;
    for &val in bytes_view {
        n = n * 10 + (val - b'0') as i32;
    }
    n
}

fn main() {
    let filename = env::args_os().nth(1).expect("usage: program <filename>");
    rotate(&filename);
}
