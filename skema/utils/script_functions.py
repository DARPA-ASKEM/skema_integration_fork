import ast
import dill
import os.path
import json

# import astpp

from skema.gromet.fn import (
    GrometFNModuleCollection,
)

from skema.utils.fold import dictionary_to_gromet_json, del_nulls
from skema.program_analysis.PyAST2CAST import py_ast_to_cast
from skema.program_analysis.CAST2GrFN import cast
from skema.program_analysis.CAST2GrFN.model.cast import SourceRef
from skema.program_analysis.CAST2GrFN.cast import CAST
from skema.program_analysis.CAST2GrFN.visitors.cast_to_agraph_visitor import (
    CASTToAGraphVisitor,
)
from skema.program_analysis.CAST2GrFN.ann_cast.cast_to_annotated_cast import (
    CastToAnnotatedCastVisitor,
)
from skema.program_analysis.CAST2GrFN.ann_cast.id_collapse_pass import (
    IdCollapsePass,
)
from skema.program_analysis.CAST2GrFN.ann_cast.container_scope_pass import (
    ContainerScopePass,
)
from skema.program_analysis.CAST2GrFN.ann_cast.variable_version_pass import (
    VariableVersionPass,
)
from skema.program_analysis.CAST2GrFN.ann_cast.grfn_var_creation_pass import (
    GrfnVarCreationPass,
)
from skema.program_analysis.CAST2GrFN.ann_cast.grfn_assignment_pass import (
    GrfnAssignmentPass,
)
from skema.program_analysis.CAST2GrFN.ann_cast.lambda_expression_pass import (
    LambdaExpressionPass,
)
from skema.program_analysis.CAST2GrFN.ann_cast.to_grfn_pass import ToGrfnPass
from skema.program_analysis.CAST2GrFN.ann_cast.to_gromet_pass import (
    ToGrometPass,
)

from skema.code2fn.defined_types import System
from typing import Union
import tempfile

def process_file(system_source: Union[str,System], write_to_file=False):

    if isinstance(system_source, str):
        system_name = os.path.basename(system_source).strip(".py")
        root_path = os.path.dirname(system_source)

        # Create temporary system_filepaths file
        tmp = tempfile.NamedTemporaryFile(mode="w", delete=False)
        tmp.write(os.path.basename(system_source))
        tmp.close()

        gromet_collection = process_file_system(system_name, root_path, tmp.name, write_to_file)

        # Delete temporary system_filepaths file
        os.unlink(tmp.name)

    elif isinstance(system_source, System):
        system_name = system_source.system_name 
        gromet_collection = process_file_system(system_name, system_source, None, write_to_file)

    return gromet_collection

def process_file_system(system_name,  system_source: Union[str, System], files=None, write_to_file=False):
    # Create list of files either by getting it directly from the System, or by opening and reading an external file 
    if isinstance(system_source, str):
        file_list = open(files, "r").readlines()
        root_dir = system_source.strip() 
    elif isinstance(system_source, System):
        file_list = system_source.files
        root_dir = ""  # Required for backwards compatibility with path as input

    
    module_collection = GrometFNModuleCollection(
        schema_version="0.1.5",
        name=system_name,
        modules=[],
        module_index=[],
        executables=[],
    )

    for index,f in enumerate(file_list):
        # Get either the full path to the source file, or the source file as a string
        if isinstance(system_source, str):
            full_file = os.path.join(os.path.normpath(root_dir), f.rstrip("\n"))
        elif isinstance(system_source, System):
            # Convert module json to Python obj and get file blob as str
            full_file = system_source.blobs[index]
        
        try:
            cast = python_to_cast(full_file, isinstance(system_source, System), cast_obj=True)
            generated_gromet = ann_cast_pipeline(
                cast, gromet=True, to_file=False, from_obj=True
            )

            # Then, after we generate the GroMEt we store it in the 'modules' field
            # and store its path in the 'module_index' field
            module_collection.modules.append(generated_gromet)

            # DONE: Change this so that it's the dotted path from the root
            # i.e. like model.view.sir" like it shows up in Python
            source_directory = os.path.basename(
                os.path.normpath(root_dir)
            )  # We just need the last directory of the path, not the complete path
            os_module_path = os.path.join(source_directory, f)
           
           # Normalize the path across os and then convert to module dot notation
            python_module_path = ".".join(os.path.normpath(os_module_path).split(os.path.sep))
            python_module_path = python_module_path.replace(".py", "").strip()
            module_collection.module_index.append(python_module_path)

            # Done: Determine how we know a gromet goes in the 'executable' field
            # We do this by finding all user_defined top level functions in the Gromet
            # and check if the name 'main' is among them
            function_networks = [
                fn.value
                for fn in generated_gromet.attributes
                if fn.type == "FN"
            ]
            defined_functions = [
                fn.b[0].name
                for fn in function_networks
                if fn.b[0].function_type == "FUNCTION"
            ]
            if "main" in defined_functions:
                module_collection.executables.append(len(module_collection.module_index))
        except ImportError:
            print("FAILURE")

    # After we go through the whole system, we can then write out the module_collection
    if write_to_file:
        with open(f"{system_name}--Gromet-FN-auto.json", "w") as f:
            gromet_collection_dict = module_collection.to_dict()
            f.write(
                dictionary_to_gromet_json(del_nulls(gromet_collection_dict))
            )

    return module_collection


def python_to_cast(
    py_source, # Can be either a string or a path
    py_source_is_str=False,
    agraph=False,
    astprint=False,
    std_out=False,
    rawjson=False,
    legacy=False,
    cast_obj=False,
):
    if py_source_is_str:
        file_contents = py_source 
        file_name = "" # Required for backwards compatibility
    else:
        # Open Python file as a giant string
        with open(py_source, "r") as f:
            file_contents = f.read()
            file_name = os.path.basename(py_source)
    line_count = file_contents.count("\n") 

    # Create a PyASTToCAST Object
    if legacy:
        convert = py_ast_to_cast.PyASTToCAST(file_name, legacy=True)
    else:
        convert = py_ast_to_cast.PyASTToCAST(file_name)

    # Additional option to allow us to view the PyAST
    # using the astpp module
    if astprint:
        # astpp.parseprint(file_contents)
        print("AST Printing Currently Disabled")
        pass

    # Parse the python program's AST and create the CAST
    contents = ast.parse(file_contents)
    C = convert.visit(contents, {}, {})
    C.source_refs = [SourceRef(file_name, None, None, 1, line_count)]
    out_cast = cast.CAST([C], "python")

    if agraph:
        V = CASTToAGraphVisitor(out_cast)
        last_slash_idx = file_name.rfind("/")
        file_ending_idx = file_name.rfind(".")
        pdf_file_name = (
            f"{file_name[last_slash_idx + 1 : file_ending_idx]}.pdf"
        )
        V.to_pdf(pdf_file_name)

    # Then, print CAST as JSON
    if cast_obj:
        return out_cast
    else:
        if rawjson:
            print(
                json.dumps(
                    out_cast.to_json_object(), sort_keys=True, indent=None
                )
            )
        else:
            if std_out:
                print(out_cast.to_json_str())
            else:
                out_name = file_name.split(".")[0]
                print("Writing CAST to " + out_name + "--CAST.json")
                out_handle = open(out_name + "--CAST.json", "w")
                out_handle.write(out_cast.to_json_str())


def ann_cast_pipeline(
    cast_instance,
    to_file=True,
    gromet=False,
    grfn_2_2=False,
    a_graph=False,
    from_obj=False,
    indent_level=0,
):
    """cast_to_annotated.py

    This function reads a JSON file that contains the CAST representation
    of a program, and transforms it to annotated CAST. It then calls a
    series of passes that each augment the information in the annotatd CAST nodes
    in preparation for the GrFN generation.

    One command-line argument is expected, namely the name of the JSON file that
    contains the CAST data.
    TODO: Update this docstring as the program has been tweaked so that this is a function instead of
    the program
    """

    if from_obj:
        f_name = ""
        cast = cast_instance
    else:
        f_name = cast_instance
        f_name = f_name.split("/")[-1]
        file_contents = open(f_name, "r").read()

        cast_json = CAST([], "python")
        cast = cast_json.from_json_str(file_contents)

    visitor = CastToAnnotatedCastVisitor(cast)
    # The Annotated Cast is an attribute of the PipelineState object
    pipeline_state = visitor.generate_annotated_cast(grfn_2_2)

    # TODO: make filename creation more resilient

    print("Calling IdCollapsePass------------------------")
    IdCollapsePass(pipeline_state)

    print("\nCalling ContainerScopePass-------------------")
    ContainerScopePass(pipeline_state)

    print("\nCalling VariableVersionPass-------------------")
    VariableVersionPass(pipeline_state)

    # NOTE: CASTToAGraphVisitor uses misc.uuid, so placing it here means
    # that the generated GrFN uuids will not be consistent with GrFN uuids
    # created during test runtime. So, do not use these GrFN jsons as expected
    # json for testing
    f_name = f_name.replace("--CAST.json", "")
    if a_graph:
        agraph = CASTToAGraphVisitor(pipeline_state)
        pdf_file_name = f"{f_name}-AnnCast.pdf"
        agraph.to_pdf(pdf_file_name)

    print("\nCalling GrfnVarCreationPass-------------------")
    GrfnVarCreationPass(pipeline_state)

    print("\nCalling GrfnAssignmentPass-------------------")
    GrfnAssignmentPass(pipeline_state)

    print("\nCalling LambdaExpressionPass-------------------")
    LambdaExpressionPass(pipeline_state)

    if gromet:
        print("\nCalling ToGrometPass-----------------------")
        ToGrometPass(pipeline_state)

        if to_file:
            with open(f"{f_name}--Gromet-FN-auto.json", "w") as f:
                gromet_collection_dict = (
                    pipeline_state.gromet_collection.to_dict()
                )
                f.write(
                    dictionary_to_gromet_json(
                        del_nulls(gromet_collection_dict), level=indent_level
                    )
                )
        else:
            return pipeline_state.gromet_collection
    else:
        print("\nCalling ToGrfnPass-------------------")
        ToGrfnPass(pipeline_state)
        grfn = pipeline_state.get_grfn()
        grfn.to_json_file(f"{f_name}--AC-GrFN.json")

        grfn_agraph = grfn.to_AGraph()
        grfn_agraph.draw(f"{f_name}--AC-GrFN.pdf", prog="dot")

        print("\nGenerating pickled AnnCast nodes-----------------")
        pickled_file_name = f"{f_name}--AnnCast.pickled"
        with open(pickled_file_name, "wb") as pkfile:
            dill.dump(pipeline_state, pkfile)
