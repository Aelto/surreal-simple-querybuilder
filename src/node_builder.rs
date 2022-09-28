use std::fmt::Display;

pub trait ToNodeBuilder<T: Display = Self>: Display {
  /// Draws the start of a relation `->node`
  ///
  /// # Example
  /// ```
  /// use surreal_simple_querybuilder::prelude::*;
  ///
  /// let s = "user".with("project");
  ///
  /// assert_eq!("user->project", s);
  /// ```
  fn with(&self, relation_or_node: &str) -> String {
    format!("{self}->{relation_or_node}")
  }

  /// Draws the end of a relation `<-node`
  ///
  /// # Example
  /// ```
  /// use surreal_simple_querybuilder::prelude::*;
  ///
  /// let s = "user".from("project");
  ///
  /// assert_eq!("user<-project", s);
  /// ```
  fn from(&self, node: &str) -> String {
    format!("{self}<-{node}")
  }

  /// Take the current string and add in front of it the given label name as to
  /// make a string of the following format `LabelName:CurrentString`
  ///
  /// # Example
  /// ```
  /// use surreal_simple_querybuilder::prelude::*;
  ///
  /// let label = "John".as_named_label("Account");
  ///
  /// assert_eq!(label, "Account:John");
  /// ```
  fn as_named_label(&self, label_name: &str) -> String {
    format!("{label_name}:{self}")
  }

  /// # Example
  /// ```
  /// use surreal_simple_querybuilder::prelude::*;
  ///
  /// let s = "user".equals("John");
  ///
  /// // Note that it doesn't add quotes around strings
  /// assert_eq!("user = John", s);
  /// ```
  fn equals(&self, value: &str) -> String {
    format!("{self} = {value}")
  }

  /// Take the current string and add `= $current_string` after it
  ///
  /// # Example
  /// ```
  /// use surreal_simple_querybuilder::prelude::*;
  ///
  /// let s = "account".equals_parameterized();
  ///
  /// assert_eq!("account = $account", s);
  /// ```
  fn equals_parameterized(&self) -> String {
    format!("{self} = ${self}")
  }
}

impl<'a> ToNodeBuilder for &'a str {}

pub trait NodeBuilder<T: Display = Self>: Display {
  /// Draws the start of a relation `->node`
  ///
  /// # Example
  /// ```
  /// use surreal_simple_querybuilder::prelude::*;
  ///
  /// let s = "user".with("project");
  ///
  /// assert_eq!("user->project", s);
  /// ```
  fn with(&mut self, relation_or_node: &str) -> &mut String;

  /// Allows you to pass a lambda that should mutate the current string when the
  /// passed `condition` is `true`. If `condition` is `false` then the `action`
  /// lambda is ignored and the string stays intact.
  ///
  /// # Example
  /// ```
  /// use surreal_simple_querybuilder::prelude::*;
  ///
  /// // demonstrate how the given closure is ignored if the condition is `false`
  /// let mut label = "John".as_named_label("User");
  /// let intact = &mut label
  ///   .if_then(false, |s| s.with("LOVES").with("User"))
  ///   .with("FRIEND")
  ///   .with("User");
  ///
  /// assert_eq!("User:John->FRIEND->User", *intact);
  ///
  /// // demonstrate how the given closure is executed if the condition is `true`
  /// let mut label = "John".as_named_label("User");
  /// let modified = &mut label
  ///   .if_then(true, |s| s.with("LOVES").with("User"))
  ///   .with("FRIEND")
  ///   .with("User");
  ///
  /// assert_eq!("User:John->LOVES->User->FRIEND->User", *modified);
  /// ```
  fn if_then(&mut self, condition: bool, action: fn(&mut Self) -> &mut Self) -> &mut String;

  /// Take the current string and add in front of it the given label name as to
  /// make a string of the following format `LabelName:CurrentString`
  ///
  /// # Example
  /// ```
  /// use surreal_simple_querybuilder::prelude::*;
  ///
  /// let label = "John".as_named_label("Account");
  ///
  /// assert_eq!(label, "Account:John");
  /// ```
  fn as_named_label(&self, label_name: &str) -> String {
    format!("{label_name}:{self}")
  }
}

impl NodeBuilder for String {
  fn with(&mut self, node: &str) -> &mut String {
    self.push_str("->");
    self.push_str(node);

    self
  }

  fn if_then(&mut self, condition: bool, action: fn(&mut Self) -> &mut Self) -> &mut String {
    match condition {
      true => action(self),
      false => self,
    }
  }
}
