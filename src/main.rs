use clap::{Command, Arg};
use std::{io::BufReader,io::prelude::*,fs};
use itertools::Itertools;

//TODO I need to implement for gz files, look at flate2
//? maybe check out doing a --dryrun flag that uses the counter variable 

fn main() {
  let args = parse_args();
  let threshold:f32 = *args.get_one::<f32>("THRESHOLD").expect("No threshold provided");
  //println!("{:?}",threshold);
  let fpath = match args.contains_id("INPUT"){
    true => args.get_one::<String>("INPUT").unwrap(),
    false => args.get_one::<String>("POS").unwrap()
  };
  let infile = fs::File::open(fpath).unwrap();
  let reader = BufReader::new(infile);
  //let mut counter:usize = 0;
  for (chunk,lines) in reader.lines().chunks(4).into_iter().enumerate(){
    let mut write_vec:Vec<String> = Vec::with_capacity(3);
    for (num,str_res) in lines.enumerate() {
        let str:String = str_res.unwrap();
        if num == 0 && chunk == 0 {assert_eq!(str.clone().chars().next().unwrap(),'@',"File not in FASTQ format")} // makes sure fastq format
        if num % 4 == 3 {
            if convert_to_phred(str.clone()) > threshold {
                for s in write_vec.iter(){println!("{}",s)} // printing the first 3 strings
                println!("{}",str); // printing the the phred string
                //counter += 1; // shows how many were sorted
                //println!("{} > {}",convert_to_phred(str.clone()),threshold) // for debugging
            }
            //else{
            //  println!("X   {} < {}",convert_to_phred(str.clone()),threshold)
            //}
        }
        else{write_vec.push(str)}
      }
    }
  }
  
fn convert_to_phred(qual:String) -> f32{
    let mut sum:usize = 0;
    for q in qual.chars(){
        //println!("{}",q as usize);
        sum += q as usize;
    }
    (sum as f32 / qual.chars().count() as f32) - 33.0
}

fn parse_args() -> clap::ArgMatches{
  Command::new("FastQ Sorter")
      .about("Sorts FastQ file by PHRED Score")
      .arg(Arg::new("INPUT").short('i').long("input").help("Input fastq file"))
      .arg(Arg::new("THRESHOLD").short('t').long("thresh").help("PHRED cutoff").value_parser(clap::value_parser!(f32)))
      .arg(Arg::new("POS").required_unless_present("INPUT").help("Catches extra args"))
      .get_matches()
}