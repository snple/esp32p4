#!/usr/bin/env python3
"""Generate ``timg.svd``: the complete TIMG0/TIMG1 register blocks.

The vendor SVD describes only timer 0 of each timer group; timer 1 sits in a
reserved hole between offsets 0x24 and 0x47. The two timers are the same nine
registers 0x24 bytes apart (ESP-IDF models them as
``timg_hwtimer_reg_t hw_timer[2]``), so the missing half is a copy of timer 0
with the register number bumped and the offset shifted.

``cargo xtask gen-pac`` merges the result over the vendor's TIMG peripherals, so
the HAL can talk to both counters through the PAC instead of hand-written
register access.

Usage: ``python3 xtask/gen_timg.py`` from the repository root.
"""

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
VENDOR = ROOT / "esp32p4.svd"
OUTPUT = ROOT / "timg.svd"

# The nine registers of one counter, in the order they appear.
TIMER_REGISTERS = 9
# Distance between the two counters of a group.
TIMER_STRIDE = 0x24


def peripheral(source: str, name: str) -> str:
    """The text of one ``<peripheral>`` block."""
    start = source.find(f"<name>{name}</name>")
    if start < 0:
        raise SystemExit(f"{name} not found in {VENDOR}")
    start = source.rfind("<peripheral>", 0, start)
    end = source.find("</peripheral>", start) + len("</peripheral>")
    return source[start:end]


def registers(block: str) -> list[str]:
    """The ``<register>`` blocks of a peripheral, in file order."""
    return [m.group(0) for m in re.finditer(r"<register>.*?</register>", block, re.S)]


def offset(register: str) -> int:
    return int(re.search(r"<addressOffset>([^<]+)</addressOffset>", register).group(1), 0)


def rename(register: str) -> str:
    """The same register for timer 1."""
    text = re.sub(r"<name>T0([A-Z]+)</name>", r"<name>T1\1</name>", register, count=1)
    return re.sub(
        r"<addressOffset>([^<]+)</addressOffset>",
        lambda m: f"<addressOffset>0x{int(m.group(1), 0) + TIMER_STRIDE:X}</addressOffset>",
        text,
        count=1,
    )


def emit(name: str, description: str, base: int, interrupts: list[tuple[str, int]], body: list[str]) -> str:
    lines = [
        "    <peripheral>",
        f"      <name>{name}</name>",
        f"      <description>{description}</description>",
        "      <groupName>TIMG</groupName>",
        f"      <baseAddress>0x{base:08X}</baseAddress>",
        "      <addressBlock>",
        "        <offset>0x0</offset>",
        "        <size>0x100</size>",
        "        <usage>registers</usage>",
        "      </addressBlock>",
    ]
    for interrupt, value in interrupts:
        lines += [
            "      <interrupt>",
            f"        <name>{interrupt}</name>",
            f"        <value>{value}</value>",
            "      </interrupt>",
        ]
    lines.append("      <registers>")
    lines += body
    lines += ["      </registers>", "    </peripheral>"]
    return "\n".join(lines)


def main() -> None:
    vendor = VENDOR.read_text(encoding="utf-8", errors="replace")
    timg0 = peripheral(vendor, "TIMG0")
    blocks = registers(timg0)

    timer0 = [r for r in blocks if offset(r) < TIMER_STRIDE]
    group = [r for r in blocks if offset(r) >= TIMER_STRIDE]
    if len(timer0) != TIMER_REGISTERS:
        raise SystemExit(f"expected {TIMER_REGISTERS} timer registers, found {len(timer0)}")

    # Everything in offset order, with timer 1 next to timer 0.
    body = timer0 + [rename(r) for r in timer0] + group

    out = [
        '<?xml version="1.0" encoding="UTF-8"?>',
        '<device schemaVersion="1.1">',
        "  <vendor>ESPRESSIF SYSTEMS (SHANGHAI) CO., LTD.</vendor>",
        "  <vendorID>ESPRESSIF</vendorID>",
        "  <name>ESP32-P4</name>",
        "  <version>2</version>",
        "  <description>ESP32-P4 timer groups (timer 1 completed from the TRM layout)</description>",
        "  <peripherals>",
        emit(
            "TIMG0",
            "Timer Group 0",
            0x500C2000,
            [("TG0_T0", 46), ("TG0_T1", 47), ("TG0_WDT", 48)],
            body,
        ),
        emit(
            "TIMG1",
            "Timer Group 1",
            0x500C3000,
            [("TG1_T0", 49), ("TG1_T1", 50), ("TG1_WDT", 51)],
            body,
        ),
        "  </peripherals>",
        "</device>",
        "",
    ]
    OUTPUT.write_text("\n".join(out), encoding="utf-8")
    print(f"Wrote {OUTPUT.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
