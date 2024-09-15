use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{
    new_id,
    nfa::{Status, StatusBox, NFA},
    status_rules::StatusTargetRule,
};

type DFAStatusBox = Rc<RefCell<DFAStatus>>;

struct DFAStatus {
    id: usize,
    status_set: Vec<StatusBox>,
    next: Vec<(Box<dyn StatusTargetRule>, DFAStatusBox)>,
}

pub struct DFA {
    start: DFAStatusBox,
}

impl DFA {
    pub fn new() -> Self {
        Self {
            start: DFAStatus::boxed(Vec::new(), Vec::new()),
        }
    }
}

impl DFAStatus {
    pub fn boxed(
        status_set: Vec<StatusBox>,
        next: Vec<(Box<dyn StatusTargetRule>, DFAStatusBox)>,
    ) -> DFAStatusBox {
        Rc::new(RefCell::new(Self::new(status_set, next)))
    }

    pub fn new(
        status_set: Vec<StatusBox>,
        next: Vec<(Box<dyn StatusTargetRule>, DFAStatusBox)>,
    ) -> Self {
        Self {
            id: new_id(),
            status_set,
            next,
        }
    }
}

impl From<Vec<StatusBox>> for DFAStatus {
    fn from(value: Vec<StatusBox>) -> Self {
        Self::new(value, vec![])
    }
}

impl From<NFA> for DFA {
    fn from(value: NFA) -> Self {
        let dfa = NFA::new();
        let mut status = value.get_start();
        let mut record = HashSet::new();

        let mut buf: Vec<DFAStatus> = vec![vec![status].into()];

        while let Some(item) = buf.first() {
            if let None = record.get(&item.id) {
                continue;
            }
            record.insert(item.id);

            let mut next_set: Vec<Rc<RefCell<DFAStatusBox>>> = Vec::new();

            let targets = {
                item.status_set
                    .iter()
                    .map(|v| {
                        let temp = v.borrow();
                        temp.status_set
                            .iter()
                            .map(|s| s.0.clone())
                            .collect::<Vec<_>>()
                    })
                    .flatten()
                    .collect::<Vec<_>>()
            };

            for target in targets {
                let ta_set = Status::closure_t_a(&item.status_set, target);
                let ta_set: Vec<_> = ta_set
                    .iter()
                    .map(|v| Status::closure_s(&v))
                    .flatten()
                    .collect();

                
            }

            // for each in item.status_set {
            //     for target in targets {
            //         Status::closure_t_a(&each, item.0);
            //         let list = Status::closure_s(&item.1)
            //             .iter()
            //             .map(|v| v.into())
            //             .collect();
            //         next_set.extend(list);
            //     }

            //     // let new_status = DFAStatus::new(set, vec![]);
            //     // buf.push(new_status);
            // }
        }

        unimplemented!()
    }
}
