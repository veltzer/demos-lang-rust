fn main() {
  let vector = [1, 2, 3];
  let result = vector.iter().map(|x| x * 2).collect::<Vec<i32>>();
  // another way to write the above statement 
  let result2: Vec<i32> = vector.iter().map(|x| x * 2).collect();
  println!("After mapping: {:?}", result);
  println!("After mapping: {:?}", result2);
}
