use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

//! Runtime domain exposes JavaScript runtime by means of remote evaluation and mirror objects.
//! Evaluation results are returned as mirror object that expose object type, string representation
//! and unique identifier that can be used for further object reference. Original objects are
//! maintained in memory unless they are either explicitly released or are released along with the
//! other objects in their object group.

/// Unique script identifier.

pub type ScriptId = String;

/// Represents options for serialization. Overrides 'generatePreview' and 'returnByValue'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SerializationOptions {

    pub serialization: String,
    /// Deep serialization depth. Default is full depth. Respected only in 'deep' serialization mode.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxDepth: Option<i64>,
    /// Embedder-specific parameters. For example if connected to V8 in Chrome these control DOM
    /// serialization via 'maxNodeDepth: integer' and 'includeShadowTree: "none" | "open" | "all"'.
    /// Values can be only of type string or integer.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub additionalParameters: Option<serde_json::Map<String, JsonValue>>,
}

/// Represents deep serialized value.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeepSerializedValue {

    #[serde(rename = "type")]
    pub type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<JsonValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub objectId: Option<String>,
    /// Set if value reference met more then once during serialization. In such
    /// case, value is provided only to one of the serialized values. Unique
    /// per value in the scope of one CDP call.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub weakLocalObjectReference: Option<i64>,
}

/// Unique object identifier.

pub type RemoteObjectId = String;

/// Primitive value which cannot be JSON-stringified. Includes values '-0', 'NaN', 'Infinity',
/// '-Infinity', and bigint literals.

pub type UnserializableValue = String;

/// Mirror object referencing original JavaScript object.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoteObject {
    /// Object type.

    #[serde(rename = "type")]
    pub type_: String,
    /// Object subtype hint. Specified for 'object' type values only.
    /// NOTE: If you change anything here, make sure to also update
    /// 'subtype' in 'ObjectPreview' and 'PropertyPreview' below.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
    /// Object class (constructor) name. Specified for 'object' type values only.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub className: Option<String>,
    /// Remote object value in case of primitive values or JSON values (if it was requested).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<JsonValue>,
    /// Primitive value which can not be JSON-stringified does not have 'value', but gets this
    /// property.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unserializableValue: Option<UnserializableValue>,
    /// String representation of the object.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Deep serialized value.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deepSerializedValue: Option<DeepSerializedValue>,
    /// Unique object identifier (for non-primitive values).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub objectId: Option<RemoteObjectId>,
    /// Preview containing abbreviated property values. Specified for 'object' type values only.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview: Option<ObjectPreview>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customPreview: Option<CustomPreview>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CustomPreview {
    /// The JSON-stringified result of formatter.header(object, config) call.
    /// It contains json ML array that represents RemoteObject.

    pub header: String,
    /// If formatter returns true as a result of formatter.hasBody call then bodyGetterId will
    /// contain RemoteObjectId for the function that returns result of formatter.body(object, config) call.
    /// The result value is json ML array.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bodyGetterId: Option<RemoteObjectId>,
}

/// Object containing abbreviated remote object value.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ObjectPreview {
    /// Object type.

    #[serde(rename = "type")]
    pub type_: String,
    /// Object subtype hint. Specified for 'object' type values only.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
    /// String representation of the object.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// True iff some of the properties or entries of the original object did not fit.

    pub overflow: bool,
    /// List of the properties.

    pub properties: Vec<PropertyPreview>,
    /// List of the entries. Specified for 'map' and 'set' subtype values only.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<Vec<EntryPreview>>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PropertyPreview {
    /// Property name.

    pub name: String,
    /// Object type. Accessor means that the property itself is an accessor property.

    #[serde(rename = "type")]
    pub type_: String,
    /// User-friendly property value string.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Nested value preview.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub valuePreview: Option<ObjectPreview>,
    /// Object subtype hint. Specified for 'object' type values only.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtype: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EntryPreview {
    /// Preview of the key. Specified for map-like collection entries.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<ObjectPreview>,
    /// Preview of the value.

    pub value: ObjectPreview,
}

/// Object property descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PropertyDescriptor {
    /// Property name or symbol description.

    pub name: String,
    /// The value associated with the property.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<RemoteObject>,
    /// True if the value associated with the property may be changed (data descriptors only).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub writable: Option<bool>,
    /// A function which serves as a getter for the property, or 'undefined' if there is no getter
    /// (accessor descriptors only).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<RemoteObject>,
    /// A function which serves as a setter for the property, or 'undefined' if there is no setter
    /// (accessor descriptors only).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<RemoteObject>,
    /// True if the type of this property descriptor may be changed and if the property may be
    /// deleted from the corresponding object.

    pub configurable: bool,
    /// True if this property shows up during enumeration of the properties on the corresponding
    /// object.

    pub enumerable: bool,
    /// True if the result was thrown during the evaluation.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wasThrown: Option<bool>,
    /// True if the property is owned for the object.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub isOwn: Option<bool>,
    /// Property symbol object, if the property is of the 'symbol' type.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<RemoteObject>,
}

/// Object internal property descriptor. This property isn't normally visible in JavaScript code.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct InternalPropertyDescriptor {
    /// Conventional property name.

    pub name: String,
    /// The value associated with the property.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<RemoteObject>,
}

/// Object private field descriptor.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PrivatePropertyDescriptor {
    /// Private property name.

    pub name: String,
    /// The value associated with the private property.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<RemoteObject>,
    /// A function which serves as a getter for the private property,
    /// or 'undefined' if there is no getter (accessor descriptors only).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<RemoteObject>,
    /// A function which serves as a setter for the private property,
    /// or 'undefined' if there is no setter (accessor descriptors only).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub set: Option<RemoteObject>,
}

/// Represents function call argument. Either remote object id 'objectId', primitive 'value',
/// unserializable primitive value or neither of (for undefined) them should be specified.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CallArgument {
    /// Primitive value or serializable javascript object.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<JsonValue>,
    /// Primitive value which can not be JSON-stringified.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unserializableValue: Option<UnserializableValue>,
    /// Remote object handle.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub objectId: Option<RemoteObjectId>,
}

/// Id of an execution context.

pub type ExecutionContextId = i64;

/// Description of an isolated world.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionContextDescription {
    /// Unique id of the execution context. It can be used to specify in which execution context
    /// script evaluation should be performed.

    pub id: ExecutionContextId,
    /// Execution context origin.

    pub origin: String,
    /// Human readable name describing given context.

    pub name: String,
    /// A system-unique execution context identifier. Unlike the id, this is unique across
    /// multiple processes, so can be reliably used to identify specific context while backend
    /// performs a cross-process navigation.

    pub uniqueId: String,
    /// Embedder-specific auxiliary data likely matching {isDefault: boolean, type: 'default'|'isolated'|'worker', frameId: string}

    #[serde(skip_serializing_if = "Option::is_none")]
    pub auxData: Option<serde_json::Map<String, JsonValue>>,
}

/// Detailed information about exception (or error) that was thrown during script compilation or
/// execution.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExceptionDetails {
    /// Exception id.

    pub exceptionId: u64,
    /// Exception text, which should be used together with exception object when available.

    pub text: String,
    /// Line number of the exception location (0-based).

    pub lineNumber: i64,
    /// Column number of the exception location (0-based).

    pub columnNumber: i64,
    /// Script ID of the exception location.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scriptId: Option<ScriptId>,
    /// URL of the exception location, to be used when the script was not reported.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// JavaScript stack trace if available.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stackTrace: Option<StackTrace>,
    /// Exception object if available.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exception: Option<RemoteObject>,
    /// Identifier of the context where exception happened.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub executionContextId: Option<ExecutionContextId>,
    /// Dictionary with entries of meta data that the client associated
    /// with this exception, such as information about associated network
    /// requests, etc.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exceptionMetaData: Option<serde_json::Map<String, JsonValue>>,
}

/// Number of milliseconds since epoch.

pub type Timestamp = f64;

/// Number of milliseconds.

pub type TimeDelta = f64;

/// Stack entry for runtime errors and assertions.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CallFrame {
    /// JavaScript function name.

    pub functionName: String,
    /// JavaScript script id.

    pub scriptId: ScriptId,
    /// JavaScript script name or url.

    pub url: String,
    /// JavaScript script line number (0-based).

    pub lineNumber: i64,
    /// JavaScript script column number (0-based).

    pub columnNumber: i64,
}

/// Call frames for assertions or error messages.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StackTrace {
    /// String label of this stack trace. For async traces this may be a name of the function that
    /// initiated the async call.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// JavaScript function name.

    pub callFrames: Vec<CallFrame>,
    /// Asynchronous JavaScript stack trace that preceded this stack, if available.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<Box<StackTrace>>,
    /// Asynchronous JavaScript stack trace that preceded this stack, if available.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parentId: Option<StackTraceId>,
}

/// Unique identifier of current debugger.

pub type UniqueDebuggerId = String;

/// If 'debuggerId' is set stack trace comes from another debugger and can be resolved there. This
/// allows to track cross-debugger calls. See 'Runtime.StackTrace' and 'Debugger.paused' for usages.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StackTraceId {

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub debuggerId: Option<UniqueDebuggerId>,
}

/// Add handler to promise with given promise object id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AwaitPromiseParams {
    /// Identifier of the promise.

    pub promiseObjectId: RemoteObjectId,
    /// Whether the result is expected to be a JSON object that should be sent by value.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub returnByValue: Option<bool>,
    /// Whether preview should be generated for the result.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub generatePreview: Option<bool>,
}

/// Add handler to promise with given promise object id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AwaitPromiseReturns {
    /// Promise result. Will contain rejected value if promise was rejected.

    pub result: RemoteObject,
    /// Exception details if stack strace is available.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exceptionDetails: Option<ExceptionDetails>,
}

impl AwaitPromiseParams { pub const METHOD: &'static str = "Runtime.awaitPromise"; }

impl crate::CdpCommand for AwaitPromiseParams {
    const METHOD: &'static str = "Runtime.awaitPromise";
    type Response = AwaitPromiseReturns;
}

/// Calls function with given declaration on the given object. Object group of the result is
/// inherited from the target object.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CallFunctionOnParams {
    /// Declaration of the function to call.

    pub functionDeclaration: String,
    /// Identifier of the object to call function on. Either objectId or executionContextId should
    /// be specified.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub objectId: Option<RemoteObjectId>,
    /// Call arguments. All call arguments must belong to the same JavaScript world as the target
    /// object.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<CallArgument>>,
    /// In silent mode exceptions thrown during evaluation are not reported and do not pause
    /// execution. Overrides 'setPauseOnException' state.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent: Option<bool>,
    /// Whether the result is expected to be a JSON object which should be sent by value.
    /// Can be overriden by 'serializationOptions'.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub returnByValue: Option<bool>,
    /// Whether preview should be generated for the result.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub generatePreview: Option<bool>,
    /// Whether execution should be treated as initiated by user in the UI.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub userGesture: Option<bool>,
    /// Whether execution should 'await' for resulting value and return once awaited promise is
    /// resolved.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub awaitPromise: Option<bool>,
    /// Specifies execution context which global object will be used to call function on. Either
    /// executionContextId or objectId should be specified.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub executionContextId: Option<ExecutionContextId>,
    /// Symbolic group name that can be used to release multiple objects. If objectGroup is not
    /// specified and objectId is, objectGroup will be inherited from object.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub objectGroup: Option<String>,
    /// Whether to throw an exception if side effect cannot be ruled out during evaluation.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub throwOnSideEffect: Option<bool>,
    /// An alternative way to specify the execution context to call function on.
    /// Compared to contextId that may be reused across processes, this is guaranteed to be
    /// system-unique, so it can be used to prevent accidental function call
    /// in context different than intended (e.g. as a result of navigation across process
    /// boundaries).
    /// This is mutually exclusive with 'executionContextId'.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uniqueContextId: Option<String>,
    /// Specifies the result serialization. If provided, overrides
    /// 'generatePreview' and 'returnByValue'.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub serializationOptions: Option<SerializationOptions>,
}

/// Calls function with given declaration on the given object. Object group of the result is
/// inherited from the target object.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CallFunctionOnReturns {
    /// Call result.

    pub result: RemoteObject,
    /// Exception details.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exceptionDetails: Option<ExceptionDetails>,
}

impl CallFunctionOnParams { pub const METHOD: &'static str = "Runtime.callFunctionOn"; }

impl crate::CdpCommand for CallFunctionOnParams {
    const METHOD: &'static str = "Runtime.callFunctionOn";
    type Response = CallFunctionOnReturns;
}

/// Compiles expression.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CompileScriptParams {
    /// Expression to compile.

    pub expression: String,
    /// Source url to be set for the script.

    pub sourceURL: String,
    /// Specifies whether the compiled script should be persisted.

    pub persistScript: bool,
    /// Specifies in which execution context to perform script run. If the parameter is omitted the
    /// evaluation will be performed in the context of the inspected page.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub executionContextId: Option<ExecutionContextId>,
}

/// Compiles expression.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CompileScriptReturns {
    /// Id of the script.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scriptId: Option<ScriptId>,
    /// Exception details.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exceptionDetails: Option<ExceptionDetails>,
}

impl CompileScriptParams { pub const METHOD: &'static str = "Runtime.compileScript"; }

impl crate::CdpCommand for CompileScriptParams {
    const METHOD: &'static str = "Runtime.compileScript";
    type Response = CompileScriptReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "Runtime.disable"; }

impl crate::CdpCommand for DisableParams {
    const METHOD: &'static str = "Runtime.disable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DiscardConsoleEntriesParams {}

impl DiscardConsoleEntriesParams { pub const METHOD: &'static str = "Runtime.discardConsoleEntries"; }

impl crate::CdpCommand for DiscardConsoleEntriesParams {
    const METHOD: &'static str = "Runtime.discardConsoleEntries";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams { pub const METHOD: &'static str = "Runtime.enable"; }

impl crate::CdpCommand for EnableParams {
    const METHOD: &'static str = "Runtime.enable";
    type Response = crate::EmptyReturns;
}

/// Evaluates expression on global object.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateParams {
    /// Expression to evaluate.

    pub expression: String,
    /// Symbolic group name that can be used to release multiple objects.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub objectGroup: Option<String>,
    /// Determines whether Command Line API should be available during the evaluation.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub includeCommandLineAPI: Option<bool>,
    /// In silent mode exceptions thrown during evaluation are not reported and do not pause
    /// execution. Overrides 'setPauseOnException' state.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent: Option<bool>,
    /// Specifies in which execution context to perform evaluation. If the parameter is omitted the
    /// evaluation will be performed in the context of the inspected page.
    /// This is mutually exclusive with 'uniqueContextId', which offers an
    /// alternative way to identify the execution context that is more reliable
    /// in a multi-process environment.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contextId: Option<ExecutionContextId>,
    /// Whether the result is expected to be a JSON object that should be sent by value.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub returnByValue: Option<bool>,
    /// Whether preview should be generated for the result.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub generatePreview: Option<bool>,
    /// Whether execution should be treated as initiated by user in the UI.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub userGesture: Option<bool>,
    /// Whether execution should 'await' for resulting value and return once awaited promise is
    /// resolved.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub awaitPromise: Option<bool>,
    /// Whether to throw an exception if side effect cannot be ruled out during evaluation.
    /// This implies 'disableBreaks' below.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub throwOnSideEffect: Option<bool>,
    /// Terminate execution after timing out (number of milliseconds).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<TimeDelta>,
    /// Disable breakpoints during execution.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub disableBreaks: Option<bool>,
    /// Setting this flag to true enables 'let' re-declaration and top-level 'await'.
    /// Note that 'let' variables can only be re-declared if they originate from
    /// 'replMode' themselves.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub replMode: Option<bool>,
    /// The Content Security Policy (CSP) for the target might block 'unsafe-eval'
    /// which includes eval(), Function(), setTimeout() and setInterval()
    /// when called with non-callable arguments. This flag bypasses CSP for this
    /// evaluation and allows unsafe-eval. Defaults to true.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowUnsafeEvalBlockedByCSP: Option<bool>,
    /// An alternative way to specify the execution context to evaluate in.
    /// Compared to contextId that may be reused across processes, this is guaranteed to be
    /// system-unique, so it can be used to prevent accidental evaluation of the expression
    /// in context different than intended (e.g. as a result of navigation across process
    /// boundaries).
    /// This is mutually exclusive with 'contextId'.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uniqueContextId: Option<String>,
    /// Specifies the result serialization. If provided, overrides
    /// 'generatePreview' and 'returnByValue'.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub serializationOptions: Option<SerializationOptions>,
}

/// Evaluates expression on global object.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateReturns {
    /// Evaluation result.

    pub result: RemoteObject,
    /// Exception details.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exceptionDetails: Option<ExceptionDetails>,
}

impl EvaluateParams { pub const METHOD: &'static str = "Runtime.evaluate"; }

impl crate::CdpCommand for EvaluateParams {
    const METHOD: &'static str = "Runtime.evaluate";
    type Response = EvaluateReturns;
}

/// Returns the isolate id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetIsolateIdReturns {
    /// The isolate id.

    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetIsolateIdParams {}

impl GetIsolateIdParams { pub const METHOD: &'static str = "Runtime.getIsolateId"; }

impl crate::CdpCommand for GetIsolateIdParams {
    const METHOD: &'static str = "Runtime.getIsolateId";
    type Response = GetIsolateIdReturns;
}

/// Returns the JavaScript heap usage.
/// It is the total usage of the corresponding isolate not scoped to a particular Runtime.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetHeapUsageReturns {
    /// Used JavaScript heap size in bytes.

    pub usedSize: f64,
    /// Allocated JavaScript heap size in bytes.

    pub totalSize: f64,
    /// Used size in bytes in the embedder's garbage-collected heap.

    pub embedderHeapUsedSize: f64,
    /// Size in bytes of backing storage for array buffers and external strings.

    pub backingStorageSize: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetHeapUsageParams {}

impl GetHeapUsageParams { pub const METHOD: &'static str = "Runtime.getHeapUsage"; }

impl crate::CdpCommand for GetHeapUsageParams {
    const METHOD: &'static str = "Runtime.getHeapUsage";
    type Response = GetHeapUsageReturns;
}

/// Returns properties of a given object. Object group of the result is inherited from the target
/// object.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPropertiesParams {
    /// Identifier of the object to return properties for.

    pub objectId: RemoteObjectId,
    /// If true, returns properties belonging only to the element itself, not to its prototype
    /// chain.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownProperties: Option<bool>,
    /// If true, returns accessor properties (with getter/setter) only; internal properties are not
    /// returned either.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub accessorPropertiesOnly: Option<bool>,
    /// Whether preview should be generated for the results.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub generatePreview: Option<bool>,
    /// If true, returns non-indexed properties only.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonIndexedPropertiesOnly: Option<bool>,
}

/// Returns properties of a given object. Object group of the result is inherited from the target
/// object.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPropertiesReturns {
    /// Object properties.

    pub result: Vec<PropertyDescriptor>,
    /// Internal object properties (only of the element itself).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub internalProperties: Option<Vec<InternalPropertyDescriptor>>,
    /// Object private properties.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub privateProperties: Option<Vec<PrivatePropertyDescriptor>>,
    /// Exception details.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exceptionDetails: Option<ExceptionDetails>,
}

impl GetPropertiesParams { pub const METHOD: &'static str = "Runtime.getProperties"; }

impl crate::CdpCommand for GetPropertiesParams {
    const METHOD: &'static str = "Runtime.getProperties";
    type Response = GetPropertiesReturns;
}

/// Returns all let, const and class variables from global scope.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GlobalLexicalScopeNamesParams {
    /// Specifies in which execution context to lookup global scope variables.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub executionContextId: Option<ExecutionContextId>,
}

/// Returns all let, const and class variables from global scope.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GlobalLexicalScopeNamesReturns {

    pub names: Vec<String>,
}

impl GlobalLexicalScopeNamesParams { pub const METHOD: &'static str = "Runtime.globalLexicalScopeNames"; }

impl crate::CdpCommand for GlobalLexicalScopeNamesParams {
    const METHOD: &'static str = "Runtime.globalLexicalScopeNames";
    type Response = GlobalLexicalScopeNamesReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryObjectsParams {
    /// Identifier of the prototype to return objects for.

    pub prototypeObjectId: RemoteObjectId,
    /// Symbolic group name that can be used to release the results.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub objectGroup: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct QueryObjectsReturns {
    /// Array with objects.

    pub objects: RemoteObject,
}

impl QueryObjectsParams { pub const METHOD: &'static str = "Runtime.queryObjects"; }

impl crate::CdpCommand for QueryObjectsParams {
    const METHOD: &'static str = "Runtime.queryObjects";
    type Response = QueryObjectsReturns;
}

/// Releases remote object with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseObjectParams {
    /// Identifier of the object to release.

    pub objectId: RemoteObjectId,
}

impl ReleaseObjectParams { pub const METHOD: &'static str = "Runtime.releaseObject"; }

impl crate::CdpCommand for ReleaseObjectParams {
    const METHOD: &'static str = "Runtime.releaseObject";
    type Response = crate::EmptyReturns;
}

/// Releases all remote objects that belong to a given group.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReleaseObjectGroupParams {
    /// Symbolic object group name.

    pub objectGroup: String,
}

impl ReleaseObjectGroupParams { pub const METHOD: &'static str = "Runtime.releaseObjectGroup"; }

impl crate::CdpCommand for ReleaseObjectGroupParams {
    const METHOD: &'static str = "Runtime.releaseObjectGroup";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RunIfWaitingForDebuggerParams {}

impl RunIfWaitingForDebuggerParams { pub const METHOD: &'static str = "Runtime.runIfWaitingForDebugger"; }

impl crate::CdpCommand for RunIfWaitingForDebuggerParams {
    const METHOD: &'static str = "Runtime.runIfWaitingForDebugger";
    type Response = crate::EmptyReturns;
}

/// Runs script with given id in a given context.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RunScriptParams {
    /// Id of the script to run.

    pub scriptId: ScriptId,
    /// Specifies in which execution context to perform script run. If the parameter is omitted the
    /// evaluation will be performed in the context of the inspected page.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub executionContextId: Option<ExecutionContextId>,
    /// Symbolic group name that can be used to release multiple objects.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub objectGroup: Option<String>,
    /// In silent mode exceptions thrown during evaluation are not reported and do not pause
    /// execution. Overrides 'setPauseOnException' state.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent: Option<bool>,
    /// Determines whether Command Line API should be available during the evaluation.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub includeCommandLineAPI: Option<bool>,
    /// Whether the result is expected to be a JSON object which should be sent by value.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub returnByValue: Option<bool>,
    /// Whether preview should be generated for the result.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub generatePreview: Option<bool>,
    /// Whether execution should 'await' for resulting value and return once awaited promise is
    /// resolved.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub awaitPromise: Option<bool>,
}

/// Runs script with given id in a given context.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RunScriptReturns {
    /// Run result.

    pub result: RemoteObject,
    /// Exception details.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exceptionDetails: Option<ExceptionDetails>,
}

impl RunScriptParams { pub const METHOD: &'static str = "Runtime.runScript"; }

impl crate::CdpCommand for RunScriptParams {
    const METHOD: &'static str = "Runtime.runScript";
    type Response = RunScriptReturns;
}

/// Enables or disables async call stacks tracking.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetAsyncCallStackDepthParams {
    /// Maximum depth of async call stacks. Setting to '0' will effectively disable collecting async
    /// call stacks (default).

    pub maxDepth: i64,
}

impl SetAsyncCallStackDepthParams { pub const METHOD: &'static str = "Runtime.setAsyncCallStackDepth"; }

impl crate::CdpCommand for SetAsyncCallStackDepthParams {
    const METHOD: &'static str = "Runtime.setAsyncCallStackDepth";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetCustomObjectFormatterEnabledParams {

    pub enabled: bool,
}

impl SetCustomObjectFormatterEnabledParams { pub const METHOD: &'static str = "Runtime.setCustomObjectFormatterEnabled"; }

impl crate::CdpCommand for SetCustomObjectFormatterEnabledParams {
    const METHOD: &'static str = "Runtime.setCustomObjectFormatterEnabled";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetMaxCallStackSizeToCaptureParams {

    pub size: u64,
}

impl SetMaxCallStackSizeToCaptureParams { pub const METHOD: &'static str = "Runtime.setMaxCallStackSizeToCapture"; }

impl crate::CdpCommand for SetMaxCallStackSizeToCaptureParams {
    const METHOD: &'static str = "Runtime.setMaxCallStackSizeToCapture";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerminateExecutionParams {}

impl TerminateExecutionParams { pub const METHOD: &'static str = "Runtime.terminateExecution"; }

impl crate::CdpCommand for TerminateExecutionParams {
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
pub struct AddBindingParams {

    pub name: String,
    /// If specified, the binding would only be exposed to the specified
    /// execution context. If omitted and 'executionContextName' is not set,
    /// the binding is exposed to all execution contexts of the target.
    /// This parameter is mutually exclusive with 'executionContextName'.
    /// Deprecated in favor of 'executionContextName' due to an unclear use case
    /// and bugs in implementation (crbug.com/1169639). 'executionContextId' will be
    /// removed in the future.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub executionContextId: Option<ExecutionContextId>,
    /// If specified, the binding is exposed to the executionContext with
    /// matching name, even for contexts created after the binding is added.
    /// See also 'ExecutionContext.name' and 'worldName' parameter to
    /// 'Page.addScriptToEvaluateOnNewDocument'.
    /// This parameter is mutually exclusive with 'executionContextId'.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub executionContextName: Option<String>,
}

impl AddBindingParams { pub const METHOD: &'static str = "Runtime.addBinding"; }

impl crate::CdpCommand for AddBindingParams {
    const METHOD: &'static str = "Runtime.addBinding";
    type Response = crate::EmptyReturns;
}

/// This method does not remove binding function from global object but
/// unsubscribes current runtime agent from Runtime.bindingCalled notifications.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveBindingParams {

    pub name: String,
}

impl RemoveBindingParams { pub const METHOD: &'static str = "Runtime.removeBinding"; }

impl crate::CdpCommand for RemoveBindingParams {
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
pub struct GetExceptionDetailsParams {
    /// The error object for which to resolve the exception details.

    pub errorObjectId: RemoteObjectId,
}

/// This method tries to lookup and populate exception details for a
/// JavaScript Error object.
/// Note that the stackTrace portion of the resulting exceptionDetails will
/// only be populated if the Runtime domain was enabled at the time when the
/// Error was thrown.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetExceptionDetailsReturns {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exceptionDetails: Option<ExceptionDetails>,
}

impl GetExceptionDetailsParams { pub const METHOD: &'static str = "Runtime.getExceptionDetails"; }

impl crate::CdpCommand for GetExceptionDetailsParams {
    const METHOD: &'static str = "Runtime.getExceptionDetails";
    type Response = GetExceptionDetailsReturns;
}
