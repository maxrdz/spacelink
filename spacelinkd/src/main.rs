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

mod meson;

use tokio::runtime::{Builder, Runtime};
use zbus::{connection, interface};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tokio_rt: Runtime = Builder::new_current_thread()
        .thread_stack_size(2 * 1024 * 1024)
        .build()
        .expect("Failed to start Tokio async runtime.");

    tokio_rt.block_on(async {
        let connection = connection::Builder::session()?
            .name(meson::DBUS_NAME)?
            .build()
            .await?;

        Ok(())
    })
}
