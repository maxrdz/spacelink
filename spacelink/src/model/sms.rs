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

use std::hash::Hash;

use phonenumber::PhoneNumber;

/// Data model for an SMS contact.
///
/// It's unique identifier is its hash, calculated via
/// the [`Hash`] implementation, in hexadecimal format.
pub struct SmsContact {
    name: Option<String>,
    /// If true, avatar image should be found in a cache
    /// directory under the name of this struct's hash.
    avatar: bool,
    phone_number: PhoneNumber,
}

pub type SmsContactHash = super::ModelHash;

impl Hash for SmsContact {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.phone_number.hash(state);
    }
}

impl super::Hashable for SmsContact {}

pub struct SmsConversation {
    contact: SmsContactHash,
    muted: bool,
}

pub struct SmsMessage {
    sender: SmsContact,
    /// Unix timestamp
    timestamp: i64,
    content: String,
}
