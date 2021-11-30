mod bufreader;
mod bufwriter;
mod dirwriter;
mod iotrap;
mod link;

use crate::vm::VirtualMachine;
use pangalactic_store::Store;
use pangalactic_wasmi::HostFuncResolver;

pub fn new_hostfunc_resolver<S>() -> HostFuncResolver<VirtualMachine<S>>
where
    S: Store + 'static,
{
    let mut hfr = HostFuncResolver::new();

    // BufWriter:
    hfr.add_host_fn0(self::bufwriter::bufwriter_new);
    hfr.add_host_fn3(self::bufwriter::bufwriter_write);
    hfr.add_host_fn1(self::bufwriter::bufwriter_commit);

    // DirWriter:
    hfr.add_host_fn0(self::dirwriter::dirwriter_new);

    // Link:
    hfr.add_host_fn1(self::link::link_kind);
    hfr.add_host_fn2(self::link::link_eq);
    hfr.add_host_fn1(self::link::link_load_file);

    // BufReader:
    hfr.add_host_fn3(self::bufreader::bufreader_read);

    log::debug!("Instantiated derive resolver: {:#?}", &hfr);
    hfr
}
