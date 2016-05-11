use error::*;
use rustc_mir;
use rustc::mir::mir_map::MirMap;
use rustc::ty::{self, TyCtxt};
use rustc::hir::intravisit::{self, Visitor, FnKind};
use rustc::hir::{FnDecl, Block};
use syntax::ast::NodeId;
use syntax::codemap::Span;
use std::ffi::CString;
use std::ptr;
use std::collections::HashMap;
use binaryen::*;

pub fn translate_crate<'tcx>(tcx: &TyCtxt<'tcx>,
                             mir_map: &MirMap<'tcx>) -> Result<()> {

    let module: BinaryenModuleRef;

    unsafe {
        module = BinaryenModuleCreate();
    }

    let mut fun_types = HashMap::new();

    for (&id, mir) in &mir_map.map {
        for attr in tcx.map.attrs(id) {
            let item = tcx.map.expect_item(id);

            println!("Processing function ({}): {}", id, item.name);

            unsafe {
                let sig = "ii"; // TODO: compute from function

                if !fun_types.contains_key(sig) {
                    let param = BinaryenInt32();
                    let ty = BinaryenAddFunctionType(module, CString::new(sig).unwrap().as_ptr(), BinaryenInt32(), &param, 1);
                    fun_types.insert(sig, ty);
                }

                let add = BinaryenBinary(module, BinaryenAdd(), BinaryenGetLocal(module, 0, BinaryenInt32()), BinaryenGetLocal(module, 0, BinaryenInt32()));

                BinaryenAddFunction(module, CString::new(item.name.as_str().as_bytes()).unwrap().as_ptr(), *fun_types.get(sig).unwrap(), ptr::null(), 0, add);
            }

        }
    }

    unsafe {
        BinaryenModulePrint(module);
        BinaryenModuleDispose(module);
    }

    Ok(())
}

