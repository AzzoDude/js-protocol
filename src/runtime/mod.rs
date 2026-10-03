//! Runtime domain exposes JavaScript runtime by means of remote evaluation and mirror objects.
//! Evaluation results are returned as mirror object that expose object type, string representation
//! and unique identifier that can be used for further object reference. Original objects are
//! maintained in memory unless they are either explicitly released or are released along with the
//! other objects in their object group.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Unique script identifier.

pub type ScriptId<'a> = Cow<'a, str>;

/// Represents options for serialization. Overrides 'generatePreview' and 'returnByValue'.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SerializationOptions<'a> {
    pub serialization: Cow<'a, str>,
    /// Deep serialization depth. Default is full depth. Respected only in 'deep' serialization mode.
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxDepth")]
    pub max_depth: Option<i64>,
    /// Embedder-specific parameters. For example if connected to V8 in Chrome these control DOM
    /// serialization via 'maxNodeDepth: integer' and 'includeShadowTree: "none" | "open" | "all"'.
    /// Values can be only of type string or integer.
    #[serde(skip_serializing_if = "Option::is_none", rename = "additionalParameters")]
    pub additional_parameters: Option<serde_json::Map<String, JsonValue>>,
}
/// Represents deep serialized value.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DeepSerializedValue<'a> {
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    pub object_id: Option<Cow<'a, str>>,
    /// Set if value reference met more then once during serialization. In such
    /// case, value is provided only to one of the serialized values. Unique
    /// per value in the scope of one CDP call.
    #[serde(skip_serializing_if = "Option::is_none", rename = "weakLocalObjectReference")]
    pub weak_local_object_reference: Option<i64>,
}
/// Unique object identifier.

pub type RemoteObjectId<'a> = Cow<'a, str>;

/// Primitive value which cannot be JSON-stringified. Includes values '-0', 'NaN', 'Infinity',
/// '-Infinity', and bigint literals.

pub type UnserializableValue<'a> = Cow<'a, str>;

/// Mirror object referencing original JavaScript object.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RemoteObject<'a> {
    /// Object type.
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
    /// Object subtype hint. Specified for 'object' type values only.
    /// NOTE: If you change anything here, make sure to also update
    /// 'subtype' in 'ObjectPreview' and 'PropertyPreview' below.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<Cow<'a, str>>,
    /// Object class (constructor) name. Specified for 'object' type values only.
    #[serde(skip_serializing_if = "Option::is_none", rename = "className")]
    pub class_name: Option<Cow<'a, str>>,
    /// Remote object value in case of primitive values or JSON values (if it was requested).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<JsonValue>,
    /// Primitive value which can not be JSON-stringified does not have 'value', but gets this
    /// property.
    #[serde(skip_serializing_if = "Option::is_none", rename = "unserializableValue")]
    pub unserializable_value: Option<UnserializableValue<'a>>,
    /// String representation of the object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,
    /// Deep serialized value.
    #[serde(skip_serializing_if = "Option::is_none", rename = "deepSerializedValue")]
    pub deep_serialized_value: Option<DeepSerializedValue<'a>>,
    /// Unique object identifier (for non-primitive values).
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    pub object_id: Option<RemoteObjectId<'a>>,
    /// Preview containing abbreviated property values. Specified for 'object' type values only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<ObjectPreview<'a>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "customPreview")]
    pub custom_preview: Option<CustomPreview<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CustomPreview<'a> {
    /// The JSON-stringified result of formatter.header(object, config) call.
    /// It contains json ML array that represents RemoteObject.
    pub header: Cow<'a, str>,
    /// If formatter returns true as a result of formatter.hasBody call then bodyGetterId will
    /// contain RemoteObjectId for the function that returns result of formatter.body(object, config) call.
    /// The result value is json ML array.
    #[serde(skip_serializing_if = "Option::is_none", rename = "bodyGetterId")]
    pub body_getter_id: Option<RemoteObjectId<'a>>,
}
/// Object containing abbreviated remote object value.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ObjectPreview<'a> {
    /// Object type.
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
    /// Object subtype hint. Specified for 'object' type values only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<Cow<'a, str>>,
    /// String representation of the object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,
    /// True iff some of the properties or entries of the original object did not fit.
    pub overflow: bool,
    /// List of the properties.
    pub properties: Vec<PropertyPreview<'a>>,
    /// List of the entries. Specified for 'map' and 'set' subtype values only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<EntryPreview<'a>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PropertyPreview<'a> {
    /// Property name.
    pub name: Cow<'a, str>,
    /// Object type. Accessor means that the property itself is an accessor property.
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
    /// User-friendly property value string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Cow<'a, str>>,
    /// Nested value preview.
    #[serde(skip_serializing_if = "Option::is_none", rename = "valuePreview")]
    pub value_preview: Option<ObjectPreview<'a>>,
    /// Object subtype hint. Specified for 'object' type values only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct EntryPreview<'a> {
    /// Preview of the key. Specified for map-like collection entries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<ObjectPreview<'a>>,
    /// Preview of the value.
    pub value: ObjectPreview<'a>,
}
/// Object property descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PropertyDescriptor<'a> {
    /// Property name or symbol description.
    pub name: Cow<'a, str>,
    /// The value associated with the property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<RemoteObject<'a>>,
    /// True if the value associated with the property may be changed (data descriptors only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub writable: Option<bool>,
    /// A function which serves as a getter for the property, or 'undefined' if there is no getter
    /// (accessor descriptors only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<RemoteObject<'a>>,
    /// A function which serves as a setter for the property, or 'undefined' if there is no setter
    /// (accessor descriptors only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<RemoteObject<'a>>,
    /// True if the type of this property descriptor may be changed and if the property may be
    /// deleted from the corresponding object.
    pub configurable: bool,
    /// True if this property shows up during enumeration of the properties on the corresponding
    /// object.
    pub enumerable: bool,
    /// True if the result was thrown during the evaluation.
    #[serde(skip_serializing_if = "Option::is_none", rename = "wasThrown")]
    pub was_thrown: Option<bool>,
    /// True if the property is owned for the object.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isOwn")]
    pub is_own: Option<bool>,
    /// Property symbol object, if the property is of the 'symbol' type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<RemoteObject<'a>>,
}
/// Object internal property descriptor. This property isn't normally visible in JavaScript code.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct InternalPropertyDescriptor<'a> {
    /// Conventional property name.
    pub name: Cow<'a, str>,
    /// The value associated with the property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<RemoteObject<'a>>,
}
/// Object private field descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PrivatePropertyDescriptor<'a> {
    /// Private property name.
    pub name: Cow<'a, str>,
    /// The value associated with the private property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<RemoteObject<'a>>,
    /// A function which serves as a getter for the private property,
    /// or 'undefined' if there is no getter (accessor descriptors only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<RemoteObject<'a>>,
    /// A function which serves as a setter for the private property,
    /// or 'undefined' if there is no setter (accessor descriptors only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<RemoteObject<'a>>,
}
/// Represents function call argument. Either remote object id 'objectId', primitive 'value',
/// unserializable primitive value or neither of (for undefined) them should be specified.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CallArgument<'a> {
    /// Primitive value or serializable javascript object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<JsonValue>,
    /// Primitive value which can not be JSON-stringified.
    #[serde(skip_serializing_if = "Option::is_none", rename = "unserializableValue")]
    pub unserializable_value: Option<UnserializableValue<'a>>,
    /// Remote object handle.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    pub object_id: Option<RemoteObjectId<'a>>,
}
/// Id of an execution context.

pub type ExecutionContextId = i64;

/// Description of an isolated world.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionContextDescription<'a> {
    /// Unique id of the execution context. It can be used to specify in which execution context
    /// script evaluation should be performed.
    pub id: ExecutionContextId,
    /// Execution context origin.
    pub origin: Cow<'a, str>,
    /// Human readable name describing given context.
    pub name: Cow<'a, str>,
    /// A system-unique execution context identifier. Unlike the id, this is unique across
    /// multiple processes, so can be reliably used to identify specific context while backend
    /// performs a cross-process navigation.
    #[serde(rename = "uniqueId")]
    pub unique_id: Cow<'a, str>,
    /// Embedder-specific auxiliary data likely matching {isDefault: boolean, type: 'default'|'isolated'|'worker', frameId: string}
    #[serde(skip_serializing_if = "Option::is_none", rename = "auxData")]
    pub aux_data: Option<serde_json::Map<String, JsonValue>>,
}
/// Detailed information about exception (or error) that was thrown during script compilation or
/// execution.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionDetails<'a> {
    /// Exception id.
    #[serde(rename = "exceptionId")]
    pub exception_id: u64,
    /// Exception text, which should be used together with exception object when available.
    pub text: Cow<'a, str>,
    /// Line number of the exception location (0-based).
    #[serde(rename = "lineNumber")]
    pub line_number: i64,
    /// Column number of the exception location (0-based).
    #[serde(rename = "columnNumber")]
    pub column_number: i64,
    /// Script ID of the exception location.
    #[serde(skip_serializing_if = "Option::is_none", rename = "scriptId")]
    pub script_id: Option<ScriptId<'a>>,
    /// URL of the exception location, to be used when the script was not reported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Cow<'a, str>>,
    /// JavaScript stack trace if available.
    #[serde(skip_serializing_if = "Option::is_none", rename = "stackTrace")]
    pub stack_trace: Option<StackTrace<'a>>,
    /// Exception object if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exception: Option<RemoteObject<'a>>,
    /// Identifier of the context where exception happened.
    #[serde(skip_serializing_if = "Option::is_none", rename = "executionContextId")]
    pub execution_context_id: Option<ExecutionContextId>,
    /// Dictionary with entries of meta data that the client associated
    /// with this exception, such as information about associated network
    /// requests, etc.
    #[serde(skip_serializing_if = "Option::is_none", rename = "exceptionMetaData")]
    pub exception_meta_data: Option<serde_json::Map<String, JsonValue>>,
}
/// Number of milliseconds since epoch.

pub type Timestamp = f64;

/// Number of milliseconds.

pub type TimeDelta = f64;

/// Stack entry for runtime errors and assertions.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CallFrame<'a> {
    /// JavaScript function name.
    #[serde(rename = "functionName")]
    pub function_name: Cow<'a, str>,
    /// JavaScript script id.
    #[serde(rename = "scriptId")]
    pub script_id: ScriptId<'a>,
    /// JavaScript script name or url.
    pub url: Cow<'a, str>,
    /// JavaScript script line number (0-based).
    #[serde(rename = "lineNumber")]
    pub line_number: i64,
    /// JavaScript script column number (0-based).
    #[serde(rename = "columnNumber")]
    pub column_number: i64,
}
/// Call frames for assertions or error messages.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct StackTrace<'a> {
    /// String label of this stack trace. For async traces this may be a name of the function that
    /// initiated the async call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Cow<'a, str>>,
    /// JavaScript function name.
    #[serde(rename = "callFrames")]
    pub call_frames: Vec<CallFrame<'a>>,
    /// Asynchronous JavaScript stack trace that preceded this stack, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Box<StackTrace<'a>>>,
    /// Asynchronous JavaScript stack trace that preceded this stack, if available.
    #[serde(skip_serializing_if = "Option::is_none", rename = "parentId")]
    pub parent_id: Option<StackTraceId<'a>>,
}
/// Unique identifier of current debugger.

pub type UniqueDebuggerId<'a> = Cow<'a, str>;

/// If 'debuggerId' is set stack trace comes from another debugger and can be resolved there. This
/// allows to track cross-debugger calls. See 'Runtime.StackTrace' and 'Debugger.paused' for usages.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct StackTraceId<'a> {
    pub id: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "debuggerId")]
    pub debugger_id: Option<UniqueDebuggerId<'a>>,
}
/// Add handler to promise with given promise object id.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Runtime.awaitPromise", response = "AwaitPromiseReturns<'a>")]
pub struct AwaitPromiseParams<'a> {
    /// Identifier of the promise.
    #[serde(rename = "promiseObjectId")]
    pub promise_object_id: RemoteObjectId<'a>,
    /// Whether the result is expected to be a JSON object that should be sent by value.
    #[serde(skip_serializing_if = "Option::is_none", rename = "returnByValue")]
    pub return_by_value: Option<bool>,
    /// Whether preview should be generated for the result.
    #[serde(skip_serializing_if = "Option::is_none", rename = "generatePreview")]
    pub generate_preview: Option<bool>,
}
/// Add handler to promise with given promise object id.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct AwaitPromiseReturns<'a> {
    /// Promise result. Will contain rejected value if promise was rejected.
    pub result: RemoteObject<'a>,
    /// Exception details if stack strace is available.
    #[serde(skip_serializing_if = "Option::is_none", rename = "exceptionDetails")]
    pub exception_details: Option<ExceptionDetails<'a>>,
}
/// Calls function with given declaration on the given object. Object group of the result is
/// inherited from the target object.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Runtime.callFunctionOn", response = "CallFunctionOnReturns<'a>")]
pub struct CallFunctionOnParams<'a> {
    /// Declaration of the function to call.
    #[serde(rename = "functionDeclaration")]
    pub function_declaration: Cow<'a, str>,
    /// Identifier of the object to call function on. Either objectId or executionContextId should
    /// be specified.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectId")]
    pub object_id: Option<RemoteObjectId<'a>>,
    /// Call arguments. All call arguments must belong to the same JavaScript world as the target
    /// object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<CallArgument<'a>>>,
    /// In silent mode exceptions thrown during evaluation are not reported and do not pause
    /// execution. Overrides 'setPauseOnException' state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent: Option<bool>,
    /// Whether the result is expected to be a JSON object which should be sent by value.
    /// Can be overriden by 'serializationOptions'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "returnByValue")]
    pub return_by_value: Option<bool>,
    /// Whether preview should be generated for the result.
    #[serde(skip_serializing_if = "Option::is_none", rename = "generatePreview")]
    pub generate_preview: Option<bool>,
    /// Whether execution should be treated as initiated by user in the UI.
    #[serde(skip_serializing_if = "Option::is_none", rename = "userGesture")]
    pub user_gesture: Option<bool>,
    /// Whether execution should 'await' for resulting value and return once awaited promise is
    /// resolved.
    #[serde(skip_serializing_if = "Option::is_none", rename = "awaitPromise")]
    pub await_promise: Option<bool>,
    /// Specifies execution context which global object will be used to call function on. Either
    /// executionContextId or objectId should be specified.
    #[serde(skip_serializing_if = "Option::is_none", rename = "executionContextId")]
    pub execution_context_id: Option<ExecutionContextId>,
    /// Symbolic group name that can be used to release multiple objects. If objectGroup is not
    /// specified and objectId is, objectGroup will be inherited from object.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectGroup")]
    pub object_group: Option<Cow<'a, str>>,
    /// Whether to throw an exception if side effect cannot be ruled out during evaluation.
    #[serde(skip_serializing_if = "Option::is_none", rename = "throwOnSideEffect")]
    pub throw_on_side_effect: Option<bool>,
    /// An alternative way to specify the execution context to call function on.
    /// Compared to contextId that may be reused across processes, this is guaranteed to be
    /// system-unique, so it can be used to prevent accidental function call
    /// in context different than intended (e.g. as a result of navigation across process
    /// boundaries).
    /// This is mutually exclusive with 'executionContextId'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "uniqueContextId")]
    pub unique_context_id: Option<Cow<'a, str>>,
    /// Specifies the result serialization. If provided, overrides
    /// 'generatePreview' and 'returnByValue'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "serializationOptions")]
    pub serialization_options: Option<SerializationOptions<'a>>,
}
/// Calls function with given declaration on the given object. Object group of the result is
/// inherited from the target object.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CallFunctionOnReturns<'a> {
    /// Call result.
    pub result: RemoteObject<'a>,
    /// Exception details.
    #[serde(skip_serializing_if = "Option::is_none", rename = "exceptionDetails")]
    pub exception_details: Option<ExceptionDetails<'a>>,
}
/// Compiles expression.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Runtime.compileScript", response = "CompileScriptReturns<'a>")]
pub struct CompileScriptParams<'a> {
    /// Expression to compile.
    pub expression: Cow<'a, str>,
    /// Source url to be set for the script.
    #[serde(rename = "sourceURL")]
    pub source_url: Cow<'a, str>,
    /// Specifies whether the compiled script should be persisted.
    #[serde(rename = "persistScript")]
    pub persist_script: bool,
    /// Specifies in which execution context to perform script run. If the parameter is omitted the
    /// evaluation will be performed in the context of the inspected page.
    #[serde(skip_serializing_if = "Option::is_none", rename = "executionContextId")]
    pub execution_context_id: Option<ExecutionContextId>,
}
/// Compiles expression.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CompileScriptReturns<'a> {
    /// Id of the script.
    #[serde(skip_serializing_if = "Option::is_none", rename = "scriptId")]
    pub script_id: Option<ScriptId<'a>>,
    /// Exception details.
    #[serde(skip_serializing_if = "Option::is_none", rename = "exceptionDetails")]
    pub exception_details: Option<ExceptionDetails<'a>>,
}
/// Disables reporting of execution contexts creation.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Runtime.disable")]
pub struct DisableParams {

}
/// Discards collected exceptions and console API calls.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Runtime.discardConsoleEntries")]
pub struct DiscardConsoleEntriesParams {

}
/// Enables reporting of execution contexts creation by means of 'executionContextCreated' event.
/// When the reporting gets enabled the event will be sent immediately for each existing execution
/// context.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Runtime.enable")]
pub struct EnableParams {

}
/// Evaluates expression on global object.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Runtime.evaluate", response = "EvaluateReturns<'a>")]
pub struct EvaluateParams<'a> {
    /// Expression to evaluate.
    pub expression: Cow<'a, str>,
    /// Symbolic group name that can be used to release multiple objects.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectGroup")]
    pub object_group: Option<Cow<'a, str>>,
    /// Determines whether Command Line API should be available during the evaluation.
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeCommandLineAPI")]
    pub include_command_line_api: Option<bool>,
    /// In silent mode exceptions thrown during evaluation are not reported and do not pause
    /// execution. Overrides 'setPauseOnException' state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent: Option<bool>,
    /// Specifies in which execution context to perform evaluation. If the parameter is omitted the
    /// evaluation will be performed in the context of the inspected page.
    /// This is mutually exclusive with 'uniqueContextId', which offers an
    /// alternative way to identify the execution context that is more reliable
    /// in a multi-process environment.
    #[serde(skip_serializing_if = "Option::is_none", rename = "contextId")]
    pub context_id: Option<ExecutionContextId>,
    /// Whether the result is expected to be a JSON object that should be sent by value.
    #[serde(skip_serializing_if = "Option::is_none", rename = "returnByValue")]
    pub return_by_value: Option<bool>,
    /// Whether preview should be generated for the result.
    #[serde(skip_serializing_if = "Option::is_none", rename = "generatePreview")]
    pub generate_preview: Option<bool>,
    /// Whether execution should be treated as initiated by user in the UI.
    #[serde(skip_serializing_if = "Option::is_none", rename = "userGesture")]
    pub user_gesture: Option<bool>,
    /// Whether execution should 'await' for resulting value and return once awaited promise is
    /// resolved.
    #[serde(skip_serializing_if = "Option::is_none", rename = "awaitPromise")]
    pub await_promise: Option<bool>,
    /// Whether to throw an exception if side effect cannot be ruled out during evaluation.
    /// This implies 'disableBreaks' below.
    #[serde(skip_serializing_if = "Option::is_none", rename = "throwOnSideEffect")]
    pub throw_on_side_effect: Option<bool>,
    /// Terminate execution after timing out (number of milliseconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<TimeDelta>,
    /// Disable breakpoints during execution.
    #[serde(skip_serializing_if = "Option::is_none", rename = "disableBreaks")]
    pub disable_breaks: Option<bool>,
    /// Setting this flag to true enables 'let' re-declaration and top-level 'await'.
    /// Note that 'let' variables can only be re-declared if they originate from
    /// 'replMode' themselves.
    #[serde(skip_serializing_if = "Option::is_none", rename = "replMode")]
    pub repl_mode: Option<bool>,
    /// The Content Security Policy (CSP) for the target might block 'unsafe-eval'
    /// which includes eval(), Function(), setTimeout() and setInterval()
    /// when called with non-callable arguments. This flag bypasses CSP for this
    /// evaluation and allows unsafe-eval. Defaults to true.
    #[serde(skip_serializing_if = "Option::is_none", rename = "allowUnsafeEvalBlockedByCSP")]
    pub allow_unsafe_eval_blocked_by_csp: Option<bool>,
    /// An alternative way to specify the execution context to evaluate in.
    /// Compared to contextId that may be reused across processes, this is guaranteed to be
    /// system-unique, so it can be used to prevent accidental evaluation of the expression
    /// in context different than intended (e.g. as a result of navigation across process
    /// boundaries).
    /// This is mutually exclusive with 'contextId'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "uniqueContextId")]
    pub unique_context_id: Option<Cow<'a, str>>,
    /// Specifies the result serialization. If provided, overrides
    /// 'generatePreview' and 'returnByValue'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "serializationOptions")]
    pub serialization_options: Option<SerializationOptions<'a>>,
}
/// Evaluates expression on global object.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateReturns<'a> {
    /// Evaluation result.
    pub result: RemoteObject<'a>,
    /// Exception details.
    #[serde(skip_serializing_if = "Option::is_none", rename = "exceptionDetails")]
    pub exception_details: Option<ExceptionDetails<'a>>,
}
/// Returns the isolate id.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Runtime.getIsolateId", response = "GetIsolateIdReturns<'a>")]
pub struct GetIsolateIdParams {

}
/// Returns the isolate id.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetIsolateIdReturns<'a> {
    /// The isolate id.
    pub id: Cow<'a, str>,
}
/// Returns the JavaScript heap usage.
/// It is the total usage of the corresponding isolate not scoped to a particular Runtime.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Runtime.getHeapUsage", response = "GetHeapUsageReturns")]
pub struct GetHeapUsageParams {

}
/// Returns the JavaScript heap usage.
/// It is the total usage of the corresponding isolate not scoped to a particular Runtime.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetHeapUsageReturns {
    /// Used JavaScript heap size in bytes.
    #[serde(rename = "usedSize")]
    pub used_size: f64,
    /// Allocated JavaScript heap size in bytes.
    #[serde(rename = "totalSize")]
    pub total_size: f64,
    /// Used size in bytes in the embedder's garbage-collected heap.
    #[serde(rename = "embedderHeapUsedSize")]
    pub embedder_heap_used_size: f64,
    /// Size in bytes of backing storage for array buffers and external strings.
    #[serde(rename = "backingStorageSize")]
    pub backing_storage_size: f64,
}
/// Returns properties of a given object. Object group of the result is inherited from the target
/// object.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Runtime.getProperties", response = "GetPropertiesReturns<'a>")]
pub struct GetPropertiesParams<'a> {
    /// Identifier of the object to return properties for.
    #[serde(rename = "objectId")]
    pub object_id: RemoteObjectId<'a>,
    /// If true, returns properties belonging only to the element itself, not to its prototype
    /// chain.
    #[serde(skip_serializing_if = "Option::is_none", rename = "ownProperties")]
    pub own_properties: Option<bool>,
    /// If true, returns accessor properties (with getter/setter) only; internal properties are not
    /// returned either.
    #[serde(skip_serializing_if = "Option::is_none", rename = "accessorPropertiesOnly")]
    pub accessor_properties_only: Option<bool>,
    /// Whether preview should be generated for the results.
    #[serde(skip_serializing_if = "Option::is_none", rename = "generatePreview")]
    pub generate_preview: Option<bool>,
    /// If true, returns non-indexed properties only.
    #[serde(skip_serializing_if = "Option::is_none", rename = "nonIndexedPropertiesOnly")]
    pub non_indexed_properties_only: Option<bool>,
}
/// Returns properties of a given object. Object group of the result is inherited from the target
/// object.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetPropertiesReturns<'a> {
    /// Object properties.
    pub result: Vec<PropertyDescriptor<'a>>,
    /// Internal object properties (only of the element itself).
    #[serde(skip_serializing_if = "Option::is_none", rename = "internalProperties")]
    pub internal_properties: Option<Vec<InternalPropertyDescriptor<'a>>>,
    /// Object private properties.
    #[serde(skip_serializing_if = "Option::is_none", rename = "privateProperties")]
    pub private_properties: Option<Vec<PrivatePropertyDescriptor<'a>>>,
    /// Exception details.
    #[serde(skip_serializing_if = "Option::is_none", rename = "exceptionDetails")]
    pub exception_details: Option<ExceptionDetails<'a>>,
}
/// Returns all let, const and class variables from global scope.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Runtime.globalLexicalScopeNames", response = "GlobalLexicalScopeNamesReturns<'a>")]
pub struct GlobalLexicalScopeNamesParams {
    /// Specifies in which execution context to lookup global scope variables.
    #[serde(skip_serializing_if = "Option::is_none", rename = "executionContextId")]
    pub execution_context_id: Option<ExecutionContextId>,
}
/// Returns all let, const and class variables from global scope.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GlobalLexicalScopeNamesReturns<'a> {
    pub names: Vec<Cow<'a, str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Runtime.queryObjects", response = "QueryObjectsReturns<'a>")]
pub struct QueryObjectsParams<'a> {
    /// Identifier of the prototype to return objects for.
    #[serde(rename = "prototypeObjectId")]
    pub prototype_object_id: RemoteObjectId<'a>,
    /// Symbolic group name that can be used to release the results.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectGroup")]
    pub object_group: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct QueryObjectsReturns<'a> {
    /// Array with objects.
    pub objects: RemoteObject<'a>,
}
/// Releases remote object with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Runtime.releaseObject")]
pub struct ReleaseObjectParams<'a> {
    /// Identifier of the object to release.
    #[serde(rename = "objectId")]
    pub object_id: RemoteObjectId<'a>,
}
/// Releases all remote objects that belong to a given group.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Runtime.releaseObjectGroup")]
pub struct ReleaseObjectGroupParams<'a> {
    /// Symbolic object group name.
    #[serde(rename = "objectGroup")]
    pub object_group: Cow<'a, str>,
}
/// Tells inspected instance to run if it was waiting for debugger to attach.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Runtime.runIfWaitingForDebugger")]
pub struct RunIfWaitingForDebuggerParams {

}
/// Runs script with given id in a given context.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Runtime.runScript", response = "RunScriptReturns<'a>")]
pub struct RunScriptParams<'a> {
    /// Id of the script to run.
    #[serde(rename = "scriptId")]
    pub script_id: ScriptId<'a>,
    /// Specifies in which execution context to perform script run. If the parameter is omitted the
    /// evaluation will be performed in the context of the inspected page.
    #[serde(skip_serializing_if = "Option::is_none", rename = "executionContextId")]
    pub execution_context_id: Option<ExecutionContextId>,
    /// Symbolic group name that can be used to release multiple objects.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectGroup")]
    pub object_group: Option<Cow<'a, str>>,
    /// In silent mode exceptions thrown during evaluation are not reported and do not pause
    /// execution. Overrides 'setPauseOnException' state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent: Option<bool>,
    /// Determines whether Command Line API should be available during the evaluation.
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeCommandLineAPI")]
    pub include_command_line_api: Option<bool>,
    /// Whether the result is expected to be a JSON object which should be sent by value.
    #[serde(skip_serializing_if = "Option::is_none", rename = "returnByValue")]
    pub return_by_value: Option<bool>,
    /// Whether preview should be generated for the result.
    #[serde(skip_serializing_if = "Option::is_none", rename = "generatePreview")]
    pub generate_preview: Option<bool>,
    /// Whether execution should 'await' for resulting value and return once awaited promise is
    /// resolved.
    #[serde(skip_serializing_if = "Option::is_none", rename = "awaitPromise")]
    pub await_promise: Option<bool>,
}
/// Runs script with given id in a given context.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RunScriptReturns<'a> {
    /// Run result.
    pub result: RemoteObject<'a>,
    /// Exception details.
    #[serde(skip_serializing_if = "Option::is_none", rename = "exceptionDetails")]
    pub exception_details: Option<ExceptionDetails<'a>>,
}
/// Enables or disables async call stacks tracking.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Runtime.setAsyncCallStackDepth")]
pub struct SetAsyncCallStackDepthParams {
    /// Maximum depth of async call stacks. Setting to '0' will effectively disable collecting async
    /// call stacks (default).
    #[serde(rename = "maxDepth")]
    pub max_depth: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Runtime.setCustomObjectFormatterEnabled")]
pub struct SetCustomObjectFormatterEnabledParams {
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Runtime.setMaxCallStackSizeToCapture")]
pub struct SetMaxCallStackSizeToCaptureParams {
    pub size: u64,
}
/// Terminate current or next JavaScript execution.
/// Will cancel the termination when the outer-most script execution ends.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Runtime.terminateExecution")]
pub struct TerminateExecutionParams {

}
/// If executionContextId is empty, adds binding with the given name on the
/// global objects of all inspected contexts, including those created later,
/// bindings survive reloads.
/// Binding function takes exactly one argument, this argument should be string,
/// in case of any other input, function throws an exception.
/// Each binding function call produces Runtime.bindingCalled notification.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Runtime.addBinding")]
pub struct AddBindingParams<'a> {
    pub name: Cow<'a, str>,
    /// If specified, the binding would only be exposed to the specified
    /// execution context. If omitted and 'executionContextName' is not set,
    /// the binding is exposed to all execution contexts of the target.
    /// This parameter is mutually exclusive with 'executionContextName'.
    /// Deprecated in favor of 'executionContextName' due to an unclear use case
    /// and bugs in implementation (crbug.com/1169639). 'executionContextId' will be
    /// removed in the future.
    #[serde(skip_serializing_if = "Option::is_none", rename = "executionContextId")]
    pub execution_context_id: Option<ExecutionContextId>,
    /// If specified, the binding is exposed to the executionContext with
    /// matching name, even for contexts created after the binding is added.
    /// See also 'ExecutionContext.name' and 'worldName' parameter to
    /// 'Page.addScriptToEvaluateOnNewDocument'.
    /// This parameter is mutually exclusive with 'executionContextId'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "executionContextName")]
    pub execution_context_name: Option<Cow<'a, str>>,
}
/// This method does not remove binding function from global object but
/// unsubscribes current runtime agent from Runtime.bindingCalled notifications.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Runtime.removeBinding")]
pub struct RemoveBindingParams<'a> {
    pub name: Cow<'a, str>,
}
/// This method tries to lookup and populate exception details for a
/// JavaScript Error object.
/// Note that the stackTrace portion of the resulting exceptionDetails will
/// only be populated if the Runtime domain was enabled at the time when the
/// Error was thrown.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Runtime.getExceptionDetails", response = "GetExceptionDetailsReturns<'a>")]
pub struct GetExceptionDetailsParams<'a> {
    /// The error object for which to resolve the exception details.
    #[serde(rename = "errorObjectId")]
    pub error_object_id: RemoteObjectId<'a>,
}
/// This method tries to lookup and populate exception details for a
/// JavaScript Error object.
/// Note that the stackTrace portion of the resulting exceptionDetails will
/// only be populated if the Runtime domain was enabled at the time when the
/// Error was thrown.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetExceptionDetailsReturns<'a> {
    #[serde(skip_serializing_if = "Option::is_none", rename = "exceptionDetails")]
    pub exception_details: Option<ExceptionDetails<'a>>,
}
/// Notification is issued every time when binding is called.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Runtime.bindingCalled")]
pub struct BindingCalled<'a> {
    pub name: Cow<'a, str>,
    pub payload: Cow<'a, str>,
    /// Identifier of the context where the call was made.
    #[serde(rename = "executionContextId")]
    pub execution_context_id: ExecutionContextId,
}
/// Issued when console API was called.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Runtime.consoleAPICalled")]
pub struct ConsoleAPICalled<'a> {
    /// Type of the call.
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
    /// Call arguments.
    pub args: Vec<RemoteObject<'a>>,
    /// Identifier of the context where the call was made.
    #[serde(rename = "executionContextId")]
    pub execution_context_id: ExecutionContextId,
    /// Call timestamp.
    pub timestamp: Timestamp,
    /// Stack trace captured when the call was made. The async stack chain is automatically reported for
    /// the following call types: 'assert', 'error', 'trace', 'warning'. For other types the async call
    /// chain can be retrieved using 'Debugger.getStackTrace' and 'stackTrace.parentId' field.
    #[serde(skip_serializing_if = "Option::is_none", rename = "stackTrace")]
    pub stack_trace: Option<StackTrace<'a>>,
    /// Console context descriptor for calls on non-default console context (not console.*):
    /// 'anonymous#unique-logger-id' for call on unnamed context, 'name#unique-logger-id' for call
    /// on named context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<Cow<'a, str>>,
}
/// Issued when unhandled exception was revoked.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Runtime.exceptionRevoked")]
pub struct ExceptionRevoked<'a> {
    /// Reason describing why exception was revoked.
    pub reason: Cow<'a, str>,
    /// The id of revoked exception, as reported in 'exceptionThrown'.
    #[serde(rename = "exceptionId")]
    pub exception_id: u64,
}
/// Issued when exception was thrown and unhandled.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Runtime.exceptionThrown")]
pub struct ExceptionThrown<'a> {
    /// Timestamp of the exception.
    pub timestamp: Timestamp,
    #[serde(rename = "exceptionDetails")]
    pub exception_details: ExceptionDetails<'a>,
}
/// Issued when new execution context is created.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Runtime.executionContextCreated")]
pub struct ExecutionContextCreated<'a> {
    /// A newly created execution context.
    pub context: ExecutionContextDescription<'a>,
}
/// Issued when execution context is destroyed.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Runtime.executionContextDestroyed")]
pub struct ExecutionContextDestroyed<'a> {
    /// Id of the destroyed context
    #[serde(rename = "executionContextId")]
    pub execution_context_id: ExecutionContextId,
    /// Unique Id of the destroyed context
    #[serde(rename = "executionContextUniqueId")]
    pub execution_context_unique_id: Cow<'a, str>,
}
/// Issued when all executionContexts were cleared in browser

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Runtime.executionContextsCleared")]
pub struct ExecutionContextsCleared {

}
/// Issued when object should be inspected (for example, as a result of inspect() command line API
/// call).

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Runtime.inspectRequested")]
pub struct InspectRequested<'a> {
    pub object: RemoteObject<'a>,
    pub hints: serde_json::Map<String, JsonValue>,
    /// Identifier of the context where the call was made.
    #[serde(skip_serializing_if = "Option::is_none", rename = "executionContextId")]
    pub execution_context_id: Option<ExecutionContextId>,
}