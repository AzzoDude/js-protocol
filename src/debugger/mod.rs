//! Debugger domain exposes JavaScript debugging capabilities. It allows setting and removing
//! breakpoints, stepping through execution, exploring stack traces, etc.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Breakpoint identifier.

pub type BreakpointId<'a> = Cow<'a, str>;

/// Call frame identifier.

pub type CallFrameId<'a> = Cow<'a, str>;

/// Location in the source code.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Location<'a> {
    /// Script identifier as reported in the 'Debugger.scriptParsed'.
    #[serde(rename = "scriptId")]
    pub script_id: crate::runtime::ScriptId<'a>,
    /// Line number in the script (0-based).
    #[serde(rename = "lineNumber")]
    pub line_number: i64,
    /// Column number in the script (0-based).
    #[serde(skip_serializing_if = "Option::is_none", rename = "columnNumber")]
    pub column_number: Option<i64>,
}
/// Location in the source code.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ScriptPosition {
    #[serde(rename = "lineNumber")]
    pub line_number: i64,
    #[serde(rename = "columnNumber")]
    pub column_number: i64,
}
/// Location range within one script.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct LocationRange<'a> {
    #[serde(rename = "scriptId")]
    pub script_id: crate::runtime::ScriptId<'a>,
    pub start: ScriptPosition,
    pub end: ScriptPosition,
}
/// JavaScript call frame. Array of call frames form the call stack.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CallFrame<'a> {
    /// Call frame identifier. This identifier is only valid while the virtual machine is paused.
    #[serde(rename = "callFrameId")]
    pub call_frame_id: CallFrameId<'a>,
    /// Name of the JavaScript function called on this call frame.
    #[serde(rename = "functionName")]
    pub function_name: Cow<'a, str>,
    /// Location in the source code.
    #[serde(skip_serializing_if = "Option::is_none", rename = "functionLocation")]
    pub function_location: Option<Location<'a>>,
    /// Location in the source code.
    pub location: Location<'a>,
    /// JavaScript script name or url.
    /// Deprecated in favor of using the 'location.scriptId' to resolve the URL via a previously
    /// sent 'Debugger.scriptParsed' event.
    pub url: Cow<'a, str>,
    /// Scope chain for this call frame.
    #[serde(rename = "scopeChain")]
    pub scope_chain: Vec<Scope<'a>>,
    /// 'this' object for this call frame.
    pub this: crate::runtime::RemoteObject<'a>,
    /// The value being returned, if the function is at return point.
    #[serde(skip_serializing_if = "Option::is_none", rename = "returnValue")]
    pub return_value: Option<crate::runtime::RemoteObject<'a>>,
    /// Valid only while the VM is paused and indicates whether this frame
    /// can be restarted or not. Note that a 'true' value here does not
    /// guarantee that Debugger#restartFrame with this CallFrameId will be
    /// successful, but it is very likely.
    #[serde(skip_serializing_if = "Option::is_none", rename = "canBeRestarted")]
    pub can_be_restarted: Option<bool>,
}
/// Scope description.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Scope<'a> {
    /// Scope type.
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
    /// Object representing the scope. For 'global' and 'with' scopes it represents the actual
    /// object; for the rest of the scopes, it is artificial transient object enumerating scope
    /// variables as its properties.
    pub object: crate::runtime::RemoteObject<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Cow<'a, str>>,
    /// Location in the source code where scope starts
    #[serde(skip_serializing_if = "Option::is_none", rename = "startLocation")]
    pub start_location: Option<Location<'a>>,
    /// Location in the source code where scope ends
    #[serde(skip_serializing_if = "Option::is_none", rename = "endLocation")]
    pub end_location: Option<Location<'a>>,
    /// Present if the scope has no variable values to show. Absent means that
    /// the scope declares at least one variable with an available value.
    /// Empty scopes are retained in the scope chain because
    /// they can be targeted via 'evaluateOnCallFrame' (using 'scopeNumber') or
    /// matched against scopes in source maps.
    #[serde(skip_serializing_if = "Option::is_none", rename = "emptyReason")]
    pub empty_reason: Option<Cow<'a, str>>,
}
/// Search match for resource.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SearchMatch<'a> {
    /// Line number in resource content.
    #[serde(rename = "lineNumber")]
    pub line_number: f64,
    /// Line with match content.
    #[serde(rename = "lineContent")]
    pub line_content: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct BreakLocation<'a> {
    /// Script identifier as reported in the 'Debugger.scriptParsed'.
    #[serde(rename = "scriptId")]
    pub script_id: crate::runtime::ScriptId<'a>,
    /// Line number in the script (0-based).
    #[serde(rename = "lineNumber")]
    pub line_number: i64,
    /// Column number in the script (0-based).
    #[serde(skip_serializing_if = "Option::is_none", rename = "columnNumber")]
    pub column_number: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct WasmDisassemblyChunk<'a> {
    /// The next chunk of disassembled lines.
    pub lines: Vec<Cow<'a, str>>,
    /// The bytecode offsets describing the start of each line.
    #[serde(rename = "bytecodeOffsets")]
    pub bytecode_offsets: Vec<i64>,
}
/// Enum of possible script languages.

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum ScriptLanguage {
    #[default]
    #[serde(rename = "JavaScript")]
    JavaScript,
    #[serde(rename = "WebAssembly")]
    WebAssembly,
}

/// Debug symbols available for a wasm script.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DebugSymbols<'a> {
    /// Type of the debug symbols.
    #[serde(rename = "type")]
    pub type_: Cow<'a, str>,
    /// URL of the external symbol source.
    #[serde(skip_serializing_if = "Option::is_none", rename = "externalURL")]
    pub external_url: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedBreakpoint<'a> {
    /// Breakpoint unique identifier.
    #[serde(rename = "breakpointId")]
    pub breakpoint_id: BreakpointId<'a>,
    /// Actual breakpoint location.
    pub location: Location<'a>,
}
/// Continues execution until specific location is reached.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.continueToLocation")]
pub struct ContinueToLocationParams<'a> {
    /// Location to continue to.
    pub location: Location<'a>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "targetCallFrames")]
    pub target_call_frames: Option<Cow<'a, str>>,
}
/// Disables debugger for given page.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.disable")]
pub struct DisableParams {

}
/// Enables debugger for the given page. Clients should not assume that the debugging has been
/// enabled until the result for this command is received.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.enable", response = "EnableReturns<'a>")]
pub struct EnableParams {
    /// The maximum size in bytes of collected scripts (not referenced by other heap objects)
    /// the debugger can hold. Puts no limit if parameter is omitted.
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxScriptsCacheSize")]
    pub max_scripts_cache_size: Option<f64>,
}
/// Enables debugger for the given page. Clients should not assume that the debugging has been
/// enabled until the result for this command is received.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturns<'a> {
    /// Unique identifier of the debugger.
    #[serde(rename = "debuggerId")]
    pub debugger_id: crate::runtime::UniqueDebuggerId<'a>,
}
/// Evaluates expression on a given call frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.evaluateOnCallFrame", response = "EvaluateOnCallFrameReturns<'a>")]
pub struct EvaluateOnCallFrameParams<'a> {
    /// Call frame identifier to evaluate on.
    #[serde(rename = "callFrameId")]
    pub call_frame_id: CallFrameId<'a>,
    /// Expression to evaluate.
    pub expression: Cow<'a, str>,
    /// String object group name to put result into (allows rapid releasing resulting object handles
    /// using 'releaseObjectGroup').
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectGroup")]
    pub object_group: Option<Cow<'a, str>>,
    /// Specifies whether command line API should be available to the evaluated expression, defaults
    /// to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeCommandLineAPI")]
    pub include_command_line_api: Option<bool>,
    /// In silent mode exceptions thrown during evaluation are not reported and do not pause
    /// execution. Overrides 'setPauseOnException' state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub silent: Option<bool>,
    /// Whether the result is expected to be a JSON object that should be sent by value.
    #[serde(skip_serializing_if = "Option::is_none", rename = "returnByValue")]
    pub return_by_value: Option<bool>,
    /// Whether preview should be generated for the result.
    #[serde(skip_serializing_if = "Option::is_none", rename = "generatePreview")]
    pub generate_preview: Option<bool>,
    /// Whether to throw an exception if side effect cannot be ruled out during evaluation.
    #[serde(skip_serializing_if = "Option::is_none", rename = "throwOnSideEffect")]
    pub throw_on_side_effect: Option<bool>,
    /// Terminate execution after timing out (number of milliseconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<crate::runtime::TimeDelta>,
    /// Specifies the scope number to evaluate the expression in (default: 0, innermost scope).
    #[serde(skip_serializing_if = "Option::is_none", rename = "scopeNumber")]
    pub scope_number: Option<i64>,
}
/// Evaluates expression on a given call frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateOnCallFrameReturns<'a> {
    /// Object wrapper for the evaluation result.
    pub result: crate::runtime::RemoteObject<'a>,
    /// Exception details.
    #[serde(skip_serializing_if = "Option::is_none", rename = "exceptionDetails")]
    pub exception_details: Option<crate::runtime::ExceptionDetails<'a>>,
}
/// Returns possible locations for breakpoint. scriptId in start and end range locations should be
/// the same.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.getPossibleBreakpoints", response = "GetPossibleBreakpointsReturns<'a>")]
pub struct GetPossibleBreakpointsParams<'a> {
    /// Start of range to search possible breakpoint locations in.
    pub start: Location<'a>,
    /// End of range to search possible breakpoint locations in (excluding). When not specified, end
    /// of scripts is used as end of range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<Location<'a>>,
    /// Only consider locations which are in the same (non-nested) function as start.
    #[serde(skip_serializing_if = "Option::is_none", rename = "restrictToFunction")]
    pub restrict_to_function: Option<bool>,
}
/// Returns possible locations for breakpoint. scriptId in start and end range locations should be
/// the same.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetPossibleBreakpointsReturns<'a> {
    /// List of the possible breakpoint locations.
    pub locations: Vec<BreakLocation<'a>>,
}
/// Returns source for the script with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.getScriptSource", response = "GetScriptSourceReturns<'a>")]
pub struct GetScriptSourceParams<'a> {
    /// Id of the script to get source for.
    #[serde(rename = "scriptId")]
    pub script_id: crate::runtime::ScriptId<'a>,
}
/// Returns source for the script with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetScriptSourceReturns<'a> {
    /// Script source (empty in case of Wasm bytecode).
    #[serde(rename = "scriptSource")]
    pub script_source: Cow<'a, str>,
    /// Wasm bytecode. (Encoded as a base64 string when passed over JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytecode: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.disassembleWasmModule", response = "DisassembleWasmModuleReturns<'a>")]
pub struct DisassembleWasmModuleParams<'a> {
    /// Id of the script to disassemble
    #[serde(rename = "scriptId")]
    pub script_id: crate::runtime::ScriptId<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct DisassembleWasmModuleReturns<'a> {
    /// For large modules, return a stream from which additional chunks of
    /// disassembly can be read successively.
    #[serde(skip_serializing_if = "Option::is_none", rename = "streamId")]
    pub stream_id: Option<Cow<'a, str>>,
    /// The total number of lines in the disassembly text.
    #[serde(rename = "totalNumberOfLines")]
    pub total_number_of_lines: i64,
    /// The offsets of all function bodies, in the format \[start1, end1,
    /// start2, end2, ...\] where all ends are exclusive.
    #[serde(rename = "functionBodyOffsets")]
    pub function_body_offsets: Vec<i64>,
    /// The first chunk of disassembly.
    pub chunk: WasmDisassemblyChunk<'a>,
}
/// Disassemble the next chunk of lines for the module corresponding to the
/// stream. If disassembly is complete, this API will invalidate the streamId
/// and return an empty chunk. Any subsequent calls for the now invalid stream
/// will return errors.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.nextWasmDisassemblyChunk", response = "NextWasmDisassemblyChunkReturns<'a>")]
pub struct NextWasmDisassemblyChunkParams<'a> {
    #[serde(rename = "streamId")]
    pub stream_id: Cow<'a, str>,
}
/// Disassemble the next chunk of lines for the module corresponding to the
/// stream. If disassembly is complete, this API will invalidate the streamId
/// and return an empty chunk. Any subsequent calls for the now invalid stream
/// will return errors.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct NextWasmDisassemblyChunkReturns<'a> {
    /// The next chunk of disassembly.
    pub chunk: WasmDisassemblyChunk<'a>,
}
/// This command is deprecated. Use getScriptSource instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.getWasmBytecode", response = "GetWasmBytecodeReturns<'a>")]
pub struct GetWasmBytecodeParams<'a> {
    /// Id of the Wasm script to get source for.
    #[serde(rename = "scriptId")]
    pub script_id: crate::runtime::ScriptId<'a>,
}
/// This command is deprecated. Use getScriptSource instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetWasmBytecodeReturns<'a> {
    /// Script source. (Encoded as a base64 string when passed over JSON)
    pub bytecode: Cow<'a, str>,
}
/// Returns stack trace with given 'stackTraceId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.getStackTrace", response = "GetStackTraceReturns<'a>")]
pub struct GetStackTraceParams<'a> {
    #[serde(rename = "stackTraceId")]
    pub stack_trace_id: crate::runtime::StackTraceId<'a>,
}
/// Returns stack trace with given 'stackTraceId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetStackTraceReturns<'a> {
    #[serde(rename = "stackTrace")]
    pub stack_trace: crate::runtime::StackTrace<'a>,
}
/// Stops on the next JavaScript statement.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.pause")]
pub struct PauseParams {

}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.pauseOnAsyncCall")]
pub struct PauseOnAsyncCallParams<'a> {
    /// Debugger will pause when async call with given stack trace is started.
    #[serde(rename = "parentStackTraceId")]
    pub parent_stack_trace_id: crate::runtime::StackTraceId<'a>,
}
/// Removes JavaScript breakpoint.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.removeBreakpoint")]
pub struct RemoveBreakpointParams<'a> {
    #[serde(rename = "breakpointId")]
    pub breakpoint_id: BreakpointId<'a>,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.restartFrame", response = "RestartFrameReturns<'a>")]
pub struct RestartFrameParams<'a> {
    /// Call frame identifier to evaluate on.
    #[serde(rename = "callFrameId")]
    pub call_frame_id: CallFrameId<'a>,
    /// The 'mode' parameter must be present and set to 'StepInto', otherwise
    /// 'restartFrame' will error out.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<Cow<'a, str>>,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct RestartFrameReturns<'a> {
    /// New stack trace.
    #[serde(rename = "callFrames")]
    pub call_frames: Vec<CallFrame<'a>>,
    /// Async stack trace, if any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "asyncStackTrace")]
    pub async_stack_trace: Option<crate::runtime::StackTrace<'a>>,
    /// Async stack trace, if any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "asyncStackTraceId")]
    pub async_stack_trace_id: Option<crate::runtime::StackTraceId<'a>>,
}
/// Resumes JavaScript execution.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.resume")]
pub struct ResumeParams {
    /// Set to true to terminate execution upon resuming execution. In contrast
    /// to Runtime.terminateExecution, this will allows to execute further
    /// JavaScript (i.e. via evaluation) until execution of the paused code
    /// is actually resumed, at which point termination is triggered.
    /// If execution is currently not paused, this parameter has no effect.
    #[serde(skip_serializing_if = "Option::is_none", rename = "terminateOnResume")]
    pub terminate_on_resume: Option<bool>,
}
/// Searches for given string in script content.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.searchInContent", response = "SearchInContentReturns<'a>")]
pub struct SearchInContentParams<'a> {
    /// Id of the script to search in.
    #[serde(rename = "scriptId")]
    pub script_id: crate::runtime::ScriptId<'a>,
    /// String to search for.
    pub query: Cow<'a, str>,
    /// If true, search is case sensitive.
    #[serde(skip_serializing_if = "Option::is_none", rename = "caseSensitive")]
    pub case_sensitive: Option<bool>,
    /// If true, treats string parameter as regex.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isRegex")]
    pub is_regex: Option<bool>,
}
/// Searches for given string in script content.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SearchInContentReturns<'a> {
    /// List of search matches.
    pub result: Vec<SearchMatch<'a>>,
}
/// Enables or disables async call stacks tracking.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.setAsyncCallStackDepth")]
pub struct SetAsyncCallStackDepthParams {
    /// Maximum depth of async call stacks. Setting to '0' will effectively disable collecting async
    /// call stacks (default).
    #[serde(rename = "maxDepth")]
    pub max_depth: i64,
}
/// Replace previous blackbox execution contexts with passed ones. Forces backend to skip
/// stepping/pausing in scripts in these execution contexts. VM will try to leave blackboxed script by
/// performing 'step in' several times, finally resorting to 'step out' if unsuccessful.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.setBlackboxExecutionContexts")]
pub struct SetBlackboxExecutionContextsParams<'a> {
    /// Array of execution context unique ids for the debugger to ignore.
    #[serde(rename = "uniqueIds")]
    pub unique_ids: Vec<Cow<'a, str>>,
}
/// Replace previous blackbox patterns with passed ones. Forces backend to skip stepping/pausing in
/// scripts with url matching one of the patterns. VM will try to leave blackboxed script by
/// performing 'step in' several times, finally resorting to 'step out' if unsuccessful.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.setBlackboxPatterns")]
pub struct SetBlackboxPatternsParams<'a> {
    /// Array of regexps that will be used to check script url for blackbox state.
    pub patterns: Vec<Cow<'a, str>>,
    /// If true, also ignore scripts with no source url.
    #[serde(skip_serializing_if = "Option::is_none", rename = "skipAnonymous")]
    pub skip_anonymous: Option<bool>,
}
/// Makes backend skip steps in the script in blackboxed ranges. VM will try leave blacklisted
/// scripts by performing 'step in' several times, finally resorting to 'step out' if unsuccessful.
/// Positions array contains positions where blackbox state is changed. First interval isn't
/// blackboxed. Array should be sorted.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.setBlackboxedRanges")]
pub struct SetBlackboxedRangesParams<'a> {
    /// Id of the script.
    #[serde(rename = "scriptId")]
    pub script_id: crate::runtime::ScriptId<'a>,
    pub positions: Vec<ScriptPosition>,
}
/// Sets JavaScript breakpoint at a given location.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.setBreakpoint", response = "SetBreakpointReturns<'a>")]
pub struct SetBreakpointParams<'a> {
    /// Location to set breakpoint in.
    pub location: Location<'a>,
    /// Expression to use as a breakpoint condition. When specified, debugger will only stop on the
    /// breakpoint if this expression evaluates to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Cow<'a, str>>,
}
/// Sets JavaScript breakpoint at a given location.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakpointReturns<'a> {
    /// Id of the created breakpoint for further reference.
    #[serde(rename = "breakpointId")]
    pub breakpoint_id: BreakpointId<'a>,
    /// Location this breakpoint resolved into.
    #[serde(rename = "actualLocation")]
    pub actual_location: Location<'a>,
}
/// Sets instrumentation breakpoint.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.setInstrumentationBreakpoint", response = "SetInstrumentationBreakpointReturns<'a>")]
pub struct SetInstrumentationBreakpointParams<'a> {
    /// Instrumentation name.
    pub instrumentation: Cow<'a, str>,
}
/// Sets instrumentation breakpoint.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SetInstrumentationBreakpointReturns<'a> {
    /// Id of the created breakpoint for further reference.
    #[serde(rename = "breakpointId")]
    pub breakpoint_id: BreakpointId<'a>,
}
/// Sets JavaScript breakpoint at given location specified either by URL or URL regex. Once this
/// command is issued, all existing parsed scripts will have breakpoints resolved and returned in
/// 'locations' property. Further matching script parsing will result in subsequent
/// 'breakpointResolved' events issued. This logical breakpoint will survive page reloads.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.setBreakpointByUrl", response = "SetBreakpointByUrlReturns<'a>")]
pub struct SetBreakpointByUrlParams<'a> {
    /// Line number to set breakpoint at.
    #[serde(rename = "lineNumber")]
    pub line_number: i64,
    /// URL of the resources to set breakpoint on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<Cow<'a, str>>,
    /// Regex pattern for the URLs of the resources to set breakpoints on. Either 'url' or
    /// 'urlRegex' must be specified.
    #[serde(skip_serializing_if = "Option::is_none", rename = "urlRegex")]
    pub url_regex: Option<Cow<'a, str>>,
    /// Script hash of the resources to set breakpoint on.
    #[serde(skip_serializing_if = "Option::is_none", rename = "scriptHash")]
    pub script_hash: Option<Cow<'a, str>>,
    /// Offset in the line to set breakpoint at.
    #[serde(skip_serializing_if = "Option::is_none", rename = "columnNumber")]
    pub column_number: Option<i64>,
    /// Expression to use as a breakpoint condition. When specified, debugger will only stop on the
    /// breakpoint if this expression evaluates to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Cow<'a, str>>,
}
/// Sets JavaScript breakpoint at given location specified either by URL or URL regex. Once this
/// command is issued, all existing parsed scripts will have breakpoints resolved and returned in
/// 'locations' property. Further matching script parsing will result in subsequent
/// 'breakpointResolved' events issued. This logical breakpoint will survive page reloads.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakpointByUrlReturns<'a> {
    /// Id of the created breakpoint for further reference.
    #[serde(rename = "breakpointId")]
    pub breakpoint_id: BreakpointId<'a>,
    /// List of the locations this breakpoint resolved into upon addition.
    pub locations: Vec<Location<'a>>,
}
/// Sets JavaScript breakpoint before each call to the given function.
/// If another function was created from the same source as a given one,
/// calling it will also trigger the breakpoint.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.setBreakpointOnFunctionCall", response = "SetBreakpointOnFunctionCallReturns<'a>")]
pub struct SetBreakpointOnFunctionCallParams<'a> {
    /// Function object id.
    #[serde(rename = "objectId")]
    pub object_id: crate::runtime::RemoteObjectId<'a>,
    /// Expression to use as a breakpoint condition. When specified, debugger will
    /// stop on the breakpoint if this expression evaluates to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Cow<'a, str>>,
}
/// Sets JavaScript breakpoint before each call to the given function.
/// If another function was created from the same source as a given one,
/// calling it will also trigger the breakpoint.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakpointOnFunctionCallReturns<'a> {
    /// Id of the created breakpoint for further reference.
    #[serde(rename = "breakpointId")]
    pub breakpoint_id: BreakpointId<'a>,
}
/// Activates / deactivates all breakpoints on the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.setBreakpointsActive")]
pub struct SetBreakpointsActiveParams {
    /// New value for breakpoints active state.
    pub active: bool,
}
/// Defines pause on exceptions state. Can be set to stop on all exceptions, uncaught exceptions,
/// or caught exceptions, no exceptions. Initial pause on exceptions state is 'none'.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.setPauseOnExceptions")]
pub struct SetPauseOnExceptionsParams<'a> {
    /// Pause on exceptions mode.
    pub state: Cow<'a, str>,
}
/// Changes return value in top frame. Available only at return break position.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.setReturnValue")]
pub struct SetReturnValueParams<'a> {
    /// New return value.
    #[serde(rename = "newValue")]
    pub new_value: crate::runtime::CallArgument<'a>,
}
/// Live edit is no longer supported and this command always fails with a "no longer available" error.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.setScriptSource", response = "SetScriptSourceReturns<'a>")]
pub struct SetScriptSourceParams<'a> {
    /// Id of the script to edit.
    #[serde(rename = "scriptId")]
    pub script_id: crate::runtime::ScriptId<'a>,
    /// New content of the script.
    #[serde(rename = "scriptSource")]
    pub script_source: Cow<'a, str>,
    /// If true the change will not actually be applied. Dry run may be used to get result
    /// description without actually modifying the code.
    #[serde(skip_serializing_if = "Option::is_none", rename = "dryRun")]
    pub dry_run: Option<bool>,
    /// If true, then 'scriptSource' is allowed to change the function on top of the stack
    /// as long as the top-most stack frame is the only activation of that function.
    #[serde(skip_serializing_if = "Option::is_none", rename = "allowTopFrameEditing")]
    pub allow_top_frame_editing: Option<bool>,
}
/// Live edit is no longer supported and this command always fails with a "no longer available" error.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SetScriptSourceReturns<'a> {
    /// New stack trace in case editing has happened while VM was stopped.
    #[serde(skip_serializing_if = "Option::is_none", rename = "callFrames")]
    pub call_frames: Option<Vec<CallFrame<'a>>>,
    /// Whether current call stack  was modified after applying the changes.
    #[serde(skip_serializing_if = "Option::is_none", rename = "stackChanged")]
    pub stack_changed: Option<bool>,
    /// Async stack trace, if any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "asyncStackTrace")]
    pub async_stack_trace: Option<crate::runtime::StackTrace<'a>>,
    /// Async stack trace, if any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "asyncStackTraceId")]
    pub async_stack_trace_id: Option<crate::runtime::StackTraceId<'a>>,
    /// Whether the operation was successful or not. Only 'Ok' denotes a
    /// successful live edit while the other enum variants denote why
    /// the live edit failed.
    pub status: Cow<'a, str>,
    /// Exception details if any. Only present when 'status' is 'CompileError'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "exceptionDetails")]
    pub exception_details: Option<crate::runtime::ExceptionDetails<'a>>,
}
/// Makes page not interrupt on any pauses (breakpoint, exception, dom exception etc).

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.setSkipAllPauses")]
pub struct SetSkipAllPausesParams {
    /// New value for skip pauses state.
    pub skip: bool,
}
/// Changes value of variable in a callframe. Object-based scopes are not supported and must be
/// mutated manually.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.setVariableValue")]
pub struct SetVariableValueParams<'a> {
    /// 0-based number of scope as was listed in scope chain. Only 'local', 'closure' and 'catch'
    /// scope types are allowed. Other scopes could be manipulated manually.
    #[serde(rename = "scopeNumber")]
    pub scope_number: i64,
    /// Variable name.
    #[serde(rename = "variableName")]
    pub variable_name: Cow<'a, str>,
    /// New variable value.
    #[serde(rename = "newValue")]
    pub new_value: crate::runtime::CallArgument<'a>,
    /// Id of callframe that holds variable.
    #[serde(rename = "callFrameId")]
    pub call_frame_id: CallFrameId<'a>,
}
/// Steps into the function call.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.stepInto")]
pub struct StepIntoParams<'a> {
    /// Debugger will pause on the execution of the first async task which was scheduled
    /// before next pause.
    #[serde(skip_serializing_if = "Option::is_none", rename = "breakOnAsyncCall")]
    pub break_on_async_call: Option<bool>,
    /// The skipList specifies location ranges that should be skipped on step into.
    #[serde(skip_serializing_if = "Option::is_none", rename = "skipList")]
    pub skip_list: Option<Vec<LocationRange<'a>>>,
}
/// Steps out of the function call.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.stepOut")]
pub struct StepOutParams {

}
/// Steps over the statement.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.stepOver")]
pub struct StepOverParams<'a> {
    /// The skipList specifies location ranges that should be skipped on step over.
    #[serde(skip_serializing_if = "Option::is_none", rename = "skipList")]
    pub skip_list: Option<Vec<LocationRange<'a>>>,
}
/// Fired when breakpoint is resolved to an actual script and location.
/// Deprecated in favor of 'resolvedBreakpoints' in the 'scriptParsed' event.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.breakpointResolved")]
pub struct BreakpointResolved<'a> {
    /// Breakpoint unique identifier.
    #[serde(rename = "breakpointId")]
    pub breakpoint_id: BreakpointId<'a>,
    /// Actual breakpoint location.
    pub location: Location<'a>,
}
/// Fired when the virtual machine stopped on breakpoint or exception or any other stop criteria.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.paused")]
pub struct Paused<'a> {
    /// Call stack the virtual machine stopped on.
    #[serde(rename = "callFrames")]
    pub call_frames: Vec<CallFrame<'a>>,
    /// Pause reason.
    pub reason: Cow<'a, str>,
    /// Object containing break-specific auxiliary properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Map<String, JsonValue>>,
    /// Hit breakpoints IDs
    #[serde(skip_serializing_if = "Option::is_none", rename = "hitBreakpoints")]
    pub hit_breakpoints: Option<Vec<Cow<'a, str>>>,
    /// Async stack trace, if any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "asyncStackTrace")]
    pub async_stack_trace: Option<crate::runtime::StackTrace<'a>>,
    /// Async stack trace, if any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "asyncStackTraceId")]
    pub async_stack_trace_id: Option<crate::runtime::StackTraceId<'a>>,
    /// Never present, will be removed.
    #[serde(skip_serializing_if = "Option::is_none", rename = "asyncCallStackTraceId")]
    pub async_call_stack_trace_id: Option<crate::runtime::StackTraceId<'a>>,
}
/// Fired when the virtual machine resumed execution.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.resumed")]
pub struct Resumed {

}
/// Fired when virtual machine fails to parse the script.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.scriptFailedToParse")]
pub struct ScriptFailedToParse<'a> {
    /// Identifier of the script parsed.
    #[serde(rename = "scriptId")]
    pub script_id: crate::runtime::ScriptId<'a>,
    /// URL or name of the script parsed (if any).
    pub url: Cow<'a, str>,
    /// Line offset of the script within the resource with given URL (for script tags).
    #[serde(rename = "startLine")]
    pub start_line: i64,
    /// Column offset of the script within the resource with given URL.
    #[serde(rename = "startColumn")]
    pub start_column: i64,
    /// Last line of the script.
    #[serde(rename = "endLine")]
    pub end_line: i64,
    /// Length of the last line of the script.
    #[serde(rename = "endColumn")]
    pub end_column: i64,
    /// Specifies script creation context.
    #[serde(rename = "executionContextId")]
    pub execution_context_id: crate::runtime::ExecutionContextId,
    /// Content hash of the script, SHA-256.
    pub hash: Cow<'a, str>,
    /// For Wasm modules, the content of the 'build_id' custom section. For JavaScript the 'debugId' magic comment.
    #[serde(rename = "buildId")]
    pub build_id: Cow<'a, str>,
    /// Embedder-specific auxiliary data likely matching {isDefault: boolean, type: 'default'|'isolated'|'worker', frameId: string}
    #[serde(skip_serializing_if = "Option::is_none", rename = "executionContextAuxData")]
    pub execution_context_aux_data: Option<serde_json::Map<String, JsonValue>>,
    /// URL of source map associated with script (if any).
    #[serde(skip_serializing_if = "Option::is_none", rename = "sourceMapURL")]
    pub source_map_url: Option<Cow<'a, str>>,
    /// True, if this script has sourceURL.
    #[serde(skip_serializing_if = "Option::is_none", rename = "hasSourceURL")]
    pub has_source_url: Option<bool>,
    /// True, if this script is ES6 module.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isModule")]
    pub is_module: Option<bool>,
    /// This script length.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<u64>,
    /// JavaScript top stack frame of where the script parsed event was triggered if available.
    #[serde(skip_serializing_if = "Option::is_none", rename = "stackTrace")]
    pub stack_trace: Option<crate::runtime::StackTrace<'a>>,
    /// If the scriptLanguage is WebAssembly, the code section offset in the module.
    #[serde(skip_serializing_if = "Option::is_none", rename = "codeOffset")]
    pub code_offset: Option<i32>,
    /// The language of the script.
    #[serde(skip_serializing_if = "Option::is_none", rename = "scriptLanguage")]
    pub script_language: Option<crate::debugger::ScriptLanguage>,
    /// The name the embedder supplied for this script.
    #[serde(skip_serializing_if = "Option::is_none", rename = "embedderName")]
    pub embedder_name: Option<Cow<'a, str>>,
}
/// Fired when virtual machine parses script. This event is also fired for all known and uncollected
/// scripts upon enabling debugger.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Debugger.scriptParsed")]
pub struct ScriptParsed<'a> {
    /// Identifier of the script parsed.
    #[serde(rename = "scriptId")]
    pub script_id: crate::runtime::ScriptId<'a>,
    /// URL or name of the script parsed (if any).
    pub url: Cow<'a, str>,
    /// Line offset of the script within the resource with given URL (for script tags).
    #[serde(rename = "startLine")]
    pub start_line: i64,
    /// Column offset of the script within the resource with given URL.
    #[serde(rename = "startColumn")]
    pub start_column: i64,
    /// Last line of the script.
    #[serde(rename = "endLine")]
    pub end_line: i64,
    /// Length of the last line of the script.
    #[serde(rename = "endColumn")]
    pub end_column: i64,
    /// Specifies script creation context.
    #[serde(rename = "executionContextId")]
    pub execution_context_id: crate::runtime::ExecutionContextId,
    /// Content hash of the script, SHA-256.
    pub hash: Cow<'a, str>,
    /// For Wasm modules, the content of the 'build_id' custom section. For JavaScript the 'debugId' magic comment.
    #[serde(rename = "buildId")]
    pub build_id: Cow<'a, str>,
    /// Embedder-specific auxiliary data likely matching {isDefault: boolean, type: 'default'|'isolated'|'worker', frameId: string}
    #[serde(skip_serializing_if = "Option::is_none", rename = "executionContextAuxData")]
    pub execution_context_aux_data: Option<serde_json::Map<String, JsonValue>>,
    /// True, if this script is generated as a result of the live edit operation.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isLiveEdit")]
    pub is_live_edit: Option<bool>,
    /// URL of source map associated with script (if any).
    #[serde(skip_serializing_if = "Option::is_none", rename = "sourceMapURL")]
    pub source_map_url: Option<Cow<'a, str>>,
    /// True, if this script has sourceURL.
    #[serde(skip_serializing_if = "Option::is_none", rename = "hasSourceURL")]
    pub has_source_url: Option<bool>,
    /// True, if this script is ES6 module.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isModule")]
    pub is_module: Option<bool>,
    /// This script length.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<u64>,
    /// JavaScript top stack frame of where the script parsed event was triggered if available.
    #[serde(skip_serializing_if = "Option::is_none", rename = "stackTrace")]
    pub stack_trace: Option<crate::runtime::StackTrace<'a>>,
    /// If the scriptLanguage is WebAssembly, the code section offset in the module.
    #[serde(skip_serializing_if = "Option::is_none", rename = "codeOffset")]
    pub code_offset: Option<i32>,
    /// The language of the script.
    #[serde(skip_serializing_if = "Option::is_none", rename = "scriptLanguage")]
    pub script_language: Option<crate::debugger::ScriptLanguage>,
    /// If the scriptLanguage is WebAssembly, the source of debug symbols for the module.
    #[serde(skip_serializing_if = "Option::is_none", rename = "debugSymbols")]
    pub debug_symbols: Option<Vec<crate::debugger::DebugSymbols<'a>>>,
    /// The name the embedder supplied for this script.
    #[serde(skip_serializing_if = "Option::is_none", rename = "embedderName")]
    pub embedder_name: Option<Cow<'a, str>>,
    /// The list of set breakpoints in this script if calls to 'setBreakpointByUrl'
    /// matches this script's URL or hash. Clients that use this list can ignore the
    /// 'breakpointResolved' event. They are equivalent.
    #[serde(skip_serializing_if = "Option::is_none", rename = "resolvedBreakpoints")]
    pub resolved_breakpoints: Option<Vec<ResolvedBreakpoint<'a>>>,
}