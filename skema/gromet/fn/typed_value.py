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

class TypedValue(object):
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
        'type': 'str',
        'value': 'object',
        'index': 'int'
    }

    attribute_map = {
        'type': 'type',
        'value': 'value',
        'index': 'index'
    }

    def __init__(self, type=None, value=None, index=None):  # noqa: E501
        """TypedValue - a model defined in Swagger"""  # noqa: E501
        self._type = None
        self._value = None
        self._index = None
        self.discriminator = None
        if type is not None:
            self.type = type
        if value is not None:
            self.value = value
        if index is not None:
            self.index = index

    @property
    def type(self):
        """Gets the type of this TypedValue.  # noqa: E501

        The type of the value.  # noqa: E501

        :return: The type of this TypedValue.  # noqa: E501
        :rtype: str
        """
        return self._type

    @type.setter
    def type(self, type):
        """Sets the type of this TypedValue.

        The type of the value.  # noqa: E501

        :param type: The type of this TypedValue.  # noqa: E501
        :type: str
        """

        self._type = type

    @property
    def value(self):
        """Gets the value of this TypedValue.  # noqa: E501


        :return: The value of this TypedValue.  # noqa: E501
        :rtype: object
        """
        return self._value

    @value.setter
    def value(self, value):
        """Sets the value of this TypedValue.


        :param value: The value of this TypedValue.  # noqa: E501
        :type: object
        """

        self._value = value

    @property
    def index(self):
        """Gets the index of this TypedValue.  # noqa: E501

        Helper bookkeeping to facilitate keeping track of large list of TypedValues  # noqa: E501

        :return: The index of this TypedValue.  # noqa: E501
        :rtype: int
        """
        return self._index

    @index.setter
    def index(self, index):
        """Sets the index of this TypedValue.

        Helper bookkeeping to facilitate keeping track of large list of TypedValues  # noqa: E501

        :param index: The index of this TypedValue.  # noqa: E501
        :type: int
        """

        self._index = index

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
        if issubclass(TypedValue, dict):
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
        if not isinstance(other, TypedValue):
            return False

        return self.__dict__ == other.__dict__

    def __ne__(self, other):
        """Returns true if both objects are not equal"""
        return not self == other
