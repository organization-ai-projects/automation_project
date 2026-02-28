#![allow(dead_code)]
use crate::schedule::system_context::SystemContext;
use crate::schedule::system_id::SystemId;
use crate::schedule::system_stage::SystemStage;

pub trait System: Send + Sync {
    fn id(&self) -> SystemId;
    fn stage(&self) -> SystemStage;
    fn run(&self, ctx: &mut SystemContext<'_>);
}
