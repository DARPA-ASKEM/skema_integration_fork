import os
import tempfile
import glob

from fastapi import FastAPI
from fastapi import File, UploadFile

from io import BytesIO
from zipfile import ZipFile
from urllib.request import urlopen


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
    "/fn-given-filepaths",
    summary=(
        "Send a single code file,"
        " get a GroMEt FN Module collection back."
    ),
)
async def root(system: System):
    gromet_collection = process_file_system(system.system_name, system, None)
    return dictionary_to_gromet_json(del_nulls(gromet_collection.to_dict()))

@app.post(
    "/fn-given-filepaths-zip",
    summary=(
        "Send a single code file,"
        " get a GroMEt FN Module collection back."
    ),
)
async def root(file: UploadFile = File()):
    system = zip_to_system(file)
    gromet_collection = process_file_system(system.system_name, system, None)
    return dictionary_to_gromet_json(del_nulls(gromet_collection.to_dict()))

def zip_to_system(file: UploadFile) -> System:
    with ZipFile(BytesIO(file.file.read()), "r") as zip:
        
        file_list = [f for f in zip.namelist() if f.endswith(".py")]
    
        blobs=[]
        for path in file_list:
            with zip.open(path) as f:
                blobs.append(f.read())
  
        system_name = file.filename.strip(".zip")
        root_name = file.filename.strip(".zip")
        
    print("-------------------------------")
    print(file_list)
    print(system_name)
    print(root_name)
    print("--------------------------------")
    return System(files=file_list, blobs=blobs, system_name=system_name, root_name=root_name)
