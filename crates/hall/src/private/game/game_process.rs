use std::cmp::Ordering;

use crate::private::game::game_machine::GameMachineContext;
use crate::private::game::game_state::{ActorMapType, UserMapType};
use crate::private::game::{GameMachine, RemoteMapType, TargetIdType};
use hall::core::{Attributes, CardTargetMachineValue, CardTargetValue, LaunchInstruction, MachineValueType, PriorityType, RunInstruction, ValueTarget};
use hall::hall::HallCard;
use hall::player::PlayerCard;
use hall::view::GameProcessPlayerView;
use shared_net::UserIdType;

type LoopType = u8;

pub(crate) struct GameProcess {
    player_card: PlayerCard,
    launch_code: Vec<LaunchInstruction>,
    run_code: Vec<RunInstruction>,
    priority: PriorityType,
    loop_count: LoopType,
    target: TargetIdType,
    owner_id: UserIdType,
}

impl GameProcess {
    pub(crate) fn new_from_card(card: HallCard, target: TargetIdType, owner_id: UserIdType) -> (Self, usize) {
        (
            Self {
                player_card: (&card).into(),
                launch_code: card.launch_code,
                run_code: card.run_code,
                priority: card.priority,
                loop_count: 0,
                target,
                owner_id,
            },
            card.delay as usize,
        )
    }

    pub(crate) fn launch(&mut self, attrs: &Attributes) -> bool {
        for code in &self.launch_code {
            match code {
                LaunchInstruction::Loop(loop_count) => self.loop_count = loop_count.resolve(attrs) as LoopType,
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

    pub(crate) fn build_executable(&self, attrs: &Attributes) -> GameProcessExecutor {
        GameProcessExecutor {
            target: self.target,
            run_code: self.run_code.clone(),
            attrs: *attrs,
        }
    }

    pub(crate) fn tick(&mut self) -> bool {
        if self.loop_count == 0 {
            return false;
        }
        self.loop_count -= 1;
        true
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
            CardTargetValue::Machine(machine_kind) => match machine_kind {
                CardTargetMachineValue::Any => matches!(self.target, TargetIdType::Local(_) | TargetIdType::Remote(_)),
                CardTargetMachineValue::Local => matches!(self.target, TargetIdType::Local(_)),
                CardTargetMachineValue::Remote => matches!(self.target, TargetIdType::Remote(_)),
            },
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
            local: process.owner_id == user_id,
        }
    }
}

pub(crate) struct GameProcessExecutor {
    target: TargetIdType,
    run_code: Vec<RunInstruction>,
    attrs: Attributes,
}

impl GameProcessExecutor {
    pub(crate) fn execute(&self, users: &mut UserMapType, remotes: &mut RemoteMapType, _actors: &mut ActorMapType) {
        for instruction in &self.run_code {
            match instruction {
                RunInstruction::IncV(value, amount) => {
                    if let Some(machine) = Self::resolve_target_machine(self.target, users, remotes) {
                        Self::execute_incv(&mut machine.context, *value, amount.resolve(&self.attrs));
                    }
                }
                RunInstruction::DecV(value, amount) => {
                    if let Some(machine) = Self::resolve_target_machine(self.target, users, remotes) {
                        Self::execute_decv(&mut machine.context, *value, amount.resolve(&self.attrs));
                    }
                }
                RunInstruction::NoOp => {}
            }
        }
    }

    fn resolve_target_machine<'a>(target: TargetIdType, users: &'a mut UserMapType, remotes: &'a mut RemoteMapType) -> Option<&'a mut GameMachine> {
        match target {
            TargetIdType::Local(id) => users.get_mut(&id).map(|user| &mut user.machine),
            TargetIdType::Remote(id) => remotes.get_mut(&id).map(|remote| &mut remote.machine),
        }
    }

    fn execute_incv(context: &mut GameMachineContext, value: ValueTarget, amount: MachineValueType) {
        match value {
            ValueTarget::None => {}
            ValueTarget::FreeSpace => context.free_space = context.free_space.saturating_add(amount),
            ValueTarget::ThermalCapacity => context.thermal_capacity = context.thermal_capacity.saturating_add(amount),
            ValueTarget::SystemHealth => context.system_health = context.system_health.saturating_add(amount),
            ValueTarget::OpenPorts => context.open_ports = context.open_ports.saturating_add(amount),
        }
    }

    fn execute_decv(context: &mut GameMachineContext, value: ValueTarget, amount: MachineValueType) {
        match value {
            ValueTarget::None => {}
            ValueTarget::FreeSpace => context.free_space = context.free_space.saturating_sub(amount),
            ValueTarget::ThermalCapacity => context.thermal_capacity = context.thermal_capacity.saturating_sub(amount),
            ValueTarget::SystemHealth => context.system_health = context.system_health.saturating_sub(amount),
            ValueTarget::OpenPorts => context.open_ports = context.open_ports.saturating_sub(amount),
        }
    }
}
