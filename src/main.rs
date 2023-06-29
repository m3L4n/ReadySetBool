mod simple_operations {
    pub mod adder; // Importe le module adder depuis le fichier adder.rs
    pub mod gray_code;
    pub mod multiplier; // Importe le module adder depuis le fichier adder.rs
}
mod truth_operations {
    pub mod boolean_evaluation;
    pub mod truth_table;
}
mod form {
    pub mod conjonctive_normal_form;
    pub mod negation_normal_form;
    pub mod sat;
}
mod set {
    pub mod powerset;
    pub mod set_evaluation;
}
mod curve {
    pub mod inverse_map;
    pub mod map;
}

use curve::inverse_map::test_inverse_map;
use curve::map::test_map;
use form::conjonctive_normal_form::test_cnf;
use form::negation_normal_form::test_nnf;
use form::sat::test_sat;
use set::powerset::test_powerset;
use set::set_evaluation::test_set_eval;
use simple_operations::adder::test_adder;
use simple_operations::gray_code::test_gray_code;
use simple_operations::multiplier::test_multiplier;
use truth_operations::boolean_evaluation::test_boolean;
use truth_operations::truth_table::test_truthtable;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let props = args[1].clone();
        if props == "adder" {
            test_adder();
        } else if props == "multiplier" {
            test_multiplier();
        } else if props == "gray_code" {
            test_gray_code();
        } else if props == "boolean" {
            test_boolean();
        } else if props == "truth_table" {
            test_truthtable();
        } else if props == "nnf" {
            test_nnf();
        } else if props == "cnf" {
            test_cnf();
        } else if props == "sat" {
            test_sat();
        } else if props == "powerset" {
            test_powerset();
        } else if props == "set" {
            test_set_eval();
        } else if props == "map" {
            test_map();
        } else if props == "inverse" {
            test_inverse_map();
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::curve::inverse_map::test_inverse_map;
    use crate::curve::map::test_map;
    use crate::set::powerset::test_powerset;
    use crate::set::set_evaluation::test_set_eval;

    use super::test_adder;
    use super::test_boolean;
    use super::test_cnf;
    use super::test_gray_code;
    use super::test_multiplier;
    use super::test_nnf;
    use super::test_sat;
    use super::test_truthtable;

    #[test]
    fn tests_adder() {
        test_adder();
    }

    #[test]
    fn tests_multiplier() {
        test_multiplier();
    }
    #[test]
    fn tests_gray_code() {
        test_gray_code();
    }
    #[test]
    fn tests_boolean() {
        test_boolean();
    }
    #[test]
    fn tests_truth_table() {
        test_truthtable();
    }
    #[test]
    fn tests_nnf() {
        test_nnf();
    }
    #[test]
    fn tests_cnf() {
        test_cnf();
    }
    #[test]
    fn tests_sat() {
        test_sat();
    }
    #[test]
    fn tests_powerset() {
        test_powerset();
    }
    #[test]
    fn tests_set_eval() {
        test_set_eval();
    }
    #[test]
    fn tests_map() {
        test_map();
    }
    #[test]
    fn tests_inverse_map() {
        test_inverse_map();
    }
}
