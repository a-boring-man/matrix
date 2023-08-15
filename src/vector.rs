use std::ops::{Add, Mul, Sub};

#[allow(dead_code)]
struct Vector<K> {
    v: Vec::<K>,
}

#[allow(dead_code)]
impl<K> Vector<K>
where
    K: Add<Output = K> + Sub<Output = K> + Mul<Output = K> + Clone,
{
    fn self_add(&mut self, v: Vector<K>) {
        self.v = self.v.iter().zip(v.v.iter()).map(|(a, b)| a.clone() + b.clone()).collect();
    }
    fn self_sub(&mut self, v: Vector<K>) {
        self.v = self.v.iter().zip(v.v.iter()).map(|(a, b)| a.clone() - b.clone()).collect();
    }
    fn self_scale(&mut self, a: K) {
        let tmp: Vec<K> = self.v.iter().map(|e| e.clone() * a.clone()).collect();
        self.v = tmp;
    }
    fn add(&mut self, v: Vector<K>) -> Vector<K> {
        Vector {
            v: self.v.iter().zip(v.v.iter()).map(|(a, b)| a.clone() + b.clone()).collect(),
        }
    }
    fn sub(&mut self, v: Vector<K>) -> Vector<K> {
        Vector {
            v: self.v.iter().zip(v.v.iter()).map(|(a, b)| a.clone() - b.clone()).collect(),
        }
    }
    fn scale(&mut self, a: K) -> Vector<K> {
        Vector {
            v: self.v.iter().map(|e| e.clone() * a.clone()).collect(),
        }   
    }
}