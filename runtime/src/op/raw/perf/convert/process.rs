use crate::op::raw::perf::convert::Wrap;

type FromT = profiling_prelude_perf_types::config::Process;
type IntoT = perf_event_rs::config::Process;

impl From<&FromT> for Wrap<IntoT> {
    fn from(value: &FromT) -> Self {
        #[rustfmt::skip]
        let val = match value {
            FromT::Any     => IntoT::Any,
            FromT::Current => IntoT::Current,
            FromT::Pid(n)  => IntoT::Pid(*n),
        };
        Self(val)
    }
}
