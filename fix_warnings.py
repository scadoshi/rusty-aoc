#!/usr/bin/env python3
from pathlib import Path

BASE_DIR = Path("/home/scottyrayfermo/rusty-aoc/src/lib")

PART_01_TEMPLATE = """#[allow(dead_code)]
pub fn part_01(_input: &str) -> i32 {
    // TODO: implement
    0
}
"""

PART_02_TEMPLATE = """#[allow(dead_code)]
pub fn part_02(_input: &str) -> i32 {
    // TODO: implement
    0
}
"""

def fix_templates():
    """Fix all part templates to use _input instead of input."""
    print("Fixing template files to use _input...")
    
    count = 0
    for year_dir in sorted(BASE_DIR.glob("year_*")):
        if not year_dir.is_dir():
            continue
        
        day_01_dir = year_dir / "day_01"
        
        # Fix part_01.rs
        part_01_file = day_01_dir / "part_01.rs"
        if part_01_file.exists():
            part_01_file.write_text(PART_01_TEMPLATE)
            count += 1
        
        # Fix part_02.rs
        part_02_file = day_01_dir / "part_02.rs"
        if part_02_file.exists():
            part_02_file.write_text(PART_02_TEMPLATE)
            count += 1
    
    print(f"✓ Fixed {count} files")

if __name__ == "__main__":
    fix_templates()

