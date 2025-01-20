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

      fn interpolate(points: Vec<(f64, f64)>) -> Self{
         if points.is_empty(){
            DensePoly::new(vec![0.0]);
         }

         let n = points.len();
         let mut result = vec![0.0; n];
               for i in 0..n{
                let (xi , yi) = points[i];

                let mut basis = vec![1.0];
                let mut factor = 1.0;
                //this part is to evaluate the the other points except the i that is why we are checking where i != j
                for j in 0..n{
                    if i != j {
                        //this is where we are getting the other x points
                        let (xj, _) = points[j];
                        //this is where we are getting the denominator at the point i //(xi - xj)
                          factor *= xi - xj;
                                //5x**2 + 3x + 1= 0
                                //this is where we are multiplying all the top and gettting a new basis
                        let mut new_basis = vec![0.0; basis.len() + 1];
                        //this part we are getting the constant term and the x term
                        for k in 0..basis.len(){
                            new_basis[k + 1] += basis[k]; //x
                            new_basis[k] -= basis[k] * xj;
                        }
                        basis = new_basis;
                    }
                }
                let scale = yi / factor;
                for k in 0..basis.len(){
                    result[k] += scale * basis[k];
                }

                for c in result.iter_mut(){
                    if c.abs() < 1e-10{
                        *c = 0.0;
                    }
                }

                while result.len() > 1 && result.last().map_or( false, |&x| x == 0.0) {
                    result.pop();                }
               }


        DensePoly { coeffs: result }
      }

  }
  
  
  fn main() {
    //   println!("Hello, world!");
      let test_poly1 = DensePoly::new(vec![1.0,2.0]);
    //   print!("The value of f(2)={}", test_poly1.evaluate(2.0));
      let point = vec![(1.0,2.0), (2.0,4.0), (4.0,8.0)];
      print!(" the poly is : {:?}", DensePoly::interpolate(point).coeffs);
  }
  