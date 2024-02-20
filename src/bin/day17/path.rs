use std::collections::HashMap;
use crate::block::{CityBlock, Heat};

#[derive(Debug)]
pub(crate) struct CityMapPath {
    map: HashMap::<CityBlock,(Heat, Option<CityBlock>)>,
    total_heat_loss: Heat,
    target: CityBlock
}

impl CityMapPath {
    pub(crate) fn total_heat_loss(&self) -> Heat { self.total_heat_loss }
    pub(crate) fn new(map: HashMap::<CityBlock,(Heat, Option<CityBlock>)>, target: CityBlock) -> CityMapPath {
        let total_heat_loss = map[&target].0;
        CityMapPath { map, total_heat_loss, target}
    }
    pub(crate) fn iter(&self) -> PathIter {
        PathIter { path: self, current: Some(self.target) }
    }
}

pub(crate) struct PathIter<'a> {
    path: &'a CityMapPath,
    current: Option<CityBlock>
}

impl Iterator for PathIter<'_> {
    type Item = (Heat, CityBlock);

    fn next(&mut self) -> Option<Self::Item> {
        self.current
            .map(|current|{
                let (heat, parent) = self.path.map[&current];
                self.current = parent;
                (heat,current)
            })
    }
}
