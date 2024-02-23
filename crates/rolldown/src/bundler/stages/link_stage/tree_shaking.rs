use index_vec::IndexVec;
use rolldown_common::{ModuleId, StmtInfoId, SymbolRef};

use crate::bundler::{
  module::{Module, ModuleVec, NormalModule},
  utils::symbols::Symbols,
};

use super::LinkStage;

struct Context<'a> {
  modules: &'a ModuleVec,
  symbols: &'a Symbols,
  is_included_vec: &'a mut IndexVec<ModuleId, IndexVec<StmtInfoId, bool>>,
  is_module_included_vec: &'a mut IndexVec<ModuleId, bool>,
  tree_shaking: bool,
  runtime_id: ModuleId,
}

fn include_module(ctx: &mut Context, module: &NormalModule) {
  let is_included = ctx.is_module_included_vec[module.id];
  if is_included {
    return;
  }

  ctx.is_module_included_vec[module.id] = true;

  if ctx.tree_shaking || module.id == ctx.runtime_id {
    module.stmt_infos.iter_enumerated().for_each(|(stmt_info_id, stmt_info)| {
      if stmt_info.side_effect {
        include_statement(ctx, module, stmt_info_id);
      }
    });
  } else {
    module.stmt_infos.iter_enumerated().for_each(|(stmt_info_id, _stmt_info)| {
      if stmt_info_id.index() == 0 {
        return;
      }
      include_statement(ctx, module, stmt_info_id);
    });
  }

  module.import_records.iter().for_each(|import_record| {
    let importee = &ctx.modules[import_record.resolved_module];
    if let Module::Normal(importee) = importee {
      include_module(ctx, importee);
    }
  });
}

fn include_symbol(ctx: &mut Context, symbol_ref: SymbolRef) {
  let mut canonical_ref = ctx.symbols.par_canonical_ref_for(symbol_ref);
  let canonical_ref_module = &ctx.modules[canonical_ref.owner];
  let canonical_ref_symbol = ctx.symbols.get(canonical_ref);
  if let Some(namespace_alias) = &canonical_ref_symbol.namespace_alias {
    canonical_ref = namespace_alias.namespace_ref;
  }
  let Module::Normal(canonical_ref_module) = canonical_ref_module else {
    return;
  };
  include_module(ctx, canonical_ref_module);
  canonical_ref_module
    .stmt_infos
    .declared_stmts_by_symbol(&canonical_ref)
    .iter()
    .copied()
    .for_each(|stmt_info_id| {
      include_statement(ctx, canonical_ref_module, stmt_info_id);
    });
}

fn include_statement(ctx: &mut Context, module: &NormalModule, stmt_info_id: StmtInfoId) {
  let is_included = &mut ctx.is_included_vec[module.id][stmt_info_id];
  if *is_included {
    return;
  }

  // include the statement itself
  *is_included = true;

  let stmt_info = module.stmt_infos.get(stmt_info_id);

  // include statements that are referenced by this statement
  stmt_info.declared_symbols.iter().chain(stmt_info.referenced_symbols.iter()).for_each(
    |symbol_ref| {
      include_symbol(ctx, *symbol_ref);
    },
  );
}

impl LinkStage<'_> {
  pub fn include_statements(&mut self) {
    use rayon::prelude::*;

    let mut is_included_vec: IndexVec<ModuleId, IndexVec<StmtInfoId, bool>> = self
      .modules
      .iter()
      .map(|m| match m {
        Module::Normal(m) => {
          m.stmt_infos.iter().map(|_| false).collect::<IndexVec<StmtInfoId, _>>()
        }
        Module::External(_) => IndexVec::default(),
      })
      .collect::<IndexVec<ModuleId, _>>();

    let mut is_module_included_vec: IndexVec<ModuleId, bool> =
      index_vec::index_vec![false; self.modules.len()];

    let context = &mut Context {
      modules: &self.modules,
      symbols: &self.symbols,
      is_included_vec: &mut is_included_vec,
      is_module_included_vec: &mut is_module_included_vec,
      tree_shaking: self.input_options.treeshake,
      runtime_id: self.runtime.id(),
    };

    for module in &self.modules {
      match module {
        Module::Normal(module) => {
          let mut stmt_infos = module.stmt_infos.iter_enumerated();
          // Skip the first one, because it's the namespace variable declaration.
          // We want to include it on demand.
          stmt_infos.next();
          stmt_infos.for_each(|(stmt_info_id, stmt_info)| {
            if stmt_info.side_effect {
              include_statement(context, module, stmt_info_id);
            }
          });
        }
        Module::External(_) => {}
      }
    }

    self.entries.iter().for_each(|entry| {
      let module = &self.modules[entry.id];
      let Module::Normal(module) = module else {
        return;
      };

      include_module(context, module);

      let linking_info = &self.metas[module.id];
      linking_info.sorted_exports().for_each(|(_, resolved_export)| {
        include_symbol(context, resolved_export.symbol_ref);
      });
    });

    self.modules.iter_mut().par_bridge().for_each(|module| {
      let Module::Normal(module) = module else {
        return;
      };
      module.is_included = is_module_included_vec[module.id];
      is_included_vec[module.id].iter_enumerated().for_each(|(stmt_info_id, is_included)| {
        module.stmt_infos.get_mut(stmt_info_id).is_included = *is_included;
      });
    });

    tracing::trace!(
      "included statements {:#?}",
      self
        .modules
        .iter()
        .filter_map(|m| m.as_normal())
        .map(NormalModule::to_debug_normal_module_for_tree_shaking)
        .collect::<Vec<_>>()
    );
  }
}