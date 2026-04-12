#![allow(dead_code)]
#[derive(Clone,Debug,PartialEq)]
pub enum MsgKind { Tell, Ask, Delegate, Broadcast, Report, Emergency }
pub struct Message { id: u64, from: u32, to: u32, kind: MsgKind, payload: String, trust: f64, priority: u8, timestamp: u64, read: bool }
pub struct Telepathy { inbox: Vec<Message>, outbox: Vec<Message>, sent: Vec<Message>, next_id: u64, trust_threshold: f64 }

impl Telepathy {
    pub fn new() -> Self { Self { inbox: Vec::new(), outbox: Vec::new(), sent: Vec::new(), next_id: 1, trust_threshold: 0.3 } }
    pub fn send(&mut self, from: u32, to: u32, kind: MsgKind, payload: &str, trust: f64) -> u64 {
        let id = self.next_id; self.next_id += 1;
        let ts = 0; let pri = match kind { MsgKind::Emergency => 10, MsgKind::Report => 5, _ => 3 };
        let msg = Message { id, from, to, kind, payload: payload.to_string(), trust, priority: pri, timestamp: ts, read: false };
        self.outbox.push(msg.clone()); id
    }
    pub fn broadcast(&mut self, from: u32, payload: &str, trust: f64) -> u64 {
        let id = self.next_id; self.next_id += 1;
        let msg = Message { id, from, to: 0, kind: MsgKind::Broadcast, payload: payload.to_string(), trust, priority: 2, timestamp: 0, read: false };
        self.outbox.push(msg); id
    }
    pub fn receive(&mut self, agent: u32) -> Vec<&Message> { self.inbox.iter().filter(|m| m.to == agent && !m.read).collect() }
    pub fn receive_priority(&mut self, agent: u32, min_priority: u8) -> Vec<&Message> {
        self.inbox.iter().filter(|m| m.to == agent && !m.read && m.priority >= min_priority).collect()
    }
    pub fn mark_read(&mut self, id: u64) { if let Some(m) = self.inbox.iter_mut().find(|m| m.id == id) { m.read = true; } }
    pub fn inbox_count(&self, agent: u32) -> usize { self.inbox.iter().filter(|m| m.to == agent && !m.read).count() }
    pub fn outbox_count(&self) -> usize { self.outbox.len() }
    pub fn set_trust_threshold(&mut self, t: f64) { self.trust_threshold = t; }
    pub fn filter_trust(&self, min: f64) -> Vec<&Message> { self.inbox.iter().filter(|m| m.trust >= min).collect() }
    pub fn emergency(&mut self, from: u32, payload: &str) -> u64 { self.send(from, 0, MsgKind::Emergency, payload, 1.0) }
    pub fn deliver(&mut self) { self.inbox.extend(self.outbox.drain(..)); }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_new() { let t = Telepathy::new(); assert_eq!(t.outbox_count(), 0); }
    #[test] fn test_send() { let mut t = Telepathy::new(); let id = t.send(1, 2, MsgKind::Tell, "hello", 0.8); assert!(id > 0); assert_eq!(t.outbox_count(), 1); }
    #[test] fn test_broadcast() { let mut t = Telepathy::new(); t.broadcast(1, "alert", 0.9); assert_eq!(t.outbox_count(), 1); }
    #[test] fn test_deliver() { let mut t = Telepathy::new(); t.send(1, 2, MsgKind::Tell, "hi", 0.5); t.deliver(); assert!(t.inbox_count(2) >= 1); }
    #[test] fn test_receive() { let mut t = Telepathy::new(); t.send(1, 2, MsgKind::Tell, "hi", 0.5); t.deliver(); assert_eq!(t.receive(2).len(), 1); }
    #[test] fn test_mark_read() { let mut t = Telepathy::new(); let id = t.send(1, 2, MsgKind::Tell, "hi", 0.5); t.deliver(); t.mark_read(id); assert_eq!(t.inbox_count(2), 0); }
    #[test] fn test_emergency_priority() { let mut t = Telepathy::new(); t.emergency(1, "fire!"); let msg = &t.outbox[0]; assert_eq!(msg.priority, 10); assert_eq!(msg.kind, MsgKind::Emergency); }
    #[test] fn test_filter_trust() { let mut t = Telepathy::new(); t.send(1, 2, MsgKind::Tell, "hi", 0.9); t.send(3, 2, MsgKind::Tell, "lo", 0.1); t.deliver(); assert_eq!(t.filter_trust(0.5).len(), 1); }
    #[test] fn test_trust_threshold() { let mut t = Telepathy::new(); t.set_trust_threshold(0.7); assert!((t.trust_threshold - 0.7).abs() < 1e-6); }
    #[test] fn test_receive_priority() { let mut t = Telepathy::new(); t.emergency(1, "fire!"); t.send(1, 2, MsgKind::Tell, "hi", 0.5); t.deliver(); assert_eq!(t.receive_priority(2, 5).len(), 1); }
}