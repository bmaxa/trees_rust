extern crate rand;
use rand::*;
use std::time::*;
extern crate list;
use list::*;

fn main() {
    let n = 256i32;
    let mut v:Vec<_> = (0..n).collect();
    let mut rng = thread_rng();
    rng.shuffle(&mut v);
    let mut l = List::Empty;
    for i in v {
      l = l.cons(Box::new(Node::new(i)));
    }
    println!("sum {}\n{:?}",l.sum(),l);
    let (_,mut l) = l.decons();
    println!("sum {}\n{:?}",l.sum(),l);
    let start = Instant::now();
    l = l.sort();
    let end = start.elapsed();
    println!("sorted\n{:?}",l);
    let (l,r) = l.split_at(8);
    println!("sum l {} sum r {}\n{:?}\n{:?}",l.sum(),r.sum(),l,r);
    let mut l = l.append_list(r);
    println!("sum {}\n{:?}",l.sum(),l);
    let time = (end.as_secs()*1000000000+end.subsec_nanos()as u64) as f64/ 1000000000.;
    let mut k = 0;
    for v in l.iter_mut() {
      print!(" {}",v);
      *v = k;
      k += 2;
    }
    println!("");
    for v in l.iter() {
      print!(" {}",v);
    }
    println!("");
    println!("time to sort {}",time);
}

