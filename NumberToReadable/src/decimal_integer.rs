pub mod decimal_integer {
    use crate::triple::triple::IntegerTriple;

    pub struct DecimalInteger {
        value: Vec<IntegerTriple>,
    }

    impl DecimalInteger {
        pub fn to_readable(self) -> String {
            const triple_literals: [&str; 11] = [
                "",
                "Thousand",
                "Million",
                "Billion",
                "Trillion",
                "Quadrillion",
                "Quintillion",
                "Sextillion",
                "Septillion",
                "Octillion",
                "Nonillion",
            ];
            self.value
                .iter()
                .rev()
                .enumerate()
                .rev()
                .map(|(i, x)| x.to_readable() + &triple_literals[i])
                .collect::<Vec<String>>()
                .join(" ")
        }
    }

    impl From<DecimalInteger> for i32 {
        fn from(x: DecimalInteger) -> Self {
            let mut res = 0;
            let mut power = 1;
            for x in x.value.iter().rev() {
                res += i32::from(x) * power;
                power = power * 1000;
            }
            res
        }
    }

    impl From<i32> for DecimalInteger {
        fn from(x: i32) -> Self {
            let mut value = Vec::new();
            let mut residual = x.abs();
            let is_negative = x < 0;
            // uses greater than equal because x == 0  base case
            while residual >= 0 {
                let current_thousand = (residual % 1000) as u32;
                value.push(IntegerTriple::from(current_thousand));
                if residual < 1000 {
                    // last iteration, make negative
                    let len = value.len();
                    value[len - 1].is_negative = is_negative;
                    break;
                }
                residual = residual / 1000;
            }
            println!("created");
            println!(
                "{:?}",
                value.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",")
            );
            DecimalInteger { value }
        }
    }

    impl std::fmt::Display for DecimalInteger {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let t: Vec<String> = self.value.iter().map(|x| x.to_string()).collect();
            write!(f, "{}", t.join("-"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::decimal_integer::DecimalInteger;

    #[test]
    fn test_string_conversion() {
        for x in -3000..=3000 {
            let dec = DecimalInteger::from(x);
            println!("{} {}", dec, x);
            assert_eq!(dec.to_string(), x.to_string());
        }
    }

    #[test]
    fn test_to_readable() {
        println!("");
        println!("{}", DecimalInteger::from(123456789).to_readable());
    }
}
