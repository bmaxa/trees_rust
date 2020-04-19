use super::*;
use PtrNode;
use GetPtr;
use std::fmt;
use std::fmt::Debug;

pub struct Sg {
    max_size:usize,pub counter: usize,
}

pub struct SgData;
impl Debug for SgData{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"(data)")
    }
}
impl Sg {
    pub fn new<K:Ord+Clone+Debug,V:Debug>()->Tree<K,V,SgData,Sg> {
        Tree::new(Sg{max_size:0,counter:0})
    }
    fn get_parent<K:Ord+Clone+Debug,V:Debug>(t:&mut Tree<K,V,SgData,Sg>,n:&PtrNode<K,V,SgData>)->*mut PtrNode<K,V,SgData> {
        unsafe {
        if n.parent().is_none() {
            return &mut t.root
        }
        let p:*mut Node<K,V,SgData> = n.parent().get_ptr_mut().unwrap();
        if n.parent().left().get_ptr() == n.get_ptr() {
            return &mut (*p).left
        } else {
            return &mut (*p).right
        }
        }    
    }
   fn delete<K:Ord+Clone+Debug,V:Debug>(&mut self,t:&mut Tree<K,V,SgData,Sg>,n:&mut PtrNode<K,V,SgData>){
        let mut nn = n.clone();
        if !n.left().is_none() && !n.right().is_none() {
            nn = GetPtr::pred(n);
            n.set_data(nn.data().unwrap());
        }
        
        let nnp = Sg::get_parent(t,&nn);
        if !nn.left().is_none() {
            unsafe { *nnp = nn.left(); }
            nn.left().set_parent(nn.parent());
        } else {
            unsafe { *nnp = nn.right(); }
            nn.right().set_parent(nn.parent());
        }
        nn.set_left(None);
        nn.set_right(None);
        if let Some(ptr) = nn {
            let _ = unsafe{Box::from_raw(ptr)};
        }
        t.size -= 1;
        
        if (t.size as f64) < 0.50*self.max_size as f64 {
            self.max_size = t.size;
            self.counter += 1;
            if cfg!(feature = "debug") {
                println!("counter {}",self.counter);
            }
            self.perfect_balance(t,t.root,t.size);
        }
    }
    fn for_each_node<K,V>(n:PtrNode<K,V,SgData>,v:&mut Vec<PtrNode<K,V,SgData>>)
    where K:Ord+Clone+Debug,V:Debug {
        if n.is_none() { return }
        Sg::for_each_node(n.left(),v);
        v.push(n.clone());
        Sg::for_each_node(n.right(),v);
    }
    fn perfect_balance<K,V>(&mut self,t:&mut Tree<K,V,SgData,Sg>,n:PtrNode<K,V,SgData>,size:usize)
    where K:Ord+Clone+Debug,V:Debug{
        if n.is_none() { return; }
        let mut v = Vec::new();
        v.reserve(size);
        Sg::for_each_node(n,&mut v);
        unsafe{
            let (link,parent) = (Sg::get_parent(t,&n),n.parent());
            t.size -= v.len();
            let mut sg = Sg::new();
            self.insert_divide(&mut sg,0,v.len(),&v,None,true);
            *link = sg.root;
            if !parent.is_none() { sg.root.set_parent(parent); }
            t.size += sg.size;
            sg.root = None;
        }
    }
    fn insert_divide<K,V>(&mut self,t:&mut Tree<K,V,SgData,Sg>, b:usize,e:usize,v:&Vec<PtrNode<K,V,SgData>>,parent:PtrNode<K,V,SgData>, left:bool)
    where K:Ord+Clone+Debug,V:Debug{
        assert!(b<=e);
        if e == b { return }
        let mut p = v[(b+e)/2];
        p.set_right(None);
        p.set_left(None);
        p.set_parent(parent);
        if p.parent().is_none() {
            t.root = p;
        } else {
            if left {
                p.parent().set_left(p);
            } else {
                p.parent().set_right(p);
            }
        }
        t.size += 1;
        self.insert_divide(t,b,(e+b)/2,v,p,true);
        self.insert_divide(t,(e+b)/2+1,e,v,p,false);
    }
}
impl <K:Ord+Clone+Debug,V:Debug> ITree<K,V,SgData,Sg> for Sg {
    fn insert(&mut self,t:&mut Tree<K,V,SgData,Sg>,k:K,v:V)->bool {
            if t.root.is_none() {
            t.root = Some(Box::into_raw(Box::new(Node::new(k,v,SgData))));
            t.size += 1;
            return true;
        }
        let mut depth = 0;
        let mut prev = None;
        let mut rc = None;
        let mut n = t.root.clone();
        let mut ret = false;
        while !n.is_none() {
            prev = n.clone();
            ret = k < *n.key();
            if ret {
                n = n.left();
            } else {
               rc = n.clone();
               n = n.right();
            }
            depth += 1;
        }
        if !rc.is_none() && !(*rc.key() < k) {
            return false;
        }
        n = Some(Box::into_raw(Box::new(Node::new(k,v,SgData))));
        t.size += 1;
        if !ret {
            prev.set_right(n.clone());
        } else {
            prev.set_left(n.clone());
        }
        n.set_parent(prev.clone());
        self.max_size = if self.max_size > t.size { self.max_size } else { t.size };
        let alpha = 0.60;
        
        if depth as f64 > (t.size as f64).ln()/(1.0/alpha as f64).ln(){
            let (mut p,mut child) = (n.parent(),n);
            let (mut w,mut ws) = (1,
            if p.left().get_ptr() == child.get_ptr(){p.right().weight()} else {p.left().weight()})
            ;
            let mut height = 1;
            while !p.parent().is_none() && ((w+ws+1) as f64).ln()/(1.0/alpha as f64).ln() >= height as f64 {
                child = p;
                p = p.parent();
                w += ws + 1;
                height += 1;
                ws = if p.left().get_ptr() == child.get_ptr(){p.right().weight()} else {p.left().weight()};
            }
            self.counter += 1;
            self.perfect_balance(t,p,w+ws+1);
        }
        true
    }
    fn delete(&mut self,t:&mut Tree<K,V,SgData,Sg>,k:&K)->bool {
        let mut n = t.find(k).data;
        if !n.is_none() {
            self.delete(t,&mut n);
            true
        } else {
            false
        }
    }
    fn validate(&self, t:&mut Tree<K,V,SgData,Sg>)->bool {
        validate(t.root.clone())
    }
}
fn validate<K:Ord+Clone+Debug,V:Debug>(n:PtrNode<K,V,SgData>)->bool {
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
    
    let lh = validate(n.left());
    let rh = validate(n.right());
    lh && rh
}
