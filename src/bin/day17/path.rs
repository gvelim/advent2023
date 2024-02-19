use std::collections::HashMap;
use crate::block::{CityBlock, Heat};

#[derive(Debug)]
pub(crate) struct HeatLossPath {
    map: HashMap::<CityBlock,(Heat, Option<CityBlock>)>,
    parent: Option<CityBlock>,
    heat_loss_total: Heat
}

impl HeatLossPath {
    pub(crate) fn new(map: HashMap::<CityBlock,(Heat, Option<CityBlock>)>, target: CityBlock) -> HeatLossPath {
        let heat = map[&target].0;
        HeatLossPath { map, heat_loss_total: heat, parent: Some(target)}
    }
    pub(crate) fn heat_loss_total(&self) -> Heat { self.heat_loss_total }
}

impl Iterator for HeatLossPath {
    type Item = (Heat, CityBlock);

    fn next(&mut self) -> Option<Self::Item> {
        self.parent
            .map(|current| (current, self.map[&current]))
            .map(|(cur,(heat, parent))|{
                self.parent = parent;
                (heat,cur)
            })
    }
}
