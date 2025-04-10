//! This module provides functionality to flatten a hierarchical intermediate representation (IR)
//! of a syntax tree into a flat representation. The primary purpose of this process is to
//! simplify the structure of the IR by expanding nested components and incorporating their
//! equations and subcomponents into a single flat class definition.
//!
//! The main function in this module is `flatten`, which takes a stored definition of the IR
//! and produces a flattened class definition. The process involves:
//!
//! - Identifying the main class and other class definitions from the provided IR.
//! - Iteratively expanding components in the main class that reference other class definitions.
//! - Propagating equations and subcomponents from referenced classes into the main class.
//! - Removing expanded components from the main class to ensure a flat structure.
//!
//! This module relies on visitors such as `ScopePusher` and `SubCompNamer` to handle
//! scoping and naming during the flattening process.
//!
//! # Dependencies
//! - `anyhow::Result`: For error handling.
//! - `indexmap::IndexMap`: To maintain the order of class definitions and components.
//!

use crate::ir;
use crate::ir::visitor::Visitable;
use crate::ir::visitors::scope_pusher::ScopePusher;
use crate::ir::visitors::sub_comp_namer::SubCompNamer;
use anyhow::Result;
use indexmap::IndexSet;

pub fn flatten(
    def: &ir::ast::StoredDefinition,
    model_class: &str,
) -> Result<ir::ast::ClassDefinition> {
    let parts: Vec<&str> = model_class.split('.').collect();
    if parts.len() < 1 {
        panic!(
            "Model class name '{}' Please provide a valid class name.",
            model_class
        );
    }

    let mut part_class = def.classes.get(parts[0]).unwrap_or_else(|| {
        panic!(
        "Class '{}' not found in the provided definition. Ensure that the class name is correct and exists in the IR.",
        parts[0]
        )
    });

    for i in 1..parts.len() {
        part_class = part_class.classes.get(parts[i]).unwrap_or_else(|| {
            panic!(
            "Class '{}' not found in the provided definition. Ensure that the class name is correct and exists in the IR.",
            parts[0]
            )
        });
    }

    let mut main_class = part_class.clone();
    main_class.name.text = parts.join("_");

    // create flat class
    let mut fclass = main_class.clone();

    //  handle extend clauses
    for extend in &main_class.extends {
        let class_name = extend.comp.to_string();
        let class = def
            .classes
            .get(&class_name)
            .expect(&format!("Class for extend '{}' not found", class_name));

        // add components
        for comp in &class.components {
            fclass.components.insert(comp.0.clone(), comp.1.clone());
        }

        // add equations
        for eq in &class.equations {
            fclass.equations.push(eq.clone());
        }
    }

    // expaand connection equations
    for eq in &main_class.equations {
        if let ir::ast::Equation::Connect { .. } = eq {}
    }

    // flatten the class by expanding components
    let mut scope_pusher = ScopePusher {
        global_symbols: IndexSet::from([
            "time".to_string(),
            "der".to_string(),
            "pre".to_string(),
            "cos".to_string(),
            "sin".to_string(),
            "tan".to_string(),
        ]),
        symbols: IndexSet::new(),
        comp: model_class.to_string(),
    };

    // for each component in the main class
    for (comp_name, comp) in &main_class.components {
        // if the the component type is a class
        if def.classes.contains_key(&comp.type_name.to_string()) {
            let comp_class = def.classes.get(&comp.type_name.to_string()).unwrap();

            // add equation from component to flat class
            for eq in &comp_class.equations {
                let mut feq = eq.clone();
                scope_pusher.comp = comp_name.clone();
                feq.accept(&mut scope_pusher);
                fclass.equations.push(feq);
            }

            // expand comp.sub_comp names to use underscores
            fclass.accept(&mut SubCompNamer {
                comp: comp_name.clone(),
            });

            // add subcomponents from component to flat class
            for (subcomp_name, subcomp) in &comp_class.components {
                let mut scomp = subcomp.clone();
                let name = format!("{}_{}", comp_name, subcomp_name);
                scomp.name = name.clone();
                fclass.components.insert(name, scomp);
            }

            // remove compoment from flat class, as it has been expanded
            fclass.components.swap_remove(comp_name);
        }
    }
    Ok(fclass)
}
