use super::*;
use PtrNode;
use GetPtr;
use std::fmt;
use std::fmt::Debug;

pub struct Rb;

impl Rb {
    pub fn new<K:Ord+Clone+Debug,V:Debug>()->Tree<K,V,RbData,Rb> {
        Tree::new(Rb)
    }
    fn get_parent<K:Ord+Clone+Debug,V:Debug>(t:&mut Tree<K,V,RbData,Rb>,n:&PtrNode<K,V,RbData>)->*mut PtrNode<K,V,RbData> {
    unsafe {
        if n.parent().is_none() {
            return &mut t.root
        }
        let p:*mut Node<K,V,RbData> = n.parent().get_ptr_mut().unwrap();
        if n.parent().left().get_ptr() == n.get_ptr() {
            return &mut (*p).left
        } else {
            return &mut (*p).right
        }
    }    
    }
    fn insert_helper<K:Ord+Clone+Debug,V:Debug>(t:&mut Tree<K,V,RbData,Rb>,k:K,v:V)->(PtrNode<K,V,RbData>,bool){
        let mut x = t.root.clone();
        let mut y = None;
        let mut z = None;
        if x.is_none() {
            let n = Node::new(k,v,RbData{colour:Colour::RED});
            t.root = Some(Box::into_raw(Box::new(n)));
            t.size += 1;
            z = t.root;
            return (z,true);
        }
        while !x.is_none() {
            y = x;
            if k < *x.key() {
                x = x.left();
            } else {
                z = x;
                x = x.right();
            }
        }
        if !z.is_none() && !(*z.key() < k) {
            return (z,false);
        }
        let n = Node::new(k,v,RbData{colour:Colour::RED});
        z = Some(Box::into_raw(Box::new(n)));
        z.set_parent(y);
        if *z.key() < *y.key() {
            y.set_left(z);
        } else {
            y.set_right(z);
        }
        t.size += 1;
        (z,true)
    }
    fn delete<K:Ord+Clone+Debug,V:Debug>(t:&mut Tree<K,V,RbData,Rb>,n:&mut PtrNode<K,V,RbData>){
        let mut nn = n.clone();
        if !n.left().is_none() && !n.right().is_none() {
            nn = GetPtr::pred(n);
            n.set_data(nn.data().unwrap());
        }
        
        let nnp = Rb::get_parent(t,&nn);
        let mut prev = nn.parent();
        let mut x;
        if !nn.left().is_none() {
            unsafe { *nnp = nn.left(); }
            nn.left().set_parent(nn.parent());
            x = nn.left();
        } else {
            unsafe { *nnp = nn.right(); }
            nn.right().set_parent(nn.parent());
            x = nn.right();
        }
        nn.set_left(None);
        nn.set_right(None);
        unsafe {
        if (*nn.unwrap()).n.colour == Colour::BLACK {
            Rb::delete_fixup(t,x,prev);
        }}
        if let Some(ptr) = nn {
            let _ = unsafe{Box::from_raw(ptr)};
        }
        t.size -= 1;
    }
    fn delete_fixup<K:Ord+Clone+Debug,V:Debug>(t:&mut Tree<K,V,RbData,Rb>,mut x:PtrNode<K,V,RbData>,mut p:PtrNode<K,V,RbData>){
        unsafe {
            while x.get_ptr() != t.root.get_ptr() && !is_red(x) {
                if x.get_ptr() == p.left().get_ptr() {
                    let mut w = p.right();
                    if is_red(w) {
                        (*w.unwrap()).n.colour = Colour::BLACK;
                        (*p.unwrap()).n.colour = Colour::RED;
                        Rb::rotate_left(t,&mut p);
                        w = p.right();
                    }
                    if !is_red(w.left()) && !is_red(w.right()) {
                        (*w.unwrap()).n.colour = Colour::RED;
                        x = p;
                        p = p.parent();
                    } else {
                        if !is_red(w.right()) {
                            (*w.left().unwrap()).n.colour = Colour::BLACK;
                            (*w.unwrap()).n.colour = Colour::RED;
                            Rb::rotate_right(t,&mut w);
                            w = p.right();
                        }
                        (*w.unwrap()).n.colour = (*p.unwrap()).n.colour.clone();
                        (*p.unwrap()).n.colour = Colour::BLACK;
                        (*w.right().unwrap()).n.colour = Colour::BLACK;
                        Rb::rotate_left(t,&mut p);
                        p = t.root;
                        x = p;
                    }
                } else {
                    let mut w = p.left();
                    if is_red(w) {
                        (*w.unwrap()).n.colour = Colour::BLACK;
                        (*p.unwrap()).n.colour = Colour::RED;
                        Rb::rotate_right(t,&mut p);
                        w = p.left();
                    }
                    if !is_red(w.right()) && !is_red(w.left()) {
                        (*w.unwrap()).n.colour = Colour::RED;
                        x = p;
                        p = p.parent();
                    } else {
                        if !is_red(w.left()) {
                            (*w.right().unwrap()).n.colour = Colour::BLACK;
                            (*w.unwrap()).n.colour = Colour::RED;
                            Rb::rotate_left(t,&mut w);
                            w = p.left();
                        }
                        (*w.unwrap()).n.colour = (*p.unwrap()).n.colour.clone();
                        (*p.unwrap()).n.colour = Colour::BLACK;
                        (*w.left().unwrap()).n.colour = Colour::BLACK;
                        Rb::rotate_right(t,&mut p);
                        p = t.root;
                        x = p;
                    }
                }
            }
            if !x.is_none() {
                (*x.unwrap()).n.colour = Colour::BLACK;
            }
        }
    }
    fn rotate_left<K:Ord+Clone+Debug,V:Debug>(t:&mut Tree<K,V,RbData,Rb>,x:&mut PtrNode<K,V,RbData>)->PtrNode<K,V,RbData> {
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
    fn rotate_right<K:Ord+Clone+Debug,V:Debug>(t:&mut Tree<K,V,RbData,Rb>,x:&mut PtrNode<K,V,RbData>)->PtrNode<K,V,RbData> {
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
    fn validate<K:Ord+Clone+Debug,V:Debug>(n:PtrNode<K,V,RbData>)->i32 {
    unsafe {    
        if n.is_none() { return 1; }
        if (*n.unwrap()).n.colour == Colour::RED {
        if (!n.left().is_none() && (*n.left().unwrap()).n.colour == Colour::RED) || 
            (!n.right().is_none() && (*n.right().unwrap()).n.colour == Colour::RED) {
            println!("red violation");
            return 0;
        }}
        let lh = Rb::validate(n.left());
        let rh = Rb::validate(n.right());

        if !n.left().is_none() && !(n.left().key() < n.key()) {
            println!("order violation, left");
            return 0;
        }
        if !n.right().is_none() && !(n.key() < n.right().key()) {
            println!("order violation, right");
            return 0;
        }
        if !n.left().is_none() && n.left().parent().get_ptr() != n.get_ptr() {
            println!("parent violation");
            return 0;
        }
        if !n.right().is_none() && n.right().parent().get_ptr() != n.get_ptr() {
            println!("parent violation");
            return 0;
        }
        if lh !=0 && rh !=0 && lh != rh {
            println!("black violation");
            return 0;
        }
        if lh !=0 && rh!=0 {
            if (*n.unwrap()).n.colour == Colour::RED {
                return lh;
            } else {
                return lh+1;
            }
        }
        return 0;
    }}
}

#[derive(Debug,PartialEq,Clone)]
enum Colour {RED,BLACK}

pub struct RbData{
    colour: Colour
}
impl Debug for RbData{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"(c:{:?})",self.colour)
    }
}
impl<K:Ord+Clone+Debug,V:Debug> ITree<K,V,RbData,Rb> for Rb {
    fn insert(&mut self,t:&mut Tree<K,V,RbData,Rb>,k:K,v:V)->bool{
        let (mut x,rc) = Rb::insert_helper(t,k,v);
        if !rc { return rc; }
        unsafe {
            (*x.unwrap()).n.colour = Colour::RED;
            while x.get_ptr() != t.root.get_ptr() && 
                (*x.parent().unwrap()).n.colour == Colour::RED {
                if x.parent().get_ptr() == x.parent().parent().left().get_ptr() {
                    let y = x.parent().parent().right();
                    if !y.is_none() && (*y.unwrap()).n.colour == Colour::RED {
                        (*x.parent().unwrap()).n.colour = Colour::BLACK;
                        (*y.unwrap()).n.colour = Colour::BLACK;
                        (*x.parent().parent().unwrap()).n.colour = Colour::RED;
                        x = x.parent().parent();
                    } else {
                        if x.get_ptr() == x.parent().right().get_ptr() {
                            x = x.parent();
                            Rb::rotate_left(t,&mut x);
                        }
                        (*x.parent().unwrap()).n.colour = Colour::BLACK;
                        (*x.parent().parent().unwrap()).n.colour = Colour::RED;
                        Rb::rotate_right(t,&mut x.parent().parent());
                    }
                } else {
                    let y = x.parent().parent().left();
                    if !y.is_none() && (*y.unwrap()).n.colour == Colour::RED {
                        (*x.parent().unwrap()).n.colour = Colour::BLACK;
                        (*y.unwrap()).n.colour = Colour::BLACK;
                        (*x.parent().parent().unwrap()).n.colour = Colour::RED;
                        x = x.parent().parent();
                    } else {
                        if x.get_ptr() == x.parent().left().get_ptr() {
                            x = x.parent();
                            Rb::rotate_right(t,&mut x);
                        }
                        (*x.parent().unwrap()).n.colour = Colour::BLACK;
                        (*x.parent().parent().unwrap()).n.colour = Colour::RED;
                        Rb::rotate_left(t,&mut x.parent().parent());
                    }
                }
            }
            (*t.root.unwrap()).n.colour = Colour::BLACK;
        }
        true
    }
    fn delete(&mut self,t:&mut Tree<K,V,RbData,Rb>,k:&K)->bool{
        let mut n = t.find(k).data;
        if n.is_none() {
            return false;
        }
        Rb::delete(t,&mut n);
        true
    }
    fn validate(&self,t:&mut Tree<K,V,RbData,Rb>)->bool{
        Rb::validate(t.root)>0
    }
}

fn is_red<K:Ord,V>(n:PtrNode<K,V,RbData>)->bool {
    if n.is_none() { return false; }
    unsafe {
        return (*n.unwrap()).n.colour == Colour::RED;
    }
}
