/*
    Anura
    Copyright (C) 2024 Joseph Pasfield

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

/*
    I haven't written this much const code in my life this is wack
*/

macro_rules! next {
    ($s:ident) => {{
        let e = $s.a.wrapping_sub($s.b.rotate_left(7));
        $s.a = $s.b ^ $s.c.rotate_left(13);
        $s.b = $s.c.wrapping_add($s.d.rotate_left(37));
        $s.c = $s.d.wrapping_add(e);
        $s.d = e.wrapping_add($s.a);
        $s.d
    }};
}

pub struct Generator {
    a: u16,
    b: u16,
    c: u16,
    d: u16,
}

impl Generator {
    pub const fn new(seed: u16) -> Self {
        let mut thing = Self {
            a: seed,
            b: seed,
            c: seed,
            d: seed,
        };
        // run a few iterations
        let mut i = 0;
        while i < 15 {
            let _num = next!(thing);
            i += 1;
        }

        thing
    }
}

pub fn next_value(generator: &mut Generator) -> u16 {
    next!(generator)
}