use std::arch::asm;

#[derive(Default)]
struct CpuId {
    eax: u32,
    ebx: u32,
    ecx: u32,
    edx: u32,
}

impl std::fmt::Display for CpuId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{ eax:{:x?}, ebx:{:x?}, ecx:{:x?}, edx:{:x?} }}",
            self.eax, self.ebx, self.ecx, self.edx
        )
    }
}

fn cpu_id(eax: u32, ecx: u32) -> CpuId {
    let mut cpuid = CpuId::default();
    unsafe {
        asm!(
            "cpuid",
            "mov {val}, rbx",
            val = out(reg) cpuid.ebx,
            inout("eax") eax => cpuid.eax,
            inout("ecx") ecx => cpuid.ecx,
            out("edx") cpuid.edx,
        );
    }
    cpuid
}

pub fn main() {
    println!("CpuId(eax = 7, ecx = 0) = {}", cpu_id(7, 0));
}
