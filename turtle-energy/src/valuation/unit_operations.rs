use std::ops::{Mul, Add, Sub, Div};

pub enum ReportingUnit {
    DKK,
    EUR,
    ORIG,
}

pub struct Value {
    pub value: f64,
    pub numerator_unit_id: u32,
    pub denominator_unit_id: u32,
}

impl Mul<Value> for Value {
    type Output = Option<Value>;

    fn mul(self, rhs: Value) -> Self::Output {
        if self.denominator_unit_id != rhs.denominator_unit_id
            || self.numerator_unit_id != rhs.numerator_unit_id
        {
            return None;
        }

        Some(Value {
            value: self.value * rhs.value,
            numerator_unit_id: self.numerator_unit_id,
            denominator_unit_id: self.denominator_unit_id,
        })
    }
}


impl Add<Value> for Value {
    type Output = Option<Value>;

    fn add(self, rhs: Value) -> Self::Output {
        
        if self.denominator_unit_id != rhs.denominator_unit_id
            || self.numerator_unit_id != rhs.numerator_unit_id
        {
            return None;
        }

        Some(Value {
            value: self.value + rhs.value,
            numerator_unit_id: self.numerator_unit_id,
            denominator_unit_id: self.denominator_unit_id
        })
    }
}