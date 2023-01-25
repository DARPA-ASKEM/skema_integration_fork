import os
import tempfile

from fastapi import FastAPI

#from skema.program_analysis.multi_file_ingester import process_file_system
#from skema.program_analysis.single_file_ingester import process_file
from skema.utils.script_functions import process_file, process_file_system
from skema.utils.fold import dictionary_to_gromet_json, del_nulls

from skema.code2fn.defined_types import System

app = FastAPI()



@app.get("/ping", summary="Ping endpoint to test health of service")
def ping():
    return "The Code2FN service is running."



@app.post(
    "/single",
    summary=(
        "Send a single code file,"
        " get a GroMEt FN Module collection back."
    ),
)
async def root(system: System):
     gromet_collection = process_file(system, False)
     return dictionary_to_gromet_json(del_nulls(gromet_collection.to_dict()))

@app.post(
    "/multi",
    summary=(
        "Send a system of code and filepaths of interest,"
        " get a GroMEt FN Module collection back."
    ),
)
async def root(system: System):
    gromet_collection = process_file_system(system.system_name, system, None)
    return dictionary_to_gromet_json(del_nulls(gromet_collection.to_dict()))
