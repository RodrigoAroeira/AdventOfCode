use regex_lite::Regex;

type NumType = f64;
type NumPair = (NumType, NumType);

const WEIGHT_A: NumType = 3.0;
const WEIGHT_B: NumType = 4.0;

struct Machine {
    a: NumPair,
    b: NumPair,
    prize: NumPair,
}

impl Machine {
    fn lowest_price(&self) -> NumType {
        // System of equations:
        // a.0 + b.0 = prize.0
        // Solve with linear algebra
        // Ax=B => X = (A-1)B

        let a = nalgebra::Matrix::new(self.a.0, self.b.0, self.a.1, self.b.1);
        if a.determinant() == 0 {
            return 0.0;
        }

        let b = nalgebra::Vector2::new(self.prize.0, self.prize.1);

        let Some(inv) = a.try_inverse() else {
            panic!("Matrix was not invertible!")
        };

        let x = inv * b;

        todo!("Implement Machine::lowest_price method")
    }
}

fn main() {
    println!("Hello, world!");
    let button_re = Regex::new(r"(Button [AB]: X\+(\d+), Y\+(\d+))").expect("Regex must work.");
    let prize_re = Regex::new(r"(Prize: X=(\d+), Y=(\d+))").expect("Regex must always work");
}
