//! Runtime domain exposes JavaScript runtime by means of remote evaluation and mirror objects.
//! Evaluation results are returned as mirror object that expose object type, string representation
//! and unique identifier that can be used for further object reference. Original objects are
//! maintained in memory unless they are either explicitly released or are released along with the
//! other objects in their object group.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// Unique script identifier.

pub type ScriptId<'a> = Cow<'a, str>;

/// Represents options for serialization. Overrides 'generatePreview' and 'returnByValue'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SerializationOptions<'a> {
    serialization: Cow<'a, str>,
    /// Deep serialization depth. Default is full depth. Respected only in 'deep' serialization mode.
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxDepth")]
    max_depth: Option<i64>,
    /// Embedder-specific parameters. For example if connected to V8 in Chrome these control DOM
    /// serialization via 'maxNodeDepth: integer' and 'includeShadowTree: "none" | "open" | "all"'.
    /// Values can be only of type string or integer.
    #[serde(skip_serializing_if = "Option::is_none", rename = "additionalParameters")]
    additional_parameters: Option<serde_json::Map<String, JsonValue>>,
}

impl<'a> SerializationOptions<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `serialization`: 
    pub fn builder(serialization: impl Into<Cow<'a, str>>) -> SerializationOptionsBuilder<'a> {
        SerializationOptionsBuilder {
            serialization: serialization.into(),
            max_depth: None,
            additional_parameters: None,
        }
    }
    pub fn serialization(&self) -> &str { self.serialization.as_ref() }
    /// Deep serialization depth. Default is full depth. Respected only in 'deep' serialization mode.
    pub fn max_depth(&self) -> Option<i64> { self.max_depth }
    /// Embedder-specific parameters. For example if connected to V8 in Chrome these control DOM
    /// serialization via 'maxNodeDepth: integer' and 'includeShadowTree: "none" | "open" | "all"'.
    /// Values can be only of type string or integer.
    pub fn additional_parameters(&self) -> Option<&serde_json::Map<String, JsonValue>> { self.additional_parameters.as_ref() }
}


pub struct SerializationOptionsBuilder<'a> {
    serialization: Cow<'a, str>,
    max_depth: Option<i64>,
    additional_parameters: Option<serde_json::Map<String, JsonValue>>,
}

impl<'a> SerializationOptionsBuilder<'a> {
    /// Deep serialization depth. Default is full depth. Respected only in 'deep' serialization mode.
    pub fn max_depth(mut self, max_depth: i64) -> Self { self.max_depth = Some(max_depth); self }
    /// Embedder-specific parameters. For example if connected to V8 in Chrome these control DOM
    /// serialization via 'maxNodeDepth: integer' and 'includeShadowTree: "none" | "open" | "all"'.
    /// Values can be only of type string or integer.
    pub fn additional_parameters(mut self, additional_parameters: serde_json::Map<String, JsonValue>) -> Self { self.additional_parameters = Some(additional_parameters); self }
    pub fn build(self) -> SerializationOptions<'a> {
        SerializationOptions {
            serialization: self.serialization,
            max_depth: self.max_depth,
            additional_parameters: self.additional_parameters,
        }
    }
}

/// Represents deep serialized value.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeepSerializedValue<'a> {
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    object_id: Option<Cow<'a, str>>,
    /// Set if value reference met more then once during serialization. In such
    /// case, value is provided only to one of the serialized values. Unique
    /// per value in the scope of one CDP call.
    #[serde(skip_serializing_if = "Option::is_none", rename = "weakLocalObjectReference")]
    weak_local_object_reference: Option<i64>,
}

impl<'a> DeepSerializedValue<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `type_`: 
    pub fn builder(type_: impl Into<Cow<'a, str>>) -> DeepSerializedValueBuilder<'a> {
        DeepSerializedValueBuilder {
            type_: type_.into(),
            value: None,
            object_id: None,
            weak_local_object_reference: None,
        }
    }
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    pub fn value(&self) -> Option<&JsonValue> { self.value.as_ref() }
    pub fn object_id(&self) -> Option<&str> { self.object_id.as_deref() }
    /// Set if value reference met more then once during serialization. In such
    /// case, value is provided only to one of the serialized values. Unique
    /// per value in the scope of one CDP call.
    pub fn weak_local_object_reference(&self) -> Option<i64> { self.weak_local_object_reference }
}


pub struct DeepSerializedValueBuilder<'a> {
    type_: Cow<'a, str>,
    value: Option<JsonValue>,
    object_id: Option<Cow<'a, str>>,
    weak_local_object_reference: Option<i64>,
}

impl<'a> DeepSerializedValueBuilder<'a> {
    pub fn value(mut self, value: JsonValue) -> Self { self.value = Some(value); self }
    pub fn object_id(mut self, object_id: impl Into<Cow<'a, str>>) -> Self { self.object_id = Some(object_id.into()); self }
    /// Set if value reference met more then once during serialization. In such
    /// case, value is provided only to one of the serialized values. Unique
    /// per value in the scope of one CDP call.
    pub fn weak_local_object_reference(mut self, weak_local_object_reference: i64) -> Self { self.weak_local_object_reference = Some(weak_local_object_reference); self }
    pub fn build(self) -> DeepSerializedValue<'a> {
        DeepSerializedValue {
            type_: self.type_,
            value: self.value,
            object_id: self.object_id,
            weak_local_object_reference: self.weak_local_object_reference,
        }
    }
}

/// Unique object identifier.

pub type RemoteObjectId<'a> = Cow<'a, str>;

/// Primitive value which cannot be JSON-stringified. Includes values '-0', 'NaN', 'Infinity',
/// '-Infinity', and bigint literals.

pub type UnserializableValue<'a> = Cow<'a, str>;

/// Mirror object referencing original JavaScript object.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoteObject<'a> {
    /// Object type.
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
    /// Object subtype hint. Specified for 'object' type values only.
    /// NOTE: If you change anything here, make sure to also update
    /// 'subtype' in 'ObjectPreview' and 'PropertyPreview' below.
    #[serde(skip_serializing_if = "Option::is_none")]
    subtype: Option<Cow<'a, str>>,
    /// Object class (constructor) name. Specified for 'object' type values only.
    #[serde(skip_serializing_if = "Option::is_none", rename = "className")]
    class_name: Option<Cow<'a, str>>,
    /// Remote object value in case of primitive values or JSON values (if it was requested).
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<JsonValue>,
    /// Primitive value which can not be JSON-stringified does not have 'value', but gets this
    /// property.
    #[serde(skip_serializing_if = "Option::is_none", rename = "unserializableValue")]
    unserializable_value: Option<UnserializableValue<'a>>,
    /// String representation of the object.
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<Cow<'a, str>>,
    /// Deep serialized value.
    #[serde(skip_serializing_if = "Option::is_none", rename = "deepSerializedValue")]
    deep_serialized_value: Option<DeepSerializedValue<'a>>,
    /// Unique object identifier (for non-primitive values).
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    object_id: Option<RemoteObjectId<'a>>,
    /// Preview containing abbreviated property values. Specified for 'object' type values only.
    #[serde(skip_serializing_if = "Option::is_none")]
    preview: Option<ObjectPreview<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "customPreview")]
    custom_preview: Option<CustomPreview<'a>>,
}

impl<'a> RemoteObject<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `type_`: Object type.
    pub fn builder(type_: impl Into<Cow<'a, str>>) -> RemoteObjectBuilder<'a> {
        RemoteObjectBuilder {
            type_: type_.into(),
            subtype: None,
            class_name: None,
            value: None,
            unserializable_value: None,
            description: None,
            deep_serialized_value: None,
            object_id: None,
            preview: None,
            custom_preview: None,
        }
    }
    /// Object type.
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    /// Object subtype hint. Specified for 'object' type values only.
    /// NOTE: If you change anything here, make sure to also update
    /// 'subtype' in 'ObjectPreview' and 'PropertyPreview' below.
    pub fn subtype(&self) -> Option<&str> { self.subtype.as_deref() }
    /// Object class (constructor) name. Specified for 'object' type values only.
    pub fn class_name(&self) -> Option<&str> { self.class_name.as_deref() }
    /// Remote object value in case of primitive values or JSON values (if it was requested).
    pub fn value(&self) -> Option<&JsonValue> { self.value.as_ref() }
    /// Primitive value which can not be JSON-stringified does not have 'value', but gets this
    /// property.
    pub fn unserializable_value(&self) -> Option<&UnserializableValue<'a>> { self.unserializable_value.as_ref() }
    /// String representation of the object.
    pub fn description(&self) -> Option<&str> { self.description.as_deref() }
    /// Deep serialized value.
    pub fn deep_serialized_value(&self) -> Option<&DeepSerializedValue<'a>> { self.deep_serialized_value.as_ref() }
    /// Unique object identifier (for non-primitive values).
    pub fn object_id(&self) -> Option<&RemoteObjectId<'a>> { self.object_id.as_ref() }
    /// Preview containing abbreviated property values. Specified for 'object' type values only.
    pub fn preview(&self) -> Option<&ObjectPreview<'a>> { self.preview.as_ref() }
    pub fn custom_preview(&self) -> Option<&CustomPreview<'a>> { self.custom_preview.as_ref() }
}


pub struct RemoteObjectBuilder<'a> {
    type_: Cow<'a, str>,
    subtype: Option<Cow<'a, str>>,
    class_name: Option<Cow<'a, str>>,
    value: Option<JsonValue>,
    unserializable_value: Option<UnserializableValue<'a>>,
    description: Option<Cow<'a, str>>,
    deep_serialized_value: Option<DeepSerializedValue<'a>>,
    object_id: Option<RemoteObjectId<'a>>,
    preview: Option<ObjectPreview<'a>>,
    custom_preview: Option<CustomPreview<'a>>,
}

impl<'a> RemoteObjectBuilder<'a> {
    /// Object subtype hint. Specified for 'object' type values only.
    /// NOTE: If you change anything here, make sure to also update
    /// 'subtype' in 'ObjectPreview' and 'PropertyPreview' below.
    pub fn subtype(mut self, subtype: impl Into<Cow<'a, str>>) -> Self { self.subtype = Some(subtype.into()); self }
    /// Object class (constructor) name. Specified for 'object' type values only.
    pub fn class_name(mut self, class_name: impl Into<Cow<'a, str>>) -> Self { self.class_name = Some(class_name.into()); self }
    /// Remote object value in case of primitive values or JSON values (if it was requested).
    pub fn value(mut self, value: JsonValue) -> Self { self.value = Some(value); self }
    /// Primitive value which can not be JSON-stringified does not have 'value', but gets this
    /// property.
    pub fn unserializable_value(mut self, unserializable_value: impl Into<UnserializableValue<'a>>) -> Self { self.unserializable_value = Some(unserializable_value.into()); self }
    /// String representation of the object.
    pub fn description(mut self, description: impl Into<Cow<'a, str>>) -> Self { self.description = Some(description.into()); self }
    /// Deep serialized value.
    pub fn deep_serialized_value(mut self, deep_serialized_value: DeepSerializedValue<'a>) -> Self { self.deep_serialized_value = Some(deep_serialized_value); self }
    /// Unique object identifier (for non-primitive values).
    pub fn object_id(mut self, object_id: impl Into<RemoteObjectId<'a>>) -> Self { self.object_id = Some(object_id.into()); self }
    /// Preview containing abbreviated property values. Specified for 'object' type values only.
    pub fn preview(mut self, preview: ObjectPreview<'a>) -> Self { self.preview = Some(preview); self }
    pub fn custom_preview(mut self, custom_preview: CustomPreview<'a>) -> Self { self.custom_preview = Some(custom_preview); self }
    pub fn build(self) -> RemoteObject<'a> {
        RemoteObject {
            type_: self.type_,
            subtype: self.subtype,
            class_name: self.class_name,
            value: self.value,
            unserializable_value: self.unserializable_value,
            description: self.description,
            deep_serialized_value: self.deep_serialized_value,
            object_id: self.object_id,
            preview: self.preview,
            custom_preview: self.custom_preview,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CustomPreview<'a> {
    /// The JSON-stringified result of formatter.header(object, config) call.
    /// It contains json ML array that represents RemoteObject.
    header: Cow<'a, str>,
    /// If formatter returns true as a result of formatter.hasBody call then bodyGetterId will
    /// contain RemoteObjectId for the function that returns result of formatter.body(object, config) call.
    /// The result value is json ML array.
    #[serde(skip_serializing_if = "Option::is_none", rename = "bodyGetterId")]
    body_getter_id: Option<RemoteObjectId<'a>>,
}

impl<'a> CustomPreview<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `header`: The JSON-stringified result of formatter.header(object, config) call. It contains json ML array that represents RemoteObject.
    pub fn builder(header: impl Into<Cow<'a, str>>) -> CustomPreviewBuilder<'a> {
        CustomPreviewBuilder {
            header: header.into(),
            body_getter_id: None,
        }
    }
    /// The JSON-stringified result of formatter.header(object, config) call.
    /// It contains json ML array that represents RemoteObject.
    pub fn header(&self) -> &str { self.header.as_ref() }
    /// If formatter returns true as a result of formatter.hasBody call then bodyGetterId will
    /// contain RemoteObjectId for the function that returns result of formatter.body(object, config) call.
    /// The result value is json ML array.
    pub fn body_getter_id(&self) -> Option<&RemoteObjectId<'a>> { self.body_getter_id.as_ref() }
}


pub struct CustomPreviewBuilder<'a> {
    header: Cow<'a, str>,
    body_getter_id: Option<RemoteObjectId<'a>>,
}

impl<'a> CustomPreviewBuilder<'a> {
    /// If formatter returns true as a result of formatter.hasBody call then bodyGetterId will
    /// contain RemoteObjectId for the function that returns result of formatter.body(object, config) call.
    /// The result value is json ML array.
    pub fn body_getter_id(mut self, body_getter_id: impl Into<RemoteObjectId<'a>>) -> Self { self.body_getter_id = Some(body_getter_id.into()); self }
    pub fn build(self) -> CustomPreview<'a> {
        CustomPreview {
            header: self.header,
            body_getter_id: self.body_getter_id,
        }
    }
}

/// Object containing abbreviated remote object value.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjectPreview<'a> {
    /// Object type.
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
    /// Object subtype hint. Specified for 'object' type values only.
    #[serde(skip_serializing_if = "Option::is_none")]
    subtype: Option<Cow<'a, str>>,
    /// String representation of the object.
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<Cow<'a, str>>,
    /// True iff some of the properties or entries of the original object did not fit.
    overflow: bool,
    /// List of the properties.
    properties: Vec<PropertyPreview<'a>>,
    /// List of the entries. Specified for 'map' and 'set' subtype values only.
    #[serde(skip_serializing_if = "Option::is_none")]
    entries: Option<Vec<EntryPreview<'a>>>,
}

impl<'a> ObjectPreview<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `type_`: Object type.
    /// * `overflow`: True iff some of the properties or entries of the original object did not fit.
    /// * `properties`: List of the properties.
    pub fn builder(type_: impl Into<Cow<'a, str>>, overflow: bool, properties: Vec<PropertyPreview<'a>>) -> ObjectPreviewBuilder<'a> {
        ObjectPreviewBuilder {
            type_: type_.into(),
            subtype: None,
            description: None,
            overflow: overflow,
            properties: properties,
            entries: None,
        }
    }
    /// Object type.
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    /// Object subtype hint. Specified for 'object' type values only.
    pub fn subtype(&self) -> Option<&str> { self.subtype.as_deref() }
    /// String representation of the object.
    pub fn description(&self) -> Option<&str> { self.description.as_deref() }
    /// True iff some of the properties or entries of the original object did not fit.
    pub fn overflow(&self) -> bool { self.overflow }
    /// List of the properties.
    pub fn properties(&self) -> &[PropertyPreview<'a>] { &self.properties }
    /// List of the entries. Specified for 'map' and 'set' subtype values only.
    pub fn entries(&self) -> Option<&[EntryPreview<'a>]> { self.entries.as_deref() }
}


pub struct ObjectPreviewBuilder<'a> {
    type_: Cow<'a, str>,
    subtype: Option<Cow<'a, str>>,
    description: Option<Cow<'a, str>>,
    overflow: bool,
    properties: Vec<PropertyPreview<'a>>,
    entries: Option<Vec<EntryPreview<'a>>>,
}

impl<'a> ObjectPreviewBuilder<'a> {
    /// Object subtype hint. Specified for 'object' type values only.
    pub fn subtype(mut self, subtype: impl Into<Cow<'a, str>>) -> Self { self.subtype = Some(subtype.into()); self }
    /// String representation of the object.
    pub fn description(mut self, description: impl Into<Cow<'a, str>>) -> Self { self.description = Some(description.into()); self }
    /// List of the entries. Specified for 'map' and 'set' subtype values only.
    pub fn entries(mut self, entries: Vec<EntryPreview<'a>>) -> Self { self.entries = Some(entries); self }
    pub fn build(self) -> ObjectPreview<'a> {
        ObjectPreview {
            type_: self.type_,
            subtype: self.subtype,
            description: self.description,
            overflow: self.overflow,
            properties: self.properties,
            entries: self.entries,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PropertyPreview<'a> {
    /// Property name.
    name: Cow<'a, str>,
    /// Object type. Accessor means that the property itself is an accessor property.
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
    /// User-friendly property value string.
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<Cow<'a, str>>,
    /// Nested value preview.
    #[serde(skip_serializing_if = "Option::is_none", rename = "valuePreview")]
    value_preview: Option<ObjectPreview<'a>>,
    /// Object subtype hint. Specified for 'object' type values only.
    #[serde(skip_serializing_if = "Option::is_none")]
    subtype: Option<Cow<'a, str>>,
}

impl<'a> PropertyPreview<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `name`: Property name.
    /// * `type_`: Object type. Accessor means that the property itself is an accessor property.
    pub fn builder(name: impl Into<Cow<'a, str>>, type_: impl Into<Cow<'a, str>>) -> PropertyPreviewBuilder<'a> {
        PropertyPreviewBuilder {
            name: name.into(),
            type_: type_.into(),
            value: None,
            value_preview: None,
            subtype: None,
        }
    }
    /// Property name.
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// Object type. Accessor means that the property itself is an accessor property.
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    /// User-friendly property value string.
    pub fn value(&self) -> Option<&str> { self.value.as_deref() }
    /// Nested value preview.
    pub fn value_preview(&self) -> Option<&ObjectPreview<'a>> { self.value_preview.as_ref() }
    /// Object subtype hint. Specified for 'object' type values only.
    pub fn subtype(&self) -> Option<&str> { self.subtype.as_deref() }
}


pub struct PropertyPreviewBuilder<'a> {
    name: Cow<'a, str>,
    type_: Cow<'a, str>,
    value: Option<Cow<'a, str>>,
    value_preview: Option<ObjectPreview<'a>>,
    subtype: Option<Cow<'a, str>>,
}

impl<'a> PropertyPreviewBuilder<'a> {
    /// User-friendly property value string.
    pub fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self { self.value = Some(value.into()); self }
    /// Nested value preview.
    pub fn value_preview(mut self, value_preview: ObjectPreview<'a>) -> Self { self.value_preview = Some(value_preview); self }
    /// Object subtype hint. Specified for 'object' type values only.
    pub fn subtype(mut self, subtype: impl Into<Cow<'a, str>>) -> Self { self.subtype = Some(subtype.into()); self }
    pub fn build(self) -> PropertyPreview<'a> {
        PropertyPreview {
            name: self.name,
            type_: self.type_,
            value: self.value,
            value_preview: self.value_preview,
            subtype: self.subtype,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EntryPreview<'a> {
    /// Preview of the key. Specified for map-like collection entries.
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<ObjectPreview<'a>>,
    /// Preview of the value.
    value: ObjectPreview<'a>,
}

impl<'a> EntryPreview<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `value`: Preview of the value.
    pub fn builder(value: ObjectPreview<'a>) -> EntryPreviewBuilder<'a> {
        EntryPreviewBuilder {
            key: None,
            value: value,
        }
    }
    /// Preview of the key. Specified for map-like collection entries.
    pub fn key(&self) -> Option<&ObjectPreview<'a>> { self.key.as_ref() }
    /// Preview of the value.
    pub fn value(&self) -> &ObjectPreview<'a> { &self.value }
}


pub struct EntryPreviewBuilder<'a> {
    key: Option<ObjectPreview<'a>>,
    value: ObjectPreview<'a>,
}

impl<'a> EntryPreviewBuilder<'a> {
    /// Preview of the key. Specified for map-like collection entries.
    pub fn key(mut self, key: ObjectPreview<'a>) -> Self { self.key = Some(key); self }
    pub fn build(self) -> EntryPreview<'a> {
        EntryPreview {
            key: self.key,
            value: self.value,
        }
    }
}

/// Object property descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PropertyDescriptor<'a> {
    /// Property name or symbol description.
    name: Cow<'a, str>,
    /// The value associated with the property.
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<RemoteObject<'a>>,
    /// True if the value associated with the property may be changed (data descriptors only).
    #[serde(skip_serializing_if = "Option::is_none")]
    writable: Option<bool>,
    /// A function which serves as a getter for the property, or 'undefined' if there is no getter
    /// (accessor descriptors only).
    #[serde(skip_serializing_if = "Option::is_none")]
    get: Option<RemoteObject<'a>>,
    /// A function which serves as a setter for the property, or 'undefined' if there is no setter
    /// (accessor descriptors only).
    #[serde(skip_serializing_if = "Option::is_none")]
    set: Option<RemoteObject<'a>>,
    /// True if the type of this property descriptor may be changed and if the property may be
    /// deleted from the corresponding object.
    configurable: bool,
    /// True if this property shows up during enumeration of the properties on the corresponding
    /// object.
    enumerable: bool,
    /// True if the result was thrown during the evaluation.
    #[serde(skip_serializing_if = "Option::is_none", rename = "wasThrown")]
    was_thrown: Option<bool>,
    /// True if the property is owned for the object.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isOwn")]
    is_own: Option<bool>,
    /// Property symbol object, if the property is of the 'symbol' type.
    #[serde(skip_serializing_if = "Option::is_none")]
    symbol: Option<RemoteObject<'a>>,
}

impl<'a> PropertyDescriptor<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `name`: Property name or symbol description.
    /// * `configurable`: True if the type of this property descriptor may be changed and if the property may be deleted from the corresponding object.
    /// * `enumerable`: True if this property shows up during enumeration of the properties on the corresponding object.
    pub fn builder(name: impl Into<Cow<'a, str>>, configurable: bool, enumerable: bool) -> PropertyDescriptorBuilder<'a> {
        PropertyDescriptorBuilder {
            name: name.into(),
            value: None,
            writable: None,
            get: None,
            set: None,
            configurable: configurable,
            enumerable: enumerable,
            was_thrown: None,
            is_own: None,
            symbol: None,
        }
    }
    /// Property name or symbol description.
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// The value associated with the property.
    pub fn value(&self) -> Option<&RemoteObject<'a>> { self.value.as_ref() }
    /// True if the value associated with the property may be changed (data descriptors only).
    pub fn writable(&self) -> Option<bool> { self.writable }
    /// A function which serves as a getter for the property, or 'undefined' if there is no getter
    /// (accessor descriptors only).
    pub fn get(&self) -> Option<&RemoteObject<'a>> { self.get.as_ref() }
    /// A function which serves as a setter for the property, or 'undefined' if there is no setter
    /// (accessor descriptors only).
    pub fn set(&self) -> Option<&RemoteObject<'a>> { self.set.as_ref() }
    /// True if the type of this property descriptor may be changed and if the property may be
    /// deleted from the corresponding object.
    pub fn configurable(&self) -> bool { self.configurable }
    /// True if this property shows up during enumeration of the properties on the corresponding
    /// object.
    pub fn enumerable(&self) -> bool { self.enumerable }
    /// True if the result was thrown during the evaluation.
    pub fn was_thrown(&self) -> Option<bool> { self.was_thrown }
    /// True if the property is owned for the object.
    pub fn is_own(&self) -> Option<bool> { self.is_own }
    /// Property symbol object, if the property is of the 'symbol' type.
    pub fn symbol(&self) -> Option<&RemoteObject<'a>> { self.symbol.as_ref() }
}


pub struct PropertyDescriptorBuilder<'a> {
    name: Cow<'a, str>,
    value: Option<RemoteObject<'a>>,
    writable: Option<bool>,
    get: Option<RemoteObject<'a>>,
    set: Option<RemoteObject<'a>>,
    configurable: bool,
    enumerable: bool,
    was_thrown: Option<bool>,
    is_own: Option<bool>,
    symbol: Option<RemoteObject<'a>>,
}

impl<'a> PropertyDescriptorBuilder<'a> {
    /// The value associated with the property.
    pub fn value(mut self, value: RemoteObject<'a>) -> Self { self.value = Some(value); self }
    /// True if the value associated with the property may be changed (data descriptors only).
    pub fn writable(mut self, writable: bool) -> Self { self.writable = Some(writable); self }
    /// A function which serves as a getter for the property, or 'undefined' if there is no getter
    /// (accessor descriptors only).
    pub fn get(mut self, get: RemoteObject<'a>) -> Self { self.get = Some(get); self }
    /// A function which serves as a setter for the property, or 'undefined' if there is no setter
    /// (accessor descriptors only).
    pub fn set(mut self, set: RemoteObject<'a>) -> Self { self.set = Some(set); self }
    /// True if the result was thrown during the evaluation.
    pub fn was_thrown(mut self, was_thrown: bool) -> Self { self.was_thrown = Some(was_thrown); self }
    /// True if the property is owned for the object.
    pub fn is_own(mut self, is_own: bool) -> Self { self.is_own = Some(is_own); self }
    /// Property symbol object, if the property is of the 'symbol' type.
    pub fn symbol(mut self, symbol: RemoteObject<'a>) -> Self { self.symbol = Some(symbol); self }
    pub fn build(self) -> PropertyDescriptor<'a> {
        PropertyDescriptor {
            name: self.name,
            value: self.value,
            writable: self.writable,
            get: self.get,
            set: self.set,
            configurable: self.configurable,
            enumerable: self.enumerable,
            was_thrown: self.was_thrown,
            is_own: self.is_own,
            symbol: self.symbol,
        }
    }
}

/// Object internal property descriptor. This property isn't normally visible in JavaScript code.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InternalPropertyDescriptor<'a> {
    /// Conventional property name.
    name: Cow<'a, str>,
    /// The value associated with the property.
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<RemoteObject<'a>>,
}

impl<'a> InternalPropertyDescriptor<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `name`: Conventional property name.
    pub fn builder(name: impl Into<Cow<'a, str>>) -> InternalPropertyDescriptorBuilder<'a> {
        InternalPropertyDescriptorBuilder {
            name: name.into(),
            value: None,
        }
    }
    /// Conventional property name.
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// The value associated with the property.
    pub fn value(&self) -> Option<&RemoteObject<'a>> { self.value.as_ref() }
}


pub struct InternalPropertyDescriptorBuilder<'a> {
    name: Cow<'a, str>,
    value: Option<RemoteObject<'a>>,
}

impl<'a> InternalPropertyDescriptorBuilder<'a> {
    /// The value associated with the property.
    pub fn value(mut self, value: RemoteObject<'a>) -> Self { self.value = Some(value); self }
    pub fn build(self) -> InternalPropertyDescriptor<'a> {
        InternalPropertyDescriptor {
            name: self.name,
            value: self.value,
        }
    }
}

/// Object private field descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PrivatePropertyDescriptor<'a> {
    /// Private property name.
    name: Cow<'a, str>,
    /// The value associated with the private property.
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<RemoteObject<'a>>,
    /// A function which serves as a getter for the private property,
    /// or 'undefined' if there is no getter (accessor descriptors only).
    #[serde(skip_serializing_if = "Option::is_none")]
    get: Option<RemoteObject<'a>>,
    /// A function which serves as a setter for the private property,
    /// or 'undefined' if there is no setter (accessor descriptors only).
    #[serde(skip_serializing_if = "Option::is_none")]
    set: Option<RemoteObject<'a>>,
}

impl<'a> PrivatePropertyDescriptor<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `name`: Private property name.
    pub fn builder(name: impl Into<Cow<'a, str>>) -> PrivatePropertyDescriptorBuilder<'a> {
        PrivatePropertyDescriptorBuilder {
            name: name.into(),
            value: None,
            get: None,
            set: None,
        }
    }
    /// Private property name.
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// The value associated with the private property.
    pub fn value(&self) -> Option<&RemoteObject<'a>> { self.value.as_ref() }
    /// A function which serves as a getter for the private property,
    /// or 'undefined' if there is no getter (accessor descriptors only).
    pub fn get(&self) -> Option<&RemoteObject<'a>> { self.get.as_ref() }
    /// A function which serves as a setter for the private property,
    /// or 'undefined' if there is no setter (accessor descriptors only).
    pub fn set(&self) -> Option<&RemoteObject<'a>> { self.set.as_ref() }
}


pub struct PrivatePropertyDescriptorBuilder<'a> {
    name: Cow<'a, str>,
    value: Option<RemoteObject<'a>>,
    get: Option<RemoteObject<'a>>,
    set: Option<RemoteObject<'a>>,
}

impl<'a> PrivatePropertyDescriptorBuilder<'a> {
    /// The value associated with the private property.
    pub fn value(mut self, value: RemoteObject<'a>) -> Self { self.value = Some(value); self }
    /// A function which serves as a getter for the private property,
    /// or 'undefined' if there is no getter (accessor descriptors only).
    pub fn get(mut self, get: RemoteObject<'a>) -> Self { self.get = Some(get); self }
    /// A function which serves as a setter for the private property,
    /// or 'undefined' if there is no setter (accessor descriptors only).
    pub fn set(mut self, set: RemoteObject<'a>) -> Self { self.set = Some(set); self }
    pub fn build(self) -> PrivatePropertyDescriptor<'a> {
        PrivatePropertyDescriptor {
            name: self.name,
            value: self.value,
            get: self.get,
            set: self.set,
        }
    }
}

/// Represents function call argument. Either remote object id 'objectId', primitive 'value',
/// unserializable primitive value or neither of (for undefined) them should be specified.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CallArgument<'a> {
    /// Primitive value or serializable javascript object.
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<JsonValue>,
    /// Primitive value which can not be JSON-stringified.
    #[serde(skip_serializing_if = "Option::is_none", rename = "unserializableValue")]
    unserializable_value: Option<UnserializableValue<'a>>,
    /// Remote object handle.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    object_id: Option<RemoteObjectId<'a>>,
}

impl<'a> CallArgument<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> CallArgumentBuilder<'a> {
        CallArgumentBuilder {
            value: None,
            unserializable_value: None,
            object_id: None,
        }
    }
    /// Primitive value or serializable javascript object.
    pub fn value(&self) -> Option<&JsonValue> { self.value.as_ref() }
    /// Primitive value which can not be JSON-stringified.
    pub fn unserializable_value(&self) -> Option<&UnserializableValue<'a>> { self.unserializable_value.as_ref() }
    /// Remote object handle.
    pub fn object_id(&self) -> Option<&RemoteObjectId<'a>> { self.object_id.as_ref() }
}

#[derive(Default)]
pub struct CallArgumentBuilder<'a> {
    value: Option<JsonValue>,
    unserializable_value: Option<UnserializableValue<'a>>,
    object_id: Option<RemoteObjectId<'a>>,
}

impl<'a> CallArgumentBuilder<'a> {
    /// Primitive value or serializable javascript object.
    pub fn value(mut self, value: JsonValue) -> Self { self.value = Some(value); self }
    /// Primitive value which can not be JSON-stringified.
    pub fn unserializable_value(mut self, unserializable_value: impl Into<UnserializableValue<'a>>) -> Self { self.unserializable_value = Some(unserializable_value.into()); self }
    /// Remote object handle.
    pub fn object_id(mut self, object_id: impl Into<RemoteObjectId<'a>>) -> Self { self.object_id = Some(object_id.into()); self }
    pub fn build(self) -> CallArgument<'a> {
        CallArgument {
            value: self.value,
            unserializable_value: self.unserializable_value,
            object_id: self.object_id,
        }
    }
}

/// Id of an execution context.

pub type ExecutionContextId = i64;

/// Description of an isolated world.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionContextDescription<'a> {
    /// Unique id of the execution context. It can be used to specify in which execution context
    /// script evaluation should be performed.
    id: ExecutionContextId,
    /// Execution context origin.
    origin: Cow<'a, str>,
    /// Human readable name describing given context.
    name: Cow<'a, str>,
    /// A system-unique execution context identifier. Unlike the id, this is unique across
    /// multiple processes, so can be reliably used to identify specific context while backend
    /// performs a cross-process navigation.
    #[serde(rename = "uniqueId")]
    unique_id: Cow<'a, str>,
    /// Embedder-specific auxiliary data likely matching {isDefault: boolean, type: 'default'|'isolated'|'worker', frameId: string}
    #[serde(skip_serializing_if = "Option::is_none", rename = "auxData")]
    aux_data: Option<serde_json::Map<String, JsonValue>>,
}

impl<'a> ExecutionContextDescription<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `id`: Unique id of the execution context. It can be used to specify in which execution context script evaluation should be performed.
    /// * `origin`: Execution context origin.
    /// * `name`: Human readable name describing given context.
    /// * `unique_id`: A system-unique execution context identifier. Unlike the id, this is unique across multiple processes, so can be reliably used to identify specific context while backend performs a cross-process navigation.
    pub fn builder(id: ExecutionContextId, origin: impl Into<Cow<'a, str>>, name: impl Into<Cow<'a, str>>, unique_id: impl Into<Cow<'a, str>>) -> ExecutionContextDescriptionBuilder<'a> {
        ExecutionContextDescriptionBuilder {
            id: id,
            origin: origin.into(),
            name: name.into(),
            unique_id: unique_id.into(),
            aux_data: None,
        }
    }
    /// Unique id of the execution context. It can be used to specify in which execution context
    /// script evaluation should be performed.
    pub fn id(&self) -> &ExecutionContextId { &self.id }
    /// Execution context origin.
    pub fn origin(&self) -> &str { self.origin.as_ref() }
    /// Human readable name describing given context.
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// A system-unique execution context identifier. Unlike the id, this is unique across
    /// multiple processes, so can be reliably used to identify specific context while backend
    /// performs a cross-process navigation.
    pub fn unique_id(&self) -> &str { self.unique_id.as_ref() }
    /// Embedder-specific auxiliary data likely matching {isDefault: boolean, type: 'default'|'isolated'|'worker', frameId: string}
    pub fn aux_data(&self) -> Option<&serde_json::Map<String, JsonValue>> { self.aux_data.as_ref() }
}


pub struct ExecutionContextDescriptionBuilder<'a> {
    id: ExecutionContextId,
    origin: Cow<'a, str>,
    name: Cow<'a, str>,
    unique_id: Cow<'a, str>,
    aux_data: Option<serde_json::Map<String, JsonValue>>,
}

impl<'a> ExecutionContextDescriptionBuilder<'a> {
    /// Embedder-specific auxiliary data likely matching {isDefault: boolean, type: 'default'|'isolated'|'worker', frameId: string}
    pub fn aux_data(mut self, aux_data: serde_json::Map<String, JsonValue>) -> Self { self.aux_data = Some(aux_data); self }
    pub fn build(self) -> ExecutionContextDescription<'a> {
        ExecutionContextDescription {
            id: self.id,
            origin: self.origin,
            name: self.name,
            unique_id: self.unique_id,
            aux_data: self.aux_data,
        }
    }
}

/// Detailed information about exception (or error) that was thrown during script compilation or
/// execution.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionDetails<'a> {
    /// Exception id.
    #[serde(rename = "exceptionId")]
    exception_id: u64,
    /// Exception text, which should be used together with exception object when available.
    text: Cow<'a, str>,
    /// Line number of the exception location (0-based).
    #[serde(rename = "lineNumber")]
    line_number: i64,
    /// Column number of the exception location (0-based).
    #[serde(rename = "columnNumber")]
    column_number: i64,
    /// Script ID of the exception location.
    #[serde(skip_serializing_if = "Option::is_none", rename = "scriptId")]
    script_id: Option<ScriptId<'a>>,
    /// URL of the exception location, to be used when the script was not reported.
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<Cow<'a, str>>,
    /// JavaScript stack trace if available.
    #[serde(skip_serializing_if = "Option::is_none", rename = "stackTrace")]
    stack_trace: Option<StackTrace<'a>>,
    /// Exception object if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    exception: Option<RemoteObject<'a>>,
    /// Identifier of the context where exception happened.
    #[serde(skip_serializing_if = "Option::is_none", rename = "executionContextId")]
    execution_context_id: Option<ExecutionContextId>,
    /// Dictionary with entries of meta data that the client associated
    /// with this exception, such as information about associated network
    /// requests, etc.
    #[serde(skip_serializing_if = "Option::is_none", rename = "exceptionMetaData")]
    exception_meta_data: Option<serde_json::Map<String, JsonValue>>,
}

impl<'a> ExceptionDetails<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `exception_id`: Exception id.
    /// * `text`: Exception text, which should be used together with exception object when available.
    /// * `line_number`: Line number of the exception location (0-based).
    /// * `column_number`: Column number of the exception location (0-based).
    pub fn builder(exception_id: u64, text: impl Into<Cow<'a, str>>, line_number: i64, column_number: i64) -> ExceptionDetailsBuilder<'a> {
        ExceptionDetailsBuilder {
            exception_id: exception_id,
            text: text.into(),
            line_number: line_number,
            column_number: column_number,
            script_id: None,
            url: None,
            stack_trace: None,
            exception: None,
            execution_context_id: None,
            exception_meta_data: None,
        }
    }
    /// Exception id.
    pub fn exception_id(&self) -> u64 { self.exception_id }
    /// Exception text, which should be used together with exception object when available.
    pub fn text(&self) -> &str { self.text.as_ref() }
    /// Line number of the exception location (0-based).
    pub fn line_number(&self) -> i64 { self.line_number }
    /// Column number of the exception location (0-based).
    pub fn column_number(&self) -> i64 { self.column_number }
    /// Script ID of the exception location.
    pub fn script_id(&self) -> Option<&ScriptId<'a>> { self.script_id.as_ref() }
    /// URL of the exception location, to be used when the script was not reported.
    pub fn url(&self) -> Option<&str> { self.url.as_deref() }
    /// JavaScript stack trace if available.
    pub fn stack_trace(&self) -> Option<&StackTrace<'a>> { self.stack_trace.as_ref() }
    /// Exception object if available.
    pub fn exception(&self) -> Option<&RemoteObject<'a>> { self.exception.as_ref() }
    /// Identifier of the context where exception happened.
    pub fn execution_context_id(&self) -> Option<&ExecutionContextId> { self.execution_context_id.as_ref() }
    /// Dictionary with entries of meta data that the client associated
    /// with this exception, such as information about associated network
    /// requests, etc.
    pub fn exception_meta_data(&self) -> Option<&serde_json::Map<String, JsonValue>> { self.exception_meta_data.as_ref() }
}


pub struct ExceptionDetailsBuilder<'a> {
    exception_id: u64,
    text: Cow<'a, str>,
    line_number: i64,
    column_number: i64,
    script_id: Option<ScriptId<'a>>,
    url: Option<Cow<'a, str>>,
    stack_trace: Option<StackTrace<'a>>,
    exception: Option<RemoteObject<'a>>,
    execution_context_id: Option<ExecutionContextId>,
    exception_meta_data: Option<serde_json::Map<String, JsonValue>>,
}

impl<'a> ExceptionDetailsBuilder<'a> {
    /// Script ID of the exception location.
    pub fn script_id(mut self, script_id: impl Into<ScriptId<'a>>) -> Self { self.script_id = Some(script_id.into()); self }
    /// URL of the exception location, to be used when the script was not reported.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// JavaScript stack trace if available.
    pub fn stack_trace(mut self, stack_trace: StackTrace<'a>) -> Self { self.stack_trace = Some(stack_trace); self }
    /// Exception object if available.
    pub fn exception(mut self, exception: RemoteObject<'a>) -> Self { self.exception = Some(exception); self }
    /// Identifier of the context where exception happened.
    pub fn execution_context_id(mut self, execution_context_id: ExecutionContextId) -> Self { self.execution_context_id = Some(execution_context_id); self }
    /// Dictionary with entries of meta data that the client associated
    /// with this exception, such as information about associated network
    /// requests, etc.
    pub fn exception_meta_data(mut self, exception_meta_data: serde_json::Map<String, JsonValue>) -> Self { self.exception_meta_data = Some(exception_meta_data); self }
    pub fn build(self) -> ExceptionDetails<'a> {
        ExceptionDetails {
            exception_id: self.exception_id,
            text: self.text,
            line_number: self.line_number,
            column_number: self.column_number,
            script_id: self.script_id,
            url: self.url,
            stack_trace: self.stack_trace,
            exception: self.exception,
            execution_context_id: self.execution_context_id,
            exception_meta_data: self.exception_meta_data,
        }
    }
}

/// Number of milliseconds since epoch.

pub type Timestamp = f64;

/// Number of milliseconds.

pub type TimeDelta = f64;

/// Stack entry for runtime errors and assertions.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CallFrame<'a> {
    /// JavaScript function name.
    #[serde(rename = "functionName")]
    function_name: Cow<'a, str>,
    /// JavaScript script id.
    #[serde(rename = "scriptId")]
    script_id: ScriptId<'a>,
    /// JavaScript script name or url.
    url: Cow<'a, str>,
    /// JavaScript script line number (0-based).
    #[serde(rename = "lineNumber")]
    line_number: i64,
    /// JavaScript script column number (0-based).
    #[serde(rename = "columnNumber")]
    column_number: i64,
}

impl<'a> CallFrame<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `function_name`: JavaScript function name.
    /// * `script_id`: JavaScript script id.
    /// * `url`: JavaScript script name or url.
    /// * `line_number`: JavaScript script line number (0-based).
    /// * `column_number`: JavaScript script column number (0-based).
    pub fn builder(function_name: impl Into<Cow<'a, str>>, script_id: impl Into<ScriptId<'a>>, url: impl Into<Cow<'a, str>>, line_number: i64, column_number: i64) -> CallFrameBuilder<'a> {
        CallFrameBuilder {
            function_name: function_name.into(),
            script_id: script_id.into(),
            url: url.into(),
            line_number: line_number,
            column_number: column_number,
        }
    }
    /// JavaScript function name.
    pub fn function_name(&self) -> &str { self.function_name.as_ref() }
    /// JavaScript script id.
    pub fn script_id(&self) -> &ScriptId<'a> { &self.script_id }
    /// JavaScript script name or url.
    pub fn url(&self) -> &str { self.url.as_ref() }
    /// JavaScript script line number (0-based).
    pub fn line_number(&self) -> i64 { self.line_number }
    /// JavaScript script column number (0-based).
    pub fn column_number(&self) -> i64 { self.column_number }
}


pub struct CallFrameBuilder<'a> {
    function_name: Cow<'a, str>,
    script_id: ScriptId<'a>,
    url: Cow<'a, str>,
    line_number: i64,
    column_number: i64,
}

impl<'a> CallFrameBuilder<'a> {
    pub fn build(self) -> CallFrame<'a> {
        CallFrame {
            function_name: self.function_name,
            script_id: self.script_id,
            url: self.url,
            line_number: self.line_number,
            column_number: self.column_number,
        }
    }
}

/// Call frames for assertions or error messages.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StackTrace<'a> {
    /// String label of this stack trace. For async traces this may be a name of the function that
    /// initiated the async call.
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<Cow<'a, str>>,
    /// JavaScript function name.
    #[serde(rename = "callFrames")]
    call_frames: Vec<CallFrame<'a>>,
    /// Asynchronous JavaScript stack trace that preceded this stack, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    parent: Option<Box<StackTrace<'a>>>,
    /// Asynchronous JavaScript stack trace that preceded this stack, if available.
    #[serde(skip_serializing_if = "Option::is_none", rename = "parentId")]
    parent_id: Option<StackTraceId<'a>>,
}

impl<'a> StackTrace<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `call_frames`: JavaScript function name.
    pub fn builder(call_frames: Vec<CallFrame<'a>>) -> StackTraceBuilder<'a> {
        StackTraceBuilder {
            description: None,
            call_frames: call_frames,
            parent: None,
            parent_id: None,
        }
    }
    /// String label of this stack trace. For async traces this may be a name of the function that
    /// initiated the async call.
    pub fn description(&self) -> Option<&str> { self.description.as_deref() }
    /// JavaScript function name.
    pub fn call_frames(&self) -> &[CallFrame<'a>] { &self.call_frames }
    /// Asynchronous JavaScript stack trace that preceded this stack, if available.
    pub fn parent(&self) -> Option<&StackTrace<'a>> { self.parent.as_deref() }
    /// Asynchronous JavaScript stack trace that preceded this stack, if available.
    pub fn parent_id(&self) -> Option<&StackTraceId<'a>> { self.parent_id.as_ref() }
}


pub struct StackTraceBuilder<'a> {
    description: Option<Cow<'a, str>>,
    call_frames: Vec<CallFrame<'a>>,
    parent: Option<Box<StackTrace<'a>>>,
    parent_id: Option<StackTraceId<'a>>,
}

impl<'a> StackTraceBuilder<'a> {
    /// String label of this stack trace. For async traces this may be a name of the function that
    /// initiated the async call.
    pub fn description(mut self, description: impl Into<Cow<'a, str>>) -> Self { self.description = Some(description.into()); self }
    /// Asynchronous JavaScript stack trace that preceded this stack, if available.
    pub fn parent(mut self, parent: Box<StackTrace<'a>>) -> Self { self.parent = Some(parent); self }
    /// Asynchronous JavaScript stack trace that preceded this stack, if available.
    pub fn parent_id(mut self, parent_id: StackTraceId<'a>) -> Self { self.parent_id = Some(parent_id); self }
    pub fn build(self) -> StackTrace<'a> {
        StackTrace {
            description: self.description,
            call_frames: self.call_frames,
            parent: self.parent,
            parent_id: self.parent_id,
        }
    }
}

/// Unique identifier of current debugger.

pub type UniqueDebuggerId<'a> = Cow<'a, str>;

/// If 'debuggerId' is set stack trace comes from another debugger and can be resolved there. This
/// allows to track cross-debugger calls. See 'Runtime.StackTrace' and 'Debugger.paused' for usages.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StackTraceId<'a> {
    id: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "debuggerId")]
    debugger_id: Option<UniqueDebuggerId<'a>>,
}

impl<'a> StackTraceId<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `id`: 
    pub fn builder(id: impl Into<Cow<'a, str>>) -> StackTraceIdBuilder<'a> {
        StackTraceIdBuilder {
            id: id.into(),
            debugger_id: None,
        }
    }
    pub fn id(&self) -> &str { self.id.as_ref() }
    pub fn debugger_id(&self) -> Option<&UniqueDebuggerId<'a>> { self.debugger_id.as_ref() }
}


pub struct StackTraceIdBuilder<'a> {
    id: Cow<'a, str>,
    debugger_id: Option<UniqueDebuggerId<'a>>,
}

impl<'a> StackTraceIdBuilder<'a> {
    pub fn debugger_id(mut self, debugger_id: impl Into<UniqueDebuggerId<'a>>) -> Self { self.debugger_id = Some(debugger_id.into()); self }
    pub fn build(self) -> StackTraceId<'a> {
        StackTraceId {
            id: self.id,
            debugger_id: self.debugger_id,
        }
    }
}

/// Add handler to promise with given promise object id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AwaitPromiseParams<'a> {
    /// Identifier of the promise.
    #[serde(rename = "promiseObjectId")]
    promise_object_id: RemoteObjectId<'a>,
    /// Whether the result is expected to be a JSON object that should be sent by value.
    #[serde(skip_serializing_if = "Option::is_none", rename = "returnByValue")]
    return_by_value: Option<bool>,
    /// Whether preview should be generated for the result.
    #[serde(skip_serializing_if = "Option::is_none", rename = "generatePreview")]
    generate_preview: Option<bool>,
}

impl<'a> AwaitPromiseParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `promise_object_id`: Identifier of the promise.
    pub fn builder(promise_object_id: impl Into<RemoteObjectId<'a>>) -> AwaitPromiseParamsBuilder<'a> {
        AwaitPromiseParamsBuilder {
            promise_object_id: promise_object_id.into(),
            return_by_value: None,
            generate_preview: None,
        }
    }
    /// Identifier of the promise.
    pub fn promise_object_id(&self) -> &RemoteObjectId<'a> { &self.promise_object_id }
    /// Whether the result is expected to be a JSON object that should be sent by value.
    pub fn return_by_value(&self) -> Option<bool> { self.return_by_value }
    /// Whether preview should be generated for the result.
    pub fn generate_preview(&self) -> Option<bool> { self.generate_preview }
}


pub struct AwaitPromiseParamsBuilder<'a> {
    promise_object_id: RemoteObjectId<'a>,
    return_by_value: Option<bool>,
    generate_preview: Option<bool>,
}

impl<'a> AwaitPromiseParamsBuilder<'a> {
    /// Whether the result is expected to be a JSON object that should be sent by value.
    pub fn return_by_value(mut self, return_by_value: bool) -> Self { self.return_by_value = Some(return_by_value); self }
    /// Whether preview should be generated for the result.
    pub fn generate_preview(mut self, generate_preview: bool) -> Self { self.generate_preview = Some(generate_preview); self }
    pub fn build(self) -> AwaitPromiseParams<'a> {
        AwaitPromiseParams {
            promise_object_id: self.promise_object_id,
            return_by_value: self.return_by_value,
            generate_preview: self.generate_preview,
        }
    }
}

/// Add handler to promise with given promise object id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AwaitPromiseReturns<'a> {
    /// Promise result. Will contain rejected value if promise was rejected.
    result: RemoteObject<'a>,
    /// Exception details if stack strace is available.
    #[serde(skip_serializing_if = "Option::is_none", rename = "exceptionDetails")]
    exception_details: Option<ExceptionDetails<'a>>,
}

impl<'a> AwaitPromiseReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `result`: Promise result. Will contain rejected value if promise was rejected.
    pub fn builder(result: RemoteObject<'a>) -> AwaitPromiseReturnsBuilder<'a> {
        AwaitPromiseReturnsBuilder {
            result: result,
            exception_details: None,
        }
    }
    /// Promise result. Will contain rejected value if promise was rejected.
    pub fn result(&self) -> &RemoteObject<'a> { &self.result }
    /// Exception details if stack strace is available.
    pub fn exception_details(&self) -> Option<&ExceptionDetails<'a>> { self.exception_details.as_ref() }
}


pub struct AwaitPromiseReturnsBuilder<'a> {
    result: RemoteObject<'a>,
    exception_details: Option<ExceptionDetails<'a>>,
}

impl<'a> AwaitPromiseReturnsBuilder<'a> {
    /// Exception details if stack strace is available.
    pub fn exception_details(mut self, exception_details: ExceptionDetails<'a>) -> Self { self.exception_details = Some(exception_details); self }
    pub fn build(self) -> AwaitPromiseReturns<'a> {
        AwaitPromiseReturns {
            result: self.result,
            exception_details: self.exception_details,
        }
    }
}

impl<'a> AwaitPromiseParams<'a> { pub const METHOD: &'static str = "Runtime.awaitPromise"; }

impl<'a> crate::CdpCommand<'a> for AwaitPromiseParams<'a> {
    const METHOD: &'static str = "Runtime.awaitPromise";
    type Response = AwaitPromiseReturns<'a>;
}

/// Calls function with given declaration on the given object. Object group of the result is
/// inherited from the target object.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CallFunctionOnParams<'a> {
    /// Declaration of the function to call.
    #[serde(rename = "functionDeclaration")]
    function_declaration: Cow<'a, str>,
    /// Identifier of the object to call function on. Either objectId or executionContextId should
    /// be specified.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    object_id: Option<RemoteObjectId<'a>>,
    /// Call arguments. All call arguments must belong to the same JavaScript world as the target
    /// object.
    #[serde(skip_serializing_if = "Option::is_none")]
    arguments: Option<Vec<CallArgument<'a>>>,
    /// In silent mode exceptions thrown during evaluation are not reported and do not pause
    /// execution. Overrides 'setPauseOnException' state.
    #[serde(skip_serializing_if = "Option::is_none")]
    silent: Option<bool>,
    /// Whether the result is expected to be a JSON object which should be sent by value.
    /// Can be overriden by 'serializationOptions'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "returnByValue")]
    return_by_value: Option<bool>,
    /// Whether preview should be generated for the result.
    #[serde(skip_serializing_if = "Option::is_none", rename = "generatePreview")]
    generate_preview: Option<bool>,
    /// Whether execution should be treated as initiated by user in the UI.
    #[serde(skip_serializing_if = "Option::is_none", rename = "userGesture")]
    user_gesture: Option<bool>,
    /// Whether execution should 'await' for resulting value and return once awaited promise is
    /// resolved.
    #[serde(skip_serializing_if = "Option::is_none", rename = "awaitPromise")]
    await_promise: Option<bool>,
    /// Specifies execution context which global object will be used to call function on. Either
    /// executionContextId or objectId should be specified.
    #[serde(skip_serializing_if = "Option::is_none", rename = "executionContextId")]
    execution_context_id: Option<ExecutionContextId>,
    /// Symbolic group name that can be used to release multiple objects. If objectGroup is not
    /// specified and objectId is, objectGroup will be inherited from object.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectGroup")]
    object_group: Option<Cow<'a, str>>,
    /// Whether to throw an exception if side effect cannot be ruled out during evaluation.
    #[serde(skip_serializing_if = "Option::is_none", rename = "throwOnSideEffect")]
    throw_on_side_effect: Option<bool>,
    /// An alternative way to specify the execution context to call function on.
    /// Compared to contextId that may be reused across processes, this is guaranteed to be
    /// system-unique, so it can be used to prevent accidental function call
    /// in context different than intended (e.g. as a result of navigation across process
    /// boundaries).
    /// This is mutually exclusive with 'executionContextId'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "uniqueContextId")]
    unique_context_id: Option<Cow<'a, str>>,
    /// Specifies the result serialization. If provided, overrides
    /// 'generatePreview' and 'returnByValue'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "serializationOptions")]
    serialization_options: Option<SerializationOptions<'a>>,
}

impl<'a> CallFunctionOnParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `function_declaration`: Declaration of the function to call.
    pub fn builder(function_declaration: impl Into<Cow<'a, str>>) -> CallFunctionOnParamsBuilder<'a> {
        CallFunctionOnParamsBuilder {
            function_declaration: function_declaration.into(),
            object_id: None,
            arguments: None,
            silent: None,
            return_by_value: None,
            generate_preview: None,
            user_gesture: None,
            await_promise: None,
            execution_context_id: None,
            object_group: None,
            throw_on_side_effect: None,
            unique_context_id: None,
            serialization_options: None,
        }
    }
    /// Declaration of the function to call.
    pub fn function_declaration(&self) -> &str { self.function_declaration.as_ref() }
    /// Identifier of the object to call function on. Either objectId or executionContextId should
    /// be specified.
    pub fn object_id(&self) -> Option<&RemoteObjectId<'a>> { self.object_id.as_ref() }
    /// Call arguments. All call arguments must belong to the same JavaScript world as the target
    /// object.
    pub fn arguments(&self) -> Option<&[CallArgument<'a>]> { self.arguments.as_deref() }
    /// In silent mode exceptions thrown during evaluation are not reported and do not pause
    /// execution. Overrides 'setPauseOnException' state.
    pub fn silent(&self) -> Option<bool> { self.silent }
    /// Whether the result is expected to be a JSON object which should be sent by value.
    /// Can be overriden by 'serializationOptions'.
    pub fn return_by_value(&self) -> Option<bool> { self.return_by_value }
    /// Whether preview should be generated for the result.
    pub fn generate_preview(&self) -> Option<bool> { self.generate_preview }
    /// Whether execution should be treated as initiated by user in the UI.
    pub fn user_gesture(&self) -> Option<bool> { self.user_gesture }
    /// Whether execution should 'await' for resulting value and return once awaited promise is
    /// resolved.
    pub fn await_promise(&self) -> Option<bool> { self.await_promise }
    /// Specifies execution context which global object will be used to call function on. Either
    /// executionContextId or objectId should be specified.
    pub fn execution_context_id(&self) -> Option<&ExecutionContextId> { self.execution_context_id.as_ref() }
    /// Symbolic group name that can be used to release multiple objects. If objectGroup is not
    /// specified and objectId is, objectGroup will be inherited from object.
    pub fn object_group(&self) -> Option<&str> { self.object_group.as_deref() }
    /// Whether to throw an exception if side effect cannot be ruled out during evaluation.
    pub fn throw_on_side_effect(&self) -> Option<bool> { self.throw_on_side_effect }
    /// An alternative way to specify the execution context to call function on.
    /// Compared to contextId that may be reused across processes, this is guaranteed to be
    /// system-unique, so it can be used to prevent accidental function call
    /// in context different than intended (e.g. as a result of navigation across process
    /// boundaries).
    /// This is mutually exclusive with 'executionContextId'.
    pub fn unique_context_id(&self) -> Option<&str> { self.unique_context_id.as_deref() }
    /// Specifies the result serialization. If provided, overrides
    /// 'generatePreview' and 'returnByValue'.
    pub fn serialization_options(&self) -> Option<&SerializationOptions<'a>> { self.serialization_options.as_ref() }
}


pub struct CallFunctionOnParamsBuilder<'a> {
    function_declaration: Cow<'a, str>,
    object_id: Option<RemoteObjectId<'a>>,
    arguments: Option<Vec<CallArgument<'a>>>,
    silent: Option<bool>,
    return_by_value: Option<bool>,
    generate_preview: Option<bool>,
    user_gesture: Option<bool>,
    await_promise: Option<bool>,
    execution_context_id: Option<ExecutionContextId>,
    object_group: Option<Cow<'a, str>>,
    throw_on_side_effect: Option<bool>,
    unique_context_id: Option<Cow<'a, str>>,
    serialization_options: Option<SerializationOptions<'a>>,
}

impl<'a> CallFunctionOnParamsBuilder<'a> {
    /// Identifier of the object to call function on. Either objectId or executionContextId should
    /// be specified.
    pub fn object_id(mut self, object_id: impl Into<RemoteObjectId<'a>>) -> Self { self.object_id = Some(object_id.into()); self }
    /// Call arguments. All call arguments must belong to the same JavaScript world as the target
    /// object.
    pub fn arguments(mut self, arguments: Vec<CallArgument<'a>>) -> Self { self.arguments = Some(arguments); self }
    /// In silent mode exceptions thrown during evaluation are not reported and do not pause
    /// execution. Overrides 'setPauseOnException' state.
    pub fn silent(mut self, silent: bool) -> Self { self.silent = Some(silent); self }
    /// Whether the result is expected to be a JSON object which should be sent by value.
    /// Can be overriden by 'serializationOptions'.
    pub fn return_by_value(mut self, return_by_value: bool) -> Self { self.return_by_value = Some(return_by_value); self }
    /// Whether preview should be generated for the result.
    pub fn generate_preview(mut self, generate_preview: bool) -> Self { self.generate_preview = Some(generate_preview); self }
    /// Whether execution should be treated as initiated by user in the UI.
    pub fn user_gesture(mut self, user_gesture: bool) -> Self { self.user_gesture = Some(user_gesture); self }
    /// Whether execution should 'await' for resulting value and return once awaited promise is
    /// resolved.
    pub fn await_promise(mut self, await_promise: bool) -> Self { self.await_promise = Some(await_promise); self }
    /// Specifies execution context which global object will be used to call function on. Either
    /// executionContextId or objectId should be specified.
    pub fn execution_context_id(mut self, execution_context_id: ExecutionContextId) -> Self { self.execution_context_id = Some(execution_context_id); self }
    /// Symbolic group name that can be used to release multiple objects. If objectGroup is not
    /// specified and objectId is, objectGroup will be inherited from object.
    pub fn object_group(mut self, object_group: impl Into<Cow<'a, str>>) -> Self { self.object_group = Some(object_group.into()); self }
    /// Whether to throw an exception if side effect cannot be ruled out during evaluation.
    pub fn throw_on_side_effect(mut self, throw_on_side_effect: bool) -> Self { self.throw_on_side_effect = Some(throw_on_side_effect); self }
    /// An alternative way to specify the execution context to call function on.
    /// Compared to contextId that may be reused across processes, this is guaranteed to be
    /// system-unique, so it can be used to prevent accidental function call
    /// in context different than intended (e.g. as a result of navigation across process
    /// boundaries).
    /// This is mutually exclusive with 'executionContextId'.
    pub fn unique_context_id(mut self, unique_context_id: impl Into<Cow<'a, str>>) -> Self { self.unique_context_id = Some(unique_context_id.into()); self }
    /// Specifies the result serialization. If provided, overrides
    /// 'generatePreview' and 'returnByValue'.
    pub fn serialization_options(mut self, serialization_options: SerializationOptions<'a>) -> Self { self.serialization_options = Some(serialization_options); self }
    pub fn build(self) -> CallFunctionOnParams<'a> {
        CallFunctionOnParams {
            function_declaration: self.function_declaration,
            object_id: self.object_id,
            arguments: self.arguments,
            silent: self.silent,
            return_by_value: self.return_by_value,
            generate_preview: self.generate_preview,
            user_gesture: self.user_gesture,
            await_promise: self.await_promise,
            execution_context_id: self.execution_context_id,
            object_group: self.object_group,
            throw_on_side_effect: self.throw_on_side_effect,
            unique_context_id: self.unique_context_id,
            serialization_options: self.serialization_options,
        }
    }
}

/// Calls function with given declaration on the given object. Object group of the result is
/// inherited from the target object.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CallFunctionOnReturns<'a> {
    /// Call result.
    result: RemoteObject<'a>,
    /// Exception details.
    #[serde(skip_serializing_if = "Option::is_none", rename = "exceptionDetails")]
    exception_details: Option<ExceptionDetails<'a>>,
}

impl<'a> CallFunctionOnReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `result`: Call result.
    pub fn builder(result: RemoteObject<'a>) -> CallFunctionOnReturnsBuilder<'a> {
        CallFunctionOnReturnsBuilder {
            result: result,
            exception_details: None,
        }
    }
    /// Call result.
    pub fn result(&self) -> &RemoteObject<'a> { &self.result }
    /// Exception details.
    pub fn exception_details(&self) -> Option<&ExceptionDetails<'a>> { self.exception_details.as_ref() }
}


pub struct CallFunctionOnReturnsBuilder<'a> {
    result: RemoteObject<'a>,
    exception_details: Option<ExceptionDetails<'a>>,
}

impl<'a> CallFunctionOnReturnsBuilder<'a> {
    /// Exception details.
    pub fn exception_details(mut self, exception_details: ExceptionDetails<'a>) -> Self { self.exception_details = Some(exception_details); self }
    pub fn build(self) -> CallFunctionOnReturns<'a> {
        CallFunctionOnReturns {
            result: self.result,
            exception_details: self.exception_details,
        }
    }
}

impl<'a> CallFunctionOnParams<'a> { pub const METHOD: &'static str = "Runtime.callFunctionOn"; }

impl<'a> crate::CdpCommand<'a> for CallFunctionOnParams<'a> {
    const METHOD: &'static str = "Runtime.callFunctionOn";
    type Response = CallFunctionOnReturns<'a>;
}

/// Compiles expression.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CompileScriptParams<'a> {
    /// Expression to compile.
    expression: Cow<'a, str>,
    /// Source url to be set for the script.
    #[serde(rename = "sourceURL")]
    source_url: Cow<'a, str>,
    /// Specifies whether the compiled script should be persisted.
    #[serde(rename = "persistScript")]
    persist_script: bool,
    /// Specifies in which execution context to perform script run. If the parameter is omitted the
    /// evaluation will be performed in the context of the inspected page.
    #[serde(skip_serializing_if = "Option::is_none", rename = "executionContextId")]
    execution_context_id: Option<ExecutionContextId>,
}

impl<'a> CompileScriptParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `expression`: Expression to compile.
    /// * `source_url`: Source url to be set for the script.
    /// * `persist_script`: Specifies whether the compiled script should be persisted.
    pub fn builder(expression: impl Into<Cow<'a, str>>, source_url: impl Into<Cow<'a, str>>, persist_script: bool) -> CompileScriptParamsBuilder<'a> {
        CompileScriptParamsBuilder {
            expression: expression.into(),
            source_url: source_url.into(),
            persist_script: persist_script,
            execution_context_id: None,
        }
    }
    /// Expression to compile.
    pub fn expression(&self) -> &str { self.expression.as_ref() }
    /// Source url to be set for the script.
    pub fn source_url(&self) -> &str { self.source_url.as_ref() }
    /// Specifies whether the compiled script should be persisted.
    pub fn persist_script(&self) -> bool { self.persist_script }
    /// Specifies in which execution context to perform script run. If the parameter is omitted the
    /// evaluation will be performed in the context of the inspected page.
    pub fn execution_context_id(&self) -> Option<&ExecutionContextId> { self.execution_context_id.as_ref() }
}


pub struct CompileScriptParamsBuilder<'a> {
    expression: Cow<'a, str>,
    source_url: Cow<'a, str>,
    persist_script: bool,
    execution_context_id: Option<ExecutionContextId>,
}

impl<'a> CompileScriptParamsBuilder<'a> {
    /// Specifies in which execution context to perform script run. If the parameter is omitted the
    /// evaluation will be performed in the context of the inspected page.
    pub fn execution_context_id(mut self, execution_context_id: ExecutionContextId) -> Self { self.execution_context_id = Some(execution_context_id); self }
    pub fn build(self) -> CompileScriptParams<'a> {
        CompileScriptParams {
            expression: self.expression,
            source_url: self.source_url,
            persist_script: self.persist_script,
            execution_context_id: self.execution_context_id,
        }
    }
}

/// Compiles expression.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CompileScriptReturns<'a> {
    /// Id of the script.
    #[serde(skip_serializing_if = "Option::is_none", rename = "scriptId")]
    script_id: Option<ScriptId<'a>>,
    /// Exception details.
    #[serde(skip_serializing_if = "Option::is_none", rename = "exceptionDetails")]
    exception_details: Option<ExceptionDetails<'a>>,
}

impl<'a> CompileScriptReturns<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> CompileScriptReturnsBuilder<'a> {
        CompileScriptReturnsBuilder {
            script_id: None,
            exception_details: None,
        }
    }
    /// Id of the script.
    pub fn script_id(&self) -> Option<&ScriptId<'a>> { self.script_id.as_ref() }
    /// Exception details.
    pub fn exception_details(&self) -> Option<&ExceptionDetails<'a>> { self.exception_details.as_ref() }
}

#[derive(Default)]
pub struct CompileScriptReturnsBuilder<'a> {
    script_id: Option<ScriptId<'a>>,
    exception_details: Option<ExceptionDetails<'a>>,
}

impl<'a> CompileScriptReturnsBuilder<'a> {
    /// Id of the script.
    pub fn script_id(mut self, script_id: impl Into<ScriptId<'a>>) -> Self { self.script_id = Some(script_id.into()); self }
    /// Exception details.
    pub fn exception_details(mut self, exception_details: ExceptionDetails<'a>) -> Self { self.exception_details = Some(exception_details); self }
    pub fn build(self) -> CompileScriptReturns<'a> {
        CompileScriptReturns {
            script_id: self.script_id,
            exception_details: self.exception_details,
        }
    }
}

impl<'a> CompileScriptParams<'a> { pub const METHOD: &'static str = "Runtime.compileScript"; }

impl<'a> crate::CdpCommand<'a> for CompileScriptParams<'a> {
    const METHOD: &'static str = "Runtime.compileScript";
    type Response = CompileScriptReturns<'a>;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "Runtime.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "Runtime.disable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiscardConsoleEntriesParams {}

impl DiscardConsoleEntriesParams { pub const METHOD: &'static str = "Runtime.discardConsoleEntries"; }

impl<'a> crate::CdpCommand<'a> for DiscardConsoleEntriesParams {
    const METHOD: &'static str = "Runtime.discardConsoleEntries";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams { pub const METHOD: &'static str = "Runtime.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "Runtime.enable";
    type Response = crate::EmptyReturns;
}

/// Evaluates expression on global object.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateParams<'a> {
    /// Expression to evaluate.
    expression: Cow<'a, str>,
    /// Symbolic group name that can be used to release multiple objects.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectGroup")]
    object_group: Option<Cow<'a, str>>,
    /// Determines whether Command Line API should be available during the evaluation.
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeCommandLineAPI")]
    include_command_line_api: Option<bool>,
    /// In silent mode exceptions thrown during evaluation are not reported and do not pause
    /// execution. Overrides 'setPauseOnException' state.
    #[serde(skip_serializing_if = "Option::is_none")]
    silent: Option<bool>,
    /// Specifies in which execution context to perform evaluation. If the parameter is omitted the
    /// evaluation will be performed in the context of the inspected page.
    /// This is mutually exclusive with 'uniqueContextId', which offers an
    /// alternative way to identify the execution context that is more reliable
    /// in a multi-process environment.
    #[serde(skip_serializing_if = "Option::is_none", rename = "contextId")]
    context_id: Option<ExecutionContextId>,
    /// Whether the result is expected to be a JSON object that should be sent by value.
    #[serde(skip_serializing_if = "Option::is_none", rename = "returnByValue")]
    return_by_value: Option<bool>,
    /// Whether preview should be generated for the result.
    #[serde(skip_serializing_if = "Option::is_none", rename = "generatePreview")]
    generate_preview: Option<bool>,
    /// Whether execution should be treated as initiated by user in the UI.
    #[serde(skip_serializing_if = "Option::is_none", rename = "userGesture")]
    user_gesture: Option<bool>,
    /// Whether execution should 'await' for resulting value and return once awaited promise is
    /// resolved.
    #[serde(skip_serializing_if = "Option::is_none", rename = "awaitPromise")]
    await_promise: Option<bool>,
    /// Whether to throw an exception if side effect cannot be ruled out during evaluation.
    /// This implies 'disableBreaks' below.
    #[serde(skip_serializing_if = "Option::is_none", rename = "throwOnSideEffect")]
    throw_on_side_effect: Option<bool>,
    /// Terminate execution after timing out (number of milliseconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<TimeDelta>,
    /// Disable breakpoints during execution.
    #[serde(skip_serializing_if = "Option::is_none", rename = "disableBreaks")]
    disable_breaks: Option<bool>,
    /// Setting this flag to true enables 'let' re-declaration and top-level 'await'.
    /// Note that 'let' variables can only be re-declared if they originate from
    /// 'replMode' themselves.
    #[serde(skip_serializing_if = "Option::is_none", rename = "replMode")]
    repl_mode: Option<bool>,
    /// The Content Security Policy (CSP) for the target might block 'unsafe-eval'
    /// which includes eval(), Function(), setTimeout() and setInterval()
    /// when called with non-callable arguments. This flag bypasses CSP for this
    /// evaluation and allows unsafe-eval. Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none", rename = "allowUnsafeEvalBlockedByCSP")]
    allow_unsafe_eval_blocked_by_csp: Option<bool>,
    /// An alternative way to specify the execution context to evaluate in.
    /// Compared to contextId that may be reused across processes, this is guaranteed to be
    /// system-unique, so it can be used to prevent accidental evaluation of the expression
    /// in context different than intended (e.g. as a result of navigation across process
    /// boundaries).
    /// This is mutually exclusive with 'contextId'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "uniqueContextId")]
    unique_context_id: Option<Cow<'a, str>>,
    /// Specifies the result serialization. If provided, overrides
    /// 'generatePreview' and 'returnByValue'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "serializationOptions")]
    serialization_options: Option<SerializationOptions<'a>>,
}

impl<'a> EvaluateParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `expression`: Expression to evaluate.
    pub fn builder(expression: impl Into<Cow<'a, str>>) -> EvaluateParamsBuilder<'a> {
        EvaluateParamsBuilder {
            expression: expression.into(),
            object_group: None,
            include_command_line_api: None,
            silent: None,
            context_id: None,
            return_by_value: None,
            generate_preview: None,
            user_gesture: None,
            await_promise: None,
            throw_on_side_effect: None,
            timeout: None,
            disable_breaks: None,
            repl_mode: None,
            allow_unsafe_eval_blocked_by_csp: None,
            unique_context_id: None,
            serialization_options: None,
        }
    }
    /// Expression to evaluate.
    pub fn expression(&self) -> &str { self.expression.as_ref() }
    /// Symbolic group name that can be used to release multiple objects.
    pub fn object_group(&self) -> Option<&str> { self.object_group.as_deref() }
    /// Determines whether Command Line API should be available during the evaluation.
    pub fn include_command_line_api(&self) -> Option<bool> { self.include_command_line_api }
    /// In silent mode exceptions thrown during evaluation are not reported and do not pause
    /// execution. Overrides 'setPauseOnException' state.
    pub fn silent(&self) -> Option<bool> { self.silent }
    /// Specifies in which execution context to perform evaluation. If the parameter is omitted the
    /// evaluation will be performed in the context of the inspected page.
    /// This is mutually exclusive with 'uniqueContextId', which offers an
    /// alternative way to identify the execution context that is more reliable
    /// in a multi-process environment.
    pub fn context_id(&self) -> Option<&ExecutionContextId> { self.context_id.as_ref() }
    /// Whether the result is expected to be a JSON object that should be sent by value.
    pub fn return_by_value(&self) -> Option<bool> { self.return_by_value }
    /// Whether preview should be generated for the result.
    pub fn generate_preview(&self) -> Option<bool> { self.generate_preview }
    /// Whether execution should be treated as initiated by user in the UI.
    pub fn user_gesture(&self) -> Option<bool> { self.user_gesture }
    /// Whether execution should 'await' for resulting value and return once awaited promise is
    /// resolved.
    pub fn await_promise(&self) -> Option<bool> { self.await_promise }
    /// Whether to throw an exception if side effect cannot be ruled out during evaluation.
    /// This implies 'disableBreaks' below.
    pub fn throw_on_side_effect(&self) -> Option<bool> { self.throw_on_side_effect }
    /// Terminate execution after timing out (number of milliseconds).
    pub fn timeout(&self) -> Option<&TimeDelta> { self.timeout.as_ref() }
    /// Disable breakpoints during execution.
    pub fn disable_breaks(&self) -> Option<bool> { self.disable_breaks }
    /// Setting this flag to true enables 'let' re-declaration and top-level 'await'.
    /// Note that 'let' variables can only be re-declared if they originate from
    /// 'replMode' themselves.
    pub fn repl_mode(&self) -> Option<bool> { self.repl_mode }
    /// The Content Security Policy (CSP) for the target might block 'unsafe-eval'
    /// which includes eval(), Function(), setTimeout() and setInterval()
    /// when called with non-callable arguments. This flag bypasses CSP for this
    /// evaluation and allows unsafe-eval. Defaults to true.
    pub fn allow_unsafe_eval_blocked_by_csp(&self) -> Option<bool> { self.allow_unsafe_eval_blocked_by_csp }
    /// An alternative way to specify the execution context to evaluate in.
    /// Compared to contextId that may be reused across processes, this is guaranteed to be
    /// system-unique, so it can be used to prevent accidental evaluation of the expression
    /// in context different than intended (e.g. as a result of navigation across process
    /// boundaries).
    /// This is mutually exclusive with 'contextId'.
    pub fn unique_context_id(&self) -> Option<&str> { self.unique_context_id.as_deref() }
    /// Specifies the result serialization. If provided, overrides
    /// 'generatePreview' and 'returnByValue'.
    pub fn serialization_options(&self) -> Option<&SerializationOptions<'a>> { self.serialization_options.as_ref() }
}


pub struct EvaluateParamsBuilder<'a> {
    expression: Cow<'a, str>,
    object_group: Option<Cow<'a, str>>,
    include_command_line_api: Option<bool>,
    silent: Option<bool>,
    context_id: Option<ExecutionContextId>,
    return_by_value: Option<bool>,
    generate_preview: Option<bool>,
    user_gesture: Option<bool>,
    await_promise: Option<bool>,
    throw_on_side_effect: Option<bool>,
    timeout: Option<TimeDelta>,
    disable_breaks: Option<bool>,
    repl_mode: Option<bool>,
    allow_unsafe_eval_blocked_by_csp: Option<bool>,
    unique_context_id: Option<Cow<'a, str>>,
    serialization_options: Option<SerializationOptions<'a>>,
}

impl<'a> EvaluateParamsBuilder<'a> {
    /// Symbolic group name that can be used to release multiple objects.
    pub fn object_group(mut self, object_group: impl Into<Cow<'a, str>>) -> Self { self.object_group = Some(object_group.into()); self }
    /// Determines whether Command Line API should be available during the evaluation.
    pub fn include_command_line_api(mut self, include_command_line_api: bool) -> Self { self.include_command_line_api = Some(include_command_line_api); self }
    /// In silent mode exceptions thrown during evaluation are not reported and do not pause
    /// execution. Overrides 'setPauseOnException' state.
    pub fn silent(mut self, silent: bool) -> Self { self.silent = Some(silent); self }
    /// Specifies in which execution context to perform evaluation. If the parameter is omitted the
    /// evaluation will be performed in the context of the inspected page.
    /// This is mutually exclusive with 'uniqueContextId', which offers an
    /// alternative way to identify the execution context that is more reliable
    /// in a multi-process environment.
    pub fn context_id(mut self, context_id: ExecutionContextId) -> Self { self.context_id = Some(context_id); self }
    /// Whether the result is expected to be a JSON object that should be sent by value.
    pub fn return_by_value(mut self, return_by_value: bool) -> Self { self.return_by_value = Some(return_by_value); self }
    /// Whether preview should be generated for the result.
    pub fn generate_preview(mut self, generate_preview: bool) -> Self { self.generate_preview = Some(generate_preview); self }
    /// Whether execution should be treated as initiated by user in the UI.
    pub fn user_gesture(mut self, user_gesture: bool) -> Self { self.user_gesture = Some(user_gesture); self }
    /// Whether execution should 'await' for resulting value and return once awaited promise is
    /// resolved.
    pub fn await_promise(mut self, await_promise: bool) -> Self { self.await_promise = Some(await_promise); self }
    /// Whether to throw an exception if side effect cannot be ruled out during evaluation.
    /// This implies 'disableBreaks' below.
    pub fn throw_on_side_effect(mut self, throw_on_side_effect: bool) -> Self { self.throw_on_side_effect = Some(throw_on_side_effect); self }
    /// Terminate execution after timing out (number of milliseconds).
    pub fn timeout(mut self, timeout: TimeDelta) -> Self { self.timeout = Some(timeout); self }
    /// Disable breakpoints during execution.
    pub fn disable_breaks(mut self, disable_breaks: bool) -> Self { self.disable_breaks = Some(disable_breaks); self }
    /// Setting this flag to true enables 'let' re-declaration and top-level 'await'.
    /// Note that 'let' variables can only be re-declared if they originate from
    /// 'replMode' themselves.
    pub fn repl_mode(mut self, repl_mode: bool) -> Self { self.repl_mode = Some(repl_mode); self }
    /// The Content Security Policy (CSP) for the target might block 'unsafe-eval'
    /// which includes eval(), Function(), setTimeout() and setInterval()
    /// when called with non-callable arguments. This flag bypasses CSP for this
    /// evaluation and allows unsafe-eval. Defaults to true.
    pub fn allow_unsafe_eval_blocked_by_csp(mut self, allow_unsafe_eval_blocked_by_csp: bool) -> Self { self.allow_unsafe_eval_blocked_by_csp = Some(allow_unsafe_eval_blocked_by_csp); self }
    /// An alternative way to specify the execution context to evaluate in.
    /// Compared to contextId that may be reused across processes, this is guaranteed to be
    /// system-unique, so it can be used to prevent accidental evaluation of the expression
    /// in context different than intended (e.g. as a result of navigation across process
    /// boundaries).
    /// This is mutually exclusive with 'contextId'.
    pub fn unique_context_id(mut self, unique_context_id: impl Into<Cow<'a, str>>) -> Self { self.unique_context_id = Some(unique_context_id.into()); self }
    /// Specifies the result serialization. If provided, overrides
    /// 'generatePreview' and 'returnByValue'.
    pub fn serialization_options(mut self, serialization_options: SerializationOptions<'a>) -> Self { self.serialization_options = Some(serialization_options); self }
    pub fn build(self) -> EvaluateParams<'a> {
        EvaluateParams {
            expression: self.expression,
            object_group: self.object_group,
            include_command_line_api: self.include_command_line_api,
            silent: self.silent,
            context_id: self.context_id,
            return_by_value: self.return_by_value,
            generate_preview: self.generate_preview,
            user_gesture: self.user_gesture,
            await_promise: self.await_promise,
            throw_on_side_effect: self.throw_on_side_effect,
            timeout: self.timeout,
            disable_breaks: self.disable_breaks,
            repl_mode: self.repl_mode,
            allow_unsafe_eval_blocked_by_csp: self.allow_unsafe_eval_blocked_by_csp,
            unique_context_id: self.unique_context_id,
            serialization_options: self.serialization_options,
        }
    }
}

/// Evaluates expression on global object.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateReturns<'a> {
    /// Evaluation result.
    result: RemoteObject<'a>,
    /// Exception details.
    #[serde(skip_serializing_if = "Option::is_none", rename = "exceptionDetails")]
    exception_details: Option<ExceptionDetails<'a>>,
}

impl<'a> EvaluateReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `result`: Evaluation result.
    pub fn builder(result: RemoteObject<'a>) -> EvaluateReturnsBuilder<'a> {
        EvaluateReturnsBuilder {
            result: result,
            exception_details: None,
        }
    }
    /// Evaluation result.
    pub fn result(&self) -> &RemoteObject<'a> { &self.result }
    /// Exception details.
    pub fn exception_details(&self) -> Option<&ExceptionDetails<'a>> { self.exception_details.as_ref() }
}


pub struct EvaluateReturnsBuilder<'a> {
    result: RemoteObject<'a>,
    exception_details: Option<ExceptionDetails<'a>>,
}

impl<'a> EvaluateReturnsBuilder<'a> {
    /// Exception details.
    pub fn exception_details(mut self, exception_details: ExceptionDetails<'a>) -> Self { self.exception_details = Some(exception_details); self }
    pub fn build(self) -> EvaluateReturns<'a> {
        EvaluateReturns {
            result: self.result,
            exception_details: self.exception_details,
        }
    }
}

impl<'a> EvaluateParams<'a> { pub const METHOD: &'static str = "Runtime.evaluate"; }

impl<'a> crate::CdpCommand<'a> for EvaluateParams<'a> {
    const METHOD: &'static str = "Runtime.evaluate";
    type Response = EvaluateReturns<'a>;
}

/// Returns the isolate id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetIsolateIdReturns<'a> {
    /// The isolate id.
    id: Cow<'a, str>,
}

impl<'a> GetIsolateIdReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `id`: The isolate id.
    pub fn builder(id: impl Into<Cow<'a, str>>) -> GetIsolateIdReturnsBuilder<'a> {
        GetIsolateIdReturnsBuilder {
            id: id.into(),
        }
    }
    /// The isolate id.
    pub fn id(&self) -> &str { self.id.as_ref() }
}


pub struct GetIsolateIdReturnsBuilder<'a> {
    id: Cow<'a, str>,
}

impl<'a> GetIsolateIdReturnsBuilder<'a> {
    pub fn build(self) -> GetIsolateIdReturns<'a> {
        GetIsolateIdReturns {
            id: self.id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetIsolateIdParams {}

impl GetIsolateIdParams { pub const METHOD: &'static str = "Runtime.getIsolateId"; }

impl<'a> crate::CdpCommand<'a> for GetIsolateIdParams {
    const METHOD: &'static str = "Runtime.getIsolateId";
    type Response = GetIsolateIdReturns<'a>;
}

/// Returns the JavaScript heap usage.
/// It is the total usage of the corresponding isolate not scoped to a particular Runtime.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetHeapUsageReturns {
    /// Used JavaScript heap size in bytes.
    #[serde(rename = "usedSize")]
    used_size: f64,
    /// Allocated JavaScript heap size in bytes.
    #[serde(rename = "totalSize")]
    total_size: f64,
    /// Used size in bytes in the embedder's garbage-collected heap.
    #[serde(rename = "embedderHeapUsedSize")]
    embedder_heap_used_size: f64,
    /// Size in bytes of backing storage for array buffers and external strings.
    #[serde(rename = "backingStorageSize")]
    backing_storage_size: f64,
}

impl GetHeapUsageReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `used_size`: Used JavaScript heap size in bytes.
    /// * `total_size`: Allocated JavaScript heap size in bytes.
    /// * `embedder_heap_used_size`: Used size in bytes in the embedder's garbage-collected heap.
    /// * `backing_storage_size`: Size in bytes of backing storage for array buffers and external strings.
    pub fn builder(used_size: f64, total_size: f64, embedder_heap_used_size: f64, backing_storage_size: f64) -> GetHeapUsageReturnsBuilder {
        GetHeapUsageReturnsBuilder {
            used_size: used_size,
            total_size: total_size,
            embedder_heap_used_size: embedder_heap_used_size,
            backing_storage_size: backing_storage_size,
        }
    }
    /// Used JavaScript heap size in bytes.
    pub fn used_size(&self) -> f64 { self.used_size }
    /// Allocated JavaScript heap size in bytes.
    pub fn total_size(&self) -> f64 { self.total_size }
    /// Used size in bytes in the embedder's garbage-collected heap.
    pub fn embedder_heap_used_size(&self) -> f64 { self.embedder_heap_used_size }
    /// Size in bytes of backing storage for array buffers and external strings.
    pub fn backing_storage_size(&self) -> f64 { self.backing_storage_size }
}


pub struct GetHeapUsageReturnsBuilder {
    used_size: f64,
    total_size: f64,
    embedder_heap_used_size: f64,
    backing_storage_size: f64,
}

impl GetHeapUsageReturnsBuilder {
    pub fn build(self) -> GetHeapUsageReturns {
        GetHeapUsageReturns {
            used_size: self.used_size,
            total_size: self.total_size,
            embedder_heap_used_size: self.embedder_heap_used_size,
            backing_storage_size: self.backing_storage_size,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetHeapUsageParams {}

impl GetHeapUsageParams { pub const METHOD: &'static str = "Runtime.getHeapUsage"; }

impl<'a> crate::CdpCommand<'a> for GetHeapUsageParams {
    const METHOD: &'static str = "Runtime.getHeapUsage";
    type Response = GetHeapUsageReturns;
}

/// Returns properties of a given object. Object group of the result is inherited from the target
/// object.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPropertiesParams<'a> {
    /// Identifier of the object to return properties for.
    #[serde(rename = "objectId")]
    object_id: RemoteObjectId<'a>,
    /// If true, returns properties belonging only to the element itself, not to its prototype
    /// chain.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ownProperties")]
    own_properties: Option<bool>,
    /// If true, returns accessor properties (with getter/setter) only; internal properties are not
    /// returned either.
    #[serde(skip_serializing_if = "Option::is_none", rename = "accessorPropertiesOnly")]
    accessor_properties_only: Option<bool>,
    /// Whether preview should be generated for the results.
    #[serde(skip_serializing_if = "Option::is_none", rename = "generatePreview")]
    generate_preview: Option<bool>,
    /// If true, returns non-indexed properties only.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nonIndexedPropertiesOnly")]
    non_indexed_properties_only: Option<bool>,
}

impl<'a> GetPropertiesParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `object_id`: Identifier of the object to return properties for.
    pub fn builder(object_id: impl Into<RemoteObjectId<'a>>) -> GetPropertiesParamsBuilder<'a> {
        GetPropertiesParamsBuilder {
            object_id: object_id.into(),
            own_properties: None,
            accessor_properties_only: None,
            generate_preview: None,
            non_indexed_properties_only: None,
        }
    }
    /// Identifier of the object to return properties for.
    pub fn object_id(&self) -> &RemoteObjectId<'a> { &self.object_id }
    /// If true, returns properties belonging only to the element itself, not to its prototype
    /// chain.
    pub fn own_properties(&self) -> Option<bool> { self.own_properties }
    /// If true, returns accessor properties (with getter/setter) only; internal properties are not
    /// returned either.
    pub fn accessor_properties_only(&self) -> Option<bool> { self.accessor_properties_only }
    /// Whether preview should be generated for the results.
    pub fn generate_preview(&self) -> Option<bool> { self.generate_preview }
    /// If true, returns non-indexed properties only.
    pub fn non_indexed_properties_only(&self) -> Option<bool> { self.non_indexed_properties_only }
}


pub struct GetPropertiesParamsBuilder<'a> {
    object_id: RemoteObjectId<'a>,
    own_properties: Option<bool>,
    accessor_properties_only: Option<bool>,
    generate_preview: Option<bool>,
    non_indexed_properties_only: Option<bool>,
}

impl<'a> GetPropertiesParamsBuilder<'a> {
    /// If true, returns properties belonging only to the element itself, not to its prototype
    /// chain.
    pub fn own_properties(mut self, own_properties: bool) -> Self { self.own_properties = Some(own_properties); self }
    /// If true, returns accessor properties (with getter/setter) only; internal properties are not
    /// returned either.
    pub fn accessor_properties_only(mut self, accessor_properties_only: bool) -> Self { self.accessor_properties_only = Some(accessor_properties_only); self }
    /// Whether preview should be generated for the results.
    pub fn generate_preview(mut self, generate_preview: bool) -> Self { self.generate_preview = Some(generate_preview); self }
    /// If true, returns non-indexed properties only.
    pub fn non_indexed_properties_only(mut self, non_indexed_properties_only: bool) -> Self { self.non_indexed_properties_only = Some(non_indexed_properties_only); self }
    pub fn build(self) -> GetPropertiesParams<'a> {
        GetPropertiesParams {
            object_id: self.object_id,
            own_properties: self.own_properties,
            accessor_properties_only: self.accessor_properties_only,
            generate_preview: self.generate_preview,
            non_indexed_properties_only: self.non_indexed_properties_only,
        }
    }
}

/// Returns properties of a given object. Object group of the result is inherited from the target
/// object.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPropertiesReturns<'a> {
    /// Object properties.
    result: Vec<PropertyDescriptor<'a>>,
    /// Internal object properties (only of the element itself).
    #[serde(skip_serializing_if = "Option::is_none", rename = "internalProperties")]
    internal_properties: Option<Vec<InternalPropertyDescriptor<'a>>>,
    /// Object private properties.
    #[serde(skip_serializing_if = "Option::is_none", rename = "privateProperties")]
    private_properties: Option<Vec<PrivatePropertyDescriptor<'a>>>,
    /// Exception details.
    #[serde(skip_serializing_if = "Option::is_none", rename = "exceptionDetails")]
    exception_details: Option<ExceptionDetails<'a>>,
}

impl<'a> GetPropertiesReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `result`: Object properties.
    pub fn builder(result: Vec<PropertyDescriptor<'a>>) -> GetPropertiesReturnsBuilder<'a> {
        GetPropertiesReturnsBuilder {
            result: result,
            internal_properties: None,
            private_properties: None,
            exception_details: None,
        }
    }
    /// Object properties.
    pub fn result(&self) -> &[PropertyDescriptor<'a>] { &self.result }
    /// Internal object properties (only of the element itself).
    pub fn internal_properties(&self) -> Option<&[InternalPropertyDescriptor<'a>]> { self.internal_properties.as_deref() }
    /// Object private properties.
    pub fn private_properties(&self) -> Option<&[PrivatePropertyDescriptor<'a>]> { self.private_properties.as_deref() }
    /// Exception details.
    pub fn exception_details(&self) -> Option<&ExceptionDetails<'a>> { self.exception_details.as_ref() }
}


pub struct GetPropertiesReturnsBuilder<'a> {
    result: Vec<PropertyDescriptor<'a>>,
    internal_properties: Option<Vec<InternalPropertyDescriptor<'a>>>,
    private_properties: Option<Vec<PrivatePropertyDescriptor<'a>>>,
    exception_details: Option<ExceptionDetails<'a>>,
}

impl<'a> GetPropertiesReturnsBuilder<'a> {
    /// Internal object properties (only of the element itself).
    pub fn internal_properties(mut self, internal_properties: Vec<InternalPropertyDescriptor<'a>>) -> Self { self.internal_properties = Some(internal_properties); self }
    /// Object private properties.
    pub fn private_properties(mut self, private_properties: Vec<PrivatePropertyDescriptor<'a>>) -> Self { self.private_properties = Some(private_properties); self }
    /// Exception details.
    pub fn exception_details(mut self, exception_details: ExceptionDetails<'a>) -> Self { self.exception_details = Some(exception_details); self }
    pub fn build(self) -> GetPropertiesReturns<'a> {
        GetPropertiesReturns {
            result: self.result,
            internal_properties: self.internal_properties,
            private_properties: self.private_properties,
            exception_details: self.exception_details,
        }
    }
}

impl<'a> GetPropertiesParams<'a> { pub const METHOD: &'static str = "Runtime.getProperties"; }

impl<'a> crate::CdpCommand<'a> for GetPropertiesParams<'a> {
    const METHOD: &'static str = "Runtime.getProperties";
    type Response = GetPropertiesReturns<'a>;
}

/// Returns all let, const and class variables from global scope.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GlobalLexicalScopeNamesParams {
    /// Specifies in which execution context to lookup global scope variables.
    #[serde(skip_serializing_if = "Option::is_none", rename = "executionContextId")]
    execution_context_id: Option<ExecutionContextId>,
}

impl GlobalLexicalScopeNamesParams {
    /// Creates a builder for this type.
    pub fn builder() -> GlobalLexicalScopeNamesParamsBuilder {
        GlobalLexicalScopeNamesParamsBuilder {
            execution_context_id: None,
        }
    }
    /// Specifies in which execution context to lookup global scope variables.
    pub fn execution_context_id(&self) -> Option<&ExecutionContextId> { self.execution_context_id.as_ref() }
}

#[derive(Default)]
pub struct GlobalLexicalScopeNamesParamsBuilder {
    execution_context_id: Option<ExecutionContextId>,
}

impl GlobalLexicalScopeNamesParamsBuilder {
    /// Specifies in which execution context to lookup global scope variables.
    pub fn execution_context_id(mut self, execution_context_id: ExecutionContextId) -> Self { self.execution_context_id = Some(execution_context_id); self }
    pub fn build(self) -> GlobalLexicalScopeNamesParams {
        GlobalLexicalScopeNamesParams {
            execution_context_id: self.execution_context_id,
        }
    }
}

/// Returns all let, const and class variables from global scope.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GlobalLexicalScopeNamesReturns<'a> {
    names: Vec<Cow<'a, str>>,
}

impl<'a> GlobalLexicalScopeNamesReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `names`: 
    pub fn builder(names: Vec<Cow<'a, str>>) -> GlobalLexicalScopeNamesReturnsBuilder<'a> {
        GlobalLexicalScopeNamesReturnsBuilder {
            names: names,
        }
    }
    pub fn names(&self) -> &[Cow<'a, str>] { &self.names }
}


pub struct GlobalLexicalScopeNamesReturnsBuilder<'a> {
    names: Vec<Cow<'a, str>>,
}

impl<'a> GlobalLexicalScopeNamesReturnsBuilder<'a> {
    pub fn build(self) -> GlobalLexicalScopeNamesReturns<'a> {
        GlobalLexicalScopeNamesReturns {
            names: self.names,
        }
    }
}

impl GlobalLexicalScopeNamesParams { pub const METHOD: &'static str = "Runtime.globalLexicalScopeNames"; }

impl<'a> crate::CdpCommand<'a> for GlobalLexicalScopeNamesParams {
    const METHOD: &'static str = "Runtime.globalLexicalScopeNames";
    type Response = GlobalLexicalScopeNamesReturns<'a>;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryObjectsParams<'a> {
    /// Identifier of the prototype to return objects for.
    #[serde(rename = "prototypeObjectId")]
    prototype_object_id: RemoteObjectId<'a>,
    /// Symbolic group name that can be used to release the results.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectGroup")]
    object_group: Option<Cow<'a, str>>,
}

impl<'a> QueryObjectsParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `prototype_object_id`: Identifier of the prototype to return objects for.
    pub fn builder(prototype_object_id: impl Into<RemoteObjectId<'a>>) -> QueryObjectsParamsBuilder<'a> {
        QueryObjectsParamsBuilder {
            prototype_object_id: prototype_object_id.into(),
            object_group: None,
        }
    }
    /// Identifier of the prototype to return objects for.
    pub fn prototype_object_id(&self) -> &RemoteObjectId<'a> { &self.prototype_object_id }
    /// Symbolic group name that can be used to release the results.
    pub fn object_group(&self) -> Option<&str> { self.object_group.as_deref() }
}


pub struct QueryObjectsParamsBuilder<'a> {
    prototype_object_id: RemoteObjectId<'a>,
    object_group: Option<Cow<'a, str>>,
}

impl<'a> QueryObjectsParamsBuilder<'a> {
    /// Symbolic group name that can be used to release the results.
    pub fn object_group(mut self, object_group: impl Into<Cow<'a, str>>) -> Self { self.object_group = Some(object_group.into()); self }
    pub fn build(self) -> QueryObjectsParams<'a> {
        QueryObjectsParams {
            prototype_object_id: self.prototype_object_id,
            object_group: self.object_group,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryObjectsReturns<'a> {
    /// Array with objects.
    objects: RemoteObject<'a>,
}

impl<'a> QueryObjectsReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `objects`: Array with objects.
    pub fn builder(objects: RemoteObject<'a>) -> QueryObjectsReturnsBuilder<'a> {
        QueryObjectsReturnsBuilder {
            objects: objects,
        }
    }
    /// Array with objects.
    pub fn objects(&self) -> &RemoteObject<'a> { &self.objects }
}


pub struct QueryObjectsReturnsBuilder<'a> {
    objects: RemoteObject<'a>,
}

impl<'a> QueryObjectsReturnsBuilder<'a> {
    pub fn build(self) -> QueryObjectsReturns<'a> {
        QueryObjectsReturns {
            objects: self.objects,
        }
    }
}

impl<'a> QueryObjectsParams<'a> { pub const METHOD: &'static str = "Runtime.queryObjects"; }

impl<'a> crate::CdpCommand<'a> for QueryObjectsParams<'a> {
    const METHOD: &'static str = "Runtime.queryObjects";
    type Response = QueryObjectsReturns<'a>;
}

/// Releases remote object with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseObjectParams<'a> {
    /// Identifier of the object to release.
    #[serde(rename = "objectId")]
    object_id: RemoteObjectId<'a>,
}

impl<'a> ReleaseObjectParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `object_id`: Identifier of the object to release.
    pub fn builder(object_id: impl Into<RemoteObjectId<'a>>) -> ReleaseObjectParamsBuilder<'a> {
        ReleaseObjectParamsBuilder {
            object_id: object_id.into(),
        }
    }
    /// Identifier of the object to release.
    pub fn object_id(&self) -> &RemoteObjectId<'a> { &self.object_id }
}


pub struct ReleaseObjectParamsBuilder<'a> {
    object_id: RemoteObjectId<'a>,
}

impl<'a> ReleaseObjectParamsBuilder<'a> {
    pub fn build(self) -> ReleaseObjectParams<'a> {
        ReleaseObjectParams {
            object_id: self.object_id,
        }
    }
}

impl<'a> ReleaseObjectParams<'a> { pub const METHOD: &'static str = "Runtime.releaseObject"; }

impl<'a> crate::CdpCommand<'a> for ReleaseObjectParams<'a> {
    const METHOD: &'static str = "Runtime.releaseObject";
    type Response = crate::EmptyReturns;
}

/// Releases all remote objects that belong to a given group.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseObjectGroupParams<'a> {
    /// Symbolic object group name.
    #[serde(rename = "objectGroup")]
    object_group: Cow<'a, str>,
}

impl<'a> ReleaseObjectGroupParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `object_group`: Symbolic object group name.
    pub fn builder(object_group: impl Into<Cow<'a, str>>) -> ReleaseObjectGroupParamsBuilder<'a> {
        ReleaseObjectGroupParamsBuilder {
            object_group: object_group.into(),
        }
    }
    /// Symbolic object group name.
    pub fn object_group(&self) -> &str { self.object_group.as_ref() }
}


pub struct ReleaseObjectGroupParamsBuilder<'a> {
    object_group: Cow<'a, str>,
}

impl<'a> ReleaseObjectGroupParamsBuilder<'a> {
    pub fn build(self) -> ReleaseObjectGroupParams<'a> {
        ReleaseObjectGroupParams {
            object_group: self.object_group,
        }
    }
}

impl<'a> ReleaseObjectGroupParams<'a> { pub const METHOD: &'static str = "Runtime.releaseObjectGroup"; }

impl<'a> crate::CdpCommand<'a> for ReleaseObjectGroupParams<'a> {
    const METHOD: &'static str = "Runtime.releaseObjectGroup";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RunIfWaitingForDebuggerParams {}

impl RunIfWaitingForDebuggerParams { pub const METHOD: &'static str = "Runtime.runIfWaitingForDebugger"; }

impl<'a> crate::CdpCommand<'a> for RunIfWaitingForDebuggerParams {
    const METHOD: &'static str = "Runtime.runIfWaitingForDebugger";
    type Response = crate::EmptyReturns;
}

/// Runs script with given id in a given context.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RunScriptParams<'a> {
    /// Id of the script to run.
    #[serde(rename = "scriptId")]
    script_id: ScriptId<'a>,
    /// Specifies in which execution context to perform script run. If the parameter is omitted the
    /// evaluation will be performed in the context of the inspected page.
    #[serde(skip_serializing_if = "Option::is_none", rename = "executionContextId")]
    execution_context_id: Option<ExecutionContextId>,
    /// Symbolic group name that can be used to release multiple objects.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectGroup")]
    object_group: Option<Cow<'a, str>>,
    /// In silent mode exceptions thrown during evaluation are not reported and do not pause
    /// execution. Overrides 'setPauseOnException' state.
    #[serde(skip_serializing_if = "Option::is_none")]
    silent: Option<bool>,
    /// Determines whether Command Line API should be available during the evaluation.
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeCommandLineAPI")]
    include_command_line_api: Option<bool>,
    /// Whether the result is expected to be a JSON object which should be sent by value.
    #[serde(skip_serializing_if = "Option::is_none", rename = "returnByValue")]
    return_by_value: Option<bool>,
    /// Whether preview should be generated for the result.
    #[serde(skip_serializing_if = "Option::is_none", rename = "generatePreview")]
    generate_preview: Option<bool>,
    /// Whether execution should 'await' for resulting value and return once awaited promise is
    /// resolved.
    #[serde(skip_serializing_if = "Option::is_none", rename = "awaitPromise")]
    await_promise: Option<bool>,
}

impl<'a> RunScriptParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `script_id`: Id of the script to run.
    pub fn builder(script_id: impl Into<ScriptId<'a>>) -> RunScriptParamsBuilder<'a> {
        RunScriptParamsBuilder {
            script_id: script_id.into(),
            execution_context_id: None,
            object_group: None,
            silent: None,
            include_command_line_api: None,
            return_by_value: None,
            generate_preview: None,
            await_promise: None,
        }
    }
    /// Id of the script to run.
    pub fn script_id(&self) -> &ScriptId<'a> { &self.script_id }
    /// Specifies in which execution context to perform script run. If the parameter is omitted the
    /// evaluation will be performed in the context of the inspected page.
    pub fn execution_context_id(&self) -> Option<&ExecutionContextId> { self.execution_context_id.as_ref() }
    /// Symbolic group name that can be used to release multiple objects.
    pub fn object_group(&self) -> Option<&str> { self.object_group.as_deref() }
    /// In silent mode exceptions thrown during evaluation are not reported and do not pause
    /// execution. Overrides 'setPauseOnException' state.
    pub fn silent(&self) -> Option<bool> { self.silent }
    /// Determines whether Command Line API should be available during the evaluation.
    pub fn include_command_line_api(&self) -> Option<bool> { self.include_command_line_api }
    /// Whether the result is expected to be a JSON object which should be sent by value.
    pub fn return_by_value(&self) -> Option<bool> { self.return_by_value }
    /// Whether preview should be generated for the result.
    pub fn generate_preview(&self) -> Option<bool> { self.generate_preview }
    /// Whether execution should 'await' for resulting value and return once awaited promise is
    /// resolved.
    pub fn await_promise(&self) -> Option<bool> { self.await_promise }
}


pub struct RunScriptParamsBuilder<'a> {
    script_id: ScriptId<'a>,
    execution_context_id: Option<ExecutionContextId>,
    object_group: Option<Cow<'a, str>>,
    silent: Option<bool>,
    include_command_line_api: Option<bool>,
    return_by_value: Option<bool>,
    generate_preview: Option<bool>,
    await_promise: Option<bool>,
}

impl<'a> RunScriptParamsBuilder<'a> {
    /// Specifies in which execution context to perform script run. If the parameter is omitted the
    /// evaluation will be performed in the context of the inspected page.
    pub fn execution_context_id(mut self, execution_context_id: ExecutionContextId) -> Self { self.execution_context_id = Some(execution_context_id); self }
    /// Symbolic group name that can be used to release multiple objects.
    pub fn object_group(mut self, object_group: impl Into<Cow<'a, str>>) -> Self { self.object_group = Some(object_group.into()); self }
    /// In silent mode exceptions thrown during evaluation are not reported and do not pause
    /// execution. Overrides 'setPauseOnException' state.
    pub fn silent(mut self, silent: bool) -> Self { self.silent = Some(silent); self }
    /// Determines whether Command Line API should be available during the evaluation.
    pub fn include_command_line_api(mut self, include_command_line_api: bool) -> Self { self.include_command_line_api = Some(include_command_line_api); self }
    /// Whether the result is expected to be a JSON object which should be sent by value.
    pub fn return_by_value(mut self, return_by_value: bool) -> Self { self.return_by_value = Some(return_by_value); self }
    /// Whether preview should be generated for the result.
    pub fn generate_preview(mut self, generate_preview: bool) -> Self { self.generate_preview = Some(generate_preview); self }
    /// Whether execution should 'await' for resulting value and return once awaited promise is
    /// resolved.
    pub fn await_promise(mut self, await_promise: bool) -> Self { self.await_promise = Some(await_promise); self }
    pub fn build(self) -> RunScriptParams<'a> {
        RunScriptParams {
            script_id: self.script_id,
            execution_context_id: self.execution_context_id,
            object_group: self.object_group,
            silent: self.silent,
            include_command_line_api: self.include_command_line_api,
            return_by_value: self.return_by_value,
            generate_preview: self.generate_preview,
            await_promise: self.await_promise,
        }
    }
}

/// Runs script with given id in a given context.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RunScriptReturns<'a> {
    /// Run result.
    result: RemoteObject<'a>,
    /// Exception details.
    #[serde(skip_serializing_if = "Option::is_none", rename = "exceptionDetails")]
    exception_details: Option<ExceptionDetails<'a>>,
}

impl<'a> RunScriptReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `result`: Run result.
    pub fn builder(result: RemoteObject<'a>) -> RunScriptReturnsBuilder<'a> {
        RunScriptReturnsBuilder {
            result: result,
            exception_details: None,
        }
    }
    /// Run result.
    pub fn result(&self) -> &RemoteObject<'a> { &self.result }
    /// Exception details.
    pub fn exception_details(&self) -> Option<&ExceptionDetails<'a>> { self.exception_details.as_ref() }
}


pub struct RunScriptReturnsBuilder<'a> {
    result: RemoteObject<'a>,
    exception_details: Option<ExceptionDetails<'a>>,
}

impl<'a> RunScriptReturnsBuilder<'a> {
    /// Exception details.
    pub fn exception_details(mut self, exception_details: ExceptionDetails<'a>) -> Self { self.exception_details = Some(exception_details); self }
    pub fn build(self) -> RunScriptReturns<'a> {
        RunScriptReturns {
            result: self.result,
            exception_details: self.exception_details,
        }
    }
}

impl<'a> RunScriptParams<'a> { pub const METHOD: &'static str = "Runtime.runScript"; }

impl<'a> crate::CdpCommand<'a> for RunScriptParams<'a> {
    const METHOD: &'static str = "Runtime.runScript";
    type Response = RunScriptReturns<'a>;
}

/// Enables or disables async call stacks tracking.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetAsyncCallStackDepthParams {
    /// Maximum depth of async call stacks. Setting to '0' will effectively disable collecting async
    /// call stacks (default).
    #[serde(rename = "maxDepth")]
    max_depth: i64,
}

impl SetAsyncCallStackDepthParams {
    /// Creates a builder for this type with the required parameters:
    /// * `max_depth`: Maximum depth of async call stacks. Setting to `0` will effectively disable collecting async call stacks (default).
    pub fn builder(max_depth: i64) -> SetAsyncCallStackDepthParamsBuilder {
        SetAsyncCallStackDepthParamsBuilder {
            max_depth: max_depth,
        }
    }
    /// Maximum depth of async call stacks. Setting to '0' will effectively disable collecting async
    /// call stacks (default).
    pub fn max_depth(&self) -> i64 { self.max_depth }
}


pub struct SetAsyncCallStackDepthParamsBuilder {
    max_depth: i64,
}

impl SetAsyncCallStackDepthParamsBuilder {
    pub fn build(self) -> SetAsyncCallStackDepthParams {
        SetAsyncCallStackDepthParams {
            max_depth: self.max_depth,
        }
    }
}

impl SetAsyncCallStackDepthParams { pub const METHOD: &'static str = "Runtime.setAsyncCallStackDepth"; }

impl<'a> crate::CdpCommand<'a> for SetAsyncCallStackDepthParams {
    const METHOD: &'static str = "Runtime.setAsyncCallStackDepth";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetCustomObjectFormatterEnabledParams {
    enabled: bool,
}

impl SetCustomObjectFormatterEnabledParams {
    /// Creates a builder for this type with the required parameters:
    /// * `enabled`: 
    pub fn builder(enabled: bool) -> SetCustomObjectFormatterEnabledParamsBuilder {
        SetCustomObjectFormatterEnabledParamsBuilder {
            enabled: enabled,
        }
    }
    pub fn enabled(&self) -> bool { self.enabled }
}


pub struct SetCustomObjectFormatterEnabledParamsBuilder {
    enabled: bool,
}

impl SetCustomObjectFormatterEnabledParamsBuilder {
    pub fn build(self) -> SetCustomObjectFormatterEnabledParams {
        SetCustomObjectFormatterEnabledParams {
            enabled: self.enabled,
        }
    }
}

impl SetCustomObjectFormatterEnabledParams { pub const METHOD: &'static str = "Runtime.setCustomObjectFormatterEnabled"; }

impl<'a> crate::CdpCommand<'a> for SetCustomObjectFormatterEnabledParams {
    const METHOD: &'static str = "Runtime.setCustomObjectFormatterEnabled";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetMaxCallStackSizeToCaptureParams {
    size: u64,
}

impl SetMaxCallStackSizeToCaptureParams {
    /// Creates a builder for this type with the required parameters:
    /// * `size`: 
    pub fn builder(size: u64) -> SetMaxCallStackSizeToCaptureParamsBuilder {
        SetMaxCallStackSizeToCaptureParamsBuilder {
            size: size,
        }
    }
    pub fn size(&self) -> u64 { self.size }
}


pub struct SetMaxCallStackSizeToCaptureParamsBuilder {
    size: u64,
}

impl SetMaxCallStackSizeToCaptureParamsBuilder {
    pub fn build(self) -> SetMaxCallStackSizeToCaptureParams {
        SetMaxCallStackSizeToCaptureParams {
            size: self.size,
        }
    }
}

impl SetMaxCallStackSizeToCaptureParams { pub const METHOD: &'static str = "Runtime.setMaxCallStackSizeToCapture"; }

impl<'a> crate::CdpCommand<'a> for SetMaxCallStackSizeToCaptureParams {
    const METHOD: &'static str = "Runtime.setMaxCallStackSizeToCapture";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerminateExecutionParams {}

impl TerminateExecutionParams { pub const METHOD: &'static str = "Runtime.terminateExecution"; }

impl<'a> crate::CdpCommand<'a> for TerminateExecutionParams {
    const METHOD: &'static str = "Runtime.terminateExecution";
    type Response = crate::EmptyReturns;
}

/// If executionContextId is empty, adds binding with the given name on the
/// global objects of all inspected contexts, including those created later,
/// bindings survive reloads.
/// Binding function takes exactly one argument, this argument should be string,
/// in case of any other input, function throws an exception.
/// Each binding function call produces Runtime.bindingCalled notification.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddBindingParams<'a> {
    name: Cow<'a, str>,
    /// If specified, the binding would only be exposed to the specified
    /// execution context. If omitted and 'executionContextName' is not set,
    /// the binding is exposed to all execution contexts of the target.
    /// This parameter is mutually exclusive with 'executionContextName'.
    /// Deprecated in favor of 'executionContextName' due to an unclear use case
    /// and bugs in implementation (crbug.com/1169639). 'executionContextId' will be
    /// removed in the future.
    #[serde(skip_serializing_if = "Option::is_none", rename = "executionContextId")]
    execution_context_id: Option<ExecutionContextId>,
    /// If specified, the binding is exposed to the executionContext with
    /// matching name, even for contexts created after the binding is added.
    /// See also 'ExecutionContext.name' and 'worldName' parameter to
    /// 'Page.addScriptToEvaluateOnNewDocument'.
    /// This parameter is mutually exclusive with 'executionContextId'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "executionContextName")]
    execution_context_name: Option<Cow<'a, str>>,
}

impl<'a> AddBindingParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `name`: 
    pub fn builder(name: impl Into<Cow<'a, str>>) -> AddBindingParamsBuilder<'a> {
        AddBindingParamsBuilder {
            name: name.into(),
            execution_context_id: None,
            execution_context_name: None,
        }
    }
    pub fn name(&self) -> &str { self.name.as_ref() }
    /// If specified, the binding would only be exposed to the specified
    /// execution context. If omitted and 'executionContextName' is not set,
    /// the binding is exposed to all execution contexts of the target.
    /// This parameter is mutually exclusive with 'executionContextName'.
    /// Deprecated in favor of 'executionContextName' due to an unclear use case
    /// and bugs in implementation (crbug.com/1169639). 'executionContextId' will be
    /// removed in the future.
    pub fn execution_context_id(&self) -> Option<&ExecutionContextId> { self.execution_context_id.as_ref() }
    /// If specified, the binding is exposed to the executionContext with
    /// matching name, even for contexts created after the binding is added.
    /// See also 'ExecutionContext.name' and 'worldName' parameter to
    /// 'Page.addScriptToEvaluateOnNewDocument'.
    /// This parameter is mutually exclusive with 'executionContextId'.
    pub fn execution_context_name(&self) -> Option<&str> { self.execution_context_name.as_deref() }
}


pub struct AddBindingParamsBuilder<'a> {
    name: Cow<'a, str>,
    execution_context_id: Option<ExecutionContextId>,
    execution_context_name: Option<Cow<'a, str>>,
}

impl<'a> AddBindingParamsBuilder<'a> {
    /// If specified, the binding would only be exposed to the specified
    /// execution context. If omitted and 'executionContextName' is not set,
    /// the binding is exposed to all execution contexts of the target.
    /// This parameter is mutually exclusive with 'executionContextName'.
    /// Deprecated in favor of 'executionContextName' due to an unclear use case
    /// and bugs in implementation (crbug.com/1169639). 'executionContextId' will be
    /// removed in the future.
    pub fn execution_context_id(mut self, execution_context_id: ExecutionContextId) -> Self { self.execution_context_id = Some(execution_context_id); self }
    /// If specified, the binding is exposed to the executionContext with
    /// matching name, even for contexts created after the binding is added.
    /// See also 'ExecutionContext.name' and 'worldName' parameter to
    /// 'Page.addScriptToEvaluateOnNewDocument'.
    /// This parameter is mutually exclusive with 'executionContextId'.
    pub fn execution_context_name(mut self, execution_context_name: impl Into<Cow<'a, str>>) -> Self { self.execution_context_name = Some(execution_context_name.into()); self }
    pub fn build(self) -> AddBindingParams<'a> {
        AddBindingParams {
            name: self.name,
            execution_context_id: self.execution_context_id,
            execution_context_name: self.execution_context_name,
        }
    }
}

impl<'a> AddBindingParams<'a> { pub const METHOD: &'static str = "Runtime.addBinding"; }

impl<'a> crate::CdpCommand<'a> for AddBindingParams<'a> {
    const METHOD: &'static str = "Runtime.addBinding";
    type Response = crate::EmptyReturns;
}

/// This method does not remove binding function from global object but
/// unsubscribes current runtime agent from Runtime.bindingCalled notifications.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveBindingParams<'a> {
    name: Cow<'a, str>,
}

impl<'a> RemoveBindingParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `name`: 
    pub fn builder(name: impl Into<Cow<'a, str>>) -> RemoveBindingParamsBuilder<'a> {
        RemoveBindingParamsBuilder {
            name: name.into(),
        }
    }
    pub fn name(&self) -> &str { self.name.as_ref() }
}


pub struct RemoveBindingParamsBuilder<'a> {
    name: Cow<'a, str>,
}

impl<'a> RemoveBindingParamsBuilder<'a> {
    pub fn build(self) -> RemoveBindingParams<'a> {
        RemoveBindingParams {
            name: self.name,
        }
    }
}

impl<'a> RemoveBindingParams<'a> { pub const METHOD: &'static str = "Runtime.removeBinding"; }

impl<'a> crate::CdpCommand<'a> for RemoveBindingParams<'a> {
    const METHOD: &'static str = "Runtime.removeBinding";
    type Response = crate::EmptyReturns;
}

/// This method tries to lookup and populate exception details for a
/// JavaScript Error object.
/// Note that the stackTrace portion of the resulting exceptionDetails will
/// only be populated if the Runtime domain was enabled at the time when the
/// Error was thrown.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetExceptionDetailsParams<'a> {
    /// The error object for which to resolve the exception details.
    #[serde(rename = "errorObjectId")]
    error_object_id: RemoteObjectId<'a>,
}

impl<'a> GetExceptionDetailsParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `error_object_id`: The error object for which to resolve the exception details.
    pub fn builder(error_object_id: impl Into<RemoteObjectId<'a>>) -> GetExceptionDetailsParamsBuilder<'a> {
        GetExceptionDetailsParamsBuilder {
            error_object_id: error_object_id.into(),
        }
    }
    /// The error object for which to resolve the exception details.
    pub fn error_object_id(&self) -> &RemoteObjectId<'a> { &self.error_object_id }
}


pub struct GetExceptionDetailsParamsBuilder<'a> {
    error_object_id: RemoteObjectId<'a>,
}

impl<'a> GetExceptionDetailsParamsBuilder<'a> {
    pub fn build(self) -> GetExceptionDetailsParams<'a> {
        GetExceptionDetailsParams {
            error_object_id: self.error_object_id,
        }
    }
}

/// This method tries to lookup and populate exception details for a
/// JavaScript Error object.
/// Note that the stackTrace portion of the resulting exceptionDetails will
/// only be populated if the Runtime domain was enabled at the time when the
/// Error was thrown.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetExceptionDetailsReturns<'a> {
    #[serde(skip_serializing_if = "Option::is_none", rename = "exceptionDetails")]
    exception_details: Option<ExceptionDetails<'a>>,
}

impl<'a> GetExceptionDetailsReturns<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> GetExceptionDetailsReturnsBuilder<'a> {
        GetExceptionDetailsReturnsBuilder {
            exception_details: None,
        }
    }
    pub fn exception_details(&self) -> Option<&ExceptionDetails<'a>> { self.exception_details.as_ref() }
}

#[derive(Default)]
pub struct GetExceptionDetailsReturnsBuilder<'a> {
    exception_details: Option<ExceptionDetails<'a>>,
}

impl<'a> GetExceptionDetailsReturnsBuilder<'a> {
    pub fn exception_details(mut self, exception_details: ExceptionDetails<'a>) -> Self { self.exception_details = Some(exception_details); self }
    pub fn build(self) -> GetExceptionDetailsReturns<'a> {
        GetExceptionDetailsReturns {
            exception_details: self.exception_details,
        }
    }
}

impl<'a> GetExceptionDetailsParams<'a> { pub const METHOD: &'static str = "Runtime.getExceptionDetails"; }

impl<'a> crate::CdpCommand<'a> for GetExceptionDetailsParams<'a> {
    const METHOD: &'static str = "Runtime.getExceptionDetails";
    type Response = GetExceptionDetailsReturns<'a>;
}
