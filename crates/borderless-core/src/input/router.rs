//! Device layout and input routing — the Mouse Without Borders core logic.
//!
//! When cursor hits screen edge, ownership transfers to the neighbour device.
//! All subsequent mouse/keyboard events are forwarded over the encrypted session.

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::{InputEvent, MouseMove};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Edge { Left, Right, Top, Bottom }

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DeviceLayout {
    pub edges: HashMap<Uuid, HashMap<Edge, Uuid>>,
    pub sizes: HashMap<Uuid, (i32, i32)>,
}

impl DeviceLayout {
    pub fn connect(&mut self, from: Uuid, edge: Edge, to: Uuid) { self.edges.entry(from).or_default().insert(edge, to); }
    pub fn set_size(&mut self, id: Uuid, w: i32, h: i32) { self.sizes.insert(id, (w, h)); }
    pub fn neighbour(&self, id: Uuid, edge: Edge) -> Option<Uuid> { self.edges.get(&id)?.get(&edge).copied() }
    pub fn size_of(&self, id: Uuid) -> Option<(i32, i32)> { self.sizes.get(&id).copied() }
}

#[derive(Debug)]
pub enum Routing {
    Local,
    Forward { target: Uuid, event: InputEvent },
    SwitchTo { target: Uuid, entry_x: i32, entry_y: i32 },
}

const EDGE_PX: i32 = 2;

pub struct InputRouter {
    local_id:  Uuid,
    active:    Arc<RwLock<Uuid>>,
    layout:    Arc<RwLock<DeviceLayout>>,
    last_pos:  Arc<RwLock<(i32, i32)>>,
}

impl InputRouter {
    pub fn new(local_id: Uuid, layout: DeviceLayout) -> Self {
        Self { local_id, active: Arc::new(RwLock::new(local_id)), layout: Arc::new(RwLock::new(layout)), last_pos: Arc::new(RwLock::new((0, 0))) }
    }

    pub fn set_layout(&self, layout: DeviceLayout) { *self.layout.write().unwrap() = layout; }
    pub fn active_device(&self) -> Uuid { *self.active.read().unwrap() }
    pub fn is_remote(&self) -> bool { self.active_device() != self.local_id }

    pub fn route(&self, event: InputEvent) -> Routing {
        let active = self.active_device();
        if active != self.local_id { return Routing::Forward { target: active, event }; }
        if let InputEvent::MouseMove(ref mv) = event {
            *self.last_pos.write().unwrap() = (mv.x, mv.y);
            if let Some(routing) = self.check_edge(mv) { return routing; }
        }
        Routing::Local
    }

    pub fn recall(&self) {
        *self.active.write().unwrap() = self.local_id;
        tracing::debug!("Input router: cursor recalled to local");
    }

    fn check_edge(&self, mv: &MouseMove) -> Option<Routing> {
        let layout = self.layout.read().unwrap();
        let active = self.active_device();
        let (sw, sh) = layout.size_of(active)?;
        let edge = if mv.x <= EDGE_PX            { Some(Edge::Left)   }
              else if mv.x >= sw - EDGE_PX - 1   { Some(Edge::Right)  }
              else if mv.y <= EDGE_PX             { Some(Edge::Top)    }
              else if mv.y >= sh - EDGE_PX - 1   { Some(Edge::Bottom) }
              else                                { None               };
        let edge   = edge?;
        let target = layout.neighbour(active, edge)?;
        let (tw, th) = layout.size_of(target).unwrap_or((sw, sh));
        let (ex, ey) = entry_point(mv, edge, sw, sh, tw, th);
        *self.active.write().unwrap() = target;
        tracing::info!(from = %active, to = %target, edge = ?edge, entry = ?(ex, ey), "Cursor crossed edge");
        Some(Routing::SwitchTo { target, entry_x: ex, entry_y: ey })
    }
}

fn entry_point(mv: &MouseMove, edge: Edge, sw: i32, sh: i32, tw: i32, th: i32) -> (i32, i32) {
    match edge {
        Edge::Right  => (1,      scale(mv.y, sh, th)),
        Edge::Left   => (tw - 2, scale(mv.y, sh, th)),
        Edge::Bottom => (scale(mv.x, sw, tw), 1),
        Edge::Top    => (scale(mv.x, sw, tw), th - 2),
    }
}

fn scale(v: i32, from_max: i32, to_max: i32) -> i32 {
    if from_max == 0 { return 0; }
    ((v as f64 / from_max as f64) * to_max as f64).round() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    fn two_device_layout() -> (Uuid, Uuid, DeviceLayout) {
        let local = Uuid::new_v4(); let remote = Uuid::new_v4();
        let mut layout = DeviceLayout::default();
        layout.connect(local, Edge::Right, remote); layout.connect(remote, Edge::Left, local);
        layout.set_size(local, 1920, 1080); layout.set_size(remote, 2560, 1440);
        (local, remote, layout)
    }

    fn mv(x: i32, y: i32) -> InputEvent {
        InputEvent::MouseMove(MouseMove { x, y, dx: 0, dy: 0, screen_w: 1920, screen_h: 1080 })
    }

    #[test] fn local_event_stays_local() {
        let (local, _, layout) = two_device_layout();
        assert!(matches!(InputRouter::new(local, layout).route(mv(960, 540)), Routing::Local));
    }
    #[test] fn right_edge_switches_device() {
        let (local, remote, layout) = two_device_layout();
        let router = InputRouter::new(local, layout);
        assert!(matches!(router.route(mv(1919, 540)), Routing::SwitchTo { target, .. } if target == remote));
        assert_eq!(router.active_device(), remote);
    }
    #[test] fn subsequent_events_forwarded() {
        let (local, remote, layout) = two_device_layout();
        let router = InputRouter::new(local, layout);
        router.route(mv(1919, 540));
        assert!(matches!(router.route(mv(100, 100)), Routing::Forward { target, .. } if target == remote));
    }
    #[test] fn recall_returns_to_local() {
        let (local, remote, layout) = two_device_layout();
        let router = InputRouter::new(local, layout);
        router.route(mv(1919, 540)); assert_eq!(router.active_device(), remote);
        router.recall(); assert_eq!(router.active_device(), local);
    }
    #[test] fn entry_point_scales_correctly() {
        let mv = MouseMove { x: 1919, y: 540, dx: 0, dy: 0, screen_w: 1920, screen_h: 1080 };
        let (_, ey) = entry_point(&mv, Edge::Right, 1920, 1080, 2560, 1440);
        assert_eq!(ey, 720);
    }
}
