GadgetNES
========
Writing my first emulator in Rust!

Purpose
-------
My goal for this project is to become more familiar with Rust, develop my programming skills, and learn some emulator development to hopefully make more projects!

Design Choices
--------------
Here are some reasons for various design choices I made while writing this emulator:
* Graphics: SDL2. There are Rust bindings for SDL and there is quite a large amount of documentation and examples (mostly for C++) that is applicable. This is my first time really doing graphics/audio/input programming (outisde of FLTK in my intro comp-sci class) so I am still having to learn quite a bit while writing (which is likely contributing to the amount of time it's taking me to write this D:)

Documents
---------
These are documents or websites I have found particularly useful during this project:
* Yupferris' awesome Rustendo64 project: https://github.com/yupferris/rustendo64
* Pretty much anytime I get profoundly stuck, I'm looking at pcwalton's sprocketnes: https://github.com/pcwalton/sprocketnes
* The NESdev Wiki: http://wiki.nesdev.com/w/index.php/Nesdev_Wiki
* Forum post on NintendoAge.com: http://nintendoage.com/forum/messageview.cfm?catid=22&threadid=7155
* http://www.emulator101.com/reference/6502-reference.html
* http://6502.org/tutorials/6502opcodes.html
* https://en.wikibooks.org/wiki/6502_Assembly
* Holy crap did this help in understanding PPU operation: http://www.dustmop.io/blog/2015/04/28/nes-graphics-part-1/

Other helpful materials
-----------------------
These are other materials I found helpful while writing this emulator:
* https://github.com/amaiorano/nes-disasm/

License
-------
Licensed under GPLv3 (http://www.gnu.org/licenses/gpl-3.0.html)
