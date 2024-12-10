use std::any::Any;
use std::cmp::PartialEq;
use std::ops::{Add, Deref};
use num_traits::Float;
use num_traits::float::FloatCore;
use crate::core::omnia_types::omnia_types::{OmniaByte, OmniaChar, OmniaDecimal, OmniaInt, OmniaLong, OmniaString, OmniaUByte, OmniaUInt, OmniaULong, OmniaValue};
use crate::core::omnia_types::omnia_types::Type;

trait Node {

}

trait Expression : Node {
    fn calc(&mut self) -> Box<dyn OmniaValue>;
}
trait Statement : Node {
    fn execute(&mut self);
}
#[derive(PartialEq, Debug)]
pub enum Operator{
    PLUS,
    DIVIDE,
    MULTIPLY,
    MINUS,
    REM,

    INC,
    DEC
}
pub struct BinaryExpression {
    left: &'static mut dyn Expression,
    right: &'static mut dyn Expression,
    operator: Operator
}

impl Node for BinaryExpression {}



impl BinaryExpression {
    fn new(left: impl Expression, right: impl Expression, operator: Operator) -> BinaryExpression {
        Self {
            left,
            right,
            operator
        }
    }
    //SECTION:: Ints start

    //SECTION:: Int preparing start

    fn prepare_calc_l_int(&self, l: &OmniaInt) -> Box<dyn OmniaValue> {
        let mut r = self.right.calc();
        let binding = r.get_type();
        let r_type = binding.get_right();
        match r_type {
            Type::BYTE => {
                self.prepare_calc_int(l, &OmniaInt::get_from(r.downcast_ref::<OmniaByte>().unwrap().get_as_int32()))
            }
            Type::INT => {
                self.prepare_calc_int(l, r.downcast_ref::<OmniaInt>().unwrap())
            }
            Type::LONG => {
                self.prepare_calc_int(l, &OmniaInt::get_from(r.downcast_ref::<OmniaLong>().unwrap().get_value_as::<i64>()))
            }
            Type::DECIMAL => {
                self.prepare_calc_int(l, &OmniaInt::get_from(r.downcast_ref::<OmniaDecimal>().unwrap().get_value_as::<f64>()))
            }
            Type::STRING => {
                self.append_int_to_string(l, r.downcast_ref::<OmniaString>().unwrap())
            }
            _ => {
                panic!("Unexpected or currently unsupported type {:?}", r_type)
            }
        }
    }
    fn prepare_calc_int(&self, l: &OmniaInt, r: &OmniaInt) -> Box<dyn OmniaValue> {
        match self.operator {
            Operator::PLUS => {
                Box::new(self.add_int(l, r))
            }
            Operator::MINUS => {
                Box::new(self.sub_int(l, r))
            }
            Operator::MULTIPLY => {
                Box::new(self.mul_int(l, r))
            }
            Operator::DIVIDE => {
                Box::new(self.div_int(l, r))
            }
            Operator::REM => {
                Box::new(self.rem_int(l, r))
            }
            _ => {
                panic!("Unexpected operator")
            }
        }
    }
    //SECTION:: Int preparing end

    //SECTION:: Int arithmetics start

    fn add_int(&self, l: &OmniaInt, r: &OmniaInt) -> OmniaInt {
        if let Some(calculated) = l.get_value_as::<i32>().checked_add(r.get_value_as::<i32>()) {
            OmniaInt::get_from(calculated)
        }else {
            panic!("Got value less or more than int bounds while adding")
        }
    }
    fn sub_int(&self, l: &OmniaInt, r: &OmniaInt) -> OmniaInt {
        if let Some(calculated) = l.get_value_as::<i32>().checked_sub(r.get_value_as::<i32>()) {
            OmniaInt::get_from(calculated)
        }else {
            panic!("Got value less or more than int bounds while subtracting")
        }
    }
    fn mul_int(&self, l: & OmniaInt, r: & OmniaInt) -> OmniaInt {
        if let Some(calculated) = l.get_value_as::<i32>().checked_mul(r.get_value_as::<i32>()) {
            OmniaInt::get_from(calculated)
        }else {
            panic!("Got value less or more than int bounds while multiplying")
        }
    }
    fn div_int(&self, l: & OmniaInt, r: & OmniaInt) -> OmniaInt {
        if let Some(calculated) = l.get_value_as::<i32>().checked_div(r.get_value_as::<i32>()) {
            OmniaInt::get_from(calculated)
        }else {
            panic!("Got value less or more than int bounds while dividing (or attempted to divide by zero)")
        }
    }
    fn rem_int(&self, l: & OmniaInt, r: & OmniaInt) -> OmniaInt {
        if let Some(calculated) = l.get_value_as::<i32>().checked_rem(r.get_value_as::<i32>()) {
            OmniaInt::get_from(calculated)
        }else {
            panic!("Got value less or more than int bounds while remaindering")
        }
    }
    //SECTION:: Int arithmetics end
    fn append_int_to_string(&self, l: & OmniaInt, r: &OmniaString) -> Box<OmniaString> {
        if self.operator != Operator::PLUS {
            panic!("Only can append(+) value to string")
        }
        let mut val = r.get_as_string();
        val.push_str(&l.get_as_string());
        Box::new(OmniaString::get_from(val))
    }
    //SECTION:: Ints end

    //SECTION:: Bytes start

    //SECTION:: Byte preparing start

    fn prepare_calc_l_byte(&mut self, l: &OmniaByte) -> Box<dyn OmniaValue> {
        let mut r = self.right.calc();
        let binding = r.get_type();
        let r_type = binding.get_right();
        match r_type {
            Type::BYTE => {
                self.prepare_calc_byte(l, r.downcast_ref::<OmniaByte>().unwrap())
            }
            Type::INT => {
                self.prepare_calc_byte(l, &OmniaByte::get_from::<i8>(r.downcast_ref::<OmniaInt>().unwrap().get_value_as::<i32>().try_into().unwrap()))
            }
            Type::LONG => {
                self.prepare_calc_byte(l, &OmniaByte::get_from::<i8>(r.downcast_ref::<OmniaLong>().unwrap().get_value_as::<i64>().try_into().unwrap()))
            }
            Type::DECIMAL => {
                self.prepare_calc_byte(l, &OmniaByte::get_from(r.downcast_ref::<OmniaDecimal>().unwrap().get_value_as::<f64>()))
            }
            Type::STRING => {
                self.append_byte_to_string(l, r.downcast_ref::<OmniaString>().unwrap())
            }

            _ => {
                panic!("Unexpected or currently unsupported type {:?}", r_type)
            }
        }
    }
    fn prepare_calc_byte(&self, l: &OmniaByte, r: &OmniaByte) -> Box<OmniaByte> {
        match self.operator {
            Operator::PLUS => {
                Box::new(self.add_byte(l, r))
            }
            Operator::MINUS => {
                Box::new(self.sub_byte(l, r))
            }
            Operator::MULTIPLY => {
                Box::new(self.mul_byte(l, r))
            }
            Operator::DIVIDE => {
                Box::new(self.div_byte(l, r))
            }
            Operator::REM => {
                Box::new(self.rem_byte(l, r))
            }
            _ => {
                panic!("Unexpected operator")
            }
        }
    }
    //SECTION:: Byte preparing end

    //SECTION:: Byte arithmetics start

    fn add_byte(&self, l: &OmniaByte, r: &OmniaByte) -> OmniaByte {
        if let Some(calculated) = l.get_value_as::<i8>().checked_add(r.get_value_as::<i8>()) {
            OmniaByte::get_from(calculated)
        }else {
            panic!("Got value less or more than byte bounds while adding")
        }
    }
    fn sub_byte(&self, l: &OmniaByte, r: &OmniaByte) -> OmniaByte {
        if let Some(calculated) = l.get_value_as::<i8>().checked_sub(r.get_value_as::<i8>()) {
            OmniaByte::get_from(calculated)
        }else {
            panic!("Got value less or more than byte bounds while subtracting")
        }
    }
    fn mul_byte(&self, l: & OmniaByte, r: & OmniaByte) -> OmniaByte {
        if let Some(calculated) = l.get_value_as::<i8>().checked_mul(r.get_value_as::<i8>()) {
            OmniaByte::get_from(calculated)
        }else {
            panic!("Got value less or more than byte bounds while multiplying")
        }
    }
    fn div_byte(&self, l: & OmniaByte, r: & OmniaByte) -> OmniaByte {
        if let Some(calculated) = l.get_value_as::<i8>().checked_div(r.get_value_as::<i8>()) {
            OmniaByte::get_from(calculated)
        }else {
            panic!("Got value less or more than byte bounds while dividing (or attempted to divide by zero)")
        }
    }
    fn rem_byte(&self, l: & OmniaByte, r: & OmniaByte) -> OmniaByte {
        if let Some(calculated) = l.get_value_as::<i8>().checked_rem(r.get_value_as::<i8>()) {
            OmniaByte::get_from(calculated)
        }else {
            panic!("Got value less or more than byte bounds while remaindering")
        }
    }
    //SECTION:: Byte arithmetics end
    fn append_byte_to_string(&self, l: &OmniaByte, r: &OmniaString) -> Box<OmniaString> {
        if self.operator != Operator::PLUS {
            panic!("Only can append(+) value to string")
        }
        let mut val = r.get_as_string();
        val.push_str(&l.get_as_string());
        Box::new(OmniaString::get_from(val))
    }
    //SECTION:: Bytes end

    //SECTION:: Longs start

    fn prepare_calc_l_long(&self, l: &OmniaLong) -> Box<dyn OmniaValue> {
        let mut r = self.right.calc();
        let binding = r.get_type();
        let r_type = binding.get_right();
        match r_type {
            Type::BYTE => {
                self.prepare_calc_long(l, &OmniaLong::get_from(r.downcast_ref::<OmniaByte>().unwrap().get_value_as()))
            }
            Type::INT => {
                self.prepare_calc_long(l, &OmniaLong::get_from(r.downcast_ref::<OmniaInt>().unwrap().get_value_as()))
            }
            Type::DECIMAL => {
                self.prepare_calc_long(l, &OmniaLong::get_from(r.downcast_ref::<OmniaDecimal>().unwrap().get_value_as::<f64>()))
            }
            Type::LONG => {
                self.prepare_calc_long(l, r.downcast_ref::<OmniaLong>().unwrap())
            }
            Type::STRING => {
                self.append_long_to_string(l, r.downcast_ref::<OmniaString>().unwrap())
            }
            _ => {
                panic!("Unexpected type {:?}", r_type)
            }
        }
    }
    fn prepare_calc_long(&self, l: &OmniaLong, r: &OmniaLong) -> Box<OmniaLong> {
        match self.operator {
            Operator::PLUS => {
                Box::new(self.add_long(l, r))
            }
            Operator::MINUS => {
                Box::new(self.sub_long(l, r))
            }
            Operator::MULTIPLY => {
                Box::new(self.mul_long(l, r))
            }
            Operator::DIVIDE => {
                Box::new(self.div_long(l, r))
            }
            Operator::REM => {
                Box::new(self.rem_long(l, r))
            }
            _ => {
                panic!("Unexpected operator")
            }
        }
    }
    fn add_long(&self, l: &OmniaLong, r: &OmniaLong) -> OmniaLong {
        if let Some(calculated) = l.get_value_as::<i64>().checked_add(r.get_value_as::<i64>()) {
            OmniaLong::get_from(calculated)
        } else {
            panic!("Got value less or more than long bounds while adding")
        }
    }
    fn sub_long(&self, l: &OmniaLong, r: &OmniaLong) -> OmniaLong {
        if let Some(calculated) = l.get_value_as::<i64>().checked_sub(r.get_value_as::<i64>()) {
            OmniaLong::get_from(calculated)
        } else {
            panic!("Got value less or more than long bounds while subtracting")
        }
    }
    fn mul_long(&self, l: &OmniaLong, r: &OmniaLong) -> OmniaLong {
        if let Some(calculated) = l.get_value_as::<i64>().checked_mul(r.get_value_as::<i64>()) {
            OmniaLong::get_from(calculated)
        } else {
            panic!("Got value less or more than long bounds while multiplying")
        }
    }
    fn div_long(&self, l: &OmniaLong, r: &OmniaLong) -> OmniaLong {
        if let Some(calculated) = l.get_value_as::<i64>().checked_div(r.get_value_as::<i64>()) {
            OmniaLong::get_from(calculated)
        } else {
            panic!("Got value less or more than long bounds while dividing(Or attempted to divide by zero)")
        }
    }
    fn rem_long(&self, l: &OmniaLong, r: &OmniaLong) -> OmniaLong {
        if let Some(calculated) = l.get_value_as::<i64>().checked_rem(r.get_value_as::<i64>()) {
            OmniaLong::get_from(calculated)
        } else {
            panic!("Got value less or more than long bounds while remaindering")
        }
    }
    fn append_long_to_string(&self, l: &OmniaLong, r: &OmniaString) -> Box<OmniaString> {
        if self.operator != Operator::PLUS {
            panic!("Can only append(+) value to string")
        }
        let mut val = r.get_as_string();
        val.push_str(&l.get_as_string());
        Box::new(OmniaString::get_from(val))
    }
    //SECTION:: Longs end



    //SECTION:: UInts start

    //SECTION:: UInt preparing start

    fn prepare_calc_l_uint(&self, l: &OmniaUInt) -> Box<dyn OmniaValue> {
        let mut r = self.right.calc();
        let binding = r.get_type();
        let r_type = binding.get_right();
        match r_type {
            Type::UBYTE => {
                self.prepare_calc_uint(l, &OmniaUInt::get_from(r.downcast_ref::<OmniaUByte>().unwrap().get_value_as::<u32>()))
            }
            Type::UINT => {
                self.prepare_calc_uint(l, r.downcast_ref::<OmniaUInt>().unwrap())
            }
            Type::ULONG => {
                self.prepare_calc_uint(l, &OmniaUInt::get_from(r.downcast_ref::<OmniaULong>().unwrap().get_value_as::<u64>()))
            }
            Type::STRING => {
                self.append_uint_to_string(l, r.downcast_ref::<OmniaString>().unwrap())
            }
            Type::DECIMAL => {
                self.prepare_calc_uint(l, &OmniaUInt::get_from(r.downcast_ref::<OmniaDecimal>().unwrap().get_value_as::<f64>()))
            }
            _ => {
                panic!("Unexpected or currently unsupported type {:?}", r_type)
            }
        }
    }
    fn prepare_calc_uint(&self, l: &OmniaUInt, r: &OmniaUInt) -> Box<dyn OmniaValue> {
        match self.operator {
            Operator::PLUS => {
                Box::new(self.add_uint(l, r))
            }
            Operator::MINUS => {
                Box::new(self.sub_uint(l, r))
            }
            Operator::MULTIPLY => {
                Box::new(self.mul_uint(l, r))
            }
            Operator::DIVIDE => {
                Box::new(self.div_uint(l, r))
            }
            Operator::REM => {
                Box::new(self.rem_uint(l, r))
            }
            _ => {
                panic!("Unexpected operator")
            }
        }
    }
    //SECTION:: UInt preparing end

    //SECTION:: UInt arithmetics start

    fn add_uint(&self, l: &OmniaUInt, r: &OmniaUInt) -> OmniaUInt {
        if let Some(calculated) = l.get_value_as::<u32>().checked_add(r.get_value_as::<u32>()) {
            OmniaUInt::get_from(calculated)
        }else {
            panic!("Got value less or more than int bounds while adding")
        }
    }
    fn sub_uint(&self, l: &OmniaUInt, r: &OmniaUInt) -> OmniaUInt {
        if let Some(calculated) = l.get_value_as::<u32>().checked_sub(r.get_value_as::<u32>()) {
            OmniaUInt::get_from(calculated)
        }else {
            panic!("Got value less or more than int bounds while subtracting")
        }
    }
    fn mul_uint(&self, l: & OmniaUInt, r: & OmniaUInt) -> OmniaUInt {
        if let Some(calculated) = l.get_value_as::<u32>().checked_mul(r.get_value_as::<u32>()) {
            OmniaUInt::get_from(calculated)
        }else {
            panic!("Got value less or more than int bounds while multiplying")
        }
    }
    fn div_uint(&self, l: & OmniaUInt, r: & OmniaUInt) -> OmniaUInt {
        if let Some(calculated) = l.get_value_as::<u32>().checked_div(r.get_value_as::<u32>()) {
            OmniaUInt::get_from(calculated)
        }else {
            panic!("Got value less or more than int bounds while dividing (or attempted to divide by zero)")
        }
    }
    fn rem_uint(&self, l: & OmniaUInt, r: & OmniaUInt) -> OmniaUInt {
        if let Some(calculated) = l.get_value_as::<u32>().checked_rem(r.get_value_as::<u32>()) {
            OmniaUInt::get_from(calculated)
        }else {
            panic!("Got value less or more than int bounds while remaindering")
        }
    }
    //SECTION:: UInt arithmetics end
    fn append_uint_to_string(&self, l: & OmniaUInt, r: &OmniaString) -> Box<OmniaString> {
        if self.operator != Operator::PLUS {
            panic!("Only can append(+) value to string")
        }
        let mut val = r.get_as_string();
        val.push_str(&l.get_as_string());
        Box::new(OmniaString::get_from(val))
    }
    //SECTION:: UInts end

    //SECTION:: UBytes start

    //SECTION:: UByte preparing start

    fn prepare_calc_l_ubyte(&mut self, l: &OmniaUByte) -> Box<dyn OmniaValue> {
        let mut r = self.right.calc();
        let binding = r.get_type();
        let r_type = binding.get_right();
        match r_type {
            Type::UBYTE => {
                self.prepare_calc_ubyte(l, r.downcast_ref::<OmniaUByte>().unwrap())
            }
            Type::UINT => {
                self.prepare_calc_ubyte(l, &OmniaUByte::get_from(r.downcast_ref::<OmniaUInt>().unwrap().get_as_int32()))
            }
            Type::ULONG => {
                self.prepare_calc_ubyte(l, &OmniaUByte::get_from(r.downcast_ref::<OmniaULong>().unwrap().get_value_as::<u64>()))
            }
            Type::STRING => {
                self.append_ubyte_to_string(l, r.downcast_ref::<OmniaString>().unwrap())
            }
            Type::DECIMAL => {
                self.prepare_calc_ubyte(l, &OmniaUByte::get_from(r.downcast_ref::<OmniaDecimal>().unwrap().get_value_as::<f64>()))
            }
            _ => {
                panic!("Unexpected or currently unsupported type {:?}", r_type)
            }
        }
    }
    fn prepare_calc_ubyte(&self, l: &OmniaUByte, r: &OmniaUByte) -> Box<OmniaUByte> {
        match self.operator {
            Operator::PLUS => {
                Box::new(self.add_ubyte(l, r))
            }
            Operator::MINUS => {
                Box::new(self.sub_ubyte(l, r))
            }
            Operator::MULTIPLY => {
                Box::new(self.mul_ubyte(l, r))
            }
            Operator::DIVIDE => {
                Box::new(self.div_ubyte(l, r))
            }
            Operator::REM => {
                Box::new(self.rem_ubyte(l, r))
            }
            _ => {
                panic!("Unexpected operator")
            }
        }
    }
    //SECTION:: UByte preparing end

    //SECTION:: UByte arithmetics start

    fn add_ubyte(&self, l: &OmniaUByte, r: &OmniaUByte) -> OmniaUByte {
        if let Some(calculated) = l.get_value_as::<u8>().checked_add(r.get_value_as::<u8>()) {
            OmniaUByte::get_from(calculated)
        }else {
            panic!("Got value less or more than byte bounds while adding")
        }
    }
    fn sub_ubyte(&self, l: &OmniaUByte, r: &OmniaUByte) -> OmniaUByte {
        if let Some(calculated) = l.get_value_as::<u8>().checked_sub(r.get_value_as::<u8>()) {
            OmniaUByte::get_from(calculated)
        }else {
            panic!("Got value less or more than byte bounds while subtracting")
        }
    }
    fn mul_ubyte(&self, l: & OmniaUByte, r: & OmniaUByte) -> OmniaUByte {
        if let Some(calculated) = l.get_value_as::<u8>().checked_mul(r.get_value_as::<u8>()) {
            OmniaUByte::get_from(calculated)
        }else {
            panic!("Got value less or more than byte bounds while multiplying")
        }
    }
    fn div_ubyte(&self, l: & OmniaUByte, r: & OmniaUByte) -> OmniaUByte {
        if let Some(calculated) = l.get_value_as::<u8>().checked_div(r.get_value_as::<u8>()) {
            OmniaUByte::get_from(calculated)
        }else {
            panic!("Got value less or more than byte bounds while dividing (or attempted to divide by zero)")
        }
    }
    fn rem_ubyte(&self, l: & OmniaUByte, r: & OmniaUByte) -> OmniaUByte {
        if let Some(calculated) = l.get_value_as::<u8>().checked_rem(r.get_value_as::<u8>()) {
            OmniaUByte::get_from(calculated)
        }else {
            panic!("Got value less or more than byte bounds while remaindering")
        }
    }
    //SECTION:: UByte arithmetics end
    fn append_ubyte_to_string(&self, l: &OmniaUByte, r: &OmniaString) -> Box<OmniaString> {
        if self.operator != Operator::PLUS {
            panic!("Only can append(+) value to string")
        }
        let mut val = r.get_as_string();
        val.push_str(&l.get_as_string());
        Box::new(OmniaString::get_from(val))
    }
    //SECTION:: UBytes end

    //SECTION:: ULongs start

    fn prepare_calc_l_ulong(&self, l: &OmniaULong) -> Box<dyn OmniaValue> {
        let mut r = self.right.calc();
        let binding = r.get_type();
        let r_type = binding.get_right();
        match r_type {
            Type::UBYTE => {
                self.prepare_calc_ulong(l, &OmniaULong::get_from(r.downcast_ref::<OmniaUByte>().unwrap().get_value_as()))
            }
            Type::UINT => {
                self.prepare_calc_ulong(l, &OmniaULong::get_from(r.downcast_ref::<OmniaUInt>().unwrap().get_value_as()))
            }
            Type::ULONG => {
                self.prepare_calc_ulong(l, r.downcast_ref::<OmniaULong>().unwrap())
            }
            Type::STRING => {
                self.append_ulong_to_string(l, r.downcast_ref::<OmniaString>().unwrap())
            }
            _ => {
                panic!("Unexpected type {:?}", r_type)
            }
        }
    }
    fn prepare_calc_ulong(&self, l: &OmniaULong, r: &OmniaULong) -> Box<OmniaULong> {
        match self.operator {
            Operator::PLUS => {
                Box::new(self.add_ulong(l, r))
            }
            Operator::MINUS => {
                Box::new(self.sub_ulong(l, r))
            }
            Operator::MULTIPLY => {
                Box::new(self.mul_ulong(l, r))
            }
            Operator::DIVIDE => {
                Box::new(self.div_ulong(l, r))
            }
            Operator::REM => {
                Box::new(self.rem_ulong(l, r))
            }
            _ => {
                panic!("Unexpected operator")
            }
        }
    }
    fn add_ulong(&self, l: &OmniaULong, r: &OmniaULong) -> OmniaULong {
        if let Some(calculated) = l.get_value_as::<u64>().checked_add(r.get_value_as::<u64>()) {
            OmniaULong::get_from(calculated)
        } else {
            panic!("Got value less or more than ulong bounds while adding")
        }
    }
    fn sub_ulong(&self, l: &OmniaULong, r: &OmniaULong) -> OmniaULong {
        if let Some(calculated) = l.get_value_as::<u64>().checked_sub(r.get_value_as::<u64>()) {
            OmniaULong::get_from(calculated)
        } else {
            panic!("Got value less or more than long bounds while subtracting")
        }
    }
    fn mul_ulong(&self, l: &OmniaULong, r: &OmniaULong) -> OmniaULong {
        if let Some(calculated) = l.get_value_as::<u64>().checked_mul(r.get_value_as::<u64>()) {
            OmniaULong::get_from(calculated)
        } else {
            panic!("Got value less or more than long bounds while multiplying")
        }
    }
    fn div_ulong(&self, l: &OmniaULong, r: &OmniaULong) -> OmniaULong {
        if let Some(calculated) = l.get_value_as::<u64>().checked_div(r.get_value_as::<u64>()) {
            OmniaULong::get_from(calculated)
        } else {
            panic!("Got value less or more than long bounds while dividing(Or attempted to divide by zero)")
        }
    }
    fn rem_ulong(&self, l: &OmniaULong, r: &OmniaULong) -> OmniaULong {
        if let Some(calculated) = l.get_value_as::<u64>().checked_rem(r.get_value_as::<u64>()) {
            OmniaULong::get_from(calculated)
        } else {
            panic!("Got value less or more than long bounds while remaindering")
        }
    }
    fn append_ulong_to_string(&self, l: &OmniaULong, r: &OmniaString) -> Box<OmniaString> {
        if self.operator != Operator::PLUS {
            panic!("Can only append(+) value to string!")
        }
        let mut val = r.get_as_string();
        val.push_str(&l.get_as_string());
        Box::new(OmniaString::get_from(val))
    }
    //SECTION:: ULongs end


    //SECTION:: Strings start

    fn prepare_calc_l_string(&self, l: &OmniaString) -> Box<OmniaString> {
        let mut r = self.right.calc();
        let binding = r.get_type();
        let r_type = binding.get_right();
        match r_type {
            Type::BYTE => {
                self.append_byte_to_string(r.downcast_ref::<OmniaByte>().unwrap(), l)
            }
            Type::UBYTE => {
                self.append_ubyte_to_string(r.downcast_ref::<OmniaUByte>().unwrap(), l)
            }
            Type::INT => {
                self.append_int_to_string(r.downcast_ref::<OmniaInt>().unwrap(), l)
            }
            Type::UINT => {
                self.append_uint_to_string(r.downcast_ref::<OmniaUInt>().unwrap(), l)
            }
            Type::LONG => {
                self.append_long_to_string(r.downcast_ref::<OmniaLong>().unwrap(), l)
            }
            Type::ULONG => {
                self.append_ulong_to_string(r.downcast_ref::<OmniaULong>().unwrap(), l)
            }
            Type::STRING => {
                let temp = r.downcast_ref::<OmniaString>().unwrap();
                let mut val = l.get_as_string();
                val.push_str(&temp.get_as_string());
                Box::new(OmniaString::get_from(val))
            }
            Type::DECIMAL => {
                self.append_decimal_to_string(r.downcast_ref::<OmniaDecimal>().unwrap(), l)
            }
            Type::CHAR => {
                let temp = r.downcast_ref::<OmniaChar>().unwrap();
                let mut val = l.get_as_string();
                val.push(temp.get_value_as::<char>());
                Box::new(OmniaString::get_from(val))
            }
            _ => {
                panic!("Unexpected or unsupported type {:?}", r_type)
            }
        }
    }
    //SECTION:: Strings end

    //SECTION:: Decimals start

    fn prepare_calc_l_decimal(&self, l: &OmniaDecimal) -> Box<dyn OmniaValue> {
        let mut r = self.right.calc();
        let binding = r.get_type();
        let r_type = binding.get_right();
        match r_type {
            Type::BYTE => {
                self.prepare_calc_decimal(l, &OmniaDecimal::get_from(r.downcast_ref::<OmniaByte>().unwrap().get_as_float32()))
            }
            Type::UBYTE => {
                self.prepare_calc_decimal(l, &OmniaDecimal::get_from(r.downcast_ref::<OmniaUByte>().unwrap().get_as_float32()))
            }
            Type::INT => {
                self.prepare_calc_decimal(l, &OmniaDecimal::get_from(r.downcast_ref::<OmniaInt>().unwrap().get_as_float32()))
            }
            Type::UINT => {
                self.prepare_calc_decimal(l, &OmniaDecimal::get_from(r.downcast_ref::<OmniaUInt>().unwrap().get_value_as()))
            }
            Type::LONG => {
                self.prepare_calc_decimal(l, &OmniaDecimal::get_from(r.downcast_ref::<OmniaLong>().unwrap().get_value_as()))
            }
            Type::DECIMAL => {
                self.prepare_calc_decimal(l, r.downcast_ref::<OmniaDecimal>().unwrap())
            }
            Type::STRING => {
                self.append_decimal_to_string(l, r.downcast_ref::<OmniaString>().unwrap())
            }


            _ => {
                panic!("Unexpected or unsupported type {:?}", r_type)
            }
        }
    }
    fn prepare_calc_decimal(&self, l: &OmniaDecimal, r: &OmniaDecimal) -> Box<OmniaDecimal> {
        match self.operator {
            Operator::PLUS => {
                Box::new(self.add_decimal(l, r))
            }
            Operator::MINUS => {
                Box::new(self.sub_decimal(l, r))
            }
            Operator::MULTIPLY => {
                Box::new(self.mul_decimal(l, r))
            }
            Operator::DIVIDE => {
                Box::new(self.div_decimal(l, r))
            }
            Operator::REM => {
                Box::new(self.rem_decimal(l, r))
            }


            _ => {
                panic!("Unexpected or unsupported operator {:?}", self.operator)
            }
        }
    }
    fn add_decimal(&self, l: &OmniaDecimal, r: &OmniaDecimal) -> OmniaDecimal {
        if let Some(calculated) = l.get_value_as::<f64>().checked_add(r.get_value_as::<f64>()) {
            OmniaDecimal::get_from(calculated)
        } else {
            panic!("Got value which is out of bounds of decimal while adding")
        }
    }
    fn sub_decimal(&self, l: &OmniaDecimal, r: &OmniaDecimal) -> OmniaDecimal {
        if let Some(calculated) = l.get_value_as::<f64>().checked_sub(r.get_value_as::<f64>()) {
            OmniaDecimal::get_from(calculated)
        } else {
            panic!("Got value which is out of bounds of decimal while subtracting")
        }
    }
    fn mul_decimal(&self, l: &OmniaDecimal, r: &OmniaDecimal) -> OmniaDecimal {
        if let Some(calculated) = l.get_value_as::<f64>().checked_mul(r.get_value_as::<f64>()) {
            OmniaDecimal::get_from(calculated)
        } else {
            panic!("Got value which is out of bounds of decimal while multiplying")
        }
    }
    fn div_decimal(&self, l: &OmniaDecimal, r: &OmniaDecimal) -> OmniaDecimal {
        if let Some(calculated) = l.get_value_as::<f64>().checked_div(r.get_value_as::<f64>()) {
            OmniaDecimal::get_from(calculated)
        } else {
            panic!("Got value which is out of bounds of decimal while dividing(or attempted to divide by zero)")
        }
    }
    fn rem_decimal(&self, l: &OmniaDecimal, r: &OmniaDecimal) -> OmniaDecimal {
        if let Some(calculated) = l.get_value_as::<f64>().checked_rem(r.get_value_as::<f64>()) {
            OmniaDecimal::get_from(calculated)
        } else {
            panic!("Got value which is out of bounds of decimal while remaindering")
        }
    }
    fn append_decimal_to_string(&self, l: &OmniaDecimal, r: &OmniaString) -> Box<OmniaString> {
        let mut val = r.get_as_string();
        val.push_str(&l.get_as_string());
        Box::new(OmniaString::get_from(val))
    }
    //SECTION:: Decimals end
}
pub trait CheckerF64 {
    fn checked_add(&self, b: f64) -> Option<f64>;
    fn checked_sub(&self, b: f64) -> Option<f64>;
    fn checked_mul(&self, b: f64) -> Option<f64>;
    fn checked_div(&self, b: f64) -> Option<f64>;
    fn checked_rem(&self, b: f64) -> Option<f64>;
}
impl CheckerF64 for f64 {
    fn checked_add(&self, b: f64) -> Option<f64> {
        let result = self + b;
        if result.is_finite() {
            Some(result)
        } else {
            None
        }
    }
    fn checked_sub(&self, b: f64) -> Option<f64> {
        let result = self - b;
        if result.is_finite() {
            Some(result)
        } else {
            None
        }
    }
    fn checked_mul(&self, b: f64) -> Option<f64> {
        let result = self * b;
        if result.is_finite() {
            Some(result)
        } else {
            None
        }
    }
    fn checked_div(&self, b: f64) -> Option<f64> {
        if b == 0f64 {
            return None
        }
        let result = self / b;
        if result.is_finite() {
            Some(result)
        } else {
            None
        }
    }
    fn checked_rem(&self, b: f64) -> Option<f64> {
        let result = self % b;
        if result.is_finite() {
            Some(result)
        } else {
            None
        }
    }
}

impl Expression for BinaryExpression {
    fn calc(&mut self) -> Box<dyn OmniaValue> {
        let mut left = self.left.calc();
        let binding = left.get_type();
        let value_type = binding.get_right();
        match value_type {
            Type::BYTE => {
                if let Some(dc) = left.downcast_ref::<OmniaByte>() {
                    self.prepare_calc_l_byte(dc)
                } else {
                    panic!("Unexpected error")
                }
            }

            Type::INT => {
                if let Some(dc) = left.downcast_ref::<OmniaInt>() {
                    self.prepare_calc_l_int(dc)
                } else {
                    panic!("Unexpected error")
                }
            }
            Type::LONG => {
                if let Some(dc) = left.downcast_ref::<OmniaLong>() {
                    self.prepare_calc_l_long(dc)
                } else {
                    panic!("Unexpected error")
                }
            }

            Type::UBYTE => {
                if let Some(dc) = left.downcast_ref::<OmniaUByte>() {
                    self.prepare_calc_l_ubyte(dc)
                } else {
                    panic!("Unexpected error")
                }
            }
            Type::UINT => {
                if let Some(dc) = left.downcast_ref::<OmniaUInt>() {
                    self.prepare_calc_l_uint(dc)
                } else {
                    panic!("Unexpected error")
                }
            }
            Type::ULONG => {
                if let Some(dc) = left.downcast_ref::<OmniaULong>() {
                    self.prepare_calc_l_ulong(dc)
                } else {
                    panic!("Unexpected error")
                }
            }
            Type::STRING => {
                if let Some(dc) = left.downcast_ref::<OmniaString>() {
                   self.prepare_calc_l_string(dc)
                } else {
                    panic!("Unexpected error")
                }
            }
            Type::DECIMAL => {
                if let Some(dc) = left.downcast_ref::<OmniaDecimal>() {
                    self.prepare_calc_l_decimal(dc)
                } else {
                    panic!("Unexpected error")
                }
            }

            _ => {
                panic!("Unexpected type {:?}", left.get_type().get_right())
            }
        }

    }
}
pub struct UnaryExpression {
    left: &'static mut dyn Expression,
    operation: Operator
}
impl UnaryExpression {
    fn new(left: impl Expression, operation: Operator) -> UnaryExpression {
        Self {
            left,
            operation
        }
    }

}
impl Node for UnaryExpression {}
impl Expression for UnaryExpression {
    fn calc(&mut self) -> Box<dyn OmniaValue>{
        todo!()
    }
}