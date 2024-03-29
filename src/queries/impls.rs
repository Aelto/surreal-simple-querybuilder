use crate::prelude::*;

/// Blanket implementation for the unit type so it can be passed as a "null" type
/// of param
impl<'a> QueryBuilderInjecter<'a> for () {}

impl<'a> QueryBuilderInjecter<'a> for bool {
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    querybuilder.raw(match self {
      true => "true",
      false => "false",
    })
  }

  fn params(self, _map: &mut BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    Ok(())
  }
}

/// Allows to pass Option<T> types of injecters, useful for optional injecters:
/// ```rs
/// let should_fetch = false;
/// let maybe_fetch = false.then(|| Some(Fetch(["author"])));
/// ```
impl<'a, Injecters> QueryBuilderInjecter<'a> for Option<Injecters>
where
  Injecters: QueryBuilderInjecter<'a>,
{
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    match self {
      Some(inner) => inner.inject(querybuilder),
      None => querybuilder,
    }
  }

  fn params(self, map: &mut BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    match self {
      Some(inner) => inner.params(map),
      None => Ok(()),
    }
  }
}

/// Allows to pass a vec of Injecters
impl<'a, Injecters> QueryBuilderInjecter<'a> for Vec<Injecters>
where
  Injecters: QueryBuilderInjecter<'a>,
{
  fn inject(&self, mut querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    for injecter in self {
      querybuilder = injecter.inject(querybuilder);
    }

    querybuilder
  }

  fn params(self, map: &mut BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    for injecter in self {
      injecter.params(map)?;
    }

    Ok(())
  }
}

impl<'a, I1, I2> QueryBuilderInjecter<'a> for (I1, I2)
where
  I1: QueryBuilderInjecter<'a>,
  I2: QueryBuilderInjecter<'a>,
{
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    self.1.inject(self.0.inject(querybuilder))
  }

  fn params(self, map: &mut BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    self.1.params(map).and(self.0.params(map))
  }
}

impl<'a, I1, I2, I3> QueryBuilderInjecter<'a> for (I1, I2, I3)
where
  I1: QueryBuilderInjecter<'a>,
  I2: QueryBuilderInjecter<'a>,
  I3: QueryBuilderInjecter<'a>,
{
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    self.2.inject(self.1.inject(self.0.inject(querybuilder)))
  }

  fn params(self, map: &mut BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    self
      .2
      .params(map)
      .and(self.1.params(map).and(self.0.params(map)))
  }
}

impl<'a, I1, I2, I3, I4> QueryBuilderInjecter<'a> for (I1, I2, I3, I4)
where
  I1: QueryBuilderInjecter<'a>,
  I2: QueryBuilderInjecter<'a>,
  I3: QueryBuilderInjecter<'a>,
  I4: QueryBuilderInjecter<'a>,
{
  fn inject(&self, querybuilder: QueryBuilder<'a>) -> QueryBuilder<'a> {
    self
      .3
      .inject(self.2.inject(self.1.inject(self.0.inject(querybuilder))))
  }

  fn params(self, map: &mut BindingMap) -> serde_json::Result<()>
  where
    Self: Sized,
  {
    self.3.params(map).and(
      self
        .2
        .params(map)
        .and(self.1.params(map).and(self.0.params(map))),
    )
  }
}
