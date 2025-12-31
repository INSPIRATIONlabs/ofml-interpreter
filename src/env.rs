//! Environment for variable scoping

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::value::Value;

/// A scope containing variable bindings
#[derive(Clone)]
pub struct Scope {
    /// Variables in this scope
    vars: HashMap<String, Value>,
    /// Parent scope (for lexical scoping)
    parent: Option<Rc<RefCell<Scope>>>,
}

impl Scope {
    /// Create a new global scope
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            parent: None,
        }
    }

    /// Create a child scope
    pub fn child(parent: Rc<RefCell<Scope>>) -> Self {
        Self {
            vars: HashMap::new(),
            parent: Some(parent),
        }
    }

    /// Define a variable in this scope
    pub fn define(&mut self, name: &str, value: Value) {
        self.vars.insert(name.to_string(), value);
    }

    /// Get a variable (searches parent scopes)
    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(val) = self.vars.get(name) {
            Some(val.clone())
        } else if let Some(ref parent) = self.parent {
            parent.borrow().get(name)
        } else {
            None
        }
    }

    /// Set a variable (searches parent scopes)
    pub fn set(&mut self, name: &str, value: Value) -> bool {
        if self.vars.contains_key(name) {
            self.vars.insert(name.to_string(), value);
            true
        } else if let Some(ref parent) = self.parent {
            parent.borrow_mut().set(name, value)
        } else {
            false
        }
    }

    /// Check if variable exists
    pub fn has(&self, name: &str) -> bool {
        if self.vars.contains_key(name) {
            true
        } else if let Some(ref parent) = self.parent {
            parent.borrow().has(name)
        } else {
            false
        }
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
}

/// Environment manages the scope stack
pub struct Environment {
    /// Current scope
    current: Rc<RefCell<Scope>>,
    /// Global scope (always accessible)
    global: Rc<RefCell<Scope>>,
}

impl Environment {
    /// Create a new environment
    pub fn new() -> Self {
        let global = Rc::new(RefCell::new(Scope::new()));
        Self {
            current: global.clone(),
            global,
        }
    }

    /// Push a new scope
    pub fn push_scope(&mut self) {
        let new_scope = Scope::child(self.current.clone());
        self.current = Rc::new(RefCell::new(new_scope));
    }

    /// Pop the current scope
    pub fn pop_scope(&mut self) {
        let parent = self.current.borrow().parent.clone();
        if let Some(p) = parent {
            self.current = p;
        }
    }

    /// Define a variable in current scope
    pub fn define(&mut self, name: &str, value: Value) {
        self.current.borrow_mut().define(name, value);
    }

    /// Define a global variable
    pub fn define_global(&mut self, name: &str, value: Value) {
        self.global.borrow_mut().define(name, value);
    }

    /// Get a variable
    pub fn get(&self, name: &str) -> Option<Value> {
        self.current
            .borrow()
            .get(name)
            .or_else(|| self.global.borrow().get(name))
    }

    /// Set a variable
    pub fn set(&mut self, name: &str, value: Value) -> bool {
        self.current.borrow_mut().set(name, value.clone())
            || self.global.borrow_mut().set(name, value)
    }

    /// Get or define (for assignment to undefined variables)
    pub fn set_or_define(&mut self, name: &str, value: Value) {
        if !self.set(name, value.clone()) {
            self.define(name, value);
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
