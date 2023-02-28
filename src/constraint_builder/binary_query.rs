use super::Query;
use halo2_proofs::{
    arithmetic::{Field, FieldExt},
    plonk::{Expression, VirtualCells},
};

/// A query whose expression we promise is 0 or 1.
pub struct BinaryQuery<F: Field>(pub Query<F>);

impl<F: FieldExt> BinaryQuery<F> {
    pub fn zero() -> Self {
        Self(Query::zero())
    }

    pub fn one() -> Self {
        Self(Query::one())
    }

    pub fn and(self, other: Self) -> Self {
        !((!self).or(!other))
    }

    pub fn or(self, other: Self) -> Self {
        Self(self.0 * other.0)
    }

    pub fn condition(self, constraint: Query<F>) -> Query<F> {
        self.0 * constraint
    }
}

impl<F: Field> BinaryQuery<F> {
    pub fn run(self, meta: &mut VirtualCells<'_, F>) -> Expression<F> {
        self.0.run(meta)
    }
}

impl<F: FieldExt> std::ops::Not for BinaryQuery<F> {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(Query::one() - self.0)
    }
}