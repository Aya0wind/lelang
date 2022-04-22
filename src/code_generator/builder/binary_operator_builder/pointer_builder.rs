use crate::code_generator::builder::{LEBasicType, LEBasicTypeEnum, LEBasicValue, LEContext, LEPointerValue, LEType};
use crate::code_generator::builder::binary_operator_builder::MemberAccessOperatorBuilder;
use crate::error::CompileError;

impl<'ctx> MemberAccessOperatorBuilder<'ctx> for LEPointerValue<'ctx> {
    fn build_dot(&self, le_context: &LEContext<'ctx>, member_name: &str) -> crate::code_generator::builder::Result<LEPointerValue<'ctx>> {
        let pointed_type = self.ty.get_point_type();
        if let LEBasicTypeEnum::Struct(struct_type) = pointed_type {
            let (offset, member_type) = struct_type.get_member_offset_and_type(member_name)
                .ok_or_else(|| CompileError::NoSuchMember { member_name: member_name.into() })?;

            let member_pointer_type = LEBasicType::get_pointer_type(&member_type);

            let member_pointer_value = le_context
                .llvm_builder
                .build_struct_gep(self.llvm_value, offset, "").unwrap();

            Ok(LEPointerValue { ty: member_pointer_type, llvm_value: member_pointer_value })
        } else {
            Err(CompileError::TypeMismatched { expect: self.ty.name().into(), found: pointed_type.name().into() })
        }
    }

    fn build_index(&self, le_context: &LEContext<'ctx>, member_name: &str) -> crate::code_generator::builder::Result<LEPointerValue<'ctx>> {
        todo!()
    }
}