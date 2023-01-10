use std::collections::VecDeque;

use slotmap::{new_key_type, SlotMap};

use super::CameraBuffer;

fn find_path(camera: &CameraBuffer) -> usize {
    loop {
        // Pop a candidate from the stack
        // Check if we've reached the end
        // Consider all further moves
        // Discard moves which are not valid
        // Push new candidates on to the stack
    }
}

fn build_candidate_if_possible(context: &mut Context, strategy: &Strategy) -> Option<Candidate> {
    None
}

new_key_type! { struct CandidateId; }
new_key_type! { struct SequenceId; }

struct Context<'a> {
    camera: &'a CameraBuffer,
    candidates: SlotMap<CandidateId, Candidate>,
    sequences: SlotMap<SequenceId, Sequence>,
}

struct Candidate {
    parent: Option<CandidateId>,
    direction: Direction,
    current_pos: Position,
    traversed: Vec<Position>,
    authoring: Option<SequenceSlot>,
    main: Option<SequenceSlot>,
    a: Option<SequenceId>,
    b: Option<SequenceId>,
    c: Option<SequenceId>,
}

impl Candidate {
    fn initial(start_pos: Position, start_dir: Direction) -> Self {
        let traversed = vec![start_pos];
        Self {
            parent: None,
            direction: start_dir,
            current_pos: start_pos,
            traversed: traversed,
            authoring: Some(SequenceSlot::A),
            main: Some(SequenceSlot::A),
            a: None,
            b: None,
            c: None,
        }
    }

    // Fold over the graph to produce the movement routine in the given slot
    fn get_slot(&self, slot: SequenceSlot, context: &Context, buf: &mut VecDeque<SequenceElement>) {
        buf.clear();
        let mut parent = match slot {
            SequenceSlot::A => self.a,
            SequenceSlot::B => self.b,
            SequenceSlot::C => self.c,
        };

        while let Some(s) = parent.and_then(|p| context.sequences.get(p)) {
            buf.push_front(s.node);
            parent = s.parent;
        }
    }

    // Fold over the graph to produce the main program
    fn get_main(&self, context: &Context, buf: &mut VecDeque<SequenceSlot>) {
        buf.clear();
        let mut candidate = Some(self);

        while let Some(c) = candidate {
            if let Some(m) = c.main {
                buf.push_front(m);
            }
            candidate = c.parent.and_then(|id| context.candidates.get(id));
        }
    }
}

// The absolute longest any sequence can be, based on the text restriction, is 10
const MAX_SEQUENCE_LENGTH: usize = 10;

struct Sequence {
    parent: Option<SequenceId>,
    node: SequenceElement,
    is_locked: bool,
}

#[derive(Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
enum SequenceElement {
    TurnLeft,
    TurnRight,
    Forwards(usize),
}

impl SequenceElement {
    fn required_chars(&self) -> usize {
        match self {
            Self::TurnLeft | Self::TurnRight => 1,
            Self::Forwards(n) if n < &10 => 1,
            Self::Forwards(n) if n < &100 => 2,
            Self::Forwards(_) => panic!("You are moving too far"),
        }
    }
}

const ALL_STRATEGIES: [Strategy; 9] = [
    Strategy::AmendSequence(PathChoice::Forwards),
    Strategy::AmendSequence(PathChoice::Left),
    Strategy::AmendSequence(PathChoice::Right),
    Strategy::NewSequence(PathChoice::Forwards),
    Strategy::NewSequence(PathChoice::Left),
    Strategy::NewSequence(PathChoice::Right),
    Strategy::PlayBack(SequenceSlot::A),
    Strategy::PlayBack(SequenceSlot::B),
    Strategy::PlayBack(SequenceSlot::C),
];

#[derive(Clone, Copy)]
enum Strategy {
    AmendSequence(PathChoice),
    NewSequence(PathChoice),
    PlayBack(SequenceSlot),
}

#[derive(Clone, Copy)]
enum PathChoice {
    Forwards,
    Left,
    Right,
}

#[derive(Clone, Copy)]
enum SequenceSlot {
    A,
    B,
    C,
}
