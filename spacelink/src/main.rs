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

mod app;
mod config;
mod i18n;
mod meson;

fn main() -> cosmic::iced::Result {
    // Get the system's preferred languages.
    let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();

    // Enable localizations to be applied.
    i18n::init(&requested_languages);

    // Settings for configuring the application window and iced runtime.
    let settings = cosmic::app::Settings::default()
        .size_limits(cosmic::iced::Limits::NONE.min_width(360.0).min_height(150.0))
        .size(cosmic::iced::Size::new(360.0, 576.0));

    // Initialize Lfb for haptic feedback.
    #[cfg(feature = "use-feedbackd")]
    if let Err(lfb_error) = libfeedback::init(meson::APP_ID) {
        tracing::error!("Failed to initialize Lfb for haptic feedback: {}", lfb_error);
    }

    // Starts the application's event loop with `()` as the application's flags.
    cosmic::app::run::<app::Spacelink>(settings, ())
}
