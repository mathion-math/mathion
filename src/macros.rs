use crate::prelude::*;
use std::any::Any;

#[macro_export]
macro_rules! symbol {
	( $x:expr ) => {{
		Symbol::new($x, SymbolType::Variable, 1.0)
	}}
}

#[macro_export]
macro_rules! from_f64 {
	( $x:expr ) => {{
		Symbol::from_f64($x)
	}}
}

#[macro_export]
macro_rules! function {
	( $x:expr ) => {{
		$x.to_polynomial()
	}}
}

#[macro_export]
macro_rules! var {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec: Vec<(&'static str, f64)> = vec![];
            $(
                temp_vec.push($x);
            )*
            Variables::new(temp_vec)
        }
    };
}

#[macro_export]
macro_rules! matrix {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec: Vec<Vec<Polynomial>> = vec![];
            let mut max_columns = 0;
            $(
                if $x.len() > max_columns { max_columns = $x.len(); }
                temp_vec.push($x);
            )*
            Matrix::new(temp_vec.len(), max_columns, temp_vec)
        }
    };
}

#[macro_export]
macro_rules! row {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec: Vec<Polynomial> = vec![];
            $(
                if let Some(f) = (&$x as &Any).downcast_ref::<f32>() {
                    temp_vec.push(Polynomial::from_f64((*f).into()));
                } else if let Some(f) = (&$x as &Any).downcast_ref::<f64>() {
                    temp_vec.push(Polynomial::from_f64(*f));
                } else if let Some(f) = (&$x as &Any).downcast_ref::<Symbol>() {
                    temp_vec.push(f.to_polynomial());
                } else if let Some(f) = (&$x as &Any).downcast_ref::<Monomial>() {
                    temp_vec.push(f.to_polynomial());
                } else if let Some(f) = (&$x as &Any).downcast_ref::<Polynomial>() {
                    temp_vec.push(*f);
                }
            )*
            temp_vec
        }
    };
}