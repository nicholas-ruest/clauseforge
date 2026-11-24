import ast, math, os, re
from pathlib import Path
from types import SimpleNamespace
import regex

root = Path(os.environ["CLAUSEFORGE_UPSTREAM_ROOT"]) / "guardrail"
module = ast.parse((root / "main.py").read_text())
names = {"extract_value", "value_exists", "check_condition"}
functions = [node for node in module.body if isinstance(node, ast.FunctionDef) and node.name in names]
namespace = {"re": re, "math": math, "regex": regex}
exec(compile(ast.Module(body=functions, type_ignores=[]), str(root / "main.py"), "exec"), namespace)
check = namespace["check_condition"]
details = {
    "supplier": os.environ["CLAUSEFORGE_SUPPLIER"],
    "amendment": os.environ["CLAUSEFORGE_AMENDMENT"],
    "controls": int(os.environ["CLAUSEFORGE_AFFECTED_CONTROLS"]),
    "cure_days": int(os.environ["CLAUSEFORGE_CURE_DAYS"]),
}
analysis = SimpleNamespace(details=details)
required = [
    SimpleNamespace(key="supplier", condition_type="exists", threshold=None),
    SimpleNamespace(key="amendment", condition_type="exists", threshold=None),
    SimpleNamespace(key="cure_days", condition_type="greater", threshold=0),
]
if not all(check(analysis, condition) for condition in required):
    raise SystemExit("guardrail rejected amendment structure")
print(f"guardrail_valid:{details['controls']}:{details['cure_days']}")
