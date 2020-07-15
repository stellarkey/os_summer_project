fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut ret: Vec<Vec<i32>> = Vec::<Vec<i32>>::new();
    
    for i in 0..num_rows{
        let mut t : Vec<i32> = vec![1 as i32 ; (i + 1) as usize];
        ret.push(t);
        println!("{:?}", ret);
    }
    for i in 0..num_rows{
        for j in 1..i{
            ret[i as usize][j as usize] = ret[(i-1) as usize][(j-1) as usize] + ret[(i-1) as usize][j as usize];
        }
        println!("{:?}", ret);
    }
    ret
}

fn main() {
    println!("{:?}", generate(5));
}
