/*
    This file is part of Spacelink.

    Copyright (c) 2024 Max Rodriguez <me@maxrdz.com>

    Spacelink is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    Spacelink is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use crate::app::Flags;

use super::app::NavPage;
use super::meson;
use cosmic::cosmic_config::Config;
use cosmic::cosmic_config::{self, cosmic_config_derive::CosmicConfigEntry, CosmicConfigEntry};
use cosmic::theme;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub enum AppTheme {
    Dark,
    Light,
    #[default]
    System,
}

impl AppTheme {
    pub fn theme(&self) -> theme::Theme {
        match self {
            Self::Dark => {
                let mut t = theme::system_dark();
                t.theme_type.prefer_dark(Some(true));
                t
            }
            Self::Light => {
                let mut t = theme::system_light();
                t.theme_type.prefer_dark(Some(false));
                t
            }
            Self::System => theme::system_preference(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
pub enum TimeFormat {
    /// e.g. 12:00 PM, 12:00 AM
    #[default]
    TwelveHour,
    /// e.g. 0:00, 21:00
    TwentyFourHour,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
pub enum KeepMessages {
    OneMonth,
    OneYear,
    #[default]
    Forever,
}

pub const CONFIG_VERSION: u64 = 1;

#[derive(Debug, Clone, CosmicConfigEntry, Eq, PartialEq)]
pub struct SpacelinkConfig {
    /// If this is the first time launching Spacelink.
    pub first_launch: bool,
    pub app_theme: AppTheme,
    pub active_nav_page: NavPage,
    pub time_format: TimeFormat,
    // Storage
    pub messages_on_disk_until: KeepMessages,
    // Notifications
    pub always_notify_if_mentioned: bool,
    pub blocked_contacts: Vec<super::model::sms::SmsContactHash>,
    // SMS
    pub country_code: phonenumber::country::Id,
    pub clear_stuck_sms: bool,
    pub request_delivery_reports: bool,
    // MMS Carrier
    pub mms_enabled: bool,
    pub mmsc_url: String,
    pub mms_apn: String,
    pub mms_proxy: String,
    pub mms_port: u16,
    pub auto_create_smil: bool,
    // RCS
    pub rcs_enabled: bool,
    pub rcs_carrier_business_messages: bool,
    // PGP Encryption
    pub pgp_user_id: String,
    pub public_key_fingerprint: String,
    // Text Messaging
    pub send_read_receipts: bool,
    pub send_typing_messages: bool,
    pub return_sends_message: bool,
    pub render_attachments: bool,
    pub show_char_count: bool,
    pub strip_tracking_url_id: bool,
    /// If we have asked the user if they want Spacelink to
    /// remove tracking IDs from URLs automatically.
    pub asked_strip_tracking_url_id: bool,
}

impl Default for SpacelinkConfig {
    fn default() -> Self {
        Self {
            first_launch: false,
            app_theme: AppTheme::default(),
            active_nav_page: NavPage::default(),
            time_format: TimeFormat::default(),
            // Storage
            messages_on_disk_until: KeepMessages::default(),
            // Notifications
            always_notify_if_mentioned: true,
            blocked_contacts: vec![],
            // SMS
            country_code: phonenumber::country::Id::US,
            clear_stuck_sms: false,
            request_delivery_reports: true,
            // MMS Carrier
            mms_enabled: true,
            mmsc_url: String::default(),
            mms_apn: String::default(),
            mms_proxy: String::default(),
            mms_port: 8080,
            auto_create_smil: true,
            // RCS
            rcs_enabled: false,
            rcs_carrier_business_messages: true,
            // PGP Encryption
            pgp_user_id: String::default(),
            public_key_fingerprint: String::default(),
            // Text Messaging
            send_read_receipts: false,
            send_typing_messages: false,
            return_sends_message: true,
            render_attachments: true,
            show_char_count: true,
            strip_tracking_url_id: false,
            asked_strip_tracking_url_id: false,
        }
    }
}

impl SpacelinkConfig {
    pub fn config_handler() -> Option<Config> {
        Config::new(meson::APP_ID, CONFIG_VERSION).ok()
    }

    pub fn config() -> SpacelinkConfig {
        match Self::config_handler() {
            Some(config_handler) => {
                SpacelinkConfig::get_entry(&config_handler).unwrap_or_else(|(errs, config)| {
                    tracing::info!("errors loading config: {:?}", errs);

                    config
                })
            }
            None => SpacelinkConfig::default(),
        }
    }
}

pub fn flags() -> Flags {
    Flags {
        config_handler: SpacelinkConfig::config_handler(),
        config: SpacelinkConfig::config(),
    }
}
