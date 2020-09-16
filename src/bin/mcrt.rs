//! MCRT binary.

use arctk::{
    args,
    file::{
        Build,
        Load,
        // Save
    },
    geom::{Mesh, MeshBuilder, Tree, TreeBuilder},
    ord::Set,
    util::{banner, dir},
};
use arctk_attr::input;
use mcrt::input::Settings;
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};

/// Key type.
type Key = String;

/// Input parameters.
#[input]
struct Parameters {
    /// Adaptive mesh settings.
    tree: TreeBuilder,
    /// MCRT runtime settings.
    sett: Settings,
    /// Surfaces set.
    surfs: Set<Key, MeshBuilder>,
    // /// Attributes set.
    // attrs: Set<Key, Attributes>,
}

fn main() {
    let term_width = arctk::util::term::width().unwrap_or(80);

    banner::title("MCRT", term_width);
    let (params_path, in_dir, _out_dir) = init(term_width);
    let params = input(term_width, &in_dir, &params_path);
    let (
        tree_sett,
        _mcrt_sett,
        surfs,
        //  attrs
    ) = build(term_width, &in_dir, params);
    let _tree = grow(term_width, tree_sett, &surfs);
    // // let input = Scene::new(&tree, &mcrt_sett, &surfs, &attrs);
    // banner::section("Shining", term_width);
    // let output = multi_thread(&input, &shader, &cam).expect("Failed to perform rendering.");
    // banner::section("Saving", term_width);
    // output
    //     .img
    //     .save(&out_dir.join("render.png"))
    //     .expect("Failed to save output data.");
    banner::section("Finished", term_width);
}

/// Initialise the command line arguments and directories.
fn init(term_width: usize) -> (PathBuf, PathBuf, PathBuf) {
    banner::section("Initialisation", term_width);
    banner::sub_section("Command line arguments", term_width);
    args!(bin_path: PathBuf;
        params_path: PathBuf
    );
    println!("{:>32} : {}", "binary path", bin_path.display());
    println!("{:>32} : {}", "parameters path", params_path.display());

    banner::sub_section("Directories", term_width);
    let cwd = current_dir().expect("Failed to determine current working directory.");
    let (in_dir, out_dir) = dir::io_dirs(Some(cwd.join("input")), Some(cwd.join("output")))
        .expect("Failed to initialise directories.");
    println!("{:>32} : {}", "input directory", in_dir.display());
    println!("{:>32} : {}", "output directory", out_dir.display());

    (params_path, in_dir, out_dir)
}

/// Load the input files.
fn input(term_width: usize, in_dir: &Path, params_path: &Path) -> Parameters {
    banner::section("Input", term_width);
    banner::sub_section("Parameters", term_width);
    let path = in_dir.join(params_path);

    Parameters::load(&path).expect("Failed to load parameters file.")
}

/// Build instances.
#[allow(clippy::type_complexity)]
fn build(
    term_width: usize,
    in_dir: &Path,
    params: Parameters,
) -> (
    TreeBuilder,
    Settings,
    Set<Key, Mesh>,
    // Set<Key, Attributes>,
) {
    banner::section("Building", term_width);
    banner::sub_section("Adaptive Tree Settings", term_width);
    let tree_sett = params.tree;

    banner::sub_section("MCRT Settings", term_width);
    let mcrt_sett = params.sett;

    banner::sub_section("Surfaces", term_width);
    let surfs = params
        .surfs
        .build(in_dir)
        .expect("Failed to build surfaces.");

    // banner::sub_section("Attributes", term_width);
    // let attrs = params.attrs;

    (
        tree_sett, mcrt_sett, surfs,
        // attrs,
    )
}

/// Grow domains.
fn grow<'a>(term_width: usize, tree: TreeBuilder, surfs: &'a Set<Key, Mesh>) -> Tree<'a, &Key> {
    banner::section("Growing", term_width);

    banner::sub_section("Adaptive Tree", term_width);
    let tree = tree.build(&surfs);

    tree
}
