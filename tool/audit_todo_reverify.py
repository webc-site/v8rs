#!/usr/bin/env python3
"""
v8rs todo.md 真实性与桩代码自动对账脚本 (基于 reverify 哲学)
"""

import json
import re
import subprocess
from pathlib import Path

def main():
    root = Path(__file__).resolve().parent.parent
    res = subprocess.run(
        ["cargo", "metadata", "--format-version", "1", "--no-deps"],
        cwd=root,
        capture_output=True,
        text=True,
        check=True
    )
    metadata = json.loads(res.stdout)
    
    crate_stubs = {}
    for pkg in metadata["packages"]:
        c = pkg["name"]
        manifest_dir = Path(pkg["manifest_path"]).parent
        src_dir = manifest_dir / "src"
        if not src_dir.exists():
            continue
        
        todos = 0
        unimpls = 0
        for rs in src_dir.rglob("*.rs"):
            try:
                code = rs.read_text(encoding="utf-8")
            except Exception:
                continue
            clean_code = re.sub(r"/\*[\s\S]*?\*/", "", code)
            clean_code = re.sub(r"//.*", "", clean_code)
            todos += len(re.findall(r"\btodo!\s*\(", clean_code))
            unimpls += len(re.findall(r"\bunimplemented!\s*\(", clean_code))
        
        crate_stubs[c] = {"todos": todos, "unimpls": unimpls}

    todo_path = root / "todo.md"
    lines = todo_path.read_text(encoding="utf-8").splitlines()
    new_lines = []
    
    in_s3 = False
    refuted_count = 0

    for line in lines:
        if line.startswith("## 三、微 crate 看板"):
            in_s3 = True
            new_lines.append(line)
            continue
        if in_s3 and line.startswith("## 四、里程碑"):
            in_s3 = False
            new_lines.append(line)
            continue
            
        if in_s3 and line.strip().startswith("- ["):
            # 匹配 - [x] crate_name — ... 或 - [ ] crate_name — ...
            m = re.match(r"^(\s*-\s*\[)([ x])(\]\s+)([a-zA-Z0-9_\-]+(?:/[a-zA-Z0-9_\-]+)?)(.*)$", line)
            if m:
                prefix, state, mid, crate_field, rest = m.groups()
                # 提取主 crate 名称（可能为 a / b 形式）
                names = [n.strip() for n in crate_field.split("/")]
                
                total_todos = sum(crate_stubs.get(n, {}).get("todos", 0) for n in names)
                total_unimpls = sum(crate_stubs.get(n, {}).get("unimpls", 0) for n in names)
                
                # 清除历史驳回标记
                cleaned_rest = re.sub(r"\s*\(审计驳回:\s*REFUTED\s*-[^)]*\)", "", rest)
                
                if total_todos > 0 or total_unimpls > 0:
                    reasons = []
                    if total_todos > 0:
                        reasons.append(f"{total_todos} 处 todo!()")
                    if total_unimpls > 0:
                        reasons.append(f"{total_unimpls} 处 unimplemented!()")
                    reason_str = "、".join(reasons)
                    
                    new_line = f"{prefix} {mid}{crate_field}{cleaned_rest} (审计驳回: REFUTED - 残余 {reason_str} 桩代码)"
                    refuted_count += 1
                    new_lines.append(new_line)
                    continue
                else:
                    new_lines.append(f"{prefix}{state}{mid}{crate_field}{cleaned_rest}")
                    continue

        new_lines.append(line)

    todo_path.write_text("\n".join(new_lines) + "\n", encoding="utf-8")
    print(f"对账完成: 共发现并驳回 {refuted_count} 个虚假完成条目！")

if __name__ == "__main__":
    main()
