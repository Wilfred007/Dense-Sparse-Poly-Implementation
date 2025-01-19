// struct SparsePoly {
//     coeffs: Vec<(f64, f64)>,
//     degree: f64
// }

// impl SparsePoly {
//     fn new(coeffs: Vec<(f64, f64)>) -> SparsePoly {
//         let degree = *coeffs.iter().map(|(x, _)| x).max().unwrap();
//         SparsePoly{
//                 coeffs: coeffs,
//                 degree: degree
            
//           }
//     }
// }




// fn main() {
//     println!("Hello, world!");
// }


#[derive(Debug)]
struct SparsePoly {
    coeffs: Vec<(usize, f64)>, // Term coefficients and degrees
    degree: usize,            // Highest degree in the polynomial
}

impl SparsePoly {
    fn new(mut coeffs: Vec<(usize, f64)>) -> SparsePoly {
        coeffs.sort_by(|a, b| a.0.cmp(&b.0)); // Sort terms by degree
        let degree = coeffs.iter().map(|(x, _)| *x).max().unwrap_or(0); // Handle empty vector safely
        SparsePoly {
            coeffs,
            degree,
        }
    }
}

fn main() {
    let poly = SparsePoly::new(vec![(0, 3.0), (2, 5.0), (3, -1.0)]);
    println!("Sparse Polynomial: {:?}", poly);
    println!("Highest Degree: {}", poly.degree);
}
