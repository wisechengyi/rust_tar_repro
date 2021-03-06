extern crate flate2;
extern crate tar;

use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::path::Path;
use tar::Archive;

pub fn decompress_tgz(tar_path: &Path, output_dir: &Path) -> Result<(), std::io::Error> {
  //  let path = "archive.tar.gz";
  //
  let tar_gz = File::open(tar_path)?;
  let tar = GzDecoder::new(tar_gz);
  let mut archive = Archive::new(tar);
  archive.unpack(output_dir)?;
  //  println!("{:?}\n{:?}", tar_path, output_dir);
  Ok(())
}

pub fn compress() -> Result<(), std::io::Error> {
  let tar_gz = File::create("archive.tar.gz")?;
  let enc = GzEncoder::new(tar_gz, Compression::default());
  let mut tar = tar::Builder::new(enc);
  let mut f = File::open("test_data/hello.txt").unwrap();
  tar.append_file("gen/scrooge/297e53822ddf/wilyns.thrift.src.main.thrift.thrift-scala/c8959dfdc97a/com/twitter/wilyns/thriftscala/LookupRequest.scala",
                 &mut f )?;
  tar.finish();
  Ok(())
}
