use crate::Sequential;

/// Types which can be converted into a [Sequential] process with specific output and termination types
///
/// A blanked implementation ensures all [Sequential] types provide [IntoSequential], analogous to [Iterator] and [IntoIterator].
pub trait IntoSequential {
    /// The output of [IntoSequential::Into]
    type Output;
    /// The terminal of [IntoSequential::Into]
    type Terminal;
    /// The [Sequential] value `self` converts into
    type Into: Sequential<Output = Self::Output, Terminal = Self::Terminal>;

    /// Convert `self` into a [Sequential] type
    fn into_sequential(self) -> Self::Into;
}

impl<S> IntoSequential for S
where
    S: Sequential,
{
    type Output = <S as Sequential>::Output;
    type Terminal = <S as Sequential>::Terminal;
    type Into = S;

    fn into_sequential(self) -> Self::Into {
        self
    }
}
