pub trait KeySerializeControl {
  /// By default a ForeignKey does not serialize its value if it is in the `Loaded`
  /// state. The value would be transformed into a key using the [IntoKey] trait
  /// methods before serializing it.
  ///
  /// There are cases where this behaviour is not what you wish to happen, calling
  /// [`ForeignKey::allow_value_serialize()`] flags the ForeignKey to ignore the
  /// default behaviour and serialize any potential value it may hold.
  ///
  /// The default state of the KeySerializeControl flag is `false` and is stored
  /// in a once_cell. That means the value can only be changed once to `true`,
  /// any new attempt at updating the value will fail as there is no way to set
  /// the value back to `false` without resetting the whole ForeignKey. In which
  /// case refer to the [disallow_value_serialize] method but note that it requires
  /// a mutable reference.
  fn allow_value_serialize(&self);

  /// By default a ForeignKey does not serialize its value if it is in the `Loaded`
  /// state. The value would be transformed into a key using the [IntoKey] trait
  /// methods before serializing it.
  ///
  /// There are cases where this behaviour is not what you wish to happen, calling
  /// [`ForeignKey::allow_value_serialize()`] flags the ForeignKey to ignore the
  /// default behaviour and serialize any potential value it may hold.
  ///
  /// And this method reverts any change to the flag so that it comes back to
  /// the default behavior of not serializing the value.
  ///
  /// _Unlike the [allow_value_serialize] method, this one requires a mutable reference._
  fn disallow_value_serialize(&mut self);
}

/// Blanket implementation for anything that implements KeySerializeControl and
/// that is in a Vec.
///
/// This implementation allows calling KeySerializeControl methods directly on
/// the vector itself to mutate every single child element.
impl<T> KeySerializeControl for Vec<T>
where
  T: KeySerializeControl,
{
  fn allow_value_serialize(&self) {
    self
      .iter()
      .for_each(KeySerializeControl::allow_value_serialize);
  }

  fn disallow_value_serialize(&mut self) {
    self
      .iter_mut()
      .for_each(KeySerializeControl::disallow_value_serialize);
  }
}
