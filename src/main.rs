use flate2::write::{GzEncoder, GzDecoder};
use flate2::Compression;
use std::io::prelude::*;

struct Config{
    operation: String,
    input_file: String,
    output_file: String
}

impl Config{
    fn build()->Result<Config, String>{
        let args = std::env::args().collect::<Vec<_>>();
        if args.len()!=4{
            return Err("Invalid length".to_string());
        }
        let operation:String;
        if args[1] == "compress" || args[1] == "decompress"{
            operation = args[1].clone(); 
        } else {
            return Err("Not a valid operation".to_string());
        }
        let input_file = args[2].clone();
        let output_file = args[3].clone();

        return Ok(Config{operation:operation, input_file:input_file, output_file:output_file});
    }
}

fn main() {
    let now = std::time::Instant::now();
    let config = Config::build().unwrap_or_else(|err|{
        eprintln!("Error: {}", err);
        eprintln!("Usage: ./rustpress compress|decompress <input_file> <output_file>");
        std::process::exit(1);
    });
    let operation = config.operation;
    let input_file = config.input_file;
    let output_file = config.output_file;

    if operation == "compress" {
        compress(input_file, output_file);
        println!("Elapsed time: {}ms", now.elapsed().as_millis() as f64);
    } else{
        decompress(input_file, output_file);
        println!("Elapsed time: {}ms", now.elapsed().as_millis() as f64);
    }
}

fn compress(input_file:String, output_file:String){
    let mut encoder = GzEncoder::new(Vec::new(), Compression::new(9));
    encoder.write_all(std::fs::read_to_string(input_file.clone()).unwrap().as_bytes()).unwrap();
    let bytes = encoder.finish().unwrap();
    std::fs::write(output_file.clone(), bytes).unwrap();
    println!("{} successfully compressed to {}!", input_file, output_file);
}

fn decompress(input_file:String, output_file: String){
    let mut result = Vec::new();
    let mut decoder = GzDecoder::new(result);
    decoder.write_all(&std::fs::read(input_file.clone()).unwrap()[..]).unwrap();
    result = decoder.finish().unwrap();
    let return_string = String::from_utf8(result).unwrap();
    std::fs::write(output_file.clone(), return_string).unwrap();
    println!("{} successfully decompressed to {}!", input_file, output_file);
}
