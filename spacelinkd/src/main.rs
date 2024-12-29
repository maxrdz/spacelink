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

use smol::prelude::*;
use std::future::pending;
use zbus::{connection, interface};

#[derive(Default)]
struct Greeter;

#[interface(name = "com.maxrdz.Spacelink.Daemon1")]
impl Greeter {
    fn say_hello(&mut self, name: &str) -> String {
        format!("Hello {}! I have been called.", name)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    smol::block_on(async {
        let connection = connection::Builder::session()?
            .name(meson::DBUS_NAME)?
            .serve_at("/com/maxrdz/Spacelink/Daemon", Greeter::default())?
            .build()
            .await?;

        // Do other things or go to wait forever
        pending::<()>().await;

        Ok(())
    })
}
