pub struct Time {
    pub(crate) delta_mult: f32,
}

impl Time {
    /// #### usage in _update_ systems:
    /// ```ignore
    /// pos += velocity * time.delta_mult();
    /// ```
    ///
    /// #### usage in _frame_ systems:
    /// note: don't _modify_ pos from
    /// frame systems
    /// ```ignore
    /// pos + velocity * time.delta_mult();
    /// ```
    pub fn delta_mult(&self) -> f32 {
        self.delta_mult
    }
}
