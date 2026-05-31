use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum Architecture {
    X86_64,
    Aarch64,
    Riscv64,
}
