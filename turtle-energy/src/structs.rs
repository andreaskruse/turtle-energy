pub type CurvePoint = (f64, f64, f64);
pub type Delivery = (f64, f64, f64);
pub type Result = (f64, f64, u32);

pub struct Price {
    pub start: f64,
    pub end: f64,
    pub value: f64,
    pub error_code: u32,
}
