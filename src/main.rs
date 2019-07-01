//#![feature(rand)]
#![feature(rustc_private)]
#![feature(core_intrinsics)]
extern crate binary_trees;
use std::mem;
use binary_trees::Node;
use binary_trees::avl::*;
use binary_trees::treap::*;
use binary_trees::rb::*;
use binary_trees::sg::*;
use binary_trees::Tree;
extern crate rand;
use rand::*;
use std::time::*;
use std::collections::BTreeMap;
use std::fmt::Debug;

extern {
    fn _rdtsc()->u64;
}
static N:i32 = 1000000;

struct Trta {
    a: [i32;512]
}

fn seed()->[u32;4] {
    use std::io::prelude::*;
    use std::fs::File;

    let mut f = File::open("/dev/urandom").unwrap();
    let mut buffer = [0u32; 4];
    // read up to 16 bytes
    let n = f.read(unsafe{mem::transmute::<&mut [u32;4],&mut [u8;16]>(&mut buffer)});
    println!("seed {}",n.unwrap());
    buffer
}
fn main() {
    let a:Vec<_> = (0..10).collect();
    let mut avl = Avl::new::<i32,i32>();
    let mut treap = Treap::new::<i32,i32>();
    let mut rb = Rb::new::<i32,i32>();
    let mut sg = Sg::new::<i32,i32>();
    for i in a.iter() {
        avl.insert(*i,*i);
        treap.insert(*i,*i);
        rb.insert(*i,*i);
        sg.insert(*i,*i);
        println!("{}",rb.to_string());
        println!("{}",sg.to_string());
    }
    println!("{}\n{}",rb.to_string(),rb.validate());
    let mut iter = avl.iter();
    let mut iter1 = treap.iter();
    let mut iter2 = rb.iter();
    while let Some((i,_)) = iter.next() {
        print!("{} ",i);
        if let Some((j,_)) = iter.next_back() {
            println!("{} ",j);
        }
    }
    while let Some((i,_)) = iter1.next() {
        print!("{} ",i);
        if let Some((j,_)) = iter1.next_back() {
            println!("{} ",j);
        }
    }
    while let Some((i,_)) = iter2.next() {
        print!("{} ",i);
        if let Some((j,_)) = iter2.next_back() {
            println!("{} ",j);
        }
    }
    for i in a.iter() {
        avl.delete(i);
        println!("valid {} {}",avl.validate(),avl.size());
    }
    for i in a.iter() {
        treap.delete(i);
        println!("valid {} {}",treap.validate(),treap.size());
    }
    for i in a.iter() {
        rb.delete(i);
        println!("valid {} {}\n{}",rb.validate(),rb.size(),rb.to_string());
    }
    println!("");
    //return;
    let d:Vec<i32> = (0..N).collect();
    let mut v:Vec<String> = d.iter().map(|n| {format!("abcd avioni {}",n)}).collect();
    let mut vd = v.clone();
    let mut rng = rand::isaac::IsaacRng::from_seed(&seed());
    rng.shuffle(&mut v);
    rng.shuffle(&mut vd);
    println!("trees! {}",mem::size_of::<Node<i32,i32,AvlData>>());
    println!("treest! {}",mem::size_of::<Node<i32,i32,TreapData>>());
    println!("treesr! {}",mem::size_of::<Node<i32,i32,RbData>>());
    let mut t = Avl::new::<String,i32>();
    let mut t1 = Treap::new::<String,i32>();
    let mut t2 = Rb::new::<String,i32>();
    let mut t3 = Sg::new::<String,i32>();
    let mut bt: BTreeMap<String,i32> = BTreeMap::new();

    let start = Instant::now();
    let mut k = 0;
    let mut sum = 0;
    for i in v.iter().rev() {
        let start = unsafe {_rdtsc()};
        t.insert(i.clone(),k);
        k += 1;
        let end = unsafe{_rdtsc()};
        sum += end-start;
    }
    println!("average op {}",sum/v.len()as u64);
    let end = start.elapsed();
    let diff = (end.as_secs()*1000000000+end.subsec_nanos() as u64) as f64 / 1000000000.0;
    let (l,r) = t.weight();
    println!("t insert time {}\n{}\nheight {}\nweight ({},{})",diff,t.validate(),t.height(),l,r);

    let start = Instant::now();
    let mut k = 0;
    let mut sum = 0;
    for i in v.iter().rev() {
        let start = unsafe {_rdtsc()};
        t1.insert(i.clone(),k);
        k += 1;
        let end = unsafe{_rdtsc()};
        sum += end-start;
    }
    println!("average op {}",sum/v.len()as u64);
    let end = start.elapsed();
    let diff = (end.as_secs()*1000000000+end.subsec_nanos() as u64) as f64 / 1000000000.0;
    let (l,r) = t1.weight();
    println!("t1 insert time {}\n{}\nheight {}\nweight ({},{})",diff,t1.validate(),t1.height(),l,r);

    let start = Instant::now();
    let mut k = 0;
    let mut sum = 0;
    for i in v.iter().rev() {
        let start = unsafe {_rdtsc()};
        t2.insert(i.clone(),k);
        k += 1;
        let end = unsafe{_rdtsc()};
        sum += end-start;
    }
    println!("average op {}",sum/v.len()as u64);
    let end = start.elapsed();
    let diff = (end.as_secs()*1000000000+end.subsec_nanos() as u64) as f64 / 1000000000.0;
    let (l,r) = t2.weight();
    println!("t2 insert time {}\n{}\nheight {}\nweight ({},{})",diff,t2.validate(),t2.height(),l,r);
    let start = Instant::now();
    let mut k = 0;
    let mut sum = 0;
    for i in v.iter().rev() {
        let start = unsafe {_rdtsc()};
        t3.insert(i.clone(),k);
        k += 1;
        let end = unsafe{_rdtsc()};
        sum += end-start;
    }
    println!("average op {}",sum/v.len()as u64);
    let end = start.elapsed();
    let diff = (end.as_secs()*1000000000+end.subsec_nanos() as u64) as f64 / 1000000000.0;
    let (l,r) = t3.weight();
    println!("t3 insert time {}\n{}\nheight {}\nweight ({},{})",diff,t3.validate(),t3.height(),l,r);
    let start = Instant::now();
    let mut k = 0;
    let mut sum = 0;
    for i in v.iter().rev() {
        let start = unsafe {_rdtsc()};
        bt.insert(i.clone(),k);
        k += 1;
        let end = unsafe {_rdtsc()};
        sum += end-start;
    }
    println!("average op {}",sum/v.len()as u64);
    let end = start.elapsed();
    let diff = (end.as_secs()*1000000000+end.subsec_nanos() as u64) as f64 / 1000000000.0;
    println!("bt insert time {}\n{} size {}",diff,t2.validate(),t2.size());

    let start = Instant::now();
    let mut sum = 0;
    for i in vd.iter() {
        let j = t.find(i);
        if let Some(v) = j.value() {
            sum += *v.borrow();
        }
    }
    let end = start.elapsed();
    let diff = (end.as_secs()*1000000000+end.subsec_nanos() as u64) as f64 / 1000000000.0;
    println!("t find time {}\n{}\nsum {}",diff,t.validate(),sum);

    let start = Instant::now();
    let mut sum = 0;
    for i in vd.iter() {
        let j = t1.find(i);
        if let Some(v) = j.value() {
            sum += *v.borrow();
        }
    }
    let end = start.elapsed();
    let diff = (end.as_secs()*1000000000+end.subsec_nanos() as u64) as f64 / 1000000000.0;
    println!("t1 find time {}\n{}\nsum {}",diff,t1.validate(),sum);

    let start = Instant::now();
    let mut sum = 0;
    for i in vd.iter() {
//      let j = t2.find(i);
//      if let Some(v) = j.value() {
//      sum += *v.borrow();
        sum += t2[i];
//    }
    }
    let end = start.elapsed();
    let diff = (end.as_secs()*1000000000+end.subsec_nanos() as u64) as f64 / 1000000000.0;
    println!("t2 find time {}\n{}\nsum {}",diff,t2.validate(),sum);

    let start = Instant::now();
    let mut sum = 0;
    for i in vd.iter() {
//      let j = t2.find(i);
//      if let Some(v) = j.value() {
//      sum += *v.borrow();
        sum += t3[i];
//    }
    }
    let end = start.elapsed();
    let diff = (end.as_secs()*1000000000+end.subsec_nanos() as u64) as f64 / 1000000000.0;
    println!("t3 find time {}\n{}\nsum {}",diff,t3.validate(),sum);

    let start = Instant::now();
    let mut sum = 0;
    for i in vd.iter() {
        let a = bt.get(i).unwrap();
        sum += *a;
    }
    let end = start.elapsed();
    let diff = (end.as_secs()*1000000000+end.subsec_nanos() as u64) as f64 / 1000000000.0;
    println!("bt find time {}\n{}\nsum {}",diff,t.validate(),sum);

    let start = Instant::now();
    let mut sum:i32 = 0;
    let st = unsafe {_rdtsc()};
    for (_,v) in t.iter() {
        sum += *v.borrow();
    }
    let end = unsafe{_rdtsc()};
    println!("average op {}",(end-st)/v.len()as u64);
    let end = start.elapsed();
    let diff = (end.as_secs()*1000000000+end.subsec_nanos() as u64) as f64 / 1000000000.0;
    println!("t iter time {}\n{}\nsum {}",diff,t.validate(),sum);

    let start = Instant::now();
    let mut sum:i32 = 0;
    let st = unsafe {_rdtsc()};
    for (_,v) in t1.iter() {
        sum += *v.borrow();
    }
    let end = unsafe{_rdtsc()};
    println!("average op {}",(end-st)/v.len()as u64);
    let end = start.elapsed();
    let diff = (end.as_secs()*1000000000+end.subsec_nanos() as u64) as f64 / 1000000000.0;
    println!("t1 iter time {}\n{}\nsum {}",diff,t1.validate(),sum);

    let start = Instant::now();
    let mut sum:i32 = 0;
    let st = unsafe {_rdtsc()};
    for (_,v) in t2.iter() {
        sum += *v.borrow();
    }
    let end = unsafe{_rdtsc()};
    println!("average op {}",(end-st)/v.len()as u64);
    let end = start.elapsed();
    let diff = (end.as_secs()*1000000000+end.subsec_nanos() as u64) as f64 / 1000000000.0;
    println!("t2 iter time {}\n{}\nsum {}",diff,t2.validate(),sum);

    let start = Instant::now();
    let mut sum:i32 = 0;
    let st = unsafe {_rdtsc()};
    for (_,v) in t3.iter() {
        sum += *v.borrow();
    }
    let end = unsafe{_rdtsc()};
    println!("average op {}",(end-st)/v.len()as u64);
    let end = start.elapsed();
    let diff = (end.as_secs()*1000000000+end.subsec_nanos() as u64) as f64 / 1000000000.0;
    println!("t3 iter time {}\n{}\nsum {}",diff,t3.validate(),sum);

    let start = Instant::now();
    let mut sum:i32 = 0;
    let st = unsafe {_rdtsc()};
    for (_,v) in bt.iter() {
        sum += *v;
    }
    let end = unsafe{_rdtsc()};
    println!("average op {}",(end-st)/v.len()as u64);
    
    let end = start.elapsed();
    let diff = (end.as_secs()*1000000000+end.subsec_nanos() as u64) as f64 / 1000000000.0;
    println!("bt iter time {}\n{}\nsum {}",diff,t.validate(),sum);

    let start = Instant::now();
    for i in vd.iter()/*.take(N as usize-100)*/ {
        t.delete(&i);
//        assert!(t.validate());
    }
//    t.clear();
    let end = start.elapsed();
    let diff = (end.as_secs()*1000000000+end.subsec_nanos() as u64) as f64 / 1000000000.0;
    println!("t delete time {}\n{} size {}",diff,t.validate(),t.size());
    println!("{}",t.to_string());

    let start = Instant::now();
    for i in vd.iter()/*.take(/*N as usize-*/100)*/ {
        t1.delete(&i);
//        assert!(t1.validate());
    }
//    t.clear();
    let end = start.elapsed();
    let diff = (end.as_secs()*1000000000+end.subsec_nanos() as u64) as f64 / 1000000000.0;
    println!("t1 delete time {}\n{} size {}",diff,t1.validate(),t1.size());
    println!("{}",t1.to_string());

    let start = Instant::now();
    for i in vd.iter()/*.take(/*N as usize-*/100)*/ {
        t2.delete(&i);
//        assert!(t1.validate());
    }
//    t.clear();
    let end = start.elapsed();
    let diff = (end.as_secs()*1000000000+end.subsec_nanos() as u64) as f64 / 1000000000.0;
    println!("t2 delete time {}\n{} size {}",diff,t2.validate(),t2.size());
    println!("{}",t2.to_string());

    let start = Instant::now();
    for i in vd.iter()/*.take(/*N as usize-*/100)*/ {
        t3.delete(&i);
//        assert!(t1.validate());
    }
//    t.clear();
    let end = start.elapsed();
    let diff = (end.as_secs()*1000000000+end.subsec_nanos() as u64) as f64 / 1000000000.0;
    println!("t3 delete time {}\n{} size {}",diff,t3.validate(),t3.size());
    println!("{}",t3.to_string());

    let start = Instant::now();
    for i in vd.iter()/*.take(N as usize-100)*/ {
        bt.remove(i);
    }
//    bt.clear();
    let end = start.elapsed();
    let diff = (end.as_secs()*1000000000+end.subsec_nanos() as u64) as f64 / 1000000000.0;
    println!("bt delete time {}\n{}",diff,t.validate());
 
}
