use std::path::PathBuf;

use anyhow::{bail, Result};
use pimalaya_toolbox::config::TomlConfig;

use crate::config::{Config, ImapConfig};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Panel {
    Mailboxes,
    Envelopes,
}

#[derive(Debug, Clone)]
pub struct Mailbox {
    pub name: String,
    pub delimiter: Option<char>,
    pub subscribed: bool,
}

#[derive(Debug, Clone)]
pub struct Envelope {
    pub uid: u32,
    pub date: String,
    pub from: String,
    pub subject: String,
    pub flags: Vec<String>,
}

pub struct App {
    pub running: bool,
    pub active_panel: Panel,
    pub mailboxes: Vec<Mailbox>,
    pub mailbox_index: usize,
    pub envelopes: Vec<Envelope>,
    pub envelope_index: usize,
    pub selected_mailbox: Option<String>,
    pub account_name: String,
    pub imap_config: ImapConfig,
    pub status_message: Option<String>,
}

impl App {
    pub fn new(config_paths: &[PathBuf], account_name: Option<&str>) -> Result<Self> {
        let config = Config::from_paths_or_default(config_paths)?;
        let (name, account_config) = config.get_account(account_name)?;
        let Some(imap_config) = account_config.imap else {
            bail!("IMAP config is missing for this account")
        };

        Ok(Self {
            running: true,
            active_panel: Panel::Mailboxes,
            mailboxes: Vec::new(),
            mailbox_index: 0,
            envelopes: Vec::new(),
            envelope_index: 0,
            selected_mailbox: None,
            account_name: name,
            imap_config,
            status_message: Some("Loading mailboxes...".to_string()),
        })
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn toggle_panel(&mut self) {
        self.active_panel = match self.active_panel {
            Panel::Mailboxes => Panel::Envelopes,
            Panel::Envelopes => Panel::Mailboxes,
        };
    }

    pub fn next_item(&mut self) {
        match self.active_panel {
            Panel::Mailboxes => {
                if !self.mailboxes.is_empty() {
                    self.mailbox_index = (self.mailbox_index + 1) % self.mailboxes.len();
                }
            }
            Panel::Envelopes => {
                if !self.envelopes.is_empty() {
                    self.envelope_index = (self.envelope_index + 1) % self.envelopes.len();
                }
            }
        }
    }

    pub fn previous_item(&mut self) {
        match self.active_panel {
            Panel::Mailboxes => {
                if !self.mailboxes.is_empty() {
                    self.mailbox_index = self
                        .mailbox_index
                        .checked_sub(1)
                        .unwrap_or(self.mailboxes.len() - 1);
                }
            }
            Panel::Envelopes => {
                if !self.envelopes.is_empty() {
                    self.envelope_index = self
                        .envelope_index
                        .checked_sub(1)
                        .unwrap_or(self.envelopes.len() - 1);
                }
            }
        }
    }

    pub fn select_mailbox(&mut self) {
        if let Some(mailbox) = self.mailboxes.get(self.mailbox_index) {
            self.selected_mailbox = Some(mailbox.name.clone());
            self.envelope_index = 0;
            self.envelopes.clear();
            self.status_message = Some(format!("Loading envelopes from {}...", mailbox.name));
        }
    }

    pub fn set_mailboxes(&mut self, mailboxes: Vec<Mailbox>) {
        self.mailboxes = mailboxes;
        self.mailbox_index = 0;
        if !self.mailboxes.is_empty() {
            self.select_mailbox();
        }
        self.status_message = None;
    }

    pub fn set_envelopes(&mut self, envelopes: Vec<Envelope>) {
        self.envelopes = envelopes;
        self.envelope_index = 0;
        self.status_message = None;
    }

    pub fn set_status(&mut self, message: impl Into<String>) {
        self.status_message = Some(message.into());
    }

    pub fn clear_status(&mut self) {
        self.status_message = None;
    }
}
