use std::{cell::RefCell, rc::Rc, str::FromStr};

type StatusBox = Rc<RefCell<Status>>;

const EMPTY: char = '\0';

static mut CURRENT_ID: usize = 0;

/// a NodeStatus must be ensure it's start off a start node, and end off a end node
pub struct NFA {
    start: StatusBox,
    end: StatusBox,
}

pub struct Status {
    id: usize,
    status_type: StatusType,
    pub status_set: Vec<(char, StatusBox)>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum StatusType {
    Start,
    Node,
    End,
}

fn new_id() -> usize {
    unsafe {
        CURRENT_ID += 1;

        CURRENT_ID
    }
}

impl NFA {
    /// creates a default NFA,
    /// representing ((start)) -EMPTY-> ((end))
    pub fn new() -> Self {
        let start_node = Status::start();
        let end_node = Status::end();

        RefCell::borrow_mut(&start_node).append_next(EMPTY, Rc::clone(&end_node));

        NFA {
            start: start_node,
            end: end_node,
        }
    }

    /// creates a NFA with text by default
    ///
    /// ((s)) -text-> ((e))
    pub fn with(text: char) -> Self {
        let nfa = Self::new();
        {
            let mut status = nfa.start.borrow_mut();
            assert_eq!(status.status_set.len(), 1);
            let record = status.status_set.get_mut(0).unwrap();

            record.0 = text;
        }
        nfa
    }

    pub fn get_start(&self) -> StatusBox {
        Rc::clone(&self.start)
    }

    /// connects two NFAs
    ///
    /// before:
    /// NFA: A, B
    /// A: ((s)) -a-> ((e))
    /// B: ((s)) -b-> ((e))
    ///
    /// after:
    /// ((s)) -a-> ((empty)) -empty-> ((empty)) -> -b-> ((e))
    pub fn and(&mut self, nfa: NFA) -> &mut Self {
        {
            let mut end1 = self.end.borrow_mut();
            end1.turn_to_empty();

            {
                let mut start2 = nfa.start.borrow_mut();
                start2.turn_to_empty();
            }

            end1.append_next(EMPTY, nfa.start);
        }
        self.end = nfa.end;
        return self;
    }
}

#[derive(Debug)]
pub enum NFAError {
    ParseWrong,
}

impl FromStr for NFA {
    type Err = NFAError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = NFA::new();
        for c in s.chars() {
            let next = NFA::with(c);
            start.and(next);
        }

        Ok(start)
    }
}

impl Status {
    pub fn start() -> StatusBox {
        Rc::new(RefCell::new(Self {
            id: new_id(),
            status_type: StatusType::Start,
            status_set: Vec::new(),
        }))
    }

    pub fn end() -> StatusBox {
        Rc::new(RefCell::new(Self {
            id: new_id(),
            status_type: StatusType::End,
            status_set: Vec::new(),
        }))
    }

    pub fn node() -> StatusBox {
        Rc::new(RefCell::new(Self {
            id: new_id(),
            status_type: StatusType::Node,
            status_set: Vec::new(),
        }))
    }

    pub fn append_next(&mut self, text: char, status: StatusBox) {
        {
            let mut temp = RefCell::borrow_mut(&status);
            if temp.status_type == StatusType::Start {
                temp.turn_to_empty();
            }
        }
        self.status_set.push((text, status));
    }

    pub fn turn_to_empty(&mut self) {
        self.status_type = StatusType::Node
    }

    pub fn next(&self, text: char) -> Option<StatusBox> {
        self.status_set.iter().find_map(|v| {
            if v.0 == text {
                Some(Rc::clone(&v.1))
            } else {
                None
            }
        })
    }

    pub fn next_skip_empty(&self, text: char) -> Option<StatusBox> {
        for status in self.status_set.iter() {
            let cur = match status {
                &(EMPTY, ref next) => {
                    let next = RefCell::borrow(&next);
                    next.next_skip_empty(text)
                }
                &(target, ref next) if target == text => Some(Rc::clone(next)),
                _ => None,
            };

            if cur.is_some() {
                return cur;
            }
        }

        None
    }

    pub fn get_type(&self) -> &StatusType {
        &self.status_type
    }

    pub fn target_count(&self) -> usize {
        self.status_set.len()
    }
}

impl PartialEq for Status {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use StatusType::*;

    macro_rules! check_status {
        ($status: ident, $type: expr, $count: expr, $next_target: expr, $has_next: expr) => {
            assert_eq!($status.get_type(), &$type);
            assert_eq!($status.target_count(), $count);
            assert_eq!($status.next($next_target).is_some(), $has_next);
        };
    }

    #[test]
    fn start_nfa() {
        const START_STATUS_SHOULD_BE_ONLY_ONE_TARGET: usize = 1;
        const END_STATUS_SHOULD_BE_ZERO_TARGET: usize = 0;

        let nfa = NFA::new();

        let start = RefCell::borrow(&nfa.start);
        // let start: &_ = nfa.start.borrow();
        check_status!(
            start,
            StatusType::Start,
            START_STATUS_SHOULD_BE_ONLY_ONE_TARGET,
            EMPTY,
            true
        );

        let next = start.next(EMPTY).unwrap();
        let next = RefCell::borrow(&next);

        check_status!(
            next,
            StatusType::End,
            END_STATUS_SHOULD_BE_ZERO_TARGET,
            EMPTY,
            false
        );
    }

    #[test]
    fn start_nfa_with_text() {
        const INIT_CHAR: char = 'a';

        // ((s)) -text-> ((e))
        let nfa = NFA::with(INIT_CHAR);

        let status = RefCell::borrow(&nfa.start);

        check_status!(status, StatusType::Start, 1, EMPTY, false);
        assert!(status.next(INIT_CHAR).is_some());

        let status = status.next(INIT_CHAR).unwrap();
        let status = RefCell::borrow(&status);

        check_status!(status, StatusType::End, 0, EMPTY, false);
    }

    #[test]
    fn and_two_empty_nfa() {
        let mut nfa_1 = NFA::new();
        let nfa_2 = NFA::new();

        nfa_1.and(nfa_2);
        /*
            ((s)) -empty-> ((empty)) -empty-> ((empty)) -> -empty-> ((e))
        */

        let status = RefCell::borrow(&nfa_1.start);

        check_status!(status, Start, 1, EMPTY, true);

        let status = status.next(EMPTY).unwrap();
        let status = RefCell::borrow(&status);

        check_status!(status, Node, 1, EMPTY, true);

        let status = status.next(EMPTY).unwrap();
        let status = RefCell::borrow(&status);

        check_status!(status, Node, 1, EMPTY, true);

        let status = status.next(EMPTY).unwrap();
        let status = RefCell::borrow(&status);

        check_status!(status, End, 0, EMPTY, false);
    }

    #[test]
    fn and_two_nfa_with() {
        const NFA_1_TEXT: char = 'A';
        const NFA_2_TEXT: char = '一';

        let mut nfa_1 = NFA::with(NFA_1_TEXT);
        let nfa_2 = NFA::with(NFA_2_TEXT);

        /*
            ((s)) -A-> ((empty)) -empty-> ((empty)) -> -一-> ((e))
        */
        nfa_1.and(nfa_2);

        let status = RefCell::borrow(&nfa_1.start);
        check_status!(status, Start, 1, NFA_1_TEXT, true);

        let status = status.next(NFA_1_TEXT).unwrap();
        let status = RefCell::borrow(&status);

        check_status!(status, Node, 1, EMPTY, true);

        let status = status.next(EMPTY).unwrap();
        let status = RefCell::borrow(&status);
        check_status!(status, Node, 1, NFA_2_TEXT, true);

        let status = status.next(NFA_2_TEXT).unwrap();
        let status = RefCell::borrow(&status);
        check_status!(status, End, 0, EMPTY, false);
    }

    #[test]
    fn nfa_status_next_skip_empty() {
        //  arrage
        /*
           ((s)) -empty-> (1) -a-> ((e))
                 -empty-> (2) -b-> (3) -empty-> ((e))
                 -c-> (c) -empty-> ((e))
        */
        const TARGET_A: char = 'a';
        const TARGET_B: char = 'b';
        const TARGET_C: char = 'c';
        const TARGET_NOT_EXIST: char = '\0';

        let start = Status::start();
        let mut start = RefCell::borrow_mut(&start);

        let status_1 = Status::node();
        {
            let end_1 = Status::end();
            let mut status_1 = RefCell::borrow_mut(&status_1);
            status_1.append_next(TARGET_A, end_1);
        }

        start.append_next(EMPTY, Rc::clone(&status_1));

        let status_2 = Status::node();
        {
            let end_2 = Status::end();

            let node_2 = Status::node();
            {
                let mut node_2_tem = RefCell::borrow_mut(&node_2);
                node_2_tem.append_next(EMPTY, end_2);
            }

            let mut status_2 = RefCell::borrow_mut(&status_2);
            status_2.append_next(TARGET_B, node_2);
        }

        start.append_next(EMPTY, status_2);

        let status_3 = Status::node();
        {
            let end_3 = Status::end();
            let mut status_3 = RefCell::borrow_mut(&status_3);
            status_3.append_next(EMPTY, end_3);
        }

        start.append_next(TARGET_C, status_3);

        // action
        let next = start.next_skip_empty(TARGET_A);
        assert!(next.is_some());
        let next = next.unwrap();
        let next = RefCell::borrow(&next);

        check_status!(next, End, 0, EMPTY, false);

        let next = start.next_skip_empty(TARGET_B);
        assert!(next.is_some());
        let next = next.unwrap();
        let next = RefCell::borrow(&next);

        check_status!(next, Node, 1, EMPTY, true);

        let next = start.next_skip_empty(TARGET_C);
        assert!(next.is_some());
        let next = next.unwrap();
        let next = RefCell::borrow(&next);

        check_status!(next, Node, 1, EMPTY, true);

        let next = start.next_skip_empty(TARGET_NOT_EXIST);
        assert!(next.is_none());
    }
}
