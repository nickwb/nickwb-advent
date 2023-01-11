use std::collections::VecDeque;

use slotmap::{new_key_type, Key, SlotMap};

use super::CameraBuffer;

fn find_path(camera: &CameraBuffer) -> usize {
    let mut context = Context::new(camera);
    let context = &mut context;

    let mut stack: Vec<CandidateId> = Vec::new();
    let mut next_candidates: Vec<ChildCandidateBuilder> = Vec::with_capacity(ALL_STRATEGIES.len());

    // Insert the initial candidate
    let (start_pos, start_dir) = context.get_robot().expect("Found a robot");
    stack.push(context.insert_candidate(Candidate::initial(start_pos, start_dir)));

    while let Some(candidate_id) = stack.pop() {
        // Get the candidate
        let candidate = context
            .candidates
            .get(candidate_id)
            .expect("Found candidate");

        // Check if we've reached the end

        // Consider all further moves
        next_candidates.extend(
            ALL_STRATEGIES
                .iter()
                .filter_map(|s| build_child_candidate_if_possible(candidate, context, s)),
        );

        drop(candidate);

        // Push new candidates on to the stack
        stack.extend(next_candidates.drain(..).rev().map(|c| c.build(context)));
    }

    0
}

fn build_child_candidate_if_possible(
    candidate: &Candidate,
    context: &Context,
    strategy: &Strategy,
) -> Option<ChildCandidateBuilder> {
    todo!();

    // match strategy {
    //     Strategy::AmendSequence(step) => {
    //         if candidate.authoring.is_none() {
    //             None
    //         }
    //     }
    // }
    // Amend is only possible if the current sequence being authored is not locked
    // New sequence is only possible if there exists an unused slot
    // Don't attempt a new sequence if the authored sequence is empty (first move only)
    // A turn is only possible if they don't exceed two in a row, and if two, they should be in the same direction
    // Playback is only possible if the slot is non-empty.
    None
}

new_key_type! { struct CandidateId; }
new_key_type! { struct SequenceId; }

struct Context<'a> {
    camera: &'a CameraBuffer,
    candidates: SlotMap<CandidateId, Candidate>,
    sequences: SlotMap<SequenceId, Sequence>,
}

impl<'a> Context<'a> {
    fn new(camera: &'a CameraBuffer) -> Self {
        Self {
            camera,
            candidates: SlotMap::with_key(),
            sequences: SlotMap::with_key(),
        }
    }

    fn insert_candidate(&mut self, candidate: Candidate) -> CandidateId {
        let id = self.candidates.insert(candidate);
        self.candidates[id].id = id;
        id
    }

    fn insert_sequence(&mut self, sequence: Sequence) -> SequenceId {
        let id = self.sequences.insert(sequence);
        self.sequences[id].id = id;
        id
    }

    fn get_robot(&self) -> Option<(Position, Direction)> {
        let (x, y, state) = self.camera.find_robot()?;
        let direction = match state {
            super::RobotState::Up => Direction::Up,
            super::RobotState::Down => Direction::Down,
            super::RobotState::Left => Direction::Left,
            super::RobotState::Right => Direction::Right,
            super::RobotState::Loose => {
                return None;
            }
        };
        Some((Position { x, y }, direction))
    }
}

struct Candidate {
    id: CandidateId,
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
            id: CandidateId::null(),
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

struct ChildCandidateBuilder {
    parent: CandidateId,
    direction: Option<Direction>,
    current_pos: Option<Position>,
    traversed: Vec<Position>,
    authoring: Option<SequenceSlot>,
    main: Option<SequenceSlot>,
    a: Option<SequenceBuilder>,
    b: Option<SequenceBuilder>,
    c: Option<SequenceBuilder>,
}

impl ChildCandidateBuilder {
    fn new(parent: &Candidate) -> ChildCandidateBuilder {
        ChildCandidateBuilder {
            parent: parent.id,
            direction: None,
            current_pos: None,
            traversed: Vec::with_capacity(0),
            authoring: parent.authoring,
            main: None,
            a: None,
            b: None,
            c: None,
        }
    }

    fn build(self, context: &mut Context) -> CandidateId {
        let candidate = Candidate {
            id: CandidateId::null(),
            parent: Some(self.parent),
            direction: self.direction.unwrap(),
            current_pos: self.current_pos.unwrap(),
            traversed: self.traversed,
            authoring: self.authoring,
            main: self.main,
            a: self.a.map(|s| s.get_or_insert(context)),
            b: self.b.map(|s| s.get_or_insert(context)),
            c: self.c.map(|s| s.get_or_insert(context)),
        };
        context.insert_candidate(candidate)
    }
}

enum SequenceBuilder {
    Existing(SequenceId),
    New(Sequence),
}

impl SequenceBuilder {
    fn get_or_insert(self, context: &mut Context) -> SequenceId {
        match self {
            Self::Existing(id) => id,
            Self::New(s) => context.insert_sequence(s),
        }
    }
}

// The absolute longest any sequence can be, based on the text restriction, is 10
const MAX_SEQUENCE_LENGTH: usize = 10;

struct Sequence {
    id: SequenceId,
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
