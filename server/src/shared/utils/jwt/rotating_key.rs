use arc_swap::{ArcSwap, ArcSwapOption};
use chrono::{DateTime, Duration, Utc};
use rand::RngCore;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Key {
    pub value: Vec<u8>,
    pub version: Uuid,
    pub inactive_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl Key {
    pub fn new(inactive_at: DateTime<Utc>, expires_at: DateTime<Utc>) -> Self {
        let mut key = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut key);

        let version = Uuid::new_v4();

        Self {
            value: key.to_vec(),
            version,
            inactive_at,
            expires_at,
        }
    }
}

#[derive(Debug)]
pub struct RotatingKey {
    active_key: ArcSwap<Key>,
    inactive_key: ArcSwapOption<Key>,
    rotation_duration: Duration,
    transition_duration: Duration,
}

impl RotatingKey {
    pub fn new(rotation_duration: Duration, transition_duration: Duration) -> Self {
        assert!(
            rotation_duration > transition_duration,
            "Transition period cannot last longer than rotation period!"
        );

        let key_inactive_at = Utc::now() + rotation_duration;
        let key_expires_at = key_inactive_at + transition_duration;

        let key = Key::new(key_inactive_at, key_expires_at);

        Self {
            active_key: ArcSwap::new(Arc::new(key)),
            inactive_key: ArcSwapOption::from(None),
            rotation_duration,
            transition_duration,
        }
    }

    pub fn rotate_keys(&self) {
        let key_inactive_at = Utc::now() + self.rotation_duration;
        let key_expires_at = key_inactive_at + self.transition_duration;
        let new_key = Key::new(key_inactive_at, key_expires_at);

        let old_key = self.active_key.swap(Arc::new(new_key));
        self.inactive_key.swap(Some(old_key));
    }

    fn evict_inactive_key(&self) {
        self.inactive_key.swap(None);
    }

    pub fn verify_keys(&self) {
        if self.active_key.load().as_ref().inactive_at < Utc::now() {
            self.rotate_keys();
        }

        let inactive_key = self.inactive_key.load();
        let Some(inactive_key) = inactive_key.as_ref()
        else {
            return;
        };

        if inactive_key.expires_at < Utc::now() {
            self.evict_inactive_key();
        }
    }

    pub fn get_signing_key(&self) -> Arc<Key> {
        self.verify_keys();

        Arc::clone(&self.active_key.load())
    }

    pub fn get_verification_key(&self, version: Uuid) -> Option<Arc<Key>> {
        self.verify_keys();

        let active_key = self.active_key.load();
        if version == active_key.as_ref().version {
            return Some(Arc::clone(&active_key));
        }

        let inactive_key = self.inactive_key.load();
        if let Some(inactive_key) = inactive_key.as_ref().as_ref() {
            if version == inactive_key.version {
                return Some(Arc::clone(inactive_key));
            }
        }

        None
    }
}
