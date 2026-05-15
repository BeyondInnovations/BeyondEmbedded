# Setup Pico Development

## C/C++

[The C++ sdk](https://www.raspberrypi.com/documentation/microcontrollers/c_sdk.html)

## Setup

The setup for the Raspberry Pi Pico 2 WH is very similar, but there are a few important differences compared to the original Pico because it uses the newer **RP2350** chip.

---

# 1. Wiring the Debug Probe

The SWD pins are still:

| Pico 2 WH | Debug Probe |
| --------- | ----------- |
| SWDIO     | SWDIO       |
| SWCLK     | SWCLK       |
| GND       | GND         |

Pin locations:

![Image](https://images.openai.com/static-rsc-4/iRiyhOKJWLEjxNFepMcGfs-p45ODgOGkc_yVkUFERivfo7C3jVbFA0RS4-dFQCDAlDWZv8_LAnyLk4A2rM7smJ2TpYsbeFIZRYp0eSCDkl-rsrvUDUnd98IOkQIZlN4NKrrLawCSf9LKtghuoa9IkKjZTVbTVUoBnGcsbrvunZLaW6eWxfnZy9bnKLteBADP?purpose=fullsize)

![Image](https://images.openai.com/static-rsc-4/JkthycUZ2LMH7AydDdOcC-Sky21CN2rwWdOQ8tr079vmgKdPlH1HaVHWktkyAEvphDS7Pb-OBAYN3TgeJiAUcvmkreyF3Qdp5ND7gfJ4gLW7ZFhmOapVnT_jK1BXTG8rP3oDKE25w1nlk8LSphLztd3MwfHbZMPuDuPzsIMPMipBrs7IgauL43O15TKAEOHL?purpose=fullsize)

![Image](https://images.openai.com/static-rsc-4/Zlufp4JGA15dgLuoYfRcJUpk9--a_RBRBRkKhqbKKLkoblOKr0NrWBv5udm42ydWBdXtxseqApZv40wSivFzqoDyh3wn1coEytbaY2L_kgVNIM_r_h1W8NrankbfNR478nn3XGhO3gScvFRBPPh3o7lskQu_snHPjUeTv0Bog1hQ4kQEXioTHsHqmlM4Vp1I?purpose=fullsize)

![Image](https://images.openai.com/static-rsc-4/wR1SnDUyDgZB3J0Jd_2bxPMwSjASCLRmkxsUpN_dO-iWHSDLFVSZ0--Hg1KvatyLpQJUwo4EgnRyWJ15FTcluGpGgM8AqT-TtIz_AqRVSeTXSi3xlpoyi2Eel3cbZrFXP_L39N9mm0QJLPWgT257QnQyjPyw5RH84Tm2wD2UK-YfV-kupWUWdfJ8y0VtTbo8?purpose=fullsize)

![Image](https://images.openai.com/static-rsc-4/78s82qT3VxwUD8StPuRX2gnd-3jDulv2TPZgM39QvOutd2FzuhAgj80nTFGU6Emlf9S0Weofd6t0m_amcgnwgzOwlkbGy5ULQBPlVUSXeFiYkN8LG86CxGDytaNEp7Tg_CdKt5wIMSte-ipuGQlHf4dwLsGEATd21-GFMwsGNpoIv2sppGCrZWfIf_LK41bm?purpose=fullsize)

![Image](https://images.openai.com/static-rsc-4/axvSQ1FmIt87b95cROcK7dSrE-avYvNPO2syrXG5pT9sUO30CJwWqDYX0V6uEe3LF5jaErDasr9ZiUe_adJsMjCDFwAHLXrsB8MxWpSSuUkckPvo-P-OdHbGZIG7lZZetE-gykzlcJAZLkO-xplzAlZqB0KFSQR6bSykhIxtPj4pv3lFSQ0aLqEbLSfotmDH?purpose=fullsize)

The SWD pads are near the USB connector, same general area as Pico 1.

---

# 2. Important: Use Newer OpenOCD

Older OpenOCD versions often do **not** properly support RP2350.

On many Linux distributions, the repo package is too old.

Recommended:

* Use the Raspberry Pi packaged OpenOCD
* Or build newer OpenOCD

---

# 3. Install Raspberry Pi Pico Tools

The easiest path is using the official SDK setup.

## Ubuntu / Debian

Install basics:

```bash id="22x26s"
sudo apt install cmake gcc-arm-none-eabi gdb-multiarch build-essential git
```

Clone SDK:

```bash id="r6q1jh"
git clone https://github.com/raspberrypi/pico-sdk
```

Clone picotool:

```bash id="jlwmca"
git clone https://github.com/raspberrypi/picotool
```

Clone newer OpenOCD:

```bash id="ejql2j"
git clone https://github.com/raspberrypi/openocd.git
```

Build OpenOCD:

```bash id="n1q6w2"
cd openocd
./bootstrap
./configure --enable-cmsis-dap
make -j
sudo make install
```

---

# 4. Use the Correct OpenOCD Target

For Pico 2 / RP2350:

```bash id="9q6a6x"
openocd -f interface/cmsis-dap.cfg -f target/rp2350.cfg
```

NOT `rp2040.cfg`.

That’s the main difference.

---

# 5. Build Your Project in Debug Mode

Example:

```bash id="jlwmnl"
mkdir build
cd build
cmake -DCMAKE_BUILD_TYPE=Debug ..
make
```

---

# 6. Start GDB

```bash id="f2lgk3"
gdb-multiarch blink.elf
```

Then:

```gdb id="gvodvr"
target remote localhost:3333
monitor reset init
load
break main
continue
```

---

# 7. VS Code Setup

The Cortex Debug extension still works.

Main change:

```json id="1pk6m8"
"configFiles": [
  "interface/cmsis-dap.cfg",
  "target/rp2350.cfg"
]
```

instead of `rp2040.cfg`.

---

# 8. Optional: Picoprobe Firmware vs Debug Probe

There are two common debugging setups:

| Device                                  | Purpose            |
| --------------------------------------- | ------------------ |
| Raspberry Pi Debug Probe                | Dedicated debugger |
| Another Pico running picoprobe firmware | DIY debugger       |

You already have the better/easier solution: the official Debug Probe.

---

# 9. Verify Detection

You should see something like:

```text id="c3ghxa"
Info : CMSIS-DAP: SWD supported
Info : RP2350 detected
```

If OpenOCD still says RP2040 or fails:

* OpenOCD version is too old
* Wrong target config

---

# 10. Nice RP2350 Feature

The RP2350 in Pico 2 supports:

* ARM Cortex-M33
* optional RISC-V core support
* TrustZone features
* better debugging support

So later you can also:

* debug multicore execution
* inspect RTOS tasks
* trace interrupts more cleanly
* experiment with ARM vs RISC-V modes

That makes the Pico 2 surprisingly capable as a small embedded development platform.



You’re missing the autotools/libtool build dependencies needed to build [OpenOCD](https://github.com/raspberrypi/openocd?utm_source=chatgpt.com) from source.

On Debian/Ubuntu/NixOS dev environments, install:

```bash
sudo apt install \
  libtool \
  autoconf \
  automake \
  texinfo \
  pkg-config \
  libusb-1.0-0-dev
```

Then rerun:

```bash
./bootstrap
./configure --enable-cmsis-dap
make -j$(nproc)
sudo make install
```

---

For your setup with the Raspberry Pi Pico 2 WH and Raspberry Pi Debug Probe, the critical thing is that the resulting OpenOCD build contains:

* CMSIS-DAP support
* RP2350 target support

After installation, verify:

```bash
openocd --version
```

Then test:

```bash
openocd -f interface/cmsis-dap.cfg -f target/rp2350.cfg
```

You should see the probe and RP2350 being detected.



That means the embedded [Jim Tcl](https://jim.tcl-lang.org/home/doc/trunk/www/index.md?utm_source=chatgpt.com) dependency is missing. OpenOCD uses it internally.

Install the development package first.

On Debian/Ubuntu:

```bash id="6lnn8s"
sudo apt install libjim-dev
```

Then rerun:

```bash id="sdl3l8"
./bootstrap
./configure --enable-cmsis-dap
make -j$(nproc)
sudo make install
```

---

If `libjim-dev` is unavailable on your distro, use the bundled version instead:

```bash id="9kng1o"
./configure --enable-cmsis-dap --disable-werror
```

or explicitly:

```bash id="z20wpw"
./configure --enable-cmsis-dap --with-internal-jimtcl
```

That usually fixes it immediately.

---

For modern RP2350 / Raspberry Pi Pico 2 WH support, the Raspberry Pi OpenOCD fork is currently the correct choice, so you’re on the right path.



You’re missing the HIDAPI development library needed for CMSIS-DAP support used by the Raspberry Pi Debug Probe.

Install it:

```bash id="6e5l57"
sudo apt install libhidapi-dev
```

Then rerun configure/build:

```bash id="8o9xgl"
./configure --enable-cmsis-dap --with-internal-jimtcl
make -j$(nproc)
sudo make install
```

---

If another dependency appears afterward, install it similarly and rerun `configure`.

Typical full dependency set for building [OpenOCD](https://github.com/raspberrypi/openocd?utm_source=chatgpt.com) on Debian/Ubuntu is:

```bash id="5n3gfu"
sudo apt install \
  git \
  build-essential \
  autoconf \
  automake \
  libtool \
  pkg-config \
  texinfo \
  libusb-1.0-0-dev \
  libhidapi-dev \
  libjim-dev
```

After build/install, test with your Raspberry Pi Pico 2 WH:

```bash id="6e4m8s"
openocd -f interface/cmsis-dap.cfg -f target/rp2350.cfg
```

Expected output should include:

* CMSIS-DAP detected
* RP2350 detected
* listening on port 3333 for gdb connections


Perfect — your Raspberry Pi Debug Probe is now correctly connected to the Raspberry Pi Pico 2 WH and OpenOCD successfully detected both Cortex-M33 cores.

This part confirms it:

```text id="sw7y42"
[rp2350.cm0] Examination succeed
[rp2350.cm1] Examination succeed
starting gdb server on 3333
```

You now have a live SWD debugging session available.

---

# Next Step: Connect GDB

Open a second terminal in your build directory:

```bash id="vjlwmm"
gdb-multiarch blink.elf
```

Inside GDB:

```gdb id="vxh52g"
target remote localhost:3333
monitor reset init
load
break main
continue
```

Now execution should stop at `main()`.

---

# Basic Debug Commands

## Step line-by-line

```gdb id="92n43j"
next
```

---

## Step into functions

```gdb id="n0kh5j"
step
```

---

## Continue execution

```gdb id="pjlwm4"
continue
```

---

## Inspect variables

```gdb id="1rm7xh"
print counter
```

---

## Show source

```gdb id="k8t4ol"
list
```

---

# Important: Build With Debug Symbols

If stepping behaves strangely, rebuild:

```bash id="mjlwm0"
cmake -DCMAKE_BUILD_TYPE=Debug ..
make
```

Without debug symbols, GDB cannot map lines correctly.

---

# Multi-Core Note

The RP2350 exposes:

* `rp2350.cm0`
* `rp2350.cm1`

These are the two ARM Cortex-M33 cores.

Initially you’ll mostly debug core0.

Later you can inspect/switch cores:

```gdb id="mxz8cq"
monitor targets
```

or:

```gdb id="5jlc0t"
thread 1
thread 2
```

depending on OpenOCD/GDB mapping.

---

# Nice First Experiment

Modify blink:

```c id="64jchv"
int counter = 0;

while (true) {
    gpio_put(PICO_DEFAULT_LED_PIN, 1);
    sleep_ms(250);

    gpio_put(PICO_DEFAULT_LED_PIN, 0);
    sleep_ms(250);

    counter++;
}
```

Then:

* set breakpoint inside loop
* inspect `counter`
* single-step GPIO toggles

That gives a good feeling for embedded debugging.

---

# Optional Next Improvements

Once this works reliably, the next useful upgrades are:

* VS Code Cortex-Debug integration
* UART logging alongside GDB
* RTOS-aware debugging
* semihosting
* multicore synchronization debugging
* live peripheral register inspection

The RP2350 + Debug Probe setup is actually very capable for embedded development.
