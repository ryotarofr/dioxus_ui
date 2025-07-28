use std::collections::HashMap;
use super::get_mapped_object::get_mapped_object;

pub type CalcableObj = HashMap<String, f64>;

pub enum RhsValue {
    Object(CalcableObj),
    Number(f64),
}

fn get_calc<F>(calc: F) -> impl Fn(CalcableObj, RhsValue) -> CalcableObj
where
    F: Fn(f64, f64) -> f64 + Copy,
{
    move |lhs: CalcableObj, rhs: RhsValue| {
        get_mapped_object(lhs, |(key, l_val), _index| {
            let r_val = match &rhs {
                RhsValue::Object(obj) => obj.get(key).copied(),
                RhsValue::Number(num) => Some(*num),
            };
            
            match r_val {
                Some(r) => calc(*l_val, r),
                None => *l_val,
            }
        })
    }
}

fn get_calc_with_precision<F>(calc: F) -> impl Fn(CalcableObj, Option<u32>) -> CalcableObj
where
    F: Fn(f64) -> f64 + Copy,
{
    move |lhs: CalcableObj, decimal_places: Option<u32>| {
        let digit_adjuster = match decimal_places {
            Some(places) => 10_f64.powi(places as i32),
            None => 1.0,
        };
        
        get_mapped_object(lhs, |(_key, l_val), _index| {
            calc(*l_val * digit_adjuster) / digit_adjuster
        })
    }
}

pub struct Calc;

impl Calc {
    pub fn get<F>(calc: F) -> impl Fn(CalcableObj, RhsValue) -> CalcableObj
    where
        F: Fn(f64, f64) -> f64 + Copy,
    {
        get_calc(calc)
    }

    pub fn plus(lhs: CalcableObj, rhs: RhsValue) -> CalcableObj {
        get_calc(|l, r| l + r)(lhs, rhs)
    }

    pub fn minus(lhs: CalcableObj, rhs: RhsValue) -> CalcableObj {
        get_calc(|l, r| l - r)(lhs, rhs)
    }

    pub fn times(lhs: CalcableObj, rhs: RhsValue) -> CalcableObj {
        get_calc(|l, r| l * r)(lhs, rhs)
    }

    pub fn div(lhs: CalcableObj, rhs: RhsValue) -> CalcableObj {
        get_calc(|l, r| l / r)(lhs, rhs)
    }

    pub fn max(lhs: CalcableObj, rhs: RhsValue) -> CalcableObj {
        get_calc(|l, r| l.max(r))(lhs, rhs)
    }

    pub fn min(lhs: CalcableObj, rhs: RhsValue) -> CalcableObj {
        get_calc(|l, r| l.min(r))(lhs, rhs)
    }

    pub fn floor(lhs: CalcableObj, decimal_places: Option<u32>) -> CalcableObj {
        get_calc_with_precision(|it| it.floor())(lhs, decimal_places)
    }

    pub fn round(lhs: CalcableObj, decimal_places: Option<u32>) -> CalcableObj {
        get_calc_with_precision(|it| it.round())(lhs, decimal_places)
    }

    pub fn ceil(lhs: CalcableObj, decimal_places: Option<u32>) -> CalcableObj {
        get_calc_with_precision(|it| it.ceil())(lhs, decimal_places)
    }

    pub fn positive_diff(lhs: CalcableObj, rhs: RhsValue) -> CalcableObj {
        get_calc(|l, r| (l.max(r) - l.min(r)).abs())(lhs, rhs)
    }

    pub fn opposite(lhs: CalcableObj) -> CalcableObj {
        Self::times(lhs, RhsValue::Number(-1.0))
    }

    pub fn or_else<F>(condition: F) -> impl Fn(CalcableObj, RhsValue) -> CalcableObj
    where
        F: Fn(f64) -> bool + Copy,
    {
        move |lhs: CalcableObj, rhs: RhsValue| {
            get_calc(|l, r| if condition(l) { l } else { r })(lhs, rhs)
        }
    }
}
