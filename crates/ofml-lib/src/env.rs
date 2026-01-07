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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scope_new() {
        let scope = Scope::new();
        assert!(scope.parent.is_none());
        assert!(scope.vars.is_empty());
    }

    #[test]
    fn test_scope_default() {
        let scope = Scope::default();
        assert!(scope.parent.is_none());
    }

    #[test]
    fn test_scope_define_get() {
        let mut scope = Scope::new();
        scope.define("x", Value::Int(42));

        let val = scope.get("x");
        assert!(val.is_some());
        match val.unwrap() {
            Value::Int(n) => assert_eq!(n, 42),
            _ => panic!("Expected Int"),
        }
    }

    #[test]
    fn test_scope_get_not_found() {
        let scope = Scope::new();
        assert!(scope.get("nonexistent").is_none());
    }

    #[test]
    fn test_scope_child() {
        let parent = Rc::new(RefCell::new(Scope::new()));
        parent.borrow_mut().define("x", Value::Int(10));

        let child = Scope::child(parent.clone());

        // Child can access parent's variables
        let val = child.get("x");
        assert!(val.is_some());
    }

    #[test]
    fn test_scope_child_shadows_parent() {
        let parent = Rc::new(RefCell::new(Scope::new()));
        parent.borrow_mut().define("x", Value::Int(10));

        let mut child = Scope::child(parent.clone());
        child.define("x", Value::Int(20));

        // Child's value shadows parent
        match child.get("x").unwrap() {
            Value::Int(n) => assert_eq!(n, 20),
            _ => panic!("Expected Int"),
        }
    }

    #[test]
    fn test_scope_set_local() {
        let mut scope = Scope::new();
        scope.define("x", Value::Int(1));

        let result = scope.set("x", Value::Int(2));
        assert!(result);

        match scope.get("x").unwrap() {
            Value::Int(n) => assert_eq!(n, 2),
            _ => panic!("Expected Int"),
        }
    }

    #[test]
    fn test_scope_set_not_found() {
        let mut scope = Scope::new();
        let result = scope.set("y", Value::Int(1));
        assert!(!result);
    }

    #[test]
    fn test_scope_set_in_parent() {
        let parent = Rc::new(RefCell::new(Scope::new()));
        parent.borrow_mut().define("x", Value::Int(10));

        let mut child = Scope::child(parent.clone());
        let result = child.set("x", Value::Int(20));
        assert!(result);

        // Parent's value should be updated
        let val = parent.borrow().get("x").unwrap();
        match val {
            Value::Int(n) => assert_eq!(n, 20),
            _ => panic!("Expected Int"),
        }
    }

    #[test]
    fn test_scope_has_local() {
        let mut scope = Scope::new();
        assert!(!scope.has("x"));

        scope.define("x", Value::Int(1));
        assert!(scope.has("x"));
    }

    #[test]
    fn test_scope_has_in_parent() {
        let parent = Rc::new(RefCell::new(Scope::new()));
        parent.borrow_mut().define("x", Value::Int(10));

        let child = Scope::child(parent.clone());
        assert!(child.has("x"));
        assert!(!child.has("y"));
    }

    #[test]
    fn test_scope_clone() {
        let mut scope = Scope::new();
        scope.define("x", Value::Int(42));

        let cloned = scope.clone();
        match cloned.get("x").unwrap() {
            Value::Int(n) => assert_eq!(n, 42),
            _ => panic!("Expected Int"),
        }
    }

    #[test]
    fn test_environment_new() {
        let env = Environment::new();
        assert!(env.get("anything").is_none());
    }

    #[test]
    fn test_environment_default() {
        let env = Environment::default();
        assert!(env.get("anything").is_none());
    }

    #[test]
    fn test_environment_define_get() {
        let mut env = Environment::new();
        env.define("x", Value::Int(100));

        match env.get("x").unwrap() {
            Value::Int(n) => assert_eq!(n, 100),
            _ => panic!("Expected Int"),
        }
    }

    #[test]
    fn test_environment_define_global() {
        let mut env = Environment::new();
        env.define_global("global_x", Value::Int(500));

        // Push a new scope
        env.push_scope();

        // Global should still be accessible
        match env.get("global_x").unwrap() {
            Value::Int(n) => assert_eq!(n, 500),
            _ => panic!("Expected Int"),
        }
    }

    #[test]
    fn test_environment_push_pop_scope() {
        let mut env = Environment::new();
        env.define("outer", Value::Int(1));

        env.push_scope();
        env.define("inner", Value::Int(2));

        // Both are accessible
        assert!(env.get("outer").is_some());
        assert!(env.get("inner").is_some());

        env.pop_scope();

        // Only outer should be accessible now
        assert!(env.get("outer").is_some());
        assert!(env.get("inner").is_none());
    }

    #[test]
    fn test_environment_set() {
        let mut env = Environment::new();
        env.define("x", Value::Int(1));

        let result = env.set("x", Value::Int(10));
        assert!(result);

        match env.get("x").unwrap() {
            Value::Int(n) => assert_eq!(n, 10),
            _ => panic!("Expected Int"),
        }
    }

    #[test]
    fn test_environment_set_not_found() {
        let mut env = Environment::new();
        let result = env.set("nonexistent", Value::Int(1));
        assert!(!result);
    }

    #[test]
    fn test_environment_set_or_define_existing() {
        let mut env = Environment::new();
        env.define("x", Value::Int(1));

        env.set_or_define("x", Value::Int(100));

        match env.get("x").unwrap() {
            Value::Int(n) => assert_eq!(n, 100),
            _ => panic!("Expected Int"),
        }
    }

    #[test]
    fn test_environment_set_or_define_new() {
        let mut env = Environment::new();

        env.set_or_define("y", Value::Int(200));

        match env.get("y").unwrap() {
            Value::Int(n) => assert_eq!(n, 200),
            _ => panic!("Expected Int"),
        }
    }

    #[test]
    fn test_environment_nested_scopes() {
        let mut env = Environment::new();
        env.define("level0", Value::Int(0));

        env.push_scope();
        env.define("level1", Value::Int(1));

        env.push_scope();
        env.define("level2", Value::Int(2));

        // All levels accessible
        assert!(env.get("level0").is_some());
        assert!(env.get("level1").is_some());
        assert!(env.get("level2").is_some());

        env.pop_scope();
        assert!(env.get("level2").is_none());

        env.pop_scope();
        assert!(env.get("level1").is_none());

        // level0 is still there
        assert!(env.get("level0").is_some());
    }

    #[test]
    fn test_environment_pop_global_scope() {
        let mut env = Environment::new();
        env.define("x", Value::Int(1));

        // Popping the global scope should be a no-op
        env.pop_scope();

        // x should still be accessible
        assert!(env.get("x").is_some());
    }
}
