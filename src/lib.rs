// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-service-isolation: service isolation component model.
//!
//! Real implementation: isolations, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Isolation {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ServiceIsolationError {
    EmptyIsolationName,
    DuplicateIsolation(String),
    EmptyEventSubject,
}

impl fmt::Display for ServiceIsolationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyIsolationName => write!(f, "isolation name must not be empty"),
            Self::DuplicateIsolation(id) => write!(f, "duplicate isolation {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ServiceIsolationError {}

pub struct ServiceIsolation {
    pub name: String,
    isolations: BTreeMap<String, Isolation>,
    events: BTreeMap<String, Event>,
}

impl ServiceIsolation {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            isolations: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_isolation(&mut self, item: Isolation) -> Result<(), ServiceIsolationError> {
        if item.name.trim().is_empty() {
            return Err(ServiceIsolationError::EmptyIsolationName);
        }
        let id = item.id.clone();
        if self.isolations.contains_key(&id) {
            return Err(ServiceIsolationError::DuplicateIsolation(id.clone()));
        }
        self.isolations.insert(id, item);
        Ok(())
    }

    pub fn isolation_count(&self) -> usize {
        self.isolations.len()
    }

    pub fn get_isolation(&self, id: &str) -> Option<&Isolation> {
        self.isolations.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ServiceIsolationError> {
        if event.subject.trim().is_empty() {
            return Err(ServiceIsolationError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_isolation(id: &str, name: &str) -> Isolation {
        Isolation {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = ServiceIsolation::new("ServiceIsolation");
        assert_eq!(m.name, "ServiceIsolation");
        assert_eq!(m.isolation_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_isolation() {
        let mut m = ServiceIsolation::new("ServiceIsolation");
        m.add_isolation(sample_isolation("a1", "Alpha")).unwrap();
        assert_eq!(m.isolation_count(), 1);
        assert!(m.get_isolation("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = ServiceIsolation::new("ServiceIsolation");
        assert!(m.add_isolation(sample_isolation("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = ServiceIsolation::new("ServiceIsolation");
        m.add_isolation(sample_isolation("x", "One")).unwrap();
        assert!(m.add_isolation(sample_isolation("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = ServiceIsolation::new("ServiceIsolation");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
