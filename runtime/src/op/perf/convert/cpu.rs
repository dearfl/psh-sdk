use crate::op::perf::convert::Wrap;

type FromT = profiling_prelude_perf_types::config::Cpu;
type IntoT = perf_event_rs::config::Cpu;

impl From<&FromT> for Wrap<IntoT> {
    fn from(value: &FromT) -> Self {
        #[rustfmt::skip]
        let val = match value {
            FromT::Any   => IntoT::Any,
            FromT::Id(n) => IntoT::Id(*n) ,
        };
        Self(val)
    }
}