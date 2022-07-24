use std::sync::Arc;

use super::{PlanNode, PlanRef, PlanTreeNode};
use crate::binder::{BoundAggFunc, BoundExpr};

#[derive(Debug, Clone)]
pub struct LogicalAgg {
    agg_funcs: Vec<BoundAggFunc>,
    group_by: Vec<BoundExpr>,
    input: PlanRef,
}

impl LogicalAgg {
    pub fn new(agg_funcs: Vec<BoundAggFunc>, group_by: Vec<BoundExpr>, input: PlanRef) -> Self {
        Self {
            agg_funcs,
            group_by,
            input,
        }
    }

    pub fn agg_funcs(&self) -> Vec<BoundAggFunc> {
        self.agg_funcs.clone()
    }

    pub fn group_by(&self) -> Vec<BoundExpr> {
        self.group_by.clone()
    }

    pub fn input(&self) -> PlanRef {
        self.input.clone()
    }
}

impl PlanNode for LogicalAgg {
    fn schema(&self) -> Vec<crate::catalog::ColumnCatalog> {
        self.input.schema()
    }
}

impl PlanTreeNode for LogicalAgg {
    fn children(&self) -> Vec<PlanRef> {
        vec![self.input.clone()]
    }

    fn clone_with_children(&self, children: Vec<PlanRef>) -> PlanRef {
        assert_eq!(children.len(), 1);
        Arc::new(Self::new(
            self.agg_funcs.clone(),
            self.group_by.clone(),
            children[0].clone(),
        ))
    }
}
