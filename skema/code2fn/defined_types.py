from typing import List
from pydantic import BaseModel

class System(BaseModel):
    files: List[str]
    blobs: List[str]
    system_name: str
    root_name: str
