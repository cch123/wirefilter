extern crate wirefilter;

use std::env::args;
use std::marker::PhantomData;

use wirefilter::{CombiningOp, ComparisonOp, Context, Field, RhsValue, Type, UnaryOp};

#[derive(Clone, Copy)]
struct AstContext<'i>(PhantomData<&'i str>);

impl<'i> AstContext<'i> {
    pub fn new(_input: &'i str) -> Self {
        AstContext(PhantomData)
    }
}

#[derive(Debug)]
enum Filter<'i> {
    Compare(Field<'i>, ComparisonOp, RhsValue),
    Combine(Box<Filter<'i>>, CombiningOp, Box<Filter<'i>>),
    Unary(UnaryOp, Box<Filter<'i>>),
}

impl<'i> Context<'i> for AstContext<'i> {
    type LhsValue = Field<'i>;
    type Filter = Filter<'i>;

    fn get_field(self, path: &str) -> Option<Field> {
        Some(Field::new(path))
    }

    fn compare(self, lhs: Field, op: ComparisonOp, rhs: RhsValue) -> Result<Filter, Type> {
        Ok(Filter::Compare(lhs, op, rhs))
    }

    fn combine(self, lhs: Filter<'i>, op: CombiningOp, rhs: Filter<'i>) -> Filter<'i> {
        Filter::Combine(Box::new(lhs), op, Box::new(rhs))
    }

    fn unary(self, op: UnaryOp, arg: Filter) -> Filter {
        Filter::Unary(op, Box::new(arg))
    }
}

fn main() {
    let s = args()
        .nth(1)
        .expect("Expected an input as a command-line argument");

    let context = AstContext::new(&s);

    println!("{:#?}", wirefilter::filter(&s, context));
}
