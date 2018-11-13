use parser::expressions::*;
use parser::infix::*;

pub fn get_expression_llvm_type(expression: &Expression) -> LLVMExpressionType {
    match expression.clone() {
        Expression::IntegerLiteral(_, _) => LLVMExpressionType::Int,
        Expression::StringLiteral(_, _) => LLVMExpressionType::String,
        Expression::Boolean(_, _) => LLVMExpressionType::Boolean,
        Expression::Array(expression_type, elements) => {
            LLVMExpressionType::Array(Box::new(expression_type), elements.len() as u32)
        }
        Expression::ArrayElement(_, boxed_element, _) => get_expression_llvm_type(&boxed_element),
        Expression::Infix(infix, left, _, _) => handle_infix_type(infix, *left),
        _ => LLVMExpressionType::Null,
    }
}

pub fn handle_infix_type(infix: Infix, left: Expression) -> LLVMExpressionType {
    match infix {
        Infix::Plus => get_expression_llvm_type(&left),
        Infix::Minus => LLVMExpressionType::Int,
        Infix::Divide => LLVMExpressionType::Int,
        Infix::Multiply => LLVMExpressionType::Int,
        Infix::Rem => LLVMExpressionType::Int,
        Infix::Eq => LLVMExpressionType::Boolean,
        Infix::NotEq => LLVMExpressionType::Boolean,
        Infix::Gte => LLVMExpressionType::Boolean,
        Infix::Gt => LLVMExpressionType::Boolean,
        Infix::Lte => LLVMExpressionType::Boolean,
        Infix::Lt => LLVMExpressionType::Boolean,
    }
}