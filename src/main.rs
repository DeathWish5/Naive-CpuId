use std::arch::asm;

#[derive(Default)]
struct CpuId {
    eax: usize,
    ebx: usize,
    ecx: usize,
    edx: usize,
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

fn cpu_id(eax: usize, ecx: usize) -> CpuId {
    let mut cpuid = CpuId::default();
    unsafe {
        asm!(
            "cpuid",
            inout("eax") eax => cpuid.eax,
            // out("ebx") cpuid.ebx,
            inout("ecx") ecx => cpuid.ecx,
            out("edx") cpuid.edx,
        );
    }
    cpuid
}

pub fn main() {
    println!("CpuId(eax = 7, ecx = 0) = {}", cpu_id(7, 0));
}
