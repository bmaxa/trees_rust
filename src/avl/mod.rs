use super::*;
use PtrNode;
use GetPtr;
use std::fmt;
use std::fmt::Debug;

pub struct Avl;

impl Avl{
    pub fn new<K:Ord+Clone+Debug,V:Debug>()-> Tree<K,V,AvlData> {
        Tree::new(Box::new(Avl))
    }
    fn get_parent<K:Ord+Clone+Debug,V:Debug>(t:&mut Tree<K,V,AvlData>,n:&PtrNode<K,V,AvlData>)->*mut PtrNode<K,V,AvlData> {
    unsafe {
        if n.parent().is_none() {
            return &mut t.root
        }
        let p:*mut Node<K,V,AvlData> = n.parent().get_ptr_mut().unwrap();
        if n.parent().left().get_ptr() == n.get_ptr() {
            return &mut (*p).left
        } else {
            return &mut (*p).right
        }
    }    
    }
    fn delete<K:Ord+Clone+Debug,V:Debug>(t:&mut Tree<K,V,AvlData>,n:&mut PtrNode<K,V,AvlData>){
        let mut nn = n.clone();
        if !n.left().is_none() && !n.right().is_none() {
            nn = GetPtr::pred(n);
            n.set_data(nn.data().unwrap());
        }
        
        let nnp = Avl::get_parent(t,&nn);
        let mut prev = nn.parent();
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
        while !prev.is_none() {
            Avl::balance(t,&mut prev);
            prev = prev.parent();
        }
        t.size -= 1;
    }
    fn balance<K:Ord+Clone+Debug,V:Debug>(t:&mut Tree<K,V,AvlData>,n:&mut PtrNode<K,V,AvlData>) {
        fixheight(n);
        if bfactor(n) == 2 {
            if bfactor(&n.right())<0 {
                Avl::rotate_right(t,&mut n.right());
            }
            Avl::rotate_left(t,n);
            return;
        }
        if bfactor(n) == -2 {
            if bfactor(&n.left()) > 0 {
                Avl::rotate_left(t,&mut n.left());
            }
            Avl::rotate_right(t,n);
            return;
        }
    }
    fn rotate_left<K:Ord+Clone+Debug,V:Debug>(t:&mut Tree<K,V,AvlData>,x:&mut PtrNode<K,V,AvlData>) {
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
        fixheight(x);
        fixheight(&mut y);
    }
    fn rotate_right<K:Ord+Clone+Debug,V:Debug>(t:&mut Tree<K,V,AvlData>,x:&mut PtrNode<K,V,AvlData>) {
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
        fixheight(x);
        fixheight(&mut y);
    }
}

pub struct AvlData {
    height: i32
}

impl Debug for AvlData{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"(h:{})",self.height)
    }
}
impl<K:Ord+Clone+Debug,V:Debug> ITree<K,V,AvlData> for Avl {
    fn insert(&mut self,t:&mut Tree<K,V,AvlData>,k:K,v:V)->bool{
        if t.root.is_none() {
            t.root = Some(Box::into_raw(Box::new(Node::new(k,v,AvlData{height:1}))));
            t.size += 1;
            return true;
        }
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
        }
        if !rc.is_none() && !(*rc.key() < k) {
            return false;
        }
        n = Some(Box::into_raw(Box::new(Node::new(k,v,AvlData{height:1}))));
        t.size += 1;
        if !ret {
            prev.set_right(n.clone());
        } else {
            prev.set_left(n.clone());
        }
        n.set_parent(prev.clone());
        let mut nn = prev;
        while !nn.is_none() {
            Avl::balance(t,&mut nn);
            nn = nn.parent();
        }
        true
    }
    fn delete(&mut self,t:&mut Tree<K,V,AvlData>,k:&K)->bool{
        let mut n = t.find(k).data;
        if !n.is_none() {
            Avl::delete(t,&mut n);
            true
        } else {
            false
        }
    }
    fn validate(&self,t:&mut Tree<K,V,AvlData>)->bool{
        validate(t.root.clone())
    }
}

fn height<K:Ord,V>(n:&PtrNode<K,V,AvlData>) -> i32 {
    if let &Some(a) = n {
        unsafe {(*a).n.height}
    } else {0}
}

fn fixheight<K:Ord+Clone+Debug,V:Debug>(n:&PtrNode<K,V,AvlData>) {
    let l = height(&n.left());
    let r = height(&n.right());
    if let &Some(n) = n {
        unsafe {
        (*n).n.height = if l > r {
            l + 1
        } else {
            r + 1
        }
        };
    }
}

fn bfactor<K:Ord+Clone+Debug,V:Debug>(n: &PtrNode<K,V,AvlData>) -> i32 {
    height(&n.right()) - height(&n.left())
}

fn abs(n:i32)->i32 {
    if n<0 { -n } else { n }
}

fn validate<K:Ord+Clone+Debug,V:Debug>(n:PtrNode<K,V,AvlData>)->bool {
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

    if abs(bfactor(&n)) > 1 {
        println!("abs(balance factor) > 1 {:?}",unsafe {(*n.unwrap()).to_string()});
        return false
    }
    
    let lh = validate(n.left());
    let rh = validate(n.right());
    lh && rh
}

