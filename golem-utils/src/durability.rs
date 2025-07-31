use std::fmt::{Display, Formatter};

use golem_rust::{
    bindings::golem::durability::durability::DurableFunctionType,
    durability::Durability,
    value_and_type::{FromValueAndType, IntoValue},
    with_persistence_level, PersistenceLevel,
};

#[derive(Debug, golem_rust::FromValueAndType, golem_rust::IntoValue)]
struct UnusedError;

impl Display for UnusedError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "UnusedError")
    }
}

fn run_durably<F, P, R>(
    func_type: DurableFunctionType,
    persistence_level: PersistenceLevel,
    interface: &'static str,
    func_name: &'static str,
    params: P,
    func: F,
) -> R
where
    P: IntoValue + std::fmt::Debug + Clone,
    F: FnOnce(P) -> R,
    R: std::fmt::Debug + IntoValue + FromValueAndType + Clone,
{
    // if durability flag is enabled then run with custom durability
    #[cfg(feature = "durability")]
    {
        let durability = Durability::<R, UnusedError>::new(interface, func_name, func_type);
        if durability.is_live() {
            let result = with_persistence_level(persistence_level, || func(params.clone()));

            durability.persist_infallible(params, result)
        } else {
            durability.replay_infallible()
        }
    }

    #[cfg(not(feature = "durability"))]
    {
        func(params)
    }
}

pub fn read_remote_durably<F, P, R>(
    interface: &'static str,
    func_name: &'static str,
    params: P,
    func: F,
) -> R
where
    P: IntoValue + std::fmt::Debug + Clone,
    F: FnOnce(P) -> R,
    R: std::fmt::Debug + IntoValue + FromValueAndType + Clone,
{
    run_durably(
        DurableFunctionType::ReadRemote,
        PersistenceLevel::PersistNothing,
        interface,
        func_name,
        params,
        func,
    )
}

pub fn write_remote_durably<F, P, R>(
    interface: &'static str,
    func_name: &'static str,
    params: P,
    func: F,
) -> R
where
    P: IntoValue + std::fmt::Debug + Clone,
    F: FnOnce(P) -> R,
    R: std::fmt::Debug + IntoValue + FromValueAndType + Clone,
{
    run_durably(
        DurableFunctionType::WriteRemote,
        PersistenceLevel::PersistNothing,
        interface,
        func_name,
        params,
        func,
    )
}

pub fn write_remote_durably_with_side_effects<F, P, R>(
    interface: &'static str,
    func_name: &'static str,
    params: P,
    func: F,
) -> R
where
    P: IntoValue + std::fmt::Debug + Clone,
    F: FnOnce(P) -> R,
    R: std::fmt::Debug + IntoValue + FromValueAndType + Clone,
{
    run_durably(
        DurableFunctionType::WriteRemote,
        PersistenceLevel::PersistRemoteSideEffects,
        interface,
        func_name,
        params,
        func,
    )
}

pub fn read_remote_durably_with_side_effects<F, P, R>(
    interface: &'static str,
    func_name: &'static str,
    params: P,
    func: F,
) -> R
where
    P: IntoValue + std::fmt::Debug + Clone,
    F: FnOnce(P) -> R,
    R: std::fmt::Debug + IntoValue + FromValueAndType + Clone,
{
    run_durably(
        DurableFunctionType::ReadRemote,
        PersistenceLevel::PersistRemoteSideEffects,
        interface,
        func_name,
        params,
        func,
    )
}
