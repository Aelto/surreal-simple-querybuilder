use std::fmt::Display;
use std::ops::Deref;

pub struct RelationNode<T> {
  relation_name: &'static str,
  node: T,
}

impl<T> RelationNode<T> {
  pub fn new(relation_name: &'static str, node: T) -> Self {
    Self {
      relation_name,
      node,
    }
  }
}

impl<T> Display for RelationNode<T>
where
  T: Display,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "->{}->{}", self.relation_name, self.node)
  }
}

impl<T> Deref for RelationNode<T> {
  type Target = T;

  fn deref(&self) -> &Self::Target {
    &self.node
  }
}
