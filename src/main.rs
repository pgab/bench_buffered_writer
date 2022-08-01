#![feature(test)]

extern crate test;

const DATA_SIZE: usize = 100_000;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;


    #[bench]
    fn write_one_by_one(b: &mut Bencher) {
        use std::path::PathBuf;
        use std::io::{BufWriter, Write};
        use std::fs::File;

        b.iter(|| {
            // just do this to use the same memory as the other test
            let _d = vec![1u8;DATA_SIZE];
            let da = vec![1u8;1];
            let filepath = PathBuf::from("/tmp/write_one_by_one");
            let mut writer = BufWriter::new(File::create(filepath).expect("If we can't do that, abort"));

            for _i in 0..DATA_SIZE {
                writer.write_all(&da).expect("If we can't do that, abort");
            }
        });
    }

    #[bench]
    fn write_all(b: &mut Bencher) {
        use std::path::PathBuf;
        use std::io::{BufWriter, Write};
        use std::fs::File;

        b.iter(|| {
            let d = vec![1u8;DATA_SIZE];
            let filepath = PathBuf::from("/tmp/write_all");
            let mut writer = BufWriter::new(File::create(filepath).expect("If we can't do that, abort"));

            writer.write_all(&d).expect("If we can't do that, abort");

        });
    }

    #[bench]
    fn write_to_mem_then_all(b: &mut Bencher) {
        use std::path::PathBuf;
        use std::io::{BufWriter, Write};
        use std::fs::File;

        b.iter(|| {
            let d = vec![1u8;DATA_SIZE];
            let da = vec![1u8;1];
            let filepath = PathBuf::from("/tmp/write_to_mem_then_all");
            let mut writer = BufWriter::new(File::create(filepath).expect("If we can't do that, abort"));

            let mut tmp = Vec::with_capacity(d.len() * ::std::mem::size_of::<u8>());
            for _v in d {
                tmp.write(&da).expect("If we can't do that, abort");
            }

            writer.write_all(&tmp).expect("If we can't do that, abort");

        });
    }
}

fn main() {
    println!("Hello, world!");
}


/*


 */