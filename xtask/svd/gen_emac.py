#!/usr/bin/env python3
"""Generate the EMAC SVD fragment from the ESP32-P4 technical reference manual.

The vendor SVD (`esp32p4.svd`) does not describe the Ethernet MAC block, so the
register set is extracted from the TRM LaTeX sources instead:

    /path/to/esp-technical-reference-manual-latex/ESP32-P4/53-ETHERNETMAC/
        53-ETHERNETMAC-reg__EN.tex

Run from the repository root:

    python3 xtask/svd/gen_emac.py
    cargo xtask gen-pac

The generated `xtask/svd/emac.svd` is committed, so neither the TRM sources nor
Python are needed for a normal build.
"""

import os
import re
import sys

TRM_DEFAULT = os.path.expanduser(
    "~/code/esp32/esp-technical-reference-manual-latex/ESP32-P4/53-ETHERNETMAC/53-ETHERNETMAC-reg__EN.tex"
)
OUT_DEFAULT = os.path.join(os.path.dirname(os.path.abspath(__file__)), "emac.svd")

# `EMACCONFIG_REG` -> `EMACCONFIG`; the generated PAC prepends nothing, so the
# field names inside a register only need to be unique within that register.
REG_SUFFIX = "_REG"

ACCESS_PATTERNS = [
    (r"\(R/W\)", "read-write"),
    (r"\(RO\)", "read-only"),
    (r"\(WT\)", "write-only"),
]


def parse_tex(path):
    """Return {offset: {"name": str, "fields": [(name, lsb, width, reset)]}}."""
    with open(path, encoding="utf-8") as handle:
        text = handle.read()

    registers = {}
    # Register headers repeat for multi-page register descriptions; only the
    # first header carries all the `regfield` lines, so collect them adjacent.
    for block in re.split(r"(?=\\begin\{register\}\{H\})", text):
        header = re.match(
            r"\\begin\{register\}\{H\}\{([^}]*)\}\{(0x[0-9a-fA-F]+)\}", block
        )
        if not header:
            continue

        name = header.group(1).replace("\\_", "_")
        if name.endswith(REG_SUFFIX):
            name = name[: -len(REG_SUFFIX)]
        offset = int(header.group(2), 16)

        fields = registers.setdefault(offset, {"name": name, "fields": []})
        if not name.startswith("DMA") and fields["name"].startswith("DMA"):
            pass  # keep the first name seen for this offset

        for field_name, width, lsb, reset in re.findall(
            r"\\regfield\{([^}]*)\}\{(\d+)\}\{(\d+)\}\{([^}]*)\}", block
        ):
            if field_name == "(reserved)":
                continue

            # Some registers wrap the name in parentheses, e.g. `(DMAREBUILDINCRBURST)`.
            field_name = field_name.strip("()")
            field_name = field_name.replace("\\_", "_").replace("\\", "")
            if not field_name:
                # `EMACADDR0LOW_REG` describes its single 32-bit field as empty.
                field_name = "ADDRESS_LOW"
            fields["fields"].append(
                (field_name, int(lsb), int(width), parse_reset(reset, int(width)))
            )

    for value in registers.values():
        # TRM lists fields MSB-first; sorting makes the output stable.
        value["fields"].sort(key=lambda f: f[1])

    return registers


def parse_reset(text, width):
    text = text.strip().strip("{}").strip()
    if not text:
        return 0
    if re.fullmatch(r"[01]+", text):
        return int(text, 2)
    return int(text, 16) if text.lower().startswith("0x") else int(text, 16)


def unique_names(fields):
    """Rename collisions (and Rust keywords) so the PAC generator emits them."""
    seen = {}
    out = []
    for name, lsb, width, reset in fields:
        candidate = name
        if candidate in seen:
            seen[candidate] += 1
            candidate = f"{name}_{seen[name]}"
        else:
            seen[candidate] = 0
        out.append((candidate, lsb, width, reset))
    return out


def clean(text):
    return text.replace("&", "&amp;").replace("<", "&lt;").replace(">", "&gt;")


def render(registers):
    lines = [
        '<?xml version="1.0" encoding="UTF-8"?>',
        '<device schemaVersion="1.1">',
        "  <vendor>ESPRESSIF SYSTEMS (SHANGHAI) CO., LTD.</vendor>",
        "  <vendorID>ESPRESSIF</vendorID>",
        "  <name>ESP32-P4</name>",
        "  <version>2</version>",
        "  <description>ESP32-P4 EMAC (extracted from the TRM)</description>",
        "  <peripherals>",
        "    <peripheral>",
        "      <name>EMAC</name>",
        "      <description>Ethernet MAC (DesignWare 10/100 MAC + DMA)</description>",
        "      <groupName>EMAC</groupName>",
        "      <baseAddress>0x50098000</baseAddress>",
        "      <registers>",
    ]

    for offset in sorted(registers):
        reg = registers[offset]
        fields = unique_names(reg["fields"])
        reserved = set(range(32))
        reset_value = 0
        for _, lsb, width, reset in fields:
            reset_value |= (reset & ((1 << width) - 1)) << lsb
            reserved -= set(range(lsb, lsb + width))

        lines.append("        <register>")
        lines.append(f"          <name>{reg['name']}</name>")
        lines.append(f"          <description>{reg['name']} register</description>")
        lines.append(f"          <addressOffset>0x{offset:x}</addressOffset>")
        lines.append("          <size>0x20</size>")
        lines.append(f"          <resetValue>0x{reset_value:08X}</resetValue>")
        lines.append("          <fields>")
        for name, lsb, width, _ in fields:
            lines.append("            <field>")
            lines.append(f"              <name>{name}</name>")
            lines.append(f"              <description>{name}</description>")
            lines.append(f"              <bitOffset>{lsb}</bitOffset>")
            lines.append(f"              <bitWidth>{width}</bitWidth>")
            lines.append("              <access>read-write</access>")
            lines.append("            </field>")
        lines.append("          </fields>")
        lines.append("        </register>")

    lines += [
        "      </registers>",
        "    </peripheral>",
        "  </peripherals>",
        "</device>",
        "",
    ]
    return "\n".join(lines)


def main():
    tex = sys.argv[1] if len(sys.argv) > 1 else TRM_DEFAULT
    out = sys.argv[2] if len(sys.argv) > 2 else OUT_DEFAULT

    if not os.path.exists(tex):
        sys.exit(f"TRM register description not found: {tex}")

    registers = parse_tex(tex)
    with open(out, "w", encoding="utf-8") as handle:
        handle.write(render(registers))

    field_count = sum(len(r["fields"]) for r in registers.values())
    print(f"wrote {out}: {len(registers)} registers, {field_count} fields")


if __name__ == "__main__":
    main()
