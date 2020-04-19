//#![feature(rustc_private)]
use std::cell::RefCell;
use std::rc::Rc;
use std::iter;
use std::ops::Deref;
use std::fmt;
use std::fmt::Debug;
use std::ops::*;
pub mod avl;
pub mod treap;
pub mod rb;
pub mod sg;

type PtrNode<K,V,T> = Option<*mut Node<K,V,T>>;
pub type PtrKey<T> = *const T;
pub type PtrValue<T> = Rc<RefCell<T>>;

pub trait ITree<K:Ord+Clone+Debug,V:Debug,T:Debug,Tr:ITree<K,V,T,Tr>> {
    fn insert(&mut self,t:&mut Tree<K,V,T,Tr>,k:K,v:V)->bool;
    fn delete(&mut self,t:&mut Tree<K,V,T,Tr>,k:&K)->bool;
    fn validate(&self,t:&mut Tree<K,V,T,Tr>)->bool;
}

pub struct Tree<K:Ord+Clone+Debug,V:Debug,T:Debug,Tr:ITree<K,V,T,Tr>> {
    root: PtrNode<K,V,T>,
    size: usize,
    pub pimpl: Tr
}

impl <K:Ord+Clone+Debug,V:Debug,T:Debug,Tr:ITree<K,V,T,Tr>> Drop for Tree<K,V,T,Tr>{
    fn drop(&mut self) {
        self.clear();
    }
}
pub struct Node<K:Ord,V,T> {
    key: K,
    value: PtrValue<V>,
    parent: PtrNode<K,V,T>,
    left: PtrNode<K,V,T>,
    right: PtrNode<K,V,T>,
    n: T
}

impl <K:Ord,V,T> Drop for Node<K,V,T> {
    fn drop(&mut self) {
        if let &Some(left) = &self.left {
            let _ = unsafe{Box::from_raw(left)};
        }
        if let &Some(right) = &self.right {
            let _ = unsafe{Box::from_raw(right)};
        }
   }
}

impl<K:Ord+Clone+Debug,V:Debug,T:Debug> Node<K,V,T> {
    fn new(k:K,v:V,t:T)->Self {
        Node{
            key:k,
            value:Rc::new(RefCell::new(v)),
            parent:None,
            left:None,
            right:None,
            n: t
        }
    }
    fn parent(&self)->PtrNode<K,V,T>{
        match &self.parent {
            &Some(a) => Some(a),
            &None => None
        }
    }
    fn set_parent(&mut self,n:PtrNode<K,V,T>) {
        self.parent = n;
    }
    fn left(&self)->PtrNode<K,V,T>{
        self.left.clone()
    }
    fn set_left(&mut self,n:PtrNode<K,V,T>) {
        self.left = n;
    }
    fn right(&self)->PtrNode<K,V,T>{
        self.right.clone()
    }
    fn set_right(&mut self,n:PtrNode<K,V,T>) {
        self.right = n;
    }
    fn data(&self)->(PtrKey<K>,PtrValue<V>){
        (&self.key,self.value.clone())
    }
    fn set_data(&mut self,l:PtrKey<K>,r:PtrValue<V>) {
        self.key = unsafe {(*l).clone()};
        self.value = r;
    }
    fn to_string(&self)->String {
        format!("({:?},{:?}):{:?}",self.key,self.value.borrow().deref(),self.n)
    }
}

impl <K:Ord+Clone+Debug,V:Debug,T:Debug> Debug for Node<K,V,T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.to_string())
    }
}

trait GetPtr<K:Ord,V,T> {
    fn get_ptr(&self)->Option<*mut Node<K,V,T>>;
    fn get_ptr_mut(&self)->Option<*mut Node<K,V,T>>;
    fn pred(&self)->PtrNode<K,V,T>;
    fn succ(&self)->PtrNode<K,V,T>;
    fn parent(&self)->PtrNode<K,V,T>;
    fn set_parent(&mut self,n:PtrNode<K,V,T>);
    fn left(&self)->PtrNode<K,V,T>;
    fn set_left(&mut self,n:PtrNode<K,V,T>);
    fn right(&self)->PtrNode<K,V,T>;
    fn set_right(&mut self,n:PtrNode<K,V,T>);
    fn key(&self)->&K;
    fn data(&self)->Option<(PtrKey<K>,PtrValue<V>)>;
    fn set_data(&mut self,d:(PtrKey<K>,PtrValue<V>));
    fn weight(&self)->usize;
    fn height(&self,d:usize,max:&mut usize,avg:&mut usize,count:&mut usize);
    fn to_string(&self,prefix: String,is_tail:bool)->String;
}

impl<K:Ord+Clone+Debug,V:Debug,T:Debug> GetPtr<K,V,T> for PtrNode<K,V,T> {
    fn get_ptr(&self)->Option<*mut Node<K,V,T>> {
        self.clone()
    }
    fn get_ptr_mut(&self)->Option<*mut Node<K,V,T>> {
        self.clone()
    }
    fn pred(&self)->PtrNode<K,V,T> {
        let mut nn = self.left();
        while let Some(_) = nn.right() {
            nn = nn.right();
        }
        nn
    }
    fn succ(&self)->PtrNode<K,V,T> {
        let mut nn = self.right();
        while let Some(_) = nn.left() {
            nn = nn.left();
        }
        nn
    }
    fn parent(&self)->PtrNode<K,V,T>{
        match self {
            &Some(a) => {
                unsafe { (*a).parent() }
            }
            &None => None
        }
    }
    fn set_parent(&mut self,n:PtrNode<K,V,T>) {
        if let &mut Some(a) = self {    
            unsafe {(*a).set_parent(n)};
        }
    }
    fn left(&self)->PtrNode<K,V,T>{
        match self {
            &Some(a) => {
                unsafe{(*a).left()}
            }
            &None => None
        }
    }
    fn set_left(&mut self,n:PtrNode<K,V,T>) {
        if let &mut Some(a) = self {    
            unsafe {(*a).set_left(n)};
        }
    }
    fn right(&self)->PtrNode<K,V,T>{
        match self {
            &Some(a) => {
                unsafe{(*a).right()}
            }
            &None => None
        }
    }
    fn set_right(&mut self,n:PtrNode<K,V,T>) {
        if let &mut Some(a) = self {    
            unsafe{(*a).set_right(n)};
        }
    }
    fn key(&self)->&K {
        match self {
            &Some(a) => {
                let (k,_) = unsafe {(*a).data()};
                unsafe {&*k}
            }
            &None => panic!("key not present!")
        }
    }
    fn data(&self)->Option<(PtrKey<K>,PtrValue<V>)> {
        match self {
            &Some(a) => {
                unsafe{Some((*a).data())}
            }
            &None => None
        }
    }
    fn set_data(&mut self,d:(PtrKey<K>,PtrValue<V>)){
        if let &mut Some(a) = self {
            unsafe{(*a).set_data(d.0,d.1)};
        }
    }
    fn weight(&self)->usize {
        match self {
            &Some(a) => {
                unsafe{(*a).left().weight()+(*a).right().weight()+1}
            }
            &None => 0
        }
    }
    fn height(&self,d:usize,max:&mut usize,avg:&mut usize,count:&mut usize) {
        match self {
            &Some(a) => {
                unsafe {
                (*a).left().height(d+1,max,avg,count);
                (*a).right().height(d+1,max,avg,count);
                }
            }
            &None => { 
                if *max < d {
                    *max = d;
                }
                *avg += d;
                *count += 1;
            }
        }
    }
    fn to_string(&self,prefix: String,is_tail:bool)->String {
        let mut tmp = prefix.clone();
        tmp = if is_tail {
            tmp + "└── "
        } else {
            tmp + "├── "
        };
        if self.is_none() {
            tmp = tmp + "nil";
            return tmp;
        }
        let tmp2 = self.clone().unwrap();
        unsafe {
        tmp = tmp + &(*tmp2).to_string()+"\n";
        }
        let tmp1 = if is_tail {
            prefix + "    "
        } else {
            prefix + "│   "
        };
        tmp = tmp + &self.left().to_string(tmp1.clone(),false)+"\n";
        tmp = tmp + &self.right().to_string(tmp1,true);
        tmp
    }
}

impl<'a,K:Debug+Clone+Ord,V:'static+Debug,T:Debug,Tr:ITree<K,V,T,Tr>> IndexMut<&'a K> for Tree<K,V,T,Tr> {
    fn index_mut(&mut self,key:&K)->&mut V{
        let mut n = self.root.clone();
        while !n.is_none() {
            if key < n.key() {
                n = n.left();
            } else {    
                if key == n.key() {
                    break;
                }
                n = n.right();
            }
        }
        match n.data() {
           Some((_,rc)) => {
               let rc:*mut _ =&mut *rc.borrow_mut();
               unsafe { &mut *rc }
           },
           None => panic!("value not found")
        }
    }
}

impl<'a,K:Debug+Clone+Ord,V:'static+Debug,T:Debug,Tr:ITree<K,V,T,Tr>> Index<&'a K> for Tree<K,V,T,Tr> {
    type Output = V;
    fn index(&self,key:&K)->&V{
        let mut n = self.root.clone();
        while !n.is_none() {
            if key < n.key() {
                n = n.left();
            } else {    
                if key == n.key() {
                    break;
                }
                n = n.right();
            }
        }
        match n.data() {
           Some((_,rc)) => {
               let rc:*const _ =&*rc.borrow_mut();
               unsafe { &*rc }
           }
           None => panic!("value not found")
        }
    }
}

impl<K:Ord+Clone+Debug,V:Debug,T:Debug,Tr:ITree<K,V,T,Tr>> Tree<K,V,T,Tr>{
    pub fn new(pimpl: Tr)->Self {
        Tree{root:None,size:0,pimpl:pimpl}
    }
    pub fn size(&self)->usize {
        self.size
    }
    pub fn clear(&mut self) {  
        if let &Some(ptr) = &self.root {
            let _ = unsafe {Box::from_raw(ptr)};
        }
        self.root = None;
        self.size = 0;
    }
    pub fn find(&self,key: &K)->Iterator<K,V,T> {
        let mut rc = Iterator::new(None);
        let mut n = self.root.clone();
        while !n.is_none() {
            if key < n.key() {
                n = n.left();
            } else {    
                if key == n.key() {
                    rc = Iterator::new(n);
                    rc.data_back = None;
                    break;
                }
                n = n.right();
            }
        }
        rc
    }
    pub fn iter(&self)->Iterator<K,V,T> {
        let mut rc = Iterator::new(self.root.clone());
        if !rc.data.is_none() {
            let mut tmp = rc.data.clone();
            while !tmp.is_none() {
                rc.data = tmp.clone();
                tmp=tmp.left();
            }
        }
        if !rc.data_back.is_none() {
            let mut tmp = rc.data_back.clone();
            while !tmp.is_none() {
                rc.data_back = tmp.clone();
                tmp = tmp.right();
            }
        }
        rc
    }
    pub fn weight(&self)->(usize,usize) {
        if self.root.is_none() {
            return (0,0)
        }
        let left = self.root.left().weight();
        let right = self.root.right().weight();
        (left,right)
    }
    pub fn height(&self)->usize {
        let mut max = 0;
        let mut avg = 0;
        let mut count = 0;
        self.root.height(0,&mut max,&mut avg,&mut count);
        avg/count
    }
    pub fn to_string(&self)->String {
        if self.root.is_none() {
            return "empty tree".to_string()
        }
        self.root.to_string("".to_string(),true)
    }
    pub fn insert(&mut self,k:K,v:V)->bool {
        let pimpl:*mut Tr = &mut self.pimpl;
        unsafe {
        (*pimpl).insert(self,k,v)
        }
    }
    pub fn delete(&mut self,k:&K)->bool {
        let pimpl:*mut Tr = &mut self.pimpl;
        unsafe {
        (*pimpl).delete(self,k)
        }
    }
    pub fn validate(&mut self)->bool {
        let pimpl:*mut Tr = &mut self.pimpl;
        unsafe {
        (*pimpl).validate(self)
        }
    }
}

pub struct Iterator<K:Ord,V,T> {
    data: PtrNode<K,V,T>,
    data_back: PtrNode<K,V,T>
}

impl<K:Ord,V,T> Iterator<K,V,T> {
    fn new(n: PtrNode<K,V,T>)->Self {
        Iterator{data:n.clone(),data_back:n}
    }
}

impl<K:Ord+Debug,V:Debug,T:Debug> Iterator<K,V,T> {
    pub fn value(&self)->Option<PtrValue<V>> {
        if let &Some(a) = &self.data {
            unsafe{Some((*a).value.clone())}
        } else {
            None
        }
    }
}

impl<K:'static+Ord+Clone+Debug,V:Debug,T:Debug> iter::Iterator for Iterator<K,V,T> {
    type Item = (&'static K,PtrValue<V>);
    fn next(&mut self) -> Option<Self::Item>{
    
        let result = if let Some((k,v)) = self.data.data() {
            unsafe { Some((&*k,v)) }
        } else { None };
        
        if !self.data.is_none() && self.data.get_ptr() == self.data_back.get_ptr() {
            self.data = None;
            self.data_back = None;
        }
        if let Some(right) = self.data.right() {
            self.data = Some(right);
            while let Some(left) = self.data.left() {
                self.data = Some(left);
            }
        } else {
            let mut tmp = self.data.parent();
            while let Some(ptr) = tmp.clone() {
                if self.data.get_ptr() != tmp.right().get_ptr() {
                    break;
                }
                self.data = Some(ptr);
                tmp = tmp.parent();
            }
            self.data = tmp;
        }
        result
    }
}
impl<K:'static+Ord+Clone+Debug,V:Debug,T:Debug> iter::DoubleEndedIterator for Iterator<K,V,T> {
    fn next_back(&mut self) -> Option<(&'static K,PtrValue<V>)>{
    
        let result = if let Some((k,v)) = self.data.data() {
            unsafe { Some((&*k,v)) }
        } else { None };
        
        if !self.data_back.is_none() && self.data.get_ptr() == self.data_back.get_ptr() {
            self.data = None;
            self.data_back = None;
        }
        if let Some(left) = self.data_back.left() {
            self.data_back = Some(left);
            while let Some(right) = self.data_back.right() {
                self.data_back = Some(right);
            }
        } else {
            let mut tmp = self.data_back.parent();
            while let Some(ptr) = tmp.clone() {
                if self.data_back.get_ptr() != tmp.left().get_ptr() {
                    break;
                }
                self.data_back = Some(ptr);
                tmp = tmp.parent();
            }
            self.data_back = tmp;
        }
        result
    }
}

#[test]
fn tree_test() {
    use avl::Avl;
    let mut t = Avl::new::<i32,i32>();
    for i in 0..10 {
        t.insert(i,10-i);
    }
    println!("{}",t.to_string());
}


