/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

macro_rules! ansi_esc {
    () => ('\u{1b}');
}

macro_rules! ansi_color {
    ($color_number:expr) => (concat!(ansi_esc!(), '[', $color_number, "m"));
}

macro_rules! ansi_color_reset {
    () => (ansi_color!(0));
}

macro_rules! red {
    ($str:expr) => (concat!(ansi_color!(31), $str, ansi_color_reset!()));
}

macro_rules! yellow {
    ($str:expr) => (concat!(ansi_color!(33), $str, ansi_color_reset!()));
}

macro_rules! blue {
    ($str:expr) => (concat!(ansi_color!(34), $str, ansi_color_reset!()));
}
