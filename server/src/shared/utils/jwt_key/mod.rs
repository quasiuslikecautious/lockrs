use chrono::{DateTime, Utc, Duration};
use rand::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::utils::serde::duration_serde;

#[derive(Debug, Serialize, Deserialize)]
pub struct Key {
    pub version: Uuid,
    pub value: Vec<u8>,
    pub created_at: DateTime<Utc>,
    pub inactive_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RotatingKey {
    active_key: Box<Option<Key>>,
    inactive_key: Box<Option<Key>>,
    #[serde(with = "duration_serde")]
    rotation_duration: Duration,
    #[serde(with = "duration_serde")]
    transition_duration: Duration,
}

impl RotatingKey {
    pub fn new(rotation_duration: Duration, transition_duration: Duration) -> Self {
        assert!(rotation_duration > transition_duration, "The Transition duration must be shorter than the Rotation duration!");

        Self {
            active_key: Box::new(None),
            inactive_key: Box::new(None),
            rotation_duration,
            transition_duration,
        }
    }

    fn generate_new_key(&self) -> Key {
        let now = Utc::now();

        let mut key = vec![0u8; 32];
        rand::thread_rng().fill_bytes(&mut key);
    
        // convert to jwt key
        Key {
            version: Uuid::new_v4(),
            value: key,
            created_at: now,
            inactive_at: now + self.rotation_duration,
            expires_at: now + self.rotation_duration + self.transition_duration,
        }
    }
    
    pub fn rotate_keys(&mut self) {
        // swap active key,
        let temp = Box::new(Some(self.generate_new_key()));
        let temp = std::mem::replace(&mut self.active_key, temp);
        *self.inactive_key = *temp;
    }

    pub fn get_signing_key(&mut self) -> &Key {
        if let Some(key) = &self.active_key.as_ref() {
            if key.inactive_at < Utc::now() {
                self.rotate_keys();
            }
        } else {
            self.rotate_keys();
        }

        let key = self.active_key.as_ref();
        key.as_ref().unwrap()
    }

    pub fn get_verification_key(&self, version: Uuid) -> &Option<Key> {
        if let Some(key) = &self.active_key.as_ref() {
            if key.version == version {
                return &*self.active_key;
            }
        }

        if let Some(key) = &self.inactive_key.as_ref() {
            if key.version == version && key.expires_at > Utc::now() {
                return &*self.inactive_key;
            }
        }

        &None
    }
}
