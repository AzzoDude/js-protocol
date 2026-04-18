//! Debugger domain exposes JavaScript debugging capabilities. It allows setting and removing
//! breakpoints, stepping through execution, exploring stack traces, etc.
use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

/// Breakpoint identifier.

pub type BreakpointId = String;

/// Call frame identifier.

pub type CallFrameId = String;

/// Location in the source code.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    /// Script identifier as reported in the 'Debugger.scriptParsed'.

    pub scriptId: crate::runtime::ScriptId,
    /// Line number in the script (0-based).

    pub lineNumber: i64,
    /// Column number in the script (0-based).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub columnNumber: Option<i64>,
}

/// Location in the source code.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScriptPosition {

    pub lineNumber: i64,

    pub columnNumber: i64,
}

/// Location range within one script.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LocationRange {

    pub scriptId: crate::runtime::ScriptId,

    pub start: ScriptPosition,

    pub end: ScriptPosition,
}

/// JavaScript call frame. Array of call frames form the call stack.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CallFrame {
    /// Call frame identifier. This identifier is only valid while the virtual machine is paused.

    pub callFrameId: CallFrameId,
    /// Name of the JavaScript function called on this call frame.

    pub functionName: String,
    /// Location in the source code.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub functionLocation: Option<Location>,
    /// Location in the source code.

    pub location: Location,
    /// JavaScript script name or url.
    /// Deprecated in favor of using the 'location.scriptId' to resolve the URL via a previously
    /// sent 'Debugger.scriptParsed' event.

    pub url: String,
    /// Scope chain for this call frame.

    pub scopeChain: Vec<Scope>,
    /// 'this' object for this call frame.

    pub this: crate::runtime::RemoteObject,
    /// The value being returned, if the function is at return point.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub returnValue: Option<crate::runtime::RemoteObject>,
    /// Valid only while the VM is paused and indicates whether this frame
    /// can be restarted or not. Note that a 'true' value here does not
    /// guarantee that Debugger#restartFrame with this CallFrameId will be
    /// successful, but it is very likely.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub canBeRestarted: Option<bool>,
}

/// Scope description.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Scope {
    /// Scope type.

    #[serde(rename = "type")]
    pub type_: String,
    /// Object representing the scope. For 'global' and 'with' scopes it represents the actual
    /// object; for the rest of the scopes, it is artificial transient object enumerating scope
    /// variables as its properties.

    pub object: crate::runtime::RemoteObject,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Location in the source code where scope starts

    #[serde(skip_serializing_if = "Option::is_none")]
    pub startLocation: Option<Location>,
    /// Location in the source code where scope ends

    #[serde(skip_serializing_if = "Option::is_none")]
    pub endLocation: Option<Location>,
}

/// Search match for resource.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchMatch {
    /// Line number in resource content.

    pub lineNumber: f64,
    /// Line with match content.

    pub lineContent: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BreakLocation {
    /// Script identifier as reported in the 'Debugger.scriptParsed'.

    pub scriptId: crate::runtime::ScriptId,
    /// Line number in the script (0-based).

    pub lineNumber: i64,
    /// Column number in the script (0-based).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub columnNumber: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub type_: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WasmDisassemblyChunk {
    /// The next chunk of disassembled lines.

    pub lines: Vec<String>,
    /// The bytecode offsets describing the start of each line.

    pub bytecodeOffsets: Vec<i64>,
}

/// Enum of possible script languages.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ScriptLanguage {
    #[default]
    JavaScript,
    WebAssembly,
}

/// Debug symbols available for a wasm script.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DebugSymbols {
    /// Type of the debug symbols.

    #[serde(rename = "type")]
    pub type_: String,
    /// URL of the external symbol source.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub externalURL: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedBreakpoint {
    /// Breakpoint unique identifier.

    pub breakpointId: BreakpointId,
    /// Actual breakpoint location.

    pub location: Location,
}

/// Continues execution until specific location is reached.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContinueToLocationParams {
    /// Location to continue to.

    pub location: Location,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub targetCallFrames: Option<String>,
}

impl ContinueToLocationParams { pub const METHOD: &'static str = "Debugger.continueToLocation"; }

impl crate::CdpCommand for ContinueToLocationParams {
    const METHOD: &'static str = "Debugger.continueToLocation";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "Debugger.disable"; }

impl crate::CdpCommand for DisableParams {
    const METHOD: &'static str = "Debugger.disable";
    type Response = crate::EmptyReturns;
}

/// Enables debugger for the given page. Clients should not assume that the debugging has been
/// enabled until the result for this command is received.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnableParams {
    /// The maximum size in bytes of collected scripts (not referenced by other heap objects)
    /// the debugger can hold. Puts no limit if parameter is omitted.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxScriptsCacheSize: Option<f64>,
}

/// Enables debugger for the given page. Clients should not assume that the debugging has been
/// enabled until the result for this command is received.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturns {
    /// Unique identifier of the debugger.

    pub debuggerId: crate::runtime::UniqueDebuggerId,
}

impl EnableParams { pub const METHOD: &'static str = "Debugger.enable"; }

impl crate::CdpCommand for EnableParams {
    const METHOD: &'static str = "Debugger.enable";
    type Response = EnableReturns;
}

/// Evaluates expression on a given call frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateOnCallFrameParams {
    /// Call frame identifier to evaluate on.

    pub callFrameId: CallFrameId,
    /// Expression to evaluate.

    pub expression: String,
    /// String object group name to put result into (allows rapid releasing resulting object handles
    /// using 'releaseObjectGroup').

    #[serde(skip_serializing_if = "Option::is_none")]
    pub objectGroup: Option<String>,
    /// Specifies whether command line API should be available to the evaluated expression, defaults
    /// to false.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub includeCommandLineAPI: Option<bool>,
    /// In silent mode exceptions thrown during evaluation are not reported and do not pause
    /// execution. Overrides 'setPauseOnException' state.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent: Option<bool>,
    /// Whether the result is expected to be a JSON object that should be sent by value.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub returnByValue: Option<bool>,
    /// Whether preview should be generated for the result.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub generatePreview: Option<bool>,
    /// Whether to throw an exception if side effect cannot be ruled out during evaluation.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub throwOnSideEffect: Option<bool>,
    /// Terminate execution after timing out (number of milliseconds).

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<crate::runtime::TimeDelta>,
}

/// Evaluates expression on a given call frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateOnCallFrameReturns {
    /// Object wrapper for the evaluation result.

    pub result: crate::runtime::RemoteObject,
    /// Exception details.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exceptionDetails: Option<crate::runtime::ExceptionDetails>,
}

impl EvaluateOnCallFrameParams { pub const METHOD: &'static str = "Debugger.evaluateOnCallFrame"; }

impl crate::CdpCommand for EvaluateOnCallFrameParams {
    const METHOD: &'static str = "Debugger.evaluateOnCallFrame";
    type Response = EvaluateOnCallFrameReturns;
}

/// Returns possible locations for breakpoint. scriptId in start and end range locations should be
/// the same.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPossibleBreakpointsParams {
    /// Start of range to search possible breakpoint locations in.

    pub start: Location,
    /// End of range to search possible breakpoint locations in (excluding). When not specified, end
    /// of scripts is used as end of range.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<Location>,
    /// Only consider locations which are in the same (non-nested) function as start.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictToFunction: Option<bool>,
}

/// Returns possible locations for breakpoint. scriptId in start and end range locations should be
/// the same.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPossibleBreakpointsReturns {
    /// List of the possible breakpoint locations.

    pub locations: Vec<BreakLocation>,
}

impl GetPossibleBreakpointsParams { pub const METHOD: &'static str = "Debugger.getPossibleBreakpoints"; }

impl crate::CdpCommand for GetPossibleBreakpointsParams {
    const METHOD: &'static str = "Debugger.getPossibleBreakpoints";
    type Response = GetPossibleBreakpointsReturns;
}

/// Returns source for the script with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetScriptSourceParams {
    /// Id of the script to get source for.

    pub scriptId: crate::runtime::ScriptId,
}

/// Returns source for the script with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetScriptSourceReturns {
    /// Script source (empty in case of Wasm bytecode).

    pub scriptSource: String,
    /// Wasm bytecode. (Encoded as a base64 string when passed over JSON)

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytecode: Option<String>,
}

impl GetScriptSourceParams { pub const METHOD: &'static str = "Debugger.getScriptSource"; }

impl crate::CdpCommand for GetScriptSourceParams {
    const METHOD: &'static str = "Debugger.getScriptSource";
    type Response = GetScriptSourceReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DisassembleWasmModuleParams {
    /// Id of the script to disassemble

    pub scriptId: crate::runtime::ScriptId,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DisassembleWasmModuleReturns {
    /// For large modules, return a stream from which additional chunks of
    /// disassembly can be read successively.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub streamId: Option<String>,
    /// The total number of lines in the disassembly text.

    pub totalNumberOfLines: i64,
    /// The offsets of all function bodies, in the format [start1, end1,
    /// start2, end2, ...] where all ends are exclusive.

    pub functionBodyOffsets: Vec<i64>,
    /// The first chunk of disassembly.

    pub chunk: WasmDisassemblyChunk,
}

impl DisassembleWasmModuleParams { pub const METHOD: &'static str = "Debugger.disassembleWasmModule"; }

impl crate::CdpCommand for DisassembleWasmModuleParams {
    const METHOD: &'static str = "Debugger.disassembleWasmModule";
    type Response = DisassembleWasmModuleReturns;
}

/// Disassemble the next chunk of lines for the module corresponding to the
/// stream. If disassembly is complete, this API will invalidate the streamId
/// and return an empty chunk. Any subsequent calls for the now invalid stream
/// will return errors.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NextWasmDisassemblyChunkParams {

    pub streamId: String,
}

/// Disassemble the next chunk of lines for the module corresponding to the
/// stream. If disassembly is complete, this API will invalidate the streamId
/// and return an empty chunk. Any subsequent calls for the now invalid stream
/// will return errors.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NextWasmDisassemblyChunkReturns {
    /// The next chunk of disassembly.

    pub chunk: WasmDisassemblyChunk,
}

impl NextWasmDisassemblyChunkParams { pub const METHOD: &'static str = "Debugger.nextWasmDisassemblyChunk"; }

impl crate::CdpCommand for NextWasmDisassemblyChunkParams {
    const METHOD: &'static str = "Debugger.nextWasmDisassemblyChunk";
    type Response = NextWasmDisassemblyChunkReturns;
}

/// This command is deprecated. Use getScriptSource instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetWasmBytecodeParams {
    /// Id of the Wasm script to get source for.

    pub scriptId: crate::runtime::ScriptId,
}

/// This command is deprecated. Use getScriptSource instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetWasmBytecodeReturns {
    /// Script source. (Encoded as a base64 string when passed over JSON)

    pub bytecode: String,
}

impl GetWasmBytecodeParams { pub const METHOD: &'static str = "Debugger.getWasmBytecode"; }

impl crate::CdpCommand for GetWasmBytecodeParams {
    const METHOD: &'static str = "Debugger.getWasmBytecode";
    type Response = GetWasmBytecodeReturns;
}

/// Returns stack trace with given 'stackTraceId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetStackTraceParams {

    pub stackTraceId: crate::runtime::StackTraceId,
}

/// Returns stack trace with given 'stackTraceId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetStackTraceReturns {

    pub stackTrace: crate::runtime::StackTrace,
}

impl GetStackTraceParams { pub const METHOD: &'static str = "Debugger.getStackTrace"; }

impl crate::CdpCommand for GetStackTraceParams {
    const METHOD: &'static str = "Debugger.getStackTrace";
    type Response = GetStackTraceReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PauseParams {}

impl PauseParams { pub const METHOD: &'static str = "Debugger.pause"; }

impl crate::CdpCommand for PauseParams {
    const METHOD: &'static str = "Debugger.pause";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PauseOnAsyncCallParams {
    /// Debugger will pause when async call with given stack trace is started.

    pub parentStackTraceId: crate::runtime::StackTraceId,
}

impl PauseOnAsyncCallParams { pub const METHOD: &'static str = "Debugger.pauseOnAsyncCall"; }

impl crate::CdpCommand for PauseOnAsyncCallParams {
    const METHOD: &'static str = "Debugger.pauseOnAsyncCall";
    type Response = crate::EmptyReturns;
}

/// Removes JavaScript breakpoint.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveBreakpointParams {

    pub breakpointId: BreakpointId,
}

impl RemoveBreakpointParams { pub const METHOD: &'static str = "Debugger.removeBreakpoint"; }

impl crate::CdpCommand for RemoveBreakpointParams {
    const METHOD: &'static str = "Debugger.removeBreakpoint";
    type Response = crate::EmptyReturns;
}

/// Restarts particular call frame from the beginning. The old, deprecated
/// behavior of 'restartFrame' is to stay paused and allow further CDP commands
/// after a restart was scheduled. This can cause problems with restarting, so
/// we now continue execution immediatly after it has been scheduled until we
/// reach the beginning of the restarted frame.
/// 
/// To stay back-wards compatible, 'restartFrame' now expects a 'mode'
/// parameter to be present. If the 'mode' parameter is missing, 'restartFrame'
/// errors out.
/// 
/// The various return values are deprecated and 'callFrames' is always empty.
/// Use the call frames from the 'Debugger#paused' events instead, that fires
/// once V8 pauses at the beginning of the restarted function.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RestartFrameParams {
    /// Call frame identifier to evaluate on.

    pub callFrameId: CallFrameId,
    /// The 'mode' parameter must be present and set to 'StepInto', otherwise
    /// 'restartFrame' will error out.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

/// Restarts particular call frame from the beginning. The old, deprecated
/// behavior of 'restartFrame' is to stay paused and allow further CDP commands
/// after a restart was scheduled. This can cause problems with restarting, so
/// we now continue execution immediatly after it has been scheduled until we
/// reach the beginning of the restarted frame.
/// 
/// To stay back-wards compatible, 'restartFrame' now expects a 'mode'
/// parameter to be present. If the 'mode' parameter is missing, 'restartFrame'
/// errors out.
/// 
/// The various return values are deprecated and 'callFrames' is always empty.
/// Use the call frames from the 'Debugger#paused' events instead, that fires
/// once V8 pauses at the beginning of the restarted function.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RestartFrameReturns {
    /// New stack trace.

    pub callFrames: Vec<CallFrame>,
    /// Async stack trace, if any.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub asyncStackTrace: Option<crate::runtime::StackTrace>,
    /// Async stack trace, if any.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub asyncStackTraceId: Option<crate::runtime::StackTraceId>,
}

impl RestartFrameParams { pub const METHOD: &'static str = "Debugger.restartFrame"; }

impl crate::CdpCommand for RestartFrameParams {
    const METHOD: &'static str = "Debugger.restartFrame";
    type Response = RestartFrameReturns;
}

/// Resumes JavaScript execution.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResumeParams {
    /// Set to true to terminate execution upon resuming execution. In contrast
    /// to Runtime.terminateExecution, this will allows to execute further
    /// JavaScript (i.e. via evaluation) until execution of the paused code
    /// is actually resumed, at which point termination is triggered.
    /// If execution is currently not paused, this parameter has no effect.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminateOnResume: Option<bool>,
}

impl ResumeParams { pub const METHOD: &'static str = "Debugger.resume"; }

impl crate::CdpCommand for ResumeParams {
    const METHOD: &'static str = "Debugger.resume";
    type Response = crate::EmptyReturns;
}

/// Searches for given string in script content.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchInContentParams {
    /// Id of the script to search in.

    pub scriptId: crate::runtime::ScriptId,
    /// String to search for.

    pub query: String,
    /// If true, search is case sensitive.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub caseSensitive: Option<bool>,
    /// If true, treats string parameter as regex.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub isRegex: Option<bool>,
}

/// Searches for given string in script content.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchInContentReturns {
    /// List of search matches.

    pub result: Vec<SearchMatch>,
}

impl SearchInContentParams { pub const METHOD: &'static str = "Debugger.searchInContent"; }

impl crate::CdpCommand for SearchInContentParams {
    const METHOD: &'static str = "Debugger.searchInContent";
    type Response = SearchInContentReturns;
}

/// Enables or disables async call stacks tracking.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetAsyncCallStackDepthParams {
    /// Maximum depth of async call stacks. Setting to '0' will effectively disable collecting async
    /// call stacks (default).

    pub maxDepth: i64,
}

impl SetAsyncCallStackDepthParams { pub const METHOD: &'static str = "Debugger.setAsyncCallStackDepth"; }

impl crate::CdpCommand for SetAsyncCallStackDepthParams {
    const METHOD: &'static str = "Debugger.setAsyncCallStackDepth";
    type Response = crate::EmptyReturns;
}

/// Replace previous blackbox execution contexts with passed ones. Forces backend to skip
/// stepping/pausing in scripts in these execution contexts. VM will try to leave blackboxed script by
/// performing 'step in' several times, finally resorting to 'step out' if unsuccessful.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetBlackboxExecutionContextsParams {
    /// Array of execution context unique ids for the debugger to ignore.

    pub uniqueIds: Vec<String>,
}

impl SetBlackboxExecutionContextsParams { pub const METHOD: &'static str = "Debugger.setBlackboxExecutionContexts"; }

impl crate::CdpCommand for SetBlackboxExecutionContextsParams {
    const METHOD: &'static str = "Debugger.setBlackboxExecutionContexts";
    type Response = crate::EmptyReturns;
}

/// Replace previous blackbox patterns with passed ones. Forces backend to skip stepping/pausing in
/// scripts with url matching one of the patterns. VM will try to leave blackboxed script by
/// performing 'step in' several times, finally resorting to 'step out' if unsuccessful.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetBlackboxPatternsParams {
    /// Array of regexps that will be used to check script url for blackbox state.

    pub patterns: Vec<String>,
    /// If true, also ignore scripts with no source url.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipAnonymous: Option<bool>,
}

impl SetBlackboxPatternsParams { pub const METHOD: &'static str = "Debugger.setBlackboxPatterns"; }

impl crate::CdpCommand for SetBlackboxPatternsParams {
    const METHOD: &'static str = "Debugger.setBlackboxPatterns";
    type Response = crate::EmptyReturns;
}

/// Makes backend skip steps in the script in blackboxed ranges. VM will try leave blacklisted
/// scripts by performing 'step in' several times, finally resorting to 'step out' if unsuccessful.
/// Positions array contains positions where blackbox state is changed. First interval isn't
/// blackboxed. Array should be sorted.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetBlackboxedRangesParams {
    /// Id of the script.

    pub scriptId: crate::runtime::ScriptId,

    pub positions: Vec<ScriptPosition>,
}

impl SetBlackboxedRangesParams { pub const METHOD: &'static str = "Debugger.setBlackboxedRanges"; }

impl crate::CdpCommand for SetBlackboxedRangesParams {
    const METHOD: &'static str = "Debugger.setBlackboxedRanges";
    type Response = crate::EmptyReturns;
}

/// Sets JavaScript breakpoint at a given location.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakpointParams {
    /// Location to set breakpoint in.

    pub location: Location,
    /// Expression to use as a breakpoint condition. When specified, debugger will only stop on the
    /// breakpoint if this expression evaluates to true.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
}

/// Sets JavaScript breakpoint at a given location.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakpointReturns {
    /// Id of the created breakpoint for further reference.

    pub breakpointId: BreakpointId,
    /// Location this breakpoint resolved into.

    pub actualLocation: Location,
}

impl SetBreakpointParams { pub const METHOD: &'static str = "Debugger.setBreakpoint"; }

impl crate::CdpCommand for SetBreakpointParams {
    const METHOD: &'static str = "Debugger.setBreakpoint";
    type Response = SetBreakpointReturns;
}

/// Sets instrumentation breakpoint.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetInstrumentationBreakpointParams {
    /// Instrumentation name.

    pub instrumentation: String,
}

/// Sets instrumentation breakpoint.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetInstrumentationBreakpointReturns {
    /// Id of the created breakpoint for further reference.

    pub breakpointId: BreakpointId,
}

impl SetInstrumentationBreakpointParams { pub const METHOD: &'static str = "Debugger.setInstrumentationBreakpoint"; }

impl crate::CdpCommand for SetInstrumentationBreakpointParams {
    const METHOD: &'static str = "Debugger.setInstrumentationBreakpoint";
    type Response = SetInstrumentationBreakpointReturns;
}

/// Sets JavaScript breakpoint at given location specified either by URL or URL regex. Once this
/// command is issued, all existing parsed scripts will have breakpoints resolved and returned in
/// 'locations' property. Further matching script parsing will result in subsequent
/// 'breakpointResolved' events issued. This logical breakpoint will survive page reloads.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakpointByUrlParams {
    /// Line number to set breakpoint at.

    pub lineNumber: i64,
    /// URL of the resources to set breakpoint on.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// Regex pattern for the URLs of the resources to set breakpoints on. Either 'url' or
    /// 'urlRegex' must be specified.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub urlRegex: Option<String>,
    /// Script hash of the resources to set breakpoint on.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scriptHash: Option<String>,
    /// Offset in the line to set breakpoint at.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub columnNumber: Option<i64>,
    /// Expression to use as a breakpoint condition. When specified, debugger will only stop on the
    /// breakpoint if this expression evaluates to true.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
}

/// Sets JavaScript breakpoint at given location specified either by URL or URL regex. Once this
/// command is issued, all existing parsed scripts will have breakpoints resolved and returned in
/// 'locations' property. Further matching script parsing will result in subsequent
/// 'breakpointResolved' events issued. This logical breakpoint will survive page reloads.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakpointByUrlReturns {
    /// Id of the created breakpoint for further reference.

    pub breakpointId: BreakpointId,
    /// List of the locations this breakpoint resolved into upon addition.

    pub locations: Vec<Location>,
}

impl SetBreakpointByUrlParams { pub const METHOD: &'static str = "Debugger.setBreakpointByUrl"; }

impl crate::CdpCommand for SetBreakpointByUrlParams {
    const METHOD: &'static str = "Debugger.setBreakpointByUrl";
    type Response = SetBreakpointByUrlReturns;
}

/// Sets JavaScript breakpoint before each call to the given function.
/// If another function was created from the same source as a given one,
/// calling it will also trigger the breakpoint.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakpointOnFunctionCallParams {
    /// Function object id.

    pub objectId: crate::runtime::RemoteObjectId,
    /// Expression to use as a breakpoint condition. When specified, debugger will
    /// stop on the breakpoint if this expression evaluates to true.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
}

/// Sets JavaScript breakpoint before each call to the given function.
/// If another function was created from the same source as a given one,
/// calling it will also trigger the breakpoint.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakpointOnFunctionCallReturns {
    /// Id of the created breakpoint for further reference.

    pub breakpointId: BreakpointId,
}

impl SetBreakpointOnFunctionCallParams { pub const METHOD: &'static str = "Debugger.setBreakpointOnFunctionCall"; }

impl crate::CdpCommand for SetBreakpointOnFunctionCallParams {
    const METHOD: &'static str = "Debugger.setBreakpointOnFunctionCall";
    type Response = SetBreakpointOnFunctionCallReturns;
}

/// Activates / deactivates all breakpoints on the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakpointsActiveParams {
    /// New value for breakpoints active state.

    pub active: bool,
}

impl SetBreakpointsActiveParams { pub const METHOD: &'static str = "Debugger.setBreakpointsActive"; }

impl crate::CdpCommand for SetBreakpointsActiveParams {
    const METHOD: &'static str = "Debugger.setBreakpointsActive";
    type Response = crate::EmptyReturns;
}

/// Defines pause on exceptions state. Can be set to stop on all exceptions, uncaught exceptions,
/// or caught exceptions, no exceptions. Initial pause on exceptions state is 'none'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetPauseOnExceptionsParams {
    /// Pause on exceptions mode.

    pub state: String,
}

impl SetPauseOnExceptionsParams { pub const METHOD: &'static str = "Debugger.setPauseOnExceptions"; }

impl crate::CdpCommand for SetPauseOnExceptionsParams {
    const METHOD: &'static str = "Debugger.setPauseOnExceptions";
    type Response = crate::EmptyReturns;
}

/// Changes return value in top frame. Available only at return break position.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetReturnValueParams {
    /// New return value.

    pub newValue: crate::runtime::CallArgument,
}

impl SetReturnValueParams { pub const METHOD: &'static str = "Debugger.setReturnValue"; }

impl crate::CdpCommand for SetReturnValueParams {
    const METHOD: &'static str = "Debugger.setReturnValue";
    type Response = crate::EmptyReturns;
}

/// Edits JavaScript source live.
/// 
/// In general, functions that are currently on the stack can not be edited with
/// a single exception: If the edited function is the top-most stack frame and
/// that is the only activation of that function on the stack. In this case
/// the live edit will be successful and a 'Debugger.restartFrame' for the
/// top-most function is automatically triggered.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetScriptSourceParams {
    /// Id of the script to edit.

    pub scriptId: crate::runtime::ScriptId,
    /// New content of the script.

    pub scriptSource: String,
    /// If true the change will not actually be applied. Dry run may be used to get result
    /// description without actually modifying the code.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dryRun: Option<bool>,
    /// If true, then 'scriptSource' is allowed to change the function on top of the stack
    /// as long as the top-most stack frame is the only activation of that function.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowTopFrameEditing: Option<bool>,
}

/// Edits JavaScript source live.
/// 
/// In general, functions that are currently on the stack can not be edited with
/// a single exception: If the edited function is the top-most stack frame and
/// that is the only activation of that function on the stack. In this case
/// the live edit will be successful and a 'Debugger.restartFrame' for the
/// top-most function is automatically triggered.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetScriptSourceReturns {
    /// New stack trace in case editing has happened while VM was stopped.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub callFrames: Option<Vec<CallFrame>>,
    /// Whether current call stack  was modified after applying the changes.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stackChanged: Option<bool>,
    /// Async stack trace, if any.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub asyncStackTrace: Option<crate::runtime::StackTrace>,
    /// Async stack trace, if any.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub asyncStackTraceId: Option<crate::runtime::StackTraceId>,
    /// Whether the operation was successful or not. Only 'Ok' denotes a
    /// successful live edit while the other enum variants denote why
    /// the live edit failed.

    pub status: String,
    /// Exception details if any. Only present when 'status' is 'CompileError'.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exceptionDetails: Option<crate::runtime::ExceptionDetails>,
}

impl SetScriptSourceParams { pub const METHOD: &'static str = "Debugger.setScriptSource"; }

impl crate::CdpCommand for SetScriptSourceParams {
    const METHOD: &'static str = "Debugger.setScriptSource";
    type Response = SetScriptSourceReturns;
}

/// Makes page not interrupt on any pauses (breakpoint, exception, dom exception etc).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetSkipAllPausesParams {
    /// New value for skip pauses state.

    pub skip: bool,
}

impl SetSkipAllPausesParams { pub const METHOD: &'static str = "Debugger.setSkipAllPauses"; }

impl crate::CdpCommand for SetSkipAllPausesParams {
    const METHOD: &'static str = "Debugger.setSkipAllPauses";
    type Response = crate::EmptyReturns;
}

/// Changes value of variable in a callframe. Object-based scopes are not supported and must be
/// mutated manually.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableValueParams {
    /// 0-based number of scope as was listed in scope chain. Only 'local', 'closure' and 'catch'
    /// scope types are allowed. Other scopes could be manipulated manually.

    pub scopeNumber: i64,
    /// Variable name.

    pub variableName: String,
    /// New variable value.

    pub newValue: crate::runtime::CallArgument,
    /// Id of callframe that holds variable.

    pub callFrameId: CallFrameId,
}

impl SetVariableValueParams { pub const METHOD: &'static str = "Debugger.setVariableValue"; }

impl crate::CdpCommand for SetVariableValueParams {
    const METHOD: &'static str = "Debugger.setVariableValue";
    type Response = crate::EmptyReturns;
}

/// Steps into the function call.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StepIntoParams {
    /// Debugger will pause on the execution of the first async task which was scheduled
    /// before next pause.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakOnAsyncCall: Option<bool>,
    /// The skipList specifies location ranges that should be skipped on step into.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipList: Option<Vec<LocationRange>>,
}

impl StepIntoParams { pub const METHOD: &'static str = "Debugger.stepInto"; }

impl crate::CdpCommand for StepIntoParams {
    const METHOD: &'static str = "Debugger.stepInto";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StepOutParams {}

impl StepOutParams { pub const METHOD: &'static str = "Debugger.stepOut"; }

impl crate::CdpCommand for StepOutParams {
    const METHOD: &'static str = "Debugger.stepOut";
    type Response = crate::EmptyReturns;
}

/// Steps over the statement.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StepOverParams {
    /// The skipList specifies location ranges that should be skipped on step over.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipList: Option<Vec<LocationRange>>,
}

impl StepOverParams { pub const METHOD: &'static str = "Debugger.stepOver"; }

impl crate::CdpCommand for StepOverParams {
    const METHOD: &'static str = "Debugger.stepOver";
    type Response = crate::EmptyReturns;
}
