use crate::ast::expr::Expr;
use crate::ast::value::Value;

use super::Env;
use super::EvalContext;
use super::EvalError;
use super::Evaluable;

impl Evaluable for Expr {
    fn eval(&self, env: &Env, ctx: &mut EvalContext) -> Result<Value, EvalError> {
        ctx.cost_accum.add_cost_of(self)?;
        match self {
            Expr::Const(c) => Ok(c.v.clone()),
            Expr::CalcBlake2b256(op) => op.eval(env, ctx),
            Expr::Fold(op) => op.eval(env, ctx),
            Expr::ExtractRegisterAs(op) => op.eval(env, ctx),
            Expr::GlobalVars(op) => op.eval(env, ctx),
            Expr::MethodCall(op) => op.eval(env, ctx),
            Expr::ProperyCall(op) => op.eval(env, ctx),
            Expr::BinOp(op) => op.eval(env, ctx),
            Expr::Context => Ok(Value::Context(ctx.ctx.clone())),
            Expr::OptionGet(v) => v.eval(env, ctx),
            Expr::Apply(op) => op.eval(env, ctx),
            Expr::FuncValue(op) => op.eval(env, ctx),
            Expr::ValUse(op) => op.eval(env, ctx),
            Expr::BlockValue(op) => op.eval(env, ctx),
            Expr::SelectField(op) => op.eval(env, ctx),
            Expr::ExtractAmount(op) => op.eval(env, ctx),
            Expr::ConstPlaceholder(_) => panic!("ConstPlaceholder cannot be evaluated"),
            Expr::Collection(op) => op.eval(env, ctx),
            Expr::ValDef(_) => panic!("ValDef is evaluated in BlockValue"),
            Expr::And(op) => op.eval(env, ctx),
            Expr::Or(op) => op.eval(env, ctx),
            Expr::LogicalNot(op) => op.eval(env, ctx),
            Expr::Map(op) => op.eval(env, ctx),
            Expr::Filter(op) => op.eval(env, ctx),
            Expr::BoolToSigmaProp(op) => op.eval(env, ctx),
            Expr::Upcast(op) => op.eval(env, ctx),
        }
    }
}
