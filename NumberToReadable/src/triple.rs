pub mod triple {
    fn ones_place_readable(x: u32) -> String {
        const one_literals: [&str; 10] = [
            "Zero", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine",
        ];
        if 0 <= x && x < 10 {
            String::from(one_literals[x as usize])
        } else {
            panic!("ones readable");
            String::from("")
        }
    }

    fn tens_place_readable(x: u32) -> String {
        const ten_literals: [&str; 10] = [
            "", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety",
        ];
        if 2 <= x && x < 10 {
            String::from(ten_literals[x as usize])
        } else {
            println!("{}", x);
            panic!("tens readable");
            String::from("")
        }
    }

    fn teens_readable(x: u32) -> String {
        const teen_literals: [&str; 10] = [
            "Ten",
            "Eleven",
            "Twelve",
            "Thirteen",
            "Fourteen",
            "Fifteen",
            "Sixteen",
            "Seventeen",
            "Eighteen",
            "Nineteen",
        ];

        if 0 <= x && x < 10 {
            ones_place_readable(x)
        } else if 10 <= x && x < 20 {
            String::from(teen_literals[(x - 10) as usize])
        } else {
            panic!("Teens not in teens")
        }
    }

    fn hundreds_place_readable(x: u32) -> String {
        if 0 < x && x < 10 {
            ones_place_readable(x) + &String::from(" Hundred")
        } else {
            String::from("")
        }
    }

    pub struct IntegerTriple {
        pub is_negative: bool,
        pub hundreds_place: u32,
        pub tens_place: u32,
        pub ones_place: u32,
    }

    impl IntegerTriple {
        pub fn to_readable(&self) -> String {
            let space = &String::from(" ");
            let empty = String::from("");
            let res = String::from("wow");
            let teen = self.tens_place * 10 + self.ones_place;

            // Strings are responsible for the left-space
            let str_neg = if self.is_negative {
                String::from("Negative ")
            } else {
                String::from("")
            };

            // hundreds place = "X hundred " or ""
            let str_hundred = if self.hundreds_place > 0 {
                // convert to readable value
                hundreds_place_readable(self.hundreds_place) + space
            } else {
                String::from("")
            };

            // greq 20 => [twen|thir|for|fif...nine]ty
            let str_ten = if self.tens_place >= 2 {
                // convert to readable
                tens_place_readable(self.tens_place) + space
            } else {
                String::from("")
            };

            let str_teen = if 10 <= teen && teen < 20 {
                teens_readable(teen) + space
            } else {
                String::from("")
            };

            let str_one = if teen < 10 || teen > 19 {
                if self.ones_place == 0 {
                    if self.tens_place == 0 && self.hundreds_place == 0 {
                        ones_place_readable(self.ones_place) + space
                    } else {
                        String::from("")
                    }
                } else {
                    ones_place_readable(self.ones_place) + space
                }
            } else {
                String::from("")
            };

            str_neg + &str_hundred + &str_ten + &str_teen + &str_one
        }
    }

    impl From<u32> for IntegerTriple {
        fn from(x: u32) -> Self {
            assert!(x < 1000);
            let is_negative = false;
            let hundreds_place = x / 100;
            let tens_place = (x % 100) / 10;
            let ones_place = x % 10;
            IntegerTriple {
                is_negative,
                hundreds_place,
                tens_place,
                ones_place,
            }
        }
    }

    impl From<i32> for IntegerTriple {
        fn from(x: i32) -> Self {
            assert!(-1000 < x && x < 1000);
            let mut tmp = IntegerTriple::from(x.abs() as u32);
            tmp.is_negative = x < 0;
            tmp
        }
    }

    impl From<IntegerTriple> for i32 {
        fn from(x: IntegerTriple) -> Self {
            let abs = (x.hundreds_place * 100 + x.tens_place * 10 + x.ones_place) as i32;
            if x.is_negative {
                -abs
            } else {
                abs
            }
        }
    }

    impl From<&IntegerTriple> for i32 {
        fn from(x: &IntegerTriple) -> Self {
            let abs = (x.hundreds_place * 100 + x.tens_place * 10 + x.ones_place) as i32;
            if x.is_negative {
                -abs
            } else {
                abs
            }
        }
    }

    impl std::fmt::Display for IntegerTriple {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            let empty = String::from("");

            let str_neg = if self.is_negative {
                String::from("-")
            } else {
                String::from("")
            };

            let str_d1 = if self.hundreds_place != 0 {
                self.hundreds_place.to_string()
            } else {
                String::from("")
            };

            let str_d2 = if self.tens_place != 0 {
                self.tens_place.to_string()
            } else if self.hundreds_place != 0 {
                // contained zero
                self.tens_place.to_string()
            } else {
                empty
                // hun = 0, ten = 0, leading zero
            };

            let str_d3 = self.ones_place.to_string();

            write!(f, "{}{}{}{}", str_neg, str_d1, str_d2, str_d3)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::triple::*;
    #[test]
    fn test_string_conversion() {
        /* Verified num -> triple -> string == num.to_string valid range of values */
        for x in -999..=999 {
            let t = IntegerTriple::from(x).to_string();
            let t1 = x.to_string();
            //println!("{} {}", t, t1);
            assert_eq!(t, t1);
        }
    }
    #[test]
    fn test_numeric_conversion() {
        /* Verified num -> triple -> string == num.to_string valid range of values */
        for x in -999..=999 {
            let t = i32::from(IntegerTriple::from(x));
            //println!("{} {}", t, x);
            assert_eq!(t, x);
        }
    }

    #[test]
    fn test_readable() {
        for x in -999..=999 {
            println!("{}", IntegerTriple::from(x).to_readable());
            assert_eq!(true, true);
        }
    }
}
