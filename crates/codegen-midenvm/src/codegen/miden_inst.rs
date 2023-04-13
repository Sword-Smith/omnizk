use derive_more::From;
use derive_more::Into;

#[derive(Debug, Clone, Into, From)]
pub struct MidenInst(String);

pub struct MidenAssemblyBuilder {}

impl MidenAssemblyBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn begin(&self) -> MidenInst {
        "begin".to_string().into()
    }

    pub fn proc(&self, name: String) -> MidenInst {
        format!("proc.{name}").into()
    }

    pub fn exec(&self, name: String) -> MidenInst {
        format!("exec.{name}").into()
    }

    pub fn push(&self, num: i64) -> MidenInst {
        format!("push.{num}").into()
    }

    pub fn adv_push(&self, num: u32) -> MidenInst {
        format!("adv_push.{num}").into()
    }

    pub fn end(&self) -> MidenInst {
        "end".to_string().into()
    }

    pub fn add(&self) -> MidenInst {
        "add".to_string().into()
    }

    pub fn while_true(&self) -> MidenInst {
        "while.true".to_string().into()
    }

    pub fn sdepth(&self) -> MidenInst {
        "sdepth".to_string().into()
    }

    pub fn dup(&self, idx: u8) -> MidenInst {
        format!("dup.{idx}").into()
    }

    pub fn swap(&self, idx: u8) -> MidenInst {
        format!("swap.{idx}").into()
    }

    pub fn mul(&self) -> MidenInst {
        "mul".to_string().into()
    }

    pub fn mem_store(&self) -> MidenInst {
        "mem_store".to_string().into()
    }

    pub(crate) fn mem_load(&self) -> MidenInst {
        "mem_load".to_string().into()
    }

    pub(crate) fn sub(&self) -> MidenInst {
        "sub".to_string().into()
    }
}

impl Default for MidenAssemblyBuilder {
    fn default() -> Self {
        Self::new()
    }
}
