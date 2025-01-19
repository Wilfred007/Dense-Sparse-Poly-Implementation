struct DensePoly {
    coeffs: Vec<f64>
  }
  
  
  impl DensePoly {
      fn new(coeffs: Vec<f64>) -> Self {
        DensePoly{
          coeffs: coeffs
        }
      }
  
      fn degree(&self)-> usize {
        self.coeffs.len()-1
      }
   
      // fn evaluate(&self, x:f64) -> f64 {
      //     let n = self.coeffs.len();
      //     let mut result = 0.0;
      //     for i in 0..n{
      //       result += self.coeffs[i] * x.powf(i as f64);
      //     }
      //     result
      // }
  
      fn evaluate (&self, x:f64) -> f64 {
        self.coeffs.iter().enumerate().map(|(i,c)| c * x.powf(i as f64)).sum()
      }
  }
  
  
  fn main() {
      println!("Hello, world!");
      let test_poly1 = DensePoly::new(vec![1.0,2.0]);
      print!("The value of f(2)={}", test_poly1.evaluate(2.0));
  }
  