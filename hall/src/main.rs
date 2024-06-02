use std::env;
use std::mem::size_of;
use std::sync::{Arc, Mutex};

use rand::{distributions::Uniform, Rng};
use tokio::sync::mpsc;
use tokio::sync::mpsc::UnboundedSender;

use shared_net::{VClientMode, VRoute, VRoutedMessage, VSizedBuffer};
use shared_net::op;
use shared_net::op::Flavor;

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
enum DraftState {
    NotStarted = 1,
    Started = 2,
    Complete = 4,
}

struct User {
    user_hash: u128,
    gate: u8,
    vagabond: u8,
    picks: Vec<u128>,
    offset: usize,
}

struct Draft {
    id: u128,
    name: String,
    state: DraftState,
    max_users: usize,
    users: Vec<User>,
    round: usize,
    pick: usize,
    cards: Vec<Vec<Vec<u128>>>,
}

impl Draft {
    fn populate(&mut self) {
        self.cards = Vec::new();
        for _ in 0..3 {
            self.cards.push(self.generate_round());
        }
    }

    fn generate_round(&self) -> Vec<Vec<u128>> {
        let mut ret = Vec::new();

        for _ in 0..self.max_users {
            ret.push(self.generate_pack());
        }

        ret
    }

    fn generate_pack(&self) -> Vec<u128> {
        let mut ret = Vec::new();

        let range = Uniform::new(0_u128,u128::MAX).unwrap();
        let mut rng = rand::thread_rng();
        for _ in 0..15 {
            ret.push(rng.sample(range));
        }

        ret
    }

    fn pack_idx_for_user(pick: usize, max_users: usize, user: &User) -> usize {
        (pick + user.offset) % max_users
    }


    fn send_packs_to_users(&self, tx: UnboundedSender<VRoutedMessage>) {
        for user in self.users.iter() {
            let mut dp_out = VSizedBuffer::new(256);

            dp_out.push_route(op::Route::Some);
            dp_out.push_u8(&user.gate);
            dp_out.push_command(op::Command::DraftCards);
            dp_out.push_u8(&user.vagabond);

            let pack_idx = Draft::pack_idx_for_user(self.pick, self.max_users, user);
            let pack = &self.cards[self.round][pack_idx];

            dp_out.push_u8(&(pack.len() as u8));
            for card in pack.iter() {
                dp_out.push_u128(card);
            }

            let _ = tx.send(VRoutedMessage { route: VRoute::None, buf: dp_out });
        }
    }
}


struct Hall {
    drafts: Vec<Draft>,
}


#[tokio::main]
async fn main() -> Result<(), ()> {
    let mut args = env::args();
    let _ = args.next(); // program name
    let iface_to_courtyard = args.next().unwrap_or("[::1]:12345".to_string());

    let (dummy_tx, dummy_rx) = mpsc::unbounded_channel();

    let mut context = Hall {
        drafts: Vec::new()
    };

    context.drafts.push(Draft {
        id: 1234567,
        name: "Sample Draft".to_string(),
        state: DraftState::NotStarted,
        max_users: 1,
        users: Vec::new(),
        round: 0,
        pick: 0,
        cards: Vec::new(),
    });

    let context = Arc::new(Mutex::new(context));

    let courtyard_client = shared_net::async_client(context, Flavor::Hall, dummy_tx, dummy_rx, iface_to_courtyard, process_courtyard);

    courtyard_client.await
}

fn process_courtyard(context: Arc<Mutex<Hall>>, tx: UnboundedSender<VRoutedMessage>, mut buf: VSizedBuffer) -> VClientMode {
    match buf.pull_command() {
        op::Command::DraftList => c_draftlist(context, tx, buf),
        op::Command::DraftJoin => c_draftjoin(context, tx, buf),
        op::Command::DraftPick => c_draftpick(context, tx, buf),
        _ => {}
    }
    VClientMode::Continue
}

fn c_draftlist(context: Arc<Mutex<Hall>>, tx: UnboundedSender<VRoutedMessage>, mut buf: VSizedBuffer) {
    let gate = buf.pull_u8();
    let vagabond = buf.pull_u8();
    let _user_hash = buf.pull_u128();
    let list_filter = buf.pull_u8();

    let context = context.lock().unwrap();
    let list: Vec<&Draft> = context.drafts.iter().filter(|ob| list_filter == 0 || (ob.state as u8 & list_filter) != 0).collect();

    let entry_size = size_of::<u128>() + 64 + size_of::<u8>() + size_of::<u8>();

    let mut out = VSizedBuffer::new(6 + list.len() * entry_size);
    out.push_route(op::Route::Some);
    out.push_u8(&gate);
    out.push_command(op::Command::DraftList);
    out.push_u8(&vagabond);
    out.push_u16(&(list.len() as u16));
    for draft in list {
        out.push_u128(&draft.id);
        out.push_string(&draft.name);
        out.push_u8(&(draft.max_users as u8));
        out.push_u8(&(draft.users.len() as u8));
    }

    let _ = tx.send(VRoutedMessage { route: VRoute::None, buf: out });
}

fn c_draftjoin(context: Arc<Mutex<Hall>>, tx: UnboundedSender<VRoutedMessage>, mut buf: VSizedBuffer) {
    let gate = buf.pull_u8();
    let vagabond = buf.pull_u8();
    let user_hash = buf.pull_u128();
    let draft_id = buf.pull_u128();

    let mut out = VSizedBuffer::new(256);

    out.push_route(op::Route::Some);
    out.push_u8(&gate);
    out.push_command(op::Command::DraftJoin);
    out.push_u8(&vagabond);


    let mut context = context.lock().unwrap();
    let draft = context.drafts.iter_mut().find(|ob| ob.id == draft_id && ob.state == DraftState::NotStarted);

    if draft.is_none() {
        out.push_u128(&0);
        let _ = tx.send(VRoutedMessage { route: VRoute::None, buf: out });
        return;
    }
    let draft = draft.unwrap();

    let offset = draft.users.len();
    if offset < draft.max_users {
        if !draft.users.iter().any(|u| u.user_hash == user_hash) {
            draft.users.push(User {
                user_hash,
                gate,
                vagabond,
                picks: Vec::new(),
                offset,
            });
        }

        out.push_u128(&draft_id);
        let _ = tx.send(VRoutedMessage { route: VRoute::None, buf: out });
    } else {
        out.push_u128(&0);
        let _ = tx.send(VRoutedMessage { route: VRoute::None, buf: out });
        return;
    }

    if draft.users.len() == draft.max_users {
        draft.state = DraftState::Started;
        draft.populate();
        draft.send_packs_to_users(tx)
    }
}

fn c_draftpick(context: Arc<Mutex<Hall>>, tx: UnboundedSender<VRoutedMessage>, mut buf: VSizedBuffer) {
    let gate = buf.pull_u8();
    let vagabond = buf.pull_u8();
    let user_hash = buf.pull_u128();
    let draft_id = buf.pull_u128();
    let pick = buf.pull_u8() as usize;

    if let Some(draft) = context.lock().unwrap().drafts.iter_mut().find(|ob| ob.id == draft_id) {
        if let Some(user) = draft.users.iter_mut().find(|ob| ob.user_hash == user_hash) {
            let pack_idx = Draft::pack_idx_for_user(draft.pick, draft.max_users, user);
            if let Some(pack) = draft.cards.get_mut(draft.round)
                .and_then(|group| group.get_mut(pack_idx)) {
                if pick < pack.len() {
                    let card = pack[pick];
                    user.picks.push(card);
                    pack.remove(pick);

                    let mut out = VSizedBuffer::new(64);

                    out.push_route(op::Route::Some);
                    out.push_u8(&gate);
                    out.push_command(op::Command::DraftPick);
                    out.push_u8(&vagabond);
                    out.push_u128(&draft_id);
                    out.push_u128(&card);

                    let _ = tx.send(VRoutedMessage { route: VRoute::None, buf: out });

                    let len = draft.users[0].picks.len();

                    if draft.users.iter().all(|o| o.picks.len() == len) {
                        if draft.cards[draft.round].iter().all(|pack| pack.is_empty()) {
                            draft.pick = 0;
                            draft.round += 1;
                            if draft.round >= draft.cards.len() {
                                draft.state = DraftState::Complete;
                            }
                        } else {
                            draft.pick += 1;
                        }

                        if draft.state != DraftState::Complete {
                            draft.send_packs_to_users(tx);
                        }
                    }

                    return;
                }
            }
        }
    }

    let mut error_out = VSizedBuffer::new(64);

    error_out.push_route(op::Route::Some);
    error_out.push_u8(&gate);
    error_out.push_command(op::Command::DraftPick);
    error_out.push_u8(&vagabond);
    error_out.push_u128(&draft_id);
    error_out.push_u128(&0);

    let _ = tx.send(VRoutedMessage { route: VRoute::None, buf: error_out });
}
