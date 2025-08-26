use alohomora::policy::{Policy, PolicyAnd, Reason};
use tahini_tarpc::{TahiniDeserialize, TahiniSerialize};

#[derive(TahiniSerialize, TahiniDeserialize)]
pub struct EmailAddressPolicy;

impl Policy for EmailAddressPolicy {
    fn name(&self) -> String {
        "EmailAddressPolicy".to_string()
    }

    fn join(
        &self,
        other: alohomora::policy::AnyPolicy,
    ) -> Result<alohomora::policy::AnyPolicy, ()> {
        if other.is::<Self>() {
            Ok(Self.into_any())
        } else {
            Ok(PolicyAnd::new(Self, other).into_any())
        }
    }

    fn join_logic(&self, _other: Self) -> Result<Self, ()>
    where
        Self: Sized,
    {
        Ok(Self)
    }

    fn check(
        &self,
        _context: &alohomora::context::UnprotectedContext,
        reason: alohomora::policy::Reason<'_>,
    ) -> bool {
        if let Reason::Response = reason {
            true
        } else {
            false
        }
    }
}
