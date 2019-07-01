use crate::expression::*;
use std::cell::RefCell;
use std::collections::HashMap;

use std::string::ToString;

trait Var {
    fn get(&self) -> Value;
    fn set(&self, val: Value) -> bool;
}

#[derive(Clone, Debug)]
pub enum VarType {
    Const,
    Let,
}

#[derive(Clone, Debug)]
pub struct ValueVar {
    var_type: VarType,
    value: Option<RefCell<Value>>,
}

impl ValueVar {
    #[inline]
    pub fn new(var_type: VarType, value: Value) -> Self {
        ValueVar {
            var_type,
            value: Some(RefCell::new(value)),
        }
    }
}

impl Var for ValueVar {
    fn get(&self) -> Value {
        assert!(self.value.is_some(), "get a undefined value");
        (&self.value).as_ref().unwrap().borrow().clone()
    }

    fn set(&self, val: Value) -> bool {
        match self.var_type {
            VarType::Const => false,
            VarType::Let => {
                (&self.value).as_ref().unwrap().replace(val);
                true
            }
        }
    }
}

impl Default for Context<'_> {
    fn default() -> Self {
        Context {
            parent: None,
            variables: Default::default(),
        }
    }
}

impl Context<'_> {
    #[inline]
    pub(crate) fn init_with_parent_context<'b>(parent_ctx: &'b Context<'b>) -> Context<'b> {
        let mut ctx = Context::default();
        ctx.parent = Some(parent_ctx);
        ctx
    }
}

/// 程序上下文
#[derive(Debug)]
pub struct Context<'a> {
    /// 父级上下文
    parent: Option<&'a Context<'a>>,

    /// 变量池
    variables: HashMap<String, ValueVar>,
}

impl Context<'_> {
    pub(crate) fn get_var(&self, name: &str) -> Option<Value> {
        match self.variables.get(name) {
            Some(val) => Some(val.get()),
            None => match &self.parent {
                Some(scoop) => scoop.get_var(name),
                None => None,
            },
        }
    }

    pub fn insert_var(&mut self, name: &str, val: Value, var_type: VarType) -> bool {
        match self.get_var(name) {
            Some(_) => false,
            None => {
                self.variables
                    .insert(name.to_string(), ValueVar::new(var_type, val));
                true
            }
        }
    }

    pub(crate) fn update_var(&self, name: &str, value: Value) -> bool {
        match self.variables.get(name) {
            Some(val) => val.set(value),
            None => match &self.parent {
                Some(ctx) => (*ctx).update_var(name, value),
                None => false,
            },
        }
    }
}