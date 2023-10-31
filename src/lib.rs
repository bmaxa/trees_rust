use std::iter;
#[derive(Debug,Clone)]
pub enum List {
    Empty,
    Head(Box<Node>)
}
#[derive(Debug,Clone)]
pub struct Node {
    link:Option<Box<Node>>,
    val:i32
}
use List::*;
impl Node {
    pub fn new(v:i32)->Node{
        Node{link:None,val:v}
    }
    fn split_mutable_reference<'a>(&'a mut self)->(&'a mut i32,Option<&'a mut Box<Node>>){
      let rc2 = if let Some(ref mut node) = &mut self.link {
        Some(node)
      } else { None };
      (&mut self.val,rc2)
    }
}
impl List {
    pub fn is_empty(&self)->bool {
        match self {
            &Empty=>true,
            &Head(_)=>false
        }
    }
    pub fn cons(mut self,mut n:Box<Node>)->List {
        let rc = match self {
          Head(x) => Some(x),
          Empty => None
        };
        n.link = rc;
        self = Head(n);
        self
    }
    pub fn decons(mut self)->(Option<Box<Node>>,List) {
        if let Head(mut n) = self {
          self = match &n.link {
                Some(x)=>Head(x.clone()),
                None => Empty
          };
          n.link = None;
          (Some(n),self)
        } else {
          (None,Empty)
        }
    }
    pub fn append(self,n:Box<Node>)->List {
        let mut l = Empty;
        l = l.cons(n);
        self.append_list(l)
    }
    pub fn append_list(mut self,mut l:List)->List {
        if !l.is_empty() {
          self = self.reverse();
          while !l.is_empty() {
            let (x,xs1) = l.decons();
            let x = x.unwrap();
            self = self.cons(x);
            l = xs1;
          }
          self = self.reverse();
        }
        self
    }
    pub fn split_at(mut self,mut n:i32)->(List,List) {
        let mut l1 = Empty;
        while n>0 {
            let (nn,l) = self.decons();
            self = l;
            if let Some(x) = nn {
                l1 = l1.cons(x);
            } else { break; }
            n -= 1;
        }
        (l1.reverse(),self)
    }
    pub fn reverse(mut self)->List {
      let mut rc = Empty;
      while let (Some(x),l) = self.decons() {
        rc = rc.cons(x);
        self = l;
      }
      rc
    }
    pub fn sum(&self)->i32 {
        let mut sum = 0;
        if let &Head(ref x) = self {
          let mut rf = x;
          sum += rf.val;
          while let Some(x) = &rf.link {
            sum+=x.val;
            rf=x;
          }
        }
        sum
    }
    pub fn len(&self)->i32 {
        let mut sum = 0;
        if let &Head(ref x) = self {
          let mut rf = x;
          sum += 1;
          while let Some(x) = &rf.link {
            sum+=1;
            rf=x;
          }
        }
        sum
    }
    pub fn sort(self)->List {
        let mut rc = self;
        let n = rc.len();
        if n < 2 {
            return rc
        }
        let (x1s,x2s) = rc.split_at(n/2);
        fn merge(mut xs:List,mut ys:List) -> List {
            let mut rc = Empty;
            while !xs.is_empty() && !ys.is_empty() {
                let (x,xs1) = xs.decons();
                let (y,ys1) = ys.decons();
                let (x,y) = (x.unwrap(),y.unwrap());
                if x.val < y.val {
                    rc = rc.cons(x);
                    xs = xs1;
                    ys = ys1.cons(y);
                } else {
                    rc = rc.cons(y);
                    xs = xs1.cons(x);
                    ys = ys1;
                }
            }
            rc = rc.reverse();
            rc = rc.append_list(xs);
            rc = rc.append_list(ys);
            rc
        }
        rc = merge(x1s.sort(),x2s.sort());
        rc
    }
    pub fn iter(&self)->Iterator {
      if let Head(x) = self {
        Iterator{ptr:Some(&x)}
      } else
        { Iterator{ptr:None} }
    }
    pub fn iter_mut(&mut self)->IteratorMut {
      if let Head(ref mut x) = self {
        IteratorMut{ptr:Some(x)}
      } else
        { IteratorMut{ptr:None} }
    }
}
pub struct Iterator<'a> {
  ptr: Option<&'a Box<Node>>
}
impl<'a> iter::Iterator for Iterator<'a> {
  type Item = &'a i32;
  fn next(&mut self) -> Option<Self::Item>{
    let rc = self.ptr.clone();
    self.ptr = if let Some(node) = self.ptr {
      if let Some(ref node) = node.link {
        Some(node)
      } else { None }
    } else { None };
    if let Some(ref node) = rc {
      Some(&node.val)
    } else { None }
  }
}
pub struct IteratorMut<'a> {
  ptr: Option<&'a mut Box<Node>>
}
impl<'a> iter::Iterator for IteratorMut<'a> {
  type Item = &'a mut i32;
  fn next(&mut self) -> Option<Self::Item>{
    let items = std::mem::replace(&mut self.ptr, None);
    if let Some(node) = items {
      let (val,next) = node.split_mutable_reference();
      self.ptr = next;
      Some(val)
    } else { None }
  }
}
