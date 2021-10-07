use wasmi::ValueType;

pub trait HasGuestType {
    fn valuetype() -> ValueType;
}

impl HasGuestType for i64 {
    fn valuetype() -> ValueType {
        ValueType::I64
    }
}

impl HasGuestType for usize {
    fn valuetype() -> ValueType {
        ValueType::I64
    }
}
