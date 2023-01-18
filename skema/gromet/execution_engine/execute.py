from typing import Any
import importlib

module_imports = {}

def execute_primitive(primitive: str, inputs: list[Any]) -> Any:
    # Check if primitive is imported
    import_path = primitive.rsplit(".", 1)
    module = import_path[0]
    primitive = import_path[1]
    if module not in module_imports:
        try:
            module_imports[module] = importlib.import_module(module) 
        except:
            print(f"Could not find module to import for: {primitive}")
            return None
    
    # Attempt to execute primitive
    try:
        f = getattr(module_imports[module], primitive)
        return f(*inputs)
    except:
        print(f"Could not execute primitive: {primitive}")
        return None

primitive = "operator.add"
inputs = [1, 4]
print(execute_primitive(primitive, inputs))
print(execute_primitive("operator.sub", [1,-2]))