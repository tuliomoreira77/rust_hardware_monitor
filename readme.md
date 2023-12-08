*This repository is my first attempt to program in Rust, so I'm attending to the basics features of the language (first things first).*

It's a hardware stats monitor (clock, temp, mem usage, etc etc...). Later I want to send this information to an Arduino that will drive an OLED display, displaying the information to compose my setup.

**Right now it support this stats and hardware:**

**Geforce RTX:**
- Temperature
- Mem Clock
- Core Clock
- Mem usage (%)
- Core usage (%)
- FPS

**AMD Zen2 processors** **(I'ves tested on Ryzen 5 4500)**
- Temperature

## **Considerations**

I had borrow some code from "alex-down" on this repository (I will make a PR with the modifications I made):
https://github.com/alex-dow/winRing0-rs

And get some inspiration from (it's in C#):
https://github.com/openhardwaremonitor/openhardwaremonitor
