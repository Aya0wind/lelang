use inkwell::builder::Builder;

use crate::code_generator::builder::{LEBasicType, LEBasicTypeEnum, LEPointerValue, LEType};
use crate::code_generator::builder::binary_operator_builder::MemberAccessOperateValue;
use crate::code_generator::context::LEContext;
use crate::code_generator::Result;
use crate::error::CompileError;

impl<'ctx> MemberAccessOperateValue<'ctx> for LEPointerValue<'ctx> {
    fn build_dot_unchecked(&self, le_context: &LEContext<'ctx>, llvm_builder: &Builder<'ctx>, member_name: &str) -> Result<LEPointerValue<'ctx>> {
        let pointed_type = self.ty.get_point_type();
        if let LEBasicTypeEnum::Struct(struct_type) = pointed_type {
            let (offset, member_type) = struct_type.get_member_offset_and_type(member_name)
                .ok_or_else(|| CompileError::NoSuchMember { member_name: member_name.into() })?;

            let member_pointer_type = LEBasicType::get_pointer_type(&member_type);

            let member_pointer_value = llvm_builder.build_struct_gep(self.llvm_value, offset, "").unwrap();

            Ok(LEPointerValue { ty: member_pointer_type, llvm_value: member_pointer_value })
        } else {
            Err(CompileError::TypeMismatched { expect: self.ty.name().into(), found: pointed_type.name().into() })
        }
    }

    fn build_index_unchecked(&self, le_context: &LEContext<'ctx>, llvm_builder: &Builder<'ctx>, index: usize) -> Result<LEPointerValue<'ctx>> {
        todo!()
    }
}