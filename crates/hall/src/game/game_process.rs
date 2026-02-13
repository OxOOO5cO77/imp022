use std::cmp::Ordering;

use rand::{Rng, RngExt};

use hall_lib::core::{Attributes, CardTargetValue, DEFAULT_TOKEN_EXPIRY, LaunchInstruction, PriorityType, RunInstruction, RuntimeAmountType, TickType, Token, TokenKind, ValueTarget};
use hall_lib::message::UpdateTokenMessage;
use hall_lib::player::PlayerCard;
use hall_lib::view::GameProcessPlayerView;
use shared_net::UserIdType;

use crate::game::game_machine::GameMachineContext;
use crate::game::game_state::{ActorMapType, CardResolve, UserMapType};
use crate::game::{GameActor, GameMachine, RemoteMapType, TargetIdType};

type LoopType = u8;

pub(crate) struct GameProcess {
    player_card: PlayerCard,
    launch_code: Vec<LaunchInstruction>,
    run_code: Vec<RunInstruction>,
    priority: PriorityType,
    loop_count: LoopType,
    target: TargetIdType,
    owner: UserIdType,
    attributes: Attributes,
}

impl GameProcess {
    pub(crate) fn new_from_card_resolve(resolve: &CardResolve) -> (Self, usize) {
        (
            Self {
                player_card: (&resolve.card).into(),
                launch_code: resolve.card.launch_code.clone(),
                run_code: resolve.card.run_code.clone(),
                priority: resolve.card.priority,
                loop_count: 1,
                target: resolve.target,
                owner: resolve.local_id,
                attributes: resolve.attributes,
            },
            resolve.card.delay as usize,
        )
    }

    pub(crate) fn launch(&mut self) -> bool {
        for code in &self.launch_code {
            match code {
                LaunchInstruction::Loop(loop_count) => self.loop_count = loop_count.resolve(&self.attributes) as LoopType,
                LaunchInstruction::Targ(target) => {
                    if !self.target_valid(target) {
                        return false;
                    }
                }
                LaunchInstruction::NoOp => {}
            }
        }
        true
    }

    pub(crate) fn build_executable(&self) -> GameProcessExecutor {
        GameProcessExecutor {
            owner: self.owner,
            card: self.player_card,
            target: self.target,
            run_code: self.run_code.clone(),
            attributes: self.attributes,
        }
    }

    pub(crate) fn tick(&mut self) {
        self.loop_count = self.loop_count.saturating_sub(1);
    }
    pub(crate) fn get_loop(&self) -> LoopType {
        self.loop_count
    }
    pub(crate) fn get_card(&self) -> PlayerCard {
        self.player_card
    }
    pub(crate) fn get_priority(&self) -> PriorityType {
        self.priority
    }

    fn target_valid(&self, card_target: &CardTargetValue) -> bool {
        match card_target {
            CardTargetValue::None => false,
            CardTargetValue::Machine => matches!(self.target, TargetIdType::User(_) | TargetIdType::Remote(_)),
            CardTargetValue::Actor => matches!(self.target, TargetIdType::Actor(_)),
        }
    }
}

impl Eq for GameProcess {}

impl PartialEq for GameProcess {
    fn eq(&self, other: &Self) -> bool {
        self.priority.eq(&other.priority)
    }
}

impl Ord for GameProcess {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for GameProcess {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub(crate) trait ProcessForPlayer {
    fn process_for_player(process: &GameProcess, user_id: UserIdType) -> Self;
}

impl ProcessForPlayer for GameProcessPlayerView {
    fn process_for_player(process: &GameProcess, user_id: UserIdType) -> Self {
        Self {
            player_card: process.player_card,
            priority: process.priority,
            local: process.owner == user_id,
            attributes: process.attributes.to_arrays(),
        }
    }
}

pub(crate) struct GameProcessExecutor {
    owner: UserIdType,
    card: PlayerCard,
    target: TargetIdType,
    run_code: Vec<RunInstruction>,
    attributes: Attributes,
}

pub(crate) enum ExecutionResultKind {
    Value(ValueTarget, RuntimeAmountType),
    Token(UpdateTokenMessage),
    Deck,
}

pub(crate) struct ExecutionResult {
    pub(crate) owner: UserIdType,
    pub(crate) target: TargetIdType,
    pub(crate) results: Vec<ExecutionResultKind>,
}

impl ExecutionResult {
    pub(crate) fn new(owner: UserIdType, target: TargetIdType, results: Vec<ExecutionResultKind>) -> Self {
        Self {
            owner,
            target,
            results,
        }
    }
}

impl GameProcessExecutor {
    pub(crate) fn execute(&self, tick: TickType, users: &mut UserMapType, remotes: &mut RemoteMapType, actors: &mut ActorMapType, rng: &mut impl Rng) -> Vec<ExecutionResult> {
        let mut results = Vec::new();
        for instruction in &self.run_code {
            let result = match instruction {
                RunInstruction::NoOp => None,
                RunInstruction::IncV(value, amount) => self.execute_increase_value(users, remotes, *value, amount.resolve(&self.attributes)),
                RunInstruction::DecV(value, amount) => self.execute_decrease_value(users, remotes, *value, amount.resolve(&self.attributes)),
                RunInstruction::CpCr => self.execute_copy_credentials(users, actors, tick),
                RunInstruction::ReAu => self.execute_reauthorize(users, tick),
                RunInstruction::ShIn(amount) => self.execute_shuffle_into(users, amount.resolve(&self.attributes), rng),
            };
            if let Some(result) = result {
                results.push(ExecutionResult::new(self.owner, self.target, result));
            }
        }
        results
    }

    fn resolve_target_machine<'a>(target: TargetIdType, users: &'a mut UserMapType, remotes: &'a mut RemoteMapType) -> Option<&'a mut GameMachine> {
        match target {
            TargetIdType::User(id) => users.get_mut(&id).map(|user| &mut user.machine),
            TargetIdType::Remote(id) => remotes.get_mut(&id).map(|remote| &mut remote.machine),
            _ => None,
        }
    }

    fn resolve_target_actor(target: TargetIdType, actors: &ActorMapType) -> Option<&GameActor> {
        match target {
            TargetIdType::Actor(id) => actors.get(&id),
            _ => None,
        }
    }

    fn execute_value_change(context: &mut GameMachineContext, target: ValueTarget, amount: RuntimeAmountType, op: fn(RuntimeAmountType, RuntimeAmountType) -> RuntimeAmountType) -> Option<Vec<ExecutionResultKind>> {
        let orig_value = match target {
            ValueTarget::None => return None,
            ValueTarget::FreeSpace => context.free_space,
            ValueTarget::ThermalCapacity => context.thermal_capacity,
            ValueTarget::SystemHealth => context.system_health,
            ValueTarget::OpenPorts => context.open_ports,
        };

        let new_value = op(orig_value, amount);

        match target {
            ValueTarget::None => return None,
            ValueTarget::FreeSpace => context.free_space = new_value,
            ValueTarget::ThermalCapacity => context.thermal_capacity = new_value,
            ValueTarget::SystemHealth => context.system_health = new_value,
            ValueTarget::OpenPorts => context.open_ports = new_value,
        };

        Some(vec![ExecutionResultKind::Value(target, new_value)])
    }
}

impl GameProcessExecutor {
    fn execute_increase_value(&self, users: &mut UserMapType, remotes: &mut RemoteMapType, value: ValueTarget, amount: RuntimeAmountType) -> Option<Vec<ExecutionResultKind>> {
        if let Some(machine) = Self::resolve_target_machine(self.target, users, remotes) {
            return Self::execute_value_change(&mut machine.context, value, amount, RuntimeAmountType::saturating_add);
        }
        None
    }

    fn execute_decrease_value(&self, users: &mut UserMapType, remotes: &mut RemoteMapType, value: ValueTarget, amount: RuntimeAmountType) -> Option<Vec<ExecutionResultKind>> {
        if let Some(machine) = Self::resolve_target_machine(self.target, users, remotes) {
            return Self::execute_value_change(&mut machine.context, value, amount, RuntimeAmountType::saturating_sub);
        }
        None
    }

    fn execute_copy_credentials(&self, users: &mut UserMapType, actors: &ActorMapType, tick: TickType) -> Option<Vec<ExecutionResultKind>> {
        if let (Some(actor), Some(user)) = (Self::resolve_target_actor(self.target, actors), users.get_mut(&self.owner)) {
            let mess = user.mission_state.add_token(Token::new(TokenKind::Credentials(actor.auth_level), tick + DEFAULT_TOKEN_EXPIRY));
            return Some(vec![ExecutionResultKind::Token(mess)]);
        }
        None
    }

    fn execute_reauthorize(&self, users: &mut UserMapType, amount: TickType) -> Option<Vec<ExecutionResultKind>> {
        if let Some(user) = users.get_mut(&self.owner) {
            let mess = user.mission_state.extend_all_auth(amount);
            if !mess.is_empty() {
                return Some(mess.iter().map(|m| ExecutionResultKind::Token(m.clone())).collect());
            }
        }
        None
    }

    fn execute_shuffle_into(&self, users: &mut UserMapType, amount: RuntimeAmountType, rng: &mut impl Rng) -> Option<Vec<ExecutionResultKind>> {
        if let Some(player) = users.get_mut(&self.owner).and_then(|user| user.player.as_mut()) {
            let pos = rng.random_range(0..amount) as usize;
            player.deck.insert(pos, self.card);
            return Some(vec![ExecutionResultKind::Deck]);
        }
        None
    }
}
