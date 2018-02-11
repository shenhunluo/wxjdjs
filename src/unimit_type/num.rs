use std::ops::Add;
struct Num {
    int:Vec<i32>,
    dec:Vec<i32>,
    z:bool
}
impl Add for Num {
    type Output = Num;
    fn add(self, rhs: Num) -> Self::Output {
        if self.z == rhs.z {
            if self.dec.len() > rhs.dec.len() {
                part_add(self.dec,rhs.dec);
            }
        }
        Num{
            int: Vec::new(),
            dec: Vec::new(),
            z: false,
        }
    }
}
fn part_add(mut b:Vec<i32>, mut l:Vec<i32>) ->(bool, Vec<i32>){
    let mut t = false;
    let mut r = vec![0];
    r.pop();
    for i in 0..l.len(){
        let a:i32 = b.pop()+l.pop();
        r.push(a);
        if a>=1000000000 {
            t = 1;
        }
    }
    (t,r)
}