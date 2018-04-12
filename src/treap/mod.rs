use super::*;
use PtrNode;
use GetPtr;
use std::fmt;
use std::fmt::Debug;
extern crate rand;
use self::rand::*;
use self::rand::IsaacRng;
extern crate time;
use self::time::precise_time_ns;

pub struct Treap{
    rng: IsaacRng
}
impl Treap {
    pub fn new<K:Ord+Clone+Debug,V:Debug>()-> Tree<K,V,TreapData> {
        let rc = Treap {rng: Treap::new_from_u64(precise_time_ns()) };
        Tree::new(Box::new(rc))
    }
    fn new_from_u64(seed:u64)->IsaacRng {
        let buf:[u32;4] = [(seed>>32) as u32 ,(seed&0xffffffff) as u32,
            (seed>>32) as u32,(seed&0xffffffff) as u32];
        IsaacRng::from_seed(&buf)
    }
    fn prn(&mut self)->i32 {
        self.rng.gen()
    }
    fn get_parent<K:Ord+Clone+Debug,V:Debug>(t:&mut Tree<K,V,TreapData>,n:&PtrNode<K,V,TreapData>)->*mut PtrNode<K,V,TreapData> {
    unsafe {
        if n.parent().is_none() {
            return &mut t.root
        }
        let p:*mut Node<K,V,TreapData> = n.parent().get_ptr_mut().unwrap();
        if n.parent().left().get_ptr() == n.get_ptr() {
            return &mut (*p).left
        } else {
            return &mut (*p).right
        }
    }    
    }
    fn rotate_left<K:Ord+Clone+Debug,V:Debug>(t:&mut Tree<K,V,TreapData>,x:&mut PtrNode<K,V,TreapData>)->PtrNode<K,V,TreapData> {
        if x.right().is_none() {
            panic!("rotate_left:x.Right==nil\n{:?}\n{:?}",t.to_string(),t.size);
        }
        let mut y = x.right();
        x.set_right(y.left());
        if !y.left().is_none() {
            y.left().set_parent(x.clone());
        }
        y.set_parent(x.parent());
        if x.parent().is_none() {
            t.root = y.clone();
        } else {
            if x.get_ptr() == x.parent().left().get_ptr() {
                x.parent().set_left(y.clone());
            } else {
                x.parent().set_right(y.clone());
            }
        }
        y.set_left(x.clone());
        x.set_parent(y.clone());
        y
    }
    fn rotate_right<K:Ord+Clone+Debug,V:Debug>(t:&mut Tree<K,V,TreapData>,x:&mut PtrNode<K,V,TreapData>)->PtrNode<K,V,TreapData> {
        if x.left().is_none() {
            panic!("rotate_right:x.Left==nil\n{:?}\n{:?}",t.to_string(),t.size);
        }
        let mut y = x.left();
        x.set_left(y.right());
        if !y.right().is_none() {
            y.right().set_parent(x.clone());
        }
        y.set_parent(x.parent());
        if x.parent().is_none() {
            t.root = y.clone();
        } else {
            if x.get_ptr() == x.parent().left().get_ptr() {
                x.parent().set_left(y.clone());
            } else {
                x.parent().set_right(y.clone());
            }
        }
        y.set_right(x.clone());
        x.set_parent(y.clone());
        y
    }
    fn rebalance_left<K:Ord+Clone+Debug,V:Debug>(t:&mut Tree<K,V,TreapData>,x:&mut PtrNode<K,V,TreapData>) {
        if x.is_none() { return; }
        Treap::rebalance_left(t,&mut x.left());
        while let Some(tmpl) =  x.left() {
            let tmp = x.unwrap();
            unsafe {
            if (*tmpl).n.priority < (*tmp).n.priority {
                Treap::rotate_right(t,x);
            } else { break; }}
        }
    }
    fn rebalance_up<K:Ord+Clone+Debug,V:Debug>(t:&mut Tree<K,V,TreapData>,x:&mut PtrNode<K,V,TreapData>) {
        let mut n = x.clone();
        while let Some(parent) = n.parent() {
            let tmp = n.unwrap();
            unsafe {
            if (*tmp).n.priority < (*parent).n.priority {
                n = if n.parent().left().get_ptr() == n.get_ptr() {
                    Treap::rotate_right(t,&mut n.parent())
                } else {
                    Treap::rotate_left(t,&mut n.parent())
                }
            } else { break; }}
        }
    }
}

pub struct TreapData{
    priority:i32
}
impl Debug for TreapData{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"(p:{})",self.priority)
    }
}
impl<K:Ord+Clone+Debug,V:Debug> ITree<K,V,TreapData> for Treap {
    fn insert(&mut self,t:&mut Tree<K,V,TreapData>,k:K,v:V)->bool{
        if t.root.is_none() {
            let n = Node::new(k,v,TreapData{priority:self.prn()});
            t.root = Some(Box::into_raw(Box::new(n)));
            t.size += 1;
            return true;
        }
        let mut n = t.root.clone();
        let mut rc = None;
        let mut prev = None;
        let mut ret = false;
        while !n.is_none() {
            prev = n.clone();
            ret = k < *n.key();
            if ret {
                n = n.left();
            } else {
                rc = n;
                n = n.right();
            }
        }
        if !rc.is_none() && !(*rc.key() < k) {
            return false;
        }
        let n = Node::new(k,v,TreapData{priority:self.prn()});
        let n = Some(Box::into_raw(Box::new(n)));
        if ret {
            prev.set_left(n.clone());
            rc = prev.left();
            prev.left().set_parent(prev);
        } else {
            prev.set_right(n.clone());
            rc = prev.right();
            prev.right().set_parent(prev);
        }
        Treap::rebalance_up(t,&mut rc);
        t.size += 1;
        true
    }
    fn delete(&mut self,t:&mut Tree<K,V,TreapData>,k:&K)->bool{
        let mut rc = None;
        let mut n = t.root.clone();
        while !n.is_none() {
            let ret = k < n.key();
            if ret {
                n = n.left();
            } else {
                rc = n;
                n = n.right();
            }
        }
        if rc.is_none() || rc.key() < k {
            return false;
        }
        let mut reb = None;
        while !rc.left().is_none() && !rc.right().is_none() {
            let tmp = Treap::rotate_left(t,&mut rc);
            if reb.is_none() {
                if let Some(n) = tmp.left() {
                    let nn = tmp.unwrap();
                    unsafe {
                    if (*n).n.priority < (*nn).n.priority {
                        reb = tmp;
                    }}
                }
            }
        }
        let nnp = Treap::get_parent(t,&rc);
        if !rc.left().is_none() {
            unsafe { *nnp = rc.left(); }
            rc.left().set_parent(rc.parent());
        } else {
            unsafe { *nnp = rc.right(); }
            rc.right().set_parent(rc.parent());
        }
        rc.set_left(None);
        rc.set_right(None);
        if let Some(ptr) = rc {
            let _ = unsafe{Box::from_raw(ptr)};
        }
        t.size -= 1;
        Treap::rebalance_left(t,&mut reb);
        true
    }
    fn validate(&self,t:&mut Tree<K,V,TreapData>)->bool{
        validate(t.root.clone())
    }
}
fn validate<K:Ord+Clone+Debug,V:Debug>(n:PtrNode<K,V,TreapData>)->bool {
    if n.is_none() { return true; }
    
    if !n.left().is_none() && !(n.left().key() < n.key()) {
        return false
    }
    
    if !n.right().is_none() && !(n.key() < n.right().key()) {
        return false
    }
    
    if !n.left().is_none() && n.left().parent().get_ptr()!=n.get_ptr() {
        println!("node fail {:?} {:?}",n.left().key(),n.left().parent().key());
        return false
    }
    
    if !n.right().is_none() && n.right().parent().get_ptr()!=n.get_ptr() {
        println!("node fail {:?} {:?}",n.right().key(),n.right().parent().key());
        return false
    }

    if let Some(tmpl) = n.left() {
        let tmp = n.unwrap();
        unsafe {
        if (*tmp).n.priority > (*tmpl).n.priority {
            return false;
        }}
    }
    if let Some(tmpr) = n.right() {
        let tmp = n.unwrap();
        unsafe {
        if (*tmp).n.priority > (*tmpr).n.priority {
            return false;
        }}
    }
    let lh = validate(n.left());
    let rh = validate(n.right());
    lh && rh
}
