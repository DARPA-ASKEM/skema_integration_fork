# coding: utf-8

"""
    Grounded Model Exchange (GroMEt) schema for Function Networks

    This document defines the GroMEt Function Network data model. Note that Metadata is defined in separate spec.  __Using Swagger to Generate Class Structure__  To automatically generate Python or Java models corresponding to this document, you can use [swagger-codegen](https://swagger.io/tools/swagger-codegen/). This can be used to generate the client code based off of this spec, and in the process this will generate the data model class structure.  1. Install via the method described for your operating system    [here](https://github.com/swagger-api/swagger-codegen#Prerequisites).    Make sure to install a version after 3.0 that will support openapi 3. 2. Run swagger-codegen with the options in the example below.    The URL references where the yaml for this documentation is stored on    github. Make sure to replace CURRENT_VERSION with the correct version.    (The current version is `0.1.4`.)    To generate Java classes rather, change the `-l python` to `-l java`.    Change the value to the `-o` option to the desired output location.    ```    swagger-codegen generate -l python -o ./client -i https://raw.githubusercontent.com/ml4ai/skema-v2/master/docs/source/gromet_FN_v{CURRENT_VERSION}.yaml    ``` 3. Once it executes, the client code will be generated at your specified    location.    For python, the classes will be located in    `$OUTPUT_PATH/swagger_client/models/`.    For java, they will be located in    `$OUTPUT_PATH/src/main/java/io/swagger/client/model/`  If generating GroMEt schema data model classes in SKEMA (AutoMATES), then after generating the above, follow the instructions here: ``` <skema>/skema/model_assembly/gromet/model/README.md ```   # noqa: E501

    OpenAPI spec version: 0.1.5
    Contact: claytonm@arizona.edu
    Generated by: https://github.com/swagger-api/swagger-codegen.git
"""

import pprint
import re  # noqa: F401

import six

class ImportReference(object):
    """NOTE: This class is auto generated by the swagger code generator program.

    Do not edit the class manually.
    """
    """
    Attributes:
      swagger_types (dict): The key is attribute name
                            and the value is attribute type.
      attribute_map (dict): The key is attribute name
                            and the value is json key in definition.
    """
    swagger_types = {
        'name': 'str',
        'src_language': 'str',
        'type': 'ImportType',
        'version': 'str',
        'uri': 'TypedValue'
    }

    attribute_map = {
        'name': 'name',
        'src_language': 'src_language',
        'type': 'type',
        'version': 'version',
        'uri': 'uri'
    }

    def __init__(self, name=None, src_language=None, type=None, version=None, uri=None):  # noqa: E501
        """ImportReference - a model defined in Swagger"""  # noqa: E501
        self._name = None
        self._src_language = None
        self._type = None
        self._version = None
        self._uri = None
        self.discriminator = None
        if name is not None:
            self.name = name
        if src_language is not None:
            self.src_language = src_language
        if type is not None:
            self.type = type
        if version is not None:
            self.version = version
        if uri is not None:
            self.uri = uri

    @property
    def name(self):
        """Gets the name of this ImportReference.  # noqa: E501

        The qualified name of the import. The qualified name includes the module path to the element being imported. E.g., \"numpy.concatenate\" is the qualified name for importing the concatenate function from numpy. If the element being imported is from an existing GROMET_FN_MODULE, then the qualified names ends in a colon followed by the integer, where the integer references the index in the GrometFNModule.attributes for the corresponding FN.   # noqa: E501

        :return: The name of this ImportReference.  # noqa: E501
        :rtype: str
        """
        return self._name

    @name.setter
    def name(self, name):
        """Sets the name of this ImportReference.

        The qualified name of the import. The qualified name includes the module path to the element being imported. E.g., \"numpy.concatenate\" is the qualified name for importing the concatenate function from numpy. If the element being imported is from an existing GROMET_FN_MODULE, then the qualified names ends in a colon followed by the integer, where the integer references the index in the GrometFNModule.attributes for the corresponding FN.   # noqa: E501

        :param name: The name of this ImportReference.  # noqa: E501
        :type: str
        """

        self._name = name

    @property
    def src_language(self):
        """Gets the src_language of this ImportReference.  # noqa: E501

        The programming language of the imported module  # noqa: E501

        :return: The src_language of this ImportReference.  # noqa: E501
        :rtype: str
        """
        return self._src_language

    @src_language.setter
    def src_language(self, src_language):
        """Sets the src_language of this ImportReference.

        The programming language of the imported module  # noqa: E501

        :param src_language: The src_language of this ImportReference.  # noqa: E501
        :type: str
        """

        self._src_language = src_language

    @property
    def type(self):
        """Gets the type of this ImportReference.  # noqa: E501


        :return: The type of this ImportReference.  # noqa: E501
        :rtype: ImportType
        """
        return self._type

    @type.setter
    def type(self, type):
        """Sets the type of this ImportReference.


        :param type: The type of this ImportReference.  # noqa: E501
        :type: ImportType
        """

        self._type = type

    @property
    def version(self):
        """Gets the version of this ImportReference.  # noqa: E501

        The import source name and version (if available) of the element being imported. For example, if important a native library from Python 3.10, then this would be \"Python 3.10\".   # noqa: E501

        :return: The version of this ImportReference.  # noqa: E501
        :rtype: str
        """
        return self._version

    @version.setter
    def version(self, version):
        """Sets the version of this ImportReference.

        The import source name and version (if available) of the element being imported. For example, if important a native library from Python 3.10, then this would be \"Python 3.10\".   # noqa: E501

        :param version: The version of this ImportReference.  # noqa: E501
        :type: str
        """

        self._version = version

    @property
    def uri(self):
        """Gets the uri of this ImportReference.  # noqa: E501


        :return: The uri of this ImportReference.  # noqa: E501
        :rtype: TypedValue
        """
        return self._uri

    @uri.setter
    def uri(self, uri):
        """Sets the uri of this ImportReference.


        :param uri: The uri of this ImportReference.  # noqa: E501
        :type: TypedValue
        """

        self._uri = uri

    def to_dict(self):
        """Returns the model properties as a dict"""
        result = {}

        for attr, _ in six.iteritems(self.swagger_types):
            value = getattr(self, attr)
            if isinstance(value, list):
                result[attr] = list(map(
                    lambda x: x.to_dict() if hasattr(x, "to_dict") else x,
                    value
                ))
            elif hasattr(value, "to_dict"):
                result[attr] = value.to_dict()
            elif isinstance(value, dict):
                result[attr] = dict(map(
                    lambda item: (item[0], item[1].to_dict())
                    if hasattr(item[1], "to_dict") else item,
                    value.items()
                ))
            else:
                result[attr] = value
        if issubclass(ImportReference, dict):
            for key, value in self.items():
                result[key] = value

        return result

    def to_str(self):
        """Returns the string representation of the model"""
        return pprint.pformat(self.to_dict())

    def __repr__(self):
        """For `print` and `pprint`"""
        return self.to_str()

    def __eq__(self, other):
        """Returns true if both objects are equal"""
        if not isinstance(other, ImportReference):
            return False

        return self.__dict__ == other.__dict__

    def __ne__(self, other):
        """Returns true if both objects are not equal"""
        return not self == other
