mod payload;
use payload::prepare;

mod ncbi;
use ncbi::taxonomy_tree::{self, Tree};
use ncbi::entry_mapper::{self, Entries};

use neon::prelude::*;
use std::error::Error;
use lazy_static::lazy_static;

const NODE_PATH: &str = "lib/nodes.dmp";
const NAME_PATH: &str = "lib/names.dmp";
const MAP_PATH: &str = "lib/cluster.tsv";

lazy_static! {
    static ref TREE: Tree = {
        prepare::prepare().unwrap();
        let mut tree = taxonomy_tree::build(NODE_PATH.to_string()).unwrap();
        taxonomy_tree::add_name(&mut tree, NAME_PATH.to_string()).unwrap();
        tree
    };
    static ref ENTRIES: Entries = {
        prepare::prepare().unwrap();
        entry_mapper::map(MAP_PATH.to_string()).unwrap()
    };
}

fn update_dfs(ex: &mut [bool], tree: &Tree, entries: &Entries, xid: u32) {
    if let Some(v) = entries.get_clu(xid) {
        for &clu in &v {
            ex[clu] = true;
        }
    }
    for ch in tree.get_children(xid) {
        update_dfs(ex, tree, entries, ch);
    }
}
fn entry(taxon_id: u32) -> Result<Vec<String>, Box<dyn Error>> {
    let mut ex = vec![false; ENTRIES.clu_size()];
    update_dfs(&mut ex, &TREE, &ENTRIES, taxon_id);
    let mut ret = ex.iter().enumerate().filter(|(_, &x)| x).map(|(i, _)| ENTRIES.clu_name(i)).collect::<Vec<_>>();
    ret.sort_unstable();
    Ok(ret)
}
fn get_protein_accessions(mut cx: FunctionContext) -> JsResult<JsArray> {
    let handle = cx.argument::<JsNumber>(0)?;
    let taxon_id = handle.value(&mut cx) as u32;

    // Dummy implementation: Based on taxon_id, return some protein accessions
    let accessions = entry(taxon_id).unwrap();

    let js_array = JsArray::new(&mut cx, accessions.len() as u32);
    for (i, accession) in accessions.iter().enumerate() {
        let js_string = cx.string(accession);
        js_array.set(&mut cx, i as u32, js_string)?;
    }

    Ok(js_array)
}


#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("getProteinAccessions", get_protein_accessions)?;
    Ok(())
}
