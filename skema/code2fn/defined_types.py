from typing import List, Optional
from pydantic import BaseModel

class System(BaseModel):
    files: Optional[List[str]] = None
    blobs: List[str]
    system_name: Optional[str] = ""
    root_name: Optional[str] = ""
