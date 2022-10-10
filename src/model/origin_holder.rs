use std::fmt::Display;

///
#[derive(Clone)]
pub struct OriginHolder<const N: usize> {
  pub segments: [&'static str; N],
}

impl<const N: usize> Display for OriginHolder<N> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut segments = self.segments.iter().peekable();

    while let Some(segment) = segments.next() {
      write!(f, "{segment}")?;

      if segments.peek().is_some() {
        write!(f, ".")?;
      }
    }

    Ok(())
  }
}

impl<const N: usize> OriginHolder<N> {
  pub const fn new(segments: [&'static str; N]) -> Self {
    Self { segments }
  }
}
