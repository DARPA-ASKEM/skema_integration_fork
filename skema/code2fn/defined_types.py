from typing import List, Optional
from pydantic import BaseModel

class System(BaseModel):
    files: Optional[List[str]] = None
    blobs: Optional[List[str]]
    system_name: Optional[str] = ""
    root_name: Optional[str] = ""
