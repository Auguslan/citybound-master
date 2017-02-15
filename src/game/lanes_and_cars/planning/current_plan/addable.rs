use kay::{ID, Recipient, Actor, Fate};
use kay::swarm::{Swarm, SubActor, CreateWith};
use descartes::Band;
use core::geometry::{CPath, AnyShape};

use super::CurrentPlan;

#[derive(SubActor, Compact, Clone)]
pub struct Addable {
    _id: Option<ID>,
    path: CPath,
}

impl Addable {
    pub fn new(path: CPath) -> Self {
        Addable {
            _id: None,
            path: path,
        }
    }
}

use super::InitInteractable;
use core::ui::Add;

impl Recipient<InitInteractable> for Addable {
    fn receive(&mut self, _msg: &InitInteractable) -> Fate {
        ::core::ui::UserInterface::id() <<
        Add::Interactable3d(self.id(),
                            AnyShape::Band(Band::new(self.path.clone(), 3.0)),
                            3);
        Fate::Live
    }
}

use super::ClearInteractable;
use core::ui::Remove;

impl Recipient<ClearInteractable> for Addable {
    fn receive(&mut self, _msg: &ClearInteractable) -> Fate {
        ::core::ui::UserInterface::id() << Remove::Interactable3d(self.id());
        Fate::Die
    }
}

use core::ui::Event3d;
use super::{ChangeIntent, Intent, IntentProgress};

impl Recipient<Event3d> for Addable {
    fn receive(&mut self, msg: &Event3d) -> Fate {
        match *msg {
            Event3d::HoverStarted { .. } |
            Event3d::HoverOngoing { .. } => {
                CurrentPlan::id() << ChangeIntent(Intent::CreateNextLane, IntentProgress::Preview);
                Fate::Live
            }
            Event3d::HoverStopped => {
                CurrentPlan::id() << ChangeIntent(Intent::None, IntentProgress::Preview);
                Fate::Live
            }
            Event3d::DragStarted { .. } => {
                CurrentPlan::id() <<
                ChangeIntent(Intent::CreateNextLane, IntentProgress::Immediate);
                Fate::Live
            }
            _ => Fate::Live,
        }
    }
}

pub fn setup() {
    Swarm::<Addable>::register_default();
    Swarm::<Addable>::handle::<CreateWith<Addable, InitInteractable>>();
    Swarm::<Addable>::handle::<ClearInteractable>();
    Swarm::<Addable>::handle::<Event3d>();
}
