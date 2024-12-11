use crate::{
    code_gen::{
        js::{Tokens, VariablePath},
        utils::wrapped_brackets,
    },
    registry::TupleStructType,
};

use super::{des, ser, ts, ty_check, BindingTypeGenerateable};

impl BindingTypeGenerateable for TupleStructType {
    fn gen_ser_body(&self) -> Tokens {
        ser::gen_accessors_indexed(&self.fields, VariablePath::default())
    }

    fn gen_des_body(&self) -> Tokens {
        wrapped_brackets(des::gen_accessors_indexed(&self.fields))
    }

    fn gen_ty_check_body(&self) -> Tokens {
        ty_check::gen_array_checks(&self.fields, VariablePath::default())
    }

    fn gen_ts_typings_body(&self) -> Tokens {
        ts::gen_typings_indexed(&self.fields)
    }
}