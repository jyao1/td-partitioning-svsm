// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2022-2023 SUSE LLC
//
// Author: Joerg Roedel <jroedel@suse.de>

use crate::utils::immut_after_init::ImmutAfterInitRef;
use bitflags::bitflags;
use core::arch::x86_64::__cpuid;
use cpuarch::snp_cpuid::SnpCpuidTable;
use log;

static CPUID_PAGE: ImmutAfterInitRef<'_, SnpCpuidTable> = ImmutAfterInitRef::uninit();

pub fn register_cpuid_table(table: &'static SnpCpuidTable) {
    CPUID_PAGE
        .init_from_ref(table)
        .expect("Could not initialize CPUID page");
}

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct CpuidLeaf {
    pub cpuid_fn: u32,
    pub cpuid_subfn: u32,
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}

impl CpuidLeaf {
    pub fn new(cpuid_fn: u32, cpuid_subfn: u32) -> Self {
        CpuidLeaf {
            cpuid_fn,
            cpuid_subfn,
            eax: 0,
            ebx: 0,
            ecx: 0,
            edx: 0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct CpuidResult {
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
}

pub fn cpuid(eax: u32) -> Option<CpuidResult> {
    unsafe {
        let result = __cpuid(eax);
        Some(CpuidResult {
            eax: result.eax,
            ebx: result.ebx,
            ecx: result.ecx,
            edx: result.edx,
        })
    }
}

#[allow(unreachable_code, unused)]
pub fn cpuid_table_raw(eax: u32, ecx: u32, xcr0: u64, xss: u64) -> Option<CpuidResult> {
    panic!("cpuid_table_raw not supported");
    let count: usize = CPUID_PAGE.count as usize;

    for i in 0..count {
        if eax == CPUID_PAGE.func[i].eax_in
            && ecx == CPUID_PAGE.func[i].ecx_in
            && xcr0 == CPUID_PAGE.func[i].xcr0_in
            && xss == CPUID_PAGE.func[i].xss_in
        {
            return Some(CpuidResult {
                eax: CPUID_PAGE.func[i].eax_out,
                ebx: CPUID_PAGE.func[i].ebx_out,
                ecx: CPUID_PAGE.func[i].ecx_out,
                edx: CPUID_PAGE.func[i].edx_out,
            });
        }
    }

    None
}

pub fn cpuid_table(eax: u32) -> Option<CpuidResult> {
    cpuid_table_raw(eax, 0, 0, 0)
}

#[allow(unreachable_code, unused)]
pub fn dump_cpuid_table() {
    panic!("dump_cpuid_table not supported");
    let count = CPUID_PAGE.count as usize;

    log::trace!("CPUID Table entry count: {}", count);

    for i in 0..count {
        let eax_in = CPUID_PAGE.func[i].eax_in;
        let ecx_in = CPUID_PAGE.func[i].ecx_in;
        let xcr0_in = CPUID_PAGE.func[i].xcr0_in;
        let xss_in = CPUID_PAGE.func[i].xss_in;
        let eax_out = CPUID_PAGE.func[i].eax_out;
        let ebx_out = CPUID_PAGE.func[i].ebx_out;
        let ecx_out = CPUID_PAGE.func[i].ecx_out;
        let edx_out = CPUID_PAGE.func[i].edx_out;
        log::trace!("EAX_IN: {:#010x} ECX_IN: {:#010x} XCR0_IN: {:#010x} XSS_IN: {:#010x} EAX_OUT: {:#010x} EBX_OUT: {:#010x} ECX_OUT: {:#010x} EDX_OUT: {:#010x}",
                    eax_in, ecx_in, xcr0_in, xss_in, eax_out, ebx_out, ecx_out, edx_out);
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct Cpuid01Ecx: u32 {
        const SSE3          = 1 << 0;
        const PCLMULQDQ     = 1 << 1;
        const DTES64        = 1 << 2;
        const MONITOR       = 1 << 3;
        const DS_CPL        = 1 << 4;
        const VMX           = 1 << 5;
        const SMX           = 1 << 6;
        const EIST          = 1 << 7;
        const TM2           = 1 << 8;
        const SSSE3         = 1 << 9;
        const CNXT_ID       = 1 << 10;
        const SDBG          = 1 << 11;
        const FMA           = 1 << 12;
        const CMPXCHG16B    = 1 << 13;
        const XTPRUC        = 1 << 14;
        const PDCM          = 1 << 15;
        const PCID          = 1 << 17;
        const DCA           = 1 << 18;
        const SSE4_1        = 1 << 19;
        const SSE4_2        = 1 << 20;
        const X2APIC        = 1 << 21;
        const MOVBE         = 1 << 22;
        const POPCNT        = 1 << 23;
        const TSC_DEADLINE  = 1 << 24;
        const AESNI         = 1 << 25;
        const XSAVE         = 1 << 26;
        const OSXSAVE       = 1 << 27;
        const AVX           = 1 << 28;
        const F16C          = 1 << 29;
        const RDRAND        = 1 << 30;
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct Cpuid01Edx: u32 {
        const FPU       = 1 << 0;
        const VME       = 1 << 1;
        const DE        = 1 << 2;
        const PSE       = 1 << 3;
        const TSC       = 1 << 4;
        const MSR       = 1 << 5;
        const PAE       = 1 << 6;
        const MCE       = 1 << 7;
        const CX8       = 1 << 8;
        const APIC      = 1 << 9;
        const SEP       = 1 << 11;
        const MTRR      = 1 << 12;
        const PGE       = 1 << 13;
        const MCA       = 1 << 14;
        const CMOV      = 1 << 15;
        const PAT       = 1 << 16;
        const PSE36     = 1 << 17;
        const PSN       = 1 << 18;
        const CLFSH     = 1 << 19;
        const DTES      = 1 << 21;
        const ACPI      = 1 << 22;
        const MMX       = 1 << 23;
        const FXSR      = 1 << 24;
        const SSE       = 1 << 25;
        const SSE2      = 1 << 26;
        const SS        = 1 << 27;
        const HTT       = 1 << 28;
        const TM        = 1 << 29;
        const PBE       = 1 << 31;
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct Cpuid06Eax: u32 {
        const HWP       = 1 << 7;
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct Cpuid06Ecx: u32 {
        const HCF       = 1 << 0;
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct Cpuid07_0Ebx: u32 {
        const FSGSBASE      = 1 << 0;
        const TSC_ADJUST    = 1 << 1;
        const SGX           = 1 << 2;
        const BMI1          = 1 << 3;
        const HLT           = 1 << 4;
        const AVX2          = 1 << 5;
        const FDP_EXCP_ONLY = 1 << 6;
        const SMEP          = 1 << 7;
        const BMI2          = 1 << 8;
        const ERMS          = 1 << 9;
        const INVPCID       = 1 << 10;
        const RTM           = 1 << 11;
        const RDTM          = 1 << 12;
        const DFPU_CS_DS    = 1 << 13;
        const MPX           = 1 << 14;
        const RDTA          = 1 << 15;
        const AVX512F       = 1 << 16;
        const AVX512DQ      = 1 << 17;
        const RDSEED        = 1 << 18;
        const ADX           = 1 << 19;
        const SMAP          = 1 << 20;
        const AVX512_IFMA   = 1 << 21;
        const CLFLUSHOPT    = 1 << 23;
        const CLWB          = 1 << 24;
        const IPT           = 1 << 25;
        const AVX512PF      = 1 << 26;
        const AVX512ER      = 1 << 27;
        const AVX512CD      = 1 << 28;
        const SHA           = 1 << 29;
        const AVX512BW      = 1 << 30;
        const AVX512VL      = 1 << 31;
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct Cpuid07_0Ecx: u32 {
        const PREFETCHWT1       = 1 << 0;
        const AVX512_VBMI       = 1 << 1;
        const UMIP              = 1 << 2;
        const PKU               = 1 << 3;
        const OSPKE             = 1 << 4;
        const WAITPKG           = 1 << 5;
        const AVX512_VBMI2      = 1 << 6;
        const CET_SS            = 1 << 7;
        const GFNI              = 1 << 8;
        const VAES              = 1 << 9;
        const VPCLMULQDQ        = 1 << 10;
        const AVX512_VNNI       = 1 << 11;
        const AVX512_BITALG     = 1 << 12;
        const TME_EN            = 1 << 13;
        const AVX512_VPOPCNTDQ  = 1 << 14;
        const LA57              = 1 << 16;
        const RDPID_TSCAUX      = 1 << 22;
        const KL                = 1 << 23;
        const BUS_LOCK_DETECT   = 1 << 24;
        const CLDEMOTE          = 1 << 25;
        const MOVDIRI           = 1 << 27;
        const MOVDIRI64B        = 1 << 28;
        const ENQCMD            = 1 << 29;
        const SGX_LC            = 1 << 30;
        const PKS               = 1 << 31;
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct Cpuid07_0Edx: u32 {
        const SGX_KEYS              = 1 << 1;
        const AVX512_4VNNIW         = 1 << 2;
        const AVX512_4FMAPS         = 1 << 3;
        const FSRM                  = 1 << 4;
        const UINTR                 = 1 << 5;
        const AVX512_VP2INTERSECT   = 1 << 8;
        const SRBDS_CTRL            = 1 << 9;
        const MD_CLEAR              = 1 << 10;
        const RTM_ALWAYS_ABORT      = 1 << 11;
        const RTM_FORCE_ABORT       = 1 << 13;
        const SERIALIZE             = 1 << 14;
        const HYBRID                = 1 << 15;
        const TSXLDTRK              = 1 << 16;
        const PCONFIG               = 1 << 18;
        const LBR                   = 1 << 19;
        const CET_IBT               = 1 << 20;
        const AMX_BF16              = 1 << 22;
        const AVX512_FP16           = 1 << 23;
        const AMX_TILE              = 1 << 24;
        const AMX_INT8              = 1 << 25;
        const IBRS_IBPB             = 1 << 26;
        const STIBP                 = 1 << 27;
        const L1D_FLUSH             = 1 << 28;
        const ARCH_CAP              = 1 << 29;
        const CORE_CAP              = 1 << 30;
        const SSBD                  = 1 << 31;
    }
}

bitflags! {
    pub struct Cpuid40000001Eax: u32 {
        const KVM_FEATURE_PV_SEND_IPI = 1 << 11;
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct Cpuid80000001Edx: u32 {
        const SYSCALL       = 1 << 11;
        const NX            = 1 << 20;
        const GBPAGES       = 1 << 26;
        const RDTSC         = 1 << 27;
        const LM            = 1 << 29;
    }
}
