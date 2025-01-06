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

pub mod mms;
pub mod sms;

use std::hash::{DefaultHasher, Hash, Hasher};

/// Represents a type of SMS/MMS conversation.
pub enum Conversation {
    Individual(sms::SmsConversation),
    Group(mms::MmsGroupChat),
}

type ModelHash = u64;

trait Hashable
where
    Self: Hash,
{
    fn get_hash(model: impl Hash) -> ModelHash {
        let mut state = DefaultHasher::default();

        model.hash(&mut state);
        state.finish()
    }
}
