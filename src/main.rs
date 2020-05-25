use std::env;
use std::io::prelude::*;
use std::fs::File;

fn main() {
   let args: Vec<String> = env::args().collect();
   if args.len() < 3 {
      println!("Coinkydink\n\nUsage:\n\t{prog_name} input_file max_len", prog_name=args[0]);
      return;
   }
   let filename = &args[1];
   let max_len: usize = args[2].parse()
       .expect("Please provide a maximum length.");

   let mut f = File::open(filename).unwrap();
   let mut buffer = Vec::new();

   f.read_to_end(&mut buffer).unwrap();

   for i in (1..max_len + 1).rev() {
      let mut matchs: usize = 0;
      let mut total: usize = 0;
      for foo in 0..buffer.len() {
          for bar in (foo+i..buffer.len()).step_by(i) {
	      total += 1;
      	      if buffer[foo] == buffer[bar] {
	         matchs += 1;
	       }
       	   }
       }
       let ioc: f64 = matchs as f64 / total as f64;
       println!("{i:>2} {ioc:.3}%", i=i, ioc=ioc*100.0);
   }
}
