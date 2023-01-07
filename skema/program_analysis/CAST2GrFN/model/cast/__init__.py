# coding: utf-8

# flake8: noqa
"""
    SKEMA Common Abstract Syntax Tree (CAST)

    This document outlines the structure of the CAST that will be used as a generic representation of the semantics of a program written in any language. This will be used when creating functions networks from programs using the SKEMA Program Analysis pipeline.   __Generating Class Structure__    To automatically generate Python or Java models corresponding to this document, you can use [swagger-codegen](https://swagger.io/tools/swagger-codegen/). We can use this to generate client code based off of this spec that will also generate the class structure.    1. Install via the method described for your operating system [here](https://github.com/swagger-api/swagger-codegen#Prerequisites). Make sure to install a version after 3.0 that will support openapi 3.  2. Run swagger-codegen with the options in the example below. The URL references where the yaml for this documentation is stored on github. Make sure to replace CURRENT_VERSION with the correct version. To generate Java classes rather, change the `-l python` to `-l java`. Change the value to the `-o` option to the desired output location.       ```      swagger-codegen generate -l python -o ./client -i https://raw.githubusercontent.com/ml4ai/skema-v2/master/docs/source/cast_v{CURRENT_VERSION}.yaml      ```  3. Once it executes, the client code will be generated at your specified location. For python, the classes will be located in `$OUTPUT_PATH/swagger_client/models/`. For java, they will be located in `$OUTPUT_PATH/src/main/java/io/swagger/client/model/`      # noqa: E501

    OpenAPI spec version: 1.2.2
    
    Generated by: https://github.com/swagger-api/swagger-codegen.git
"""

from __future__ import absolute_import

# import models into model package
from skema.program_analysis.CAST2GrFN.model.cast.assignment import Assignment
from skema.program_analysis.CAST2GrFN.model.cast.ast_node import AstNode
from skema.program_analysis.CAST2GrFN.model.cast.attribute import Attribute
from skema.program_analysis.CAST2GrFN.model.cast.binary_op import BinaryOp
from skema.program_analysis.CAST2GrFN.model.cast.binary_operator import BinaryOperator
from skema.program_analysis.CAST2GrFN.model.cast.boolean import Boolean
from skema.program_analysis.CAST2GrFN.model.cast.call import Call
from skema.program_analysis.CAST2GrFN.model.cast.dict import Dict
from skema.program_analysis.CAST2GrFN.model.cast.expr import Expr
from skema.program_analysis.CAST2GrFN.model.cast.function_def import FunctionDef
from skema.program_analysis.CAST2GrFN.model.cast.list import List
from skema.program_analysis.CAST2GrFN.model.cast.literal_value import LiteralValue
from skema.program_analysis.CAST2GrFN.model.cast.loop import Loop
from skema.program_analysis.CAST2GrFN.model.cast.model_break import ModelBreak
from skema.program_analysis.CAST2GrFN.model.cast.model_continue import ModelContinue
from skema.program_analysis.CAST2GrFN.model.cast.model_if import ModelIf
from skema.program_analysis.CAST2GrFN.model.cast.model_import import ModelImport
from skema.program_analysis.CAST2GrFN.model.cast.model_return import ModelReturn
from skema.program_analysis.CAST2GrFN.model.cast.module import Module
from skema.program_analysis.CAST2GrFN.model.cast.name import Name
from skema.program_analysis.CAST2GrFN.model.cast.number import Number
from skema.program_analysis.CAST2GrFN.model.cast.record_def import RecordDef
from skema.program_analysis.CAST2GrFN.model.cast.scalar_type import ScalarType
from skema.program_analysis.CAST2GrFN.model.cast.set import Set
from skema.program_analysis.CAST2GrFN.model.cast.source_code_data_type import SourceCodeDataType
from skema.program_analysis.CAST2GrFN.model.cast.source_ref import SourceRef
from skema.program_analysis.CAST2GrFN.model.cast.string import String
from skema.program_analysis.CAST2GrFN.model.cast.structure_type import StructureType
from skema.program_analysis.CAST2GrFN.model.cast.subscript import Subscript
from skema.program_analysis.CAST2GrFN.model.cast.tuple import Tuple
from skema.program_analysis.CAST2GrFN.model.cast.unary_op import UnaryOp
from skema.program_analysis.CAST2GrFN.model.cast.unary_operator import UnaryOperator
from skema.program_analysis.CAST2GrFN.model.cast.value_constructor import ValueConstructor
from skema.program_analysis.CAST2GrFN.model.cast.var import Var
from skema.program_analysis.CAST2GrFN.model.cast.var_type import VarType