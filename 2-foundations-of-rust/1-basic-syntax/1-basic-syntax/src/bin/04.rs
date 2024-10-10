fn main() {
    let mut mult: [i32; 4] = [0, 0, 0, 0];
    let mut i = 0;
    for n in [10, 20, 30, 40] {
       if n < 25 {
        mult[i] = n * 4;
        i +=1;
       }else {
        mult[i] = n * 5;
        i +=1;
       }
    }
   println!("{:?}", mult);
}
