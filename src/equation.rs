// This is the tree-representation
// This model makes unsimplified statements unrepresentable
// Vals are nodes except groups
// pub enum Operators<'a> {
//     Plus(&'a Val<'a>, &'a Val<'a>),
//     Minus(&'a Val<'a>, &'a Val<'a>),
//     Div(&'a Val<'a>, &'a Val<'a>),
//     Mul(&'a Val<'a>, &'a Val<'a>)
// }

// pub enum Val<'a> {
//     Group(Container<'a>),
//     Num(i32),
//     Var(String),
// }

// pub enum Container<'a> {
//     Paren(Operators<'a>),
//     Log(Operators<'a>),
//     Sqrt(Operators<'a>),
//     Abs(Operators<'a>),
// }

// pub struct Equation<'a> {
//     a: &'a Operators<'a>, 
//     b: &'a Operators<'a>
// }


// This is the prefix notation approach(might be easier for solving equations, how do we represent parenthesis though?)
// 14 * (1 + 4) => 2,1;*,+;14,1,4
// (14 * 1) + 4 => 1,2;*+;14,1,4
// This allows us to represents parentheis by ordering things
// How do we represents grouping ops that do something? ie. logs, sqrt, abs, etc.
// 5(6 + sqrt(4x)) => 4321;*+sqrt*;5,6,4,x
// 5(sqrt(6+4x)) => 4321;*+sqrt*;5,6,4,x
// There could be a bug with these vectors being different lengths
pub enum Op {
    Plus,
    Minus,
    Div,
    Mul
}

pub enum Val {
    Num(i32),
    Var(String)
}

pub struct Eval {
    ops_order: Vec<i32>, // represents the order in which the operators should be applied
    ops: Vec<Op>,
    vals: Vec<Val>
}

pub struct Equality {
    side_a: Eval,
    side_b: Eval
}

impl Eval {
    pub fn sum_of(&self) {
        let mut sums: Vec<i32> = vec![]; // each value represents the coefficient of 1, z, y, x ...
        for i in 0..self.ops_order.len() {
            let order_i = self.ops_order.binary_search(&(i as i32)).expect("vec to contain a value equal to each of its indexes");
            let op = &self.ops[order_i];
            let val = &self.vals[order_i];
            match val {
                Val::Num(n) => {
                    sums[0] += n;
                },
                Val::Var(name) => {
                    name.chars()
                }
             }
        }
    }
}

impl Equality {
    pub fn from_eval(a: Eval, b: Eval) -> Equality {
        assert_eq!(a.ops.len() - 1, a.vals.len());
        assert_eq!(b.ops.len() - 1, b.vals.len());
        Equality {side_a: a, side_b: b}
    }

    pub fn is_equal(&self) {
        let sum_a = self.side_a.sum_of();
        let sum_b = self.side_b.sum_of();        

    }
}