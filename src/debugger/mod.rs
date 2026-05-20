//! Debugger domain exposes JavaScript debugging capabilities. It allows setting and removing
//! breakpoints, stepping through execution, exploring stack traces, etc.


use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// Breakpoint identifier.

pub type BreakpointId<'a> = Cow<'a, str>;

/// Call frame identifier.

pub type CallFrameId<'a> = Cow<'a, str>;

/// Location in the source code.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Location<'a> {
    /// Script identifier as reported in the 'Debugger.scriptParsed'.
    #[serde(rename = "scriptId")]
    script_id: crate::runtime::ScriptId<'a>,
    /// Line number in the script (0-based).
    #[serde(rename = "lineNumber")]
    line_number: i64,
    /// Column number in the script (0-based).
    #[serde(skip_serializing_if = "Option::is_none", rename = "columnNumber")]
    column_number: Option<i64>,
}

impl<'a> Location<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `script_id`: Script identifier as reported in the `Debugger.scriptParsed`.
    /// * `line_number`: Line number in the script (0-based).
    pub fn builder(script_id: crate::runtime::ScriptId<'a>, line_number: i64) -> LocationBuilder<'a> {
        LocationBuilder {
            script_id: script_id,
            line_number: line_number,
            column_number: None,
        }
    }
    /// Script identifier as reported in the 'Debugger.scriptParsed'.
    pub fn script_id(&self) -> &crate::runtime::ScriptId<'a> { &self.script_id }
    /// Line number in the script (0-based).
    pub fn line_number(&self) -> i64 { self.line_number }
    /// Column number in the script (0-based).
    pub fn column_number(&self) -> Option<i64> { self.column_number }
}


pub struct LocationBuilder<'a> {
    script_id: crate::runtime::ScriptId<'a>,
    line_number: i64,
    column_number: Option<i64>,
}

impl<'a> LocationBuilder<'a> {
    /// Column number in the script (0-based).
    pub fn column_number(mut self, column_number: i64) -> Self { self.column_number = Some(column_number); self }
    pub fn build(self) -> Location<'a> {
        Location {
            script_id: self.script_id,
            line_number: self.line_number,
            column_number: self.column_number,
        }
    }
}

/// Location in the source code.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScriptPosition {
    #[serde(rename = "lineNumber")]
    line_number: i64,
    #[serde(rename = "columnNumber")]
    column_number: i64,
}

impl ScriptPosition {
    /// Creates a builder for this type with the required parameters:
    /// * `line_number`: 
    /// * `column_number`: 
    pub fn builder(line_number: i64, column_number: i64) -> ScriptPositionBuilder {
        ScriptPositionBuilder {
            line_number: line_number,
            column_number: column_number,
        }
    }
    pub fn line_number(&self) -> i64 { self.line_number }
    pub fn column_number(&self) -> i64 { self.column_number }
}


pub struct ScriptPositionBuilder {
    line_number: i64,
    column_number: i64,
}

impl ScriptPositionBuilder {
    pub fn build(self) -> ScriptPosition {
        ScriptPosition {
            line_number: self.line_number,
            column_number: self.column_number,
        }
    }
}

/// Location range within one script.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LocationRange<'a> {
    #[serde(rename = "scriptId")]
    script_id: crate::runtime::ScriptId<'a>,
    start: ScriptPosition,
    end: ScriptPosition,
}

impl<'a> LocationRange<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `script_id`: 
    /// * `start`: 
    /// * `end`: 
    pub fn builder(script_id: crate::runtime::ScriptId<'a>, start: ScriptPosition, end: ScriptPosition) -> LocationRangeBuilder<'a> {
        LocationRangeBuilder {
            script_id: script_id,
            start: start,
            end: end,
        }
    }
    pub fn script_id(&self) -> &crate::runtime::ScriptId<'a> { &self.script_id }
    pub fn start(&self) -> &ScriptPosition { &self.start }
    pub fn end(&self) -> &ScriptPosition { &self.end }
}


pub struct LocationRangeBuilder<'a> {
    script_id: crate::runtime::ScriptId<'a>,
    start: ScriptPosition,
    end: ScriptPosition,
}

impl<'a> LocationRangeBuilder<'a> {
    pub fn build(self) -> LocationRange<'a> {
        LocationRange {
            script_id: self.script_id,
            start: self.start,
            end: self.end,
        }
    }
}

/// JavaScript call frame. Array of call frames form the call stack.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CallFrame<'a> {
    /// Call frame identifier. This identifier is only valid while the virtual machine is paused.
    #[serde(rename = "callFrameId")]
    call_frame_id: CallFrameId<'a>,
    /// Name of the JavaScript function called on this call frame.
    #[serde(rename = "functionName")]
    function_name: Cow<'a, str>,
    /// Location in the source code.
    #[serde(skip_serializing_if = "Option::is_none", rename = "functionLocation")]
    function_location: Option<Location<'a>>,
    /// Location in the source code.
    location: Location<'a>,
    /// JavaScript script name or url.
    /// Deprecated in favor of using the 'location.scriptId' to resolve the URL via a previously
    /// sent 'Debugger.scriptParsed' event.
    url: Cow<'a, str>,
    /// Scope chain for this call frame.
    #[serde(rename = "scopeChain")]
    scope_chain: Vec<Scope<'a>>,
    /// 'this' object for this call frame.
    this: crate::runtime::RemoteObject<'a>,
    /// The value being returned, if the function is at return point.
    #[serde(skip_serializing_if = "Option::is_none", rename = "returnValue")]
    return_value: Option<crate::runtime::RemoteObject<'a>>,
    /// Valid only while the VM is paused and indicates whether this frame
    /// can be restarted or not. Note that a 'true' value here does not
    /// guarantee that Debugger#restartFrame with this CallFrameId will be
    /// successful, but it is very likely.
    #[serde(skip_serializing_if = "Option::is_none", rename = "canBeRestarted")]
    can_be_restarted: Option<bool>,
}

impl<'a> CallFrame<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `call_frame_id`: Call frame identifier. This identifier is only valid while the virtual machine is paused.
    /// * `function_name`: Name of the JavaScript function called on this call frame.
    /// * `location`: Location in the source code.
    /// * `url`: JavaScript script name or url. Deprecated in favor of using the `location.scriptId` to resolve the URL via a previously sent `Debugger.scriptParsed` event.
    /// * `scope_chain`: Scope chain for this call frame.
    /// * `this`: `this` object for this call frame.
    pub fn builder(call_frame_id: impl Into<CallFrameId<'a>>, function_name: impl Into<Cow<'a, str>>, location: Location<'a>, url: impl Into<Cow<'a, str>>, scope_chain: Vec<Scope<'a>>, this: crate::runtime::RemoteObject<'a>) -> CallFrameBuilder<'a> {
        CallFrameBuilder {
            call_frame_id: call_frame_id.into(),
            function_name: function_name.into(),
            function_location: None,
            location: location,
            url: url.into(),
            scope_chain: scope_chain,
            this: this,
            return_value: None,
            can_be_restarted: None,
        }
    }
    /// Call frame identifier. This identifier is only valid while the virtual machine is paused.
    pub fn call_frame_id(&self) -> &CallFrameId<'a> { &self.call_frame_id }
    /// Name of the JavaScript function called on this call frame.
    pub fn function_name(&self) -> &str { self.function_name.as_ref() }
    /// Location in the source code.
    pub fn function_location(&self) -> Option<&Location<'a>> { self.function_location.as_ref() }
    /// Location in the source code.
    pub fn location(&self) -> &Location<'a> { &self.location }
    /// JavaScript script name or url.
    /// Deprecated in favor of using the 'location.scriptId' to resolve the URL via a previously
    /// sent 'Debugger.scriptParsed' event.
    pub fn url(&self) -> &str { self.url.as_ref() }
    /// Scope chain for this call frame.
    pub fn scope_chain(&self) -> &[Scope<'a>] { &self.scope_chain }
    /// 'this' object for this call frame.
    pub fn this(&self) -> &crate::runtime::RemoteObject<'a> { &self.this }
    /// The value being returned, if the function is at return point.
    pub fn return_value(&self) -> Option<&crate::runtime::RemoteObject<'a>> { self.return_value.as_ref() }
    /// Valid only while the VM is paused and indicates whether this frame
    /// can be restarted or not. Note that a 'true' value here does not
    /// guarantee that Debugger#restartFrame with this CallFrameId will be
    /// successful, but it is very likely.
    pub fn can_be_restarted(&self) -> Option<bool> { self.can_be_restarted }
}


pub struct CallFrameBuilder<'a> {
    call_frame_id: CallFrameId<'a>,
    function_name: Cow<'a, str>,
    function_location: Option<Location<'a>>,
    location: Location<'a>,
    url: Cow<'a, str>,
    scope_chain: Vec<Scope<'a>>,
    this: crate::runtime::RemoteObject<'a>,
    return_value: Option<crate::runtime::RemoteObject<'a>>,
    can_be_restarted: Option<bool>,
}

impl<'a> CallFrameBuilder<'a> {
    /// Location in the source code.
    pub fn function_location(mut self, function_location: Location<'a>) -> Self { self.function_location = Some(function_location); self }
    /// The value being returned, if the function is at return point.
    pub fn return_value(mut self, return_value: crate::runtime::RemoteObject<'a>) -> Self { self.return_value = Some(return_value); self }
    /// Valid only while the VM is paused and indicates whether this frame
    /// can be restarted or not. Note that a 'true' value here does not
    /// guarantee that Debugger#restartFrame with this CallFrameId will be
    /// successful, but it is very likely.
    pub fn can_be_restarted(mut self, can_be_restarted: bool) -> Self { self.can_be_restarted = Some(can_be_restarted); self }
    pub fn build(self) -> CallFrame<'a> {
        CallFrame {
            call_frame_id: self.call_frame_id,
            function_name: self.function_name,
            function_location: self.function_location,
            location: self.location,
            url: self.url,
            scope_chain: self.scope_chain,
            this: self.this,
            return_value: self.return_value,
            can_be_restarted: self.can_be_restarted,
        }
    }
}

/// Scope description.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Scope<'a> {
    /// Scope type.
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
    /// Object representing the scope. For 'global' and 'with' scopes it represents the actual
    /// object; for the rest of the scopes, it is artificial transient object enumerating scope
    /// variables as its properties.
    object: crate::runtime::RemoteObject<'a>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<Cow<'a, str>>,
    /// Location in the source code where scope starts
    #[serde(skip_serializing_if = "Option::is_none", rename = "startLocation")]
    start_location: Option<Location<'a>>,
    /// Location in the source code where scope ends
    #[serde(skip_serializing_if = "Option::is_none", rename = "endLocation")]
    end_location: Option<Location<'a>>,
}

impl<'a> Scope<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `type_`: Scope type.
    /// * `object`: Object representing the scope. For `global` and `with` scopes it represents the actual object; for the rest of the scopes, it is artificial transient object enumerating scope variables as its properties.
    pub fn builder(type_: impl Into<Cow<'a, str>>, object: crate::runtime::RemoteObject<'a>) -> ScopeBuilder<'a> {
        ScopeBuilder {
            type_: type_.into(),
            object: object,
            name: None,
            start_location: None,
            end_location: None,
        }
    }
    /// Scope type.
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    /// Object representing the scope. For 'global' and 'with' scopes it represents the actual
    /// object; for the rest of the scopes, it is artificial transient object enumerating scope
    /// variables as its properties.
    pub fn object(&self) -> &crate::runtime::RemoteObject<'a> { &self.object }
    pub fn name(&self) -> Option<&str> { self.name.as_deref() }
    /// Location in the source code where scope starts
    pub fn start_location(&self) -> Option<&Location<'a>> { self.start_location.as_ref() }
    /// Location in the source code where scope ends
    pub fn end_location(&self) -> Option<&Location<'a>> { self.end_location.as_ref() }
}


pub struct ScopeBuilder<'a> {
    type_: Cow<'a, str>,
    object: crate::runtime::RemoteObject<'a>,
    name: Option<Cow<'a, str>>,
    start_location: Option<Location<'a>>,
    end_location: Option<Location<'a>>,
}

impl<'a> ScopeBuilder<'a> {
    pub fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self { self.name = Some(name.into()); self }
    /// Location in the source code where scope starts
    pub fn start_location(mut self, start_location: Location<'a>) -> Self { self.start_location = Some(start_location); self }
    /// Location in the source code where scope ends
    pub fn end_location(mut self, end_location: Location<'a>) -> Self { self.end_location = Some(end_location); self }
    pub fn build(self) -> Scope<'a> {
        Scope {
            type_: self.type_,
            object: self.object,
            name: self.name,
            start_location: self.start_location,
            end_location: self.end_location,
        }
    }
}

/// Search match for resource.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchMatch<'a> {
    /// Line number in resource content.
    #[serde(rename = "lineNumber")]
    line_number: f64,
    /// Line with match content.
    #[serde(rename = "lineContent")]
    line_content: Cow<'a, str>,
}

impl<'a> SearchMatch<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `line_number`: Line number in resource content.
    /// * `line_content`: Line with match content.
    pub fn builder(line_number: f64, line_content: impl Into<Cow<'a, str>>) -> SearchMatchBuilder<'a> {
        SearchMatchBuilder {
            line_number: line_number,
            line_content: line_content.into(),
        }
    }
    /// Line number in resource content.
    pub fn line_number(&self) -> f64 { self.line_number }
    /// Line with match content.
    pub fn line_content(&self) -> &str { self.line_content.as_ref() }
}


pub struct SearchMatchBuilder<'a> {
    line_number: f64,
    line_content: Cow<'a, str>,
}

impl<'a> SearchMatchBuilder<'a> {
    pub fn build(self) -> SearchMatch<'a> {
        SearchMatch {
            line_number: self.line_number,
            line_content: self.line_content,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BreakLocation<'a> {
    /// Script identifier as reported in the 'Debugger.scriptParsed'.
    #[serde(rename = "scriptId")]
    script_id: crate::runtime::ScriptId<'a>,
    /// Line number in the script (0-based).
    #[serde(rename = "lineNumber")]
    line_number: i64,
    /// Column number in the script (0-based).
    #[serde(skip_serializing_if = "Option::is_none", rename = "columnNumber")]
    column_number: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    type_: Option<Cow<'a, str>>,
}

impl<'a> BreakLocation<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `script_id`: Script identifier as reported in the `Debugger.scriptParsed`.
    /// * `line_number`: Line number in the script (0-based).
    pub fn builder(script_id: crate::runtime::ScriptId<'a>, line_number: i64) -> BreakLocationBuilder<'a> {
        BreakLocationBuilder {
            script_id: script_id,
            line_number: line_number,
            column_number: None,
            type_: None,
        }
    }
    /// Script identifier as reported in the 'Debugger.scriptParsed'.
    pub fn script_id(&self) -> &crate::runtime::ScriptId<'a> { &self.script_id }
    /// Line number in the script (0-based).
    pub fn line_number(&self) -> i64 { self.line_number }
    /// Column number in the script (0-based).
    pub fn column_number(&self) -> Option<i64> { self.column_number }
    pub fn type_(&self) -> Option<&str> { self.type_.as_deref() }
}


pub struct BreakLocationBuilder<'a> {
    script_id: crate::runtime::ScriptId<'a>,
    line_number: i64,
    column_number: Option<i64>,
    type_: Option<Cow<'a, str>>,
}

impl<'a> BreakLocationBuilder<'a> {
    /// Column number in the script (0-based).
    pub fn column_number(mut self, column_number: i64) -> Self { self.column_number = Some(column_number); self }
    pub fn type_(mut self, type_: impl Into<Cow<'a, str>>) -> Self { self.type_ = Some(type_.into()); self }
    pub fn build(self) -> BreakLocation<'a> {
        BreakLocation {
            script_id: self.script_id,
            line_number: self.line_number,
            column_number: self.column_number,
            type_: self.type_,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct WasmDisassemblyChunk<'a> {
    /// The next chunk of disassembled lines.
    lines: Vec<Cow<'a, str>>,
    /// The bytecode offsets describing the start of each line.
    #[serde(rename = "bytecodeOffsets")]
    bytecode_offsets: Vec<i64>,
}

impl<'a> WasmDisassemblyChunk<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `lines`: The next chunk of disassembled lines.
    /// * `bytecode_offsets`: The bytecode offsets describing the start of each line.
    pub fn builder(lines: Vec<Cow<'a, str>>, bytecode_offsets: Vec<i64>) -> WasmDisassemblyChunkBuilder<'a> {
        WasmDisassemblyChunkBuilder {
            lines: lines,
            bytecode_offsets: bytecode_offsets,
        }
    }
    /// The next chunk of disassembled lines.
    pub fn lines(&self) -> &[Cow<'a, str>] { &self.lines }
    /// The bytecode offsets describing the start of each line.
    pub fn bytecode_offsets(&self) -> &[i64] { &self.bytecode_offsets }
}


pub struct WasmDisassemblyChunkBuilder<'a> {
    lines: Vec<Cow<'a, str>>,
    bytecode_offsets: Vec<i64>,
}

impl<'a> WasmDisassemblyChunkBuilder<'a> {
    pub fn build(self) -> WasmDisassemblyChunk<'a> {
        WasmDisassemblyChunk {
            lines: self.lines,
            bytecode_offsets: self.bytecode_offsets,
        }
    }
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DebugSymbols<'a> {
    /// Type of the debug symbols.
    #[serde(rename = "type")]
    type_: Cow<'a, str>,
    /// URL of the external symbol source.
    #[serde(skip_serializing_if = "Option::is_none", rename = "externalURL")]
    external_url: Option<Cow<'a, str>>,
}

impl<'a> DebugSymbols<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `type_`: Type of the debug symbols.
    pub fn builder(type_: impl Into<Cow<'a, str>>) -> DebugSymbolsBuilder<'a> {
        DebugSymbolsBuilder {
            type_: type_.into(),
            external_url: None,
        }
    }
    /// Type of the debug symbols.
    pub fn type_(&self) -> &str { self.type_.as_ref() }
    /// URL of the external symbol source.
    pub fn external_url(&self) -> Option<&str> { self.external_url.as_deref() }
}


pub struct DebugSymbolsBuilder<'a> {
    type_: Cow<'a, str>,
    external_url: Option<Cow<'a, str>>,
}

impl<'a> DebugSymbolsBuilder<'a> {
    /// URL of the external symbol source.
    pub fn external_url(mut self, external_url: impl Into<Cow<'a, str>>) -> Self { self.external_url = Some(external_url.into()); self }
    pub fn build(self) -> DebugSymbols<'a> {
        DebugSymbols {
            type_: self.type_,
            external_url: self.external_url,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedBreakpoint<'a> {
    /// Breakpoint unique identifier.
    #[serde(rename = "breakpointId")]
    breakpoint_id: BreakpointId<'a>,
    /// Actual breakpoint location.
    location: Location<'a>,
}

impl<'a> ResolvedBreakpoint<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `breakpoint_id`: Breakpoint unique identifier.
    /// * `location`: Actual breakpoint location.
    pub fn builder(breakpoint_id: impl Into<BreakpointId<'a>>, location: Location<'a>) -> ResolvedBreakpointBuilder<'a> {
        ResolvedBreakpointBuilder {
            breakpoint_id: breakpoint_id.into(),
            location: location,
        }
    }
    /// Breakpoint unique identifier.
    pub fn breakpoint_id(&self) -> &BreakpointId<'a> { &self.breakpoint_id }
    /// Actual breakpoint location.
    pub fn location(&self) -> &Location<'a> { &self.location }
}


pub struct ResolvedBreakpointBuilder<'a> {
    breakpoint_id: BreakpointId<'a>,
    location: Location<'a>,
}

impl<'a> ResolvedBreakpointBuilder<'a> {
    pub fn build(self) -> ResolvedBreakpoint<'a> {
        ResolvedBreakpoint {
            breakpoint_id: self.breakpoint_id,
            location: self.location,
        }
    }
}

/// Continues execution until specific location is reached.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ContinueToLocationParams<'a> {
    /// Location to continue to.
    location: Location<'a>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "targetCallFrames")]
    target_call_frames: Option<Cow<'a, str>>,
}

impl<'a> ContinueToLocationParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `location`: Location to continue to.
    pub fn builder(location: Location<'a>) -> ContinueToLocationParamsBuilder<'a> {
        ContinueToLocationParamsBuilder {
            location: location,
            target_call_frames: None,
        }
    }
    /// Location to continue to.
    pub fn location(&self) -> &Location<'a> { &self.location }
    pub fn target_call_frames(&self) -> Option<&str> { self.target_call_frames.as_deref() }
}


pub struct ContinueToLocationParamsBuilder<'a> {
    location: Location<'a>,
    target_call_frames: Option<Cow<'a, str>>,
}

impl<'a> ContinueToLocationParamsBuilder<'a> {
    pub fn target_call_frames(mut self, target_call_frames: impl Into<Cow<'a, str>>) -> Self { self.target_call_frames = Some(target_call_frames.into()); self }
    pub fn build(self) -> ContinueToLocationParams<'a> {
        ContinueToLocationParams {
            location: self.location,
            target_call_frames: self.target_call_frames,
        }
    }
}

impl<'a> ContinueToLocationParams<'a> { pub const METHOD: &'static str = "Debugger.continueToLocation"; }

impl<'a> crate::CdpCommand<'a> for ContinueToLocationParams<'a> {
    const METHOD: &'static str = "Debugger.continueToLocation";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "Debugger.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxScriptsCacheSize")]
    max_scripts_cache_size: Option<f64>,
}

impl EnableParams {
    /// Creates a builder for this type.
    pub fn builder() -> EnableParamsBuilder {
        EnableParamsBuilder {
            max_scripts_cache_size: None,
        }
    }
    /// The maximum size in bytes of collected scripts (not referenced by other heap objects)
    /// the debugger can hold. Puts no limit if parameter is omitted.
    pub fn max_scripts_cache_size(&self) -> Option<f64> { self.max_scripts_cache_size }
}

#[derive(Default)]
pub struct EnableParamsBuilder {
    max_scripts_cache_size: Option<f64>,
}

impl EnableParamsBuilder {
    /// The maximum size in bytes of collected scripts (not referenced by other heap objects)
    /// the debugger can hold. Puts no limit if parameter is omitted.
    pub fn max_scripts_cache_size(mut self, max_scripts_cache_size: f64) -> Self { self.max_scripts_cache_size = Some(max_scripts_cache_size); self }
    pub fn build(self) -> EnableParams {
        EnableParams {
            max_scripts_cache_size: self.max_scripts_cache_size,
        }
    }
}

/// Enables debugger for the given page. Clients should not assume that the debugging has been
/// enabled until the result for this command is received.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EnableReturns<'a> {
    /// Unique identifier of the debugger.
    #[serde(rename = "debuggerId")]
    debugger_id: crate::runtime::UniqueDebuggerId<'a>,
}

impl<'a> EnableReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `debugger_id`: Unique identifier of the debugger.
    pub fn builder(debugger_id: crate::runtime::UniqueDebuggerId<'a>) -> EnableReturnsBuilder<'a> {
        EnableReturnsBuilder {
            debugger_id: debugger_id,
        }
    }
    /// Unique identifier of the debugger.
    pub fn debugger_id(&self) -> &crate::runtime::UniqueDebuggerId<'a> { &self.debugger_id }
}


pub struct EnableReturnsBuilder<'a> {
    debugger_id: crate::runtime::UniqueDebuggerId<'a>,
}

impl<'a> EnableReturnsBuilder<'a> {
    pub fn build(self) -> EnableReturns<'a> {
        EnableReturns {
            debugger_id: self.debugger_id,
        }
    }
}

impl EnableParams { pub const METHOD: &'static str = "Debugger.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "Debugger.enable";
    type Response = EnableReturns<'a>;
}

/// Evaluates expression on a given call frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateOnCallFrameParams<'a> {
    /// Call frame identifier to evaluate on.
    #[serde(rename = "callFrameId")]
    call_frame_id: CallFrameId<'a>,
    /// Expression to evaluate.
    expression: Cow<'a, str>,
    /// String object group name to put result into (allows rapid releasing resulting object handles
    /// using 'releaseObjectGroup').
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectGroup")]
    object_group: Option<Cow<'a, str>>,
    /// Specifies whether command line API should be available to the evaluated expression, defaults
    /// to false.
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeCommandLineAPI")]
    include_command_line_api: Option<bool>,
    /// In silent mode exceptions thrown during evaluation are not reported and do not pause
    /// execution. Overrides 'setPauseOnException' state.
    #[serde(skip_serializing_if = "Option::is_none")]
    silent: Option<bool>,
    /// Whether the result is expected to be a JSON object that should be sent by value.
    #[serde(skip_serializing_if = "Option::is_none", rename = "returnByValue")]
    return_by_value: Option<bool>,
    /// Whether preview should be generated for the result.
    #[serde(skip_serializing_if = "Option::is_none", rename = "generatePreview")]
    generate_preview: Option<bool>,
    /// Whether to throw an exception if side effect cannot be ruled out during evaluation.
    #[serde(skip_serializing_if = "Option::is_none", rename = "throwOnSideEffect")]
    throw_on_side_effect: Option<bool>,
    /// Terminate execution after timing out (number of milliseconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    timeout: Option<crate::runtime::TimeDelta>,
}

impl<'a> EvaluateOnCallFrameParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `call_frame_id`: Call frame identifier to evaluate on.
    /// * `expression`: Expression to evaluate.
    pub fn builder(call_frame_id: impl Into<CallFrameId<'a>>, expression: impl Into<Cow<'a, str>>) -> EvaluateOnCallFrameParamsBuilder<'a> {
        EvaluateOnCallFrameParamsBuilder {
            call_frame_id: call_frame_id.into(),
            expression: expression.into(),
            object_group: None,
            include_command_line_api: None,
            silent: None,
            return_by_value: None,
            generate_preview: None,
            throw_on_side_effect: None,
            timeout: None,
        }
    }
    /// Call frame identifier to evaluate on.
    pub fn call_frame_id(&self) -> &CallFrameId<'a> { &self.call_frame_id }
    /// Expression to evaluate.
    pub fn expression(&self) -> &str { self.expression.as_ref() }
    /// String object group name to put result into (allows rapid releasing resulting object handles
    /// using 'releaseObjectGroup').
    pub fn object_group(&self) -> Option<&str> { self.object_group.as_deref() }
    /// Specifies whether command line API should be available to the evaluated expression, defaults
    /// to false.
    pub fn include_command_line_api(&self) -> Option<bool> { self.include_command_line_api }
    /// In silent mode exceptions thrown during evaluation are not reported and do not pause
    /// execution. Overrides 'setPauseOnException' state.
    pub fn silent(&self) -> Option<bool> { self.silent }
    /// Whether the result is expected to be a JSON object that should be sent by value.
    pub fn return_by_value(&self) -> Option<bool> { self.return_by_value }
    /// Whether preview should be generated for the result.
    pub fn generate_preview(&self) -> Option<bool> { self.generate_preview }
    /// Whether to throw an exception if side effect cannot be ruled out during evaluation.
    pub fn throw_on_side_effect(&self) -> Option<bool> { self.throw_on_side_effect }
    /// Terminate execution after timing out (number of milliseconds).
    pub fn timeout(&self) -> Option<&crate::runtime::TimeDelta> { self.timeout.as_ref() }
}


pub struct EvaluateOnCallFrameParamsBuilder<'a> {
    call_frame_id: CallFrameId<'a>,
    expression: Cow<'a, str>,
    object_group: Option<Cow<'a, str>>,
    include_command_line_api: Option<bool>,
    silent: Option<bool>,
    return_by_value: Option<bool>,
    generate_preview: Option<bool>,
    throw_on_side_effect: Option<bool>,
    timeout: Option<crate::runtime::TimeDelta>,
}

impl<'a> EvaluateOnCallFrameParamsBuilder<'a> {
    /// String object group name to put result into (allows rapid releasing resulting object handles
    /// using 'releaseObjectGroup').
    pub fn object_group(mut self, object_group: impl Into<Cow<'a, str>>) -> Self { self.object_group = Some(object_group.into()); self }
    /// Specifies whether command line API should be available to the evaluated expression, defaults
    /// to false.
    pub fn include_command_line_api(mut self, include_command_line_api: bool) -> Self { self.include_command_line_api = Some(include_command_line_api); self }
    /// In silent mode exceptions thrown during evaluation are not reported and do not pause
    /// execution. Overrides 'setPauseOnException' state.
    pub fn silent(mut self, silent: bool) -> Self { self.silent = Some(silent); self }
    /// Whether the result is expected to be a JSON object that should be sent by value.
    pub fn return_by_value(mut self, return_by_value: bool) -> Self { self.return_by_value = Some(return_by_value); self }
    /// Whether preview should be generated for the result.
    pub fn generate_preview(mut self, generate_preview: bool) -> Self { self.generate_preview = Some(generate_preview); self }
    /// Whether to throw an exception if side effect cannot be ruled out during evaluation.
    pub fn throw_on_side_effect(mut self, throw_on_side_effect: bool) -> Self { self.throw_on_side_effect = Some(throw_on_side_effect); self }
    /// Terminate execution after timing out (number of milliseconds).
    pub fn timeout(mut self, timeout: crate::runtime::TimeDelta) -> Self { self.timeout = Some(timeout); self }
    pub fn build(self) -> EvaluateOnCallFrameParams<'a> {
        EvaluateOnCallFrameParams {
            call_frame_id: self.call_frame_id,
            expression: self.expression,
            object_group: self.object_group,
            include_command_line_api: self.include_command_line_api,
            silent: self.silent,
            return_by_value: self.return_by_value,
            generate_preview: self.generate_preview,
            throw_on_side_effect: self.throw_on_side_effect,
            timeout: self.timeout,
        }
    }
}

/// Evaluates expression on a given call frame.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct EvaluateOnCallFrameReturns<'a> {
    /// Object wrapper for the evaluation result.
    result: crate::runtime::RemoteObject<'a>,
    /// Exception details.
    #[serde(skip_serializing_if = "Option::is_none", rename = "exceptionDetails")]
    exception_details: Option<crate::runtime::ExceptionDetails<'a>>,
}

impl<'a> EvaluateOnCallFrameReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `result`: Object wrapper for the evaluation result.
    pub fn builder(result: crate::runtime::RemoteObject<'a>) -> EvaluateOnCallFrameReturnsBuilder<'a> {
        EvaluateOnCallFrameReturnsBuilder {
            result: result,
            exception_details: None,
        }
    }
    /// Object wrapper for the evaluation result.
    pub fn result(&self) -> &crate::runtime::RemoteObject<'a> { &self.result }
    /// Exception details.
    pub fn exception_details(&self) -> Option<&crate::runtime::ExceptionDetails<'a>> { self.exception_details.as_ref() }
}


pub struct EvaluateOnCallFrameReturnsBuilder<'a> {
    result: crate::runtime::RemoteObject<'a>,
    exception_details: Option<crate::runtime::ExceptionDetails<'a>>,
}

impl<'a> EvaluateOnCallFrameReturnsBuilder<'a> {
    /// Exception details.
    pub fn exception_details(mut self, exception_details: crate::runtime::ExceptionDetails<'a>) -> Self { self.exception_details = Some(exception_details); self }
    pub fn build(self) -> EvaluateOnCallFrameReturns<'a> {
        EvaluateOnCallFrameReturns {
            result: self.result,
            exception_details: self.exception_details,
        }
    }
}

impl<'a> EvaluateOnCallFrameParams<'a> { pub const METHOD: &'static str = "Debugger.evaluateOnCallFrame"; }

impl<'a> crate::CdpCommand<'a> for EvaluateOnCallFrameParams<'a> {
    const METHOD: &'static str = "Debugger.evaluateOnCallFrame";
    type Response = EvaluateOnCallFrameReturns<'a>;
}

/// Returns possible locations for breakpoint. scriptId in start and end range locations should be
/// the same.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPossibleBreakpointsParams<'a> {
    /// Start of range to search possible breakpoint locations in.
    start: Location<'a>,
    /// End of range to search possible breakpoint locations in (excluding). When not specified, end
    /// of scripts is used as end of range.
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<Location<'a>>,
    /// Only consider locations which are in the same (non-nested) function as start.
    #[serde(skip_serializing_if = "Option::is_none", rename = "restrictToFunction")]
    restrict_to_function: Option<bool>,
}

impl<'a> GetPossibleBreakpointsParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `start`: Start of range to search possible breakpoint locations in.
    pub fn builder(start: Location<'a>) -> GetPossibleBreakpointsParamsBuilder<'a> {
        GetPossibleBreakpointsParamsBuilder {
            start: start,
            end: None,
            restrict_to_function: None,
        }
    }
    /// Start of range to search possible breakpoint locations in.
    pub fn start(&self) -> &Location<'a> { &self.start }
    /// End of range to search possible breakpoint locations in (excluding). When not specified, end
    /// of scripts is used as end of range.
    pub fn end(&self) -> Option<&Location<'a>> { self.end.as_ref() }
    /// Only consider locations which are in the same (non-nested) function as start.
    pub fn restrict_to_function(&self) -> Option<bool> { self.restrict_to_function }
}


pub struct GetPossibleBreakpointsParamsBuilder<'a> {
    start: Location<'a>,
    end: Option<Location<'a>>,
    restrict_to_function: Option<bool>,
}

impl<'a> GetPossibleBreakpointsParamsBuilder<'a> {
    /// End of range to search possible breakpoint locations in (excluding). When not specified, end
    /// of scripts is used as end of range.
    pub fn end(mut self, end: Location<'a>) -> Self { self.end = Some(end); self }
    /// Only consider locations which are in the same (non-nested) function as start.
    pub fn restrict_to_function(mut self, restrict_to_function: bool) -> Self { self.restrict_to_function = Some(restrict_to_function); self }
    pub fn build(self) -> GetPossibleBreakpointsParams<'a> {
        GetPossibleBreakpointsParams {
            start: self.start,
            end: self.end,
            restrict_to_function: self.restrict_to_function,
        }
    }
}

/// Returns possible locations for breakpoint. scriptId in start and end range locations should be
/// the same.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetPossibleBreakpointsReturns<'a> {
    /// List of the possible breakpoint locations.
    locations: Vec<BreakLocation<'a>>,
}

impl<'a> GetPossibleBreakpointsReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `locations`: List of the possible breakpoint locations.
    pub fn builder(locations: Vec<BreakLocation<'a>>) -> GetPossibleBreakpointsReturnsBuilder<'a> {
        GetPossibleBreakpointsReturnsBuilder {
            locations: locations,
        }
    }
    /// List of the possible breakpoint locations.
    pub fn locations(&self) -> &[BreakLocation<'a>] { &self.locations }
}


pub struct GetPossibleBreakpointsReturnsBuilder<'a> {
    locations: Vec<BreakLocation<'a>>,
}

impl<'a> GetPossibleBreakpointsReturnsBuilder<'a> {
    pub fn build(self) -> GetPossibleBreakpointsReturns<'a> {
        GetPossibleBreakpointsReturns {
            locations: self.locations,
        }
    }
}

impl<'a> GetPossibleBreakpointsParams<'a> { pub const METHOD: &'static str = "Debugger.getPossibleBreakpoints"; }

impl<'a> crate::CdpCommand<'a> for GetPossibleBreakpointsParams<'a> {
    const METHOD: &'static str = "Debugger.getPossibleBreakpoints";
    type Response = GetPossibleBreakpointsReturns<'a>;
}

/// Returns source for the script with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetScriptSourceParams<'a> {
    /// Id of the script to get source for.
    #[serde(rename = "scriptId")]
    script_id: crate::runtime::ScriptId<'a>,
}

impl<'a> GetScriptSourceParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `script_id`: Id of the script to get source for.
    pub fn builder(script_id: crate::runtime::ScriptId<'a>) -> GetScriptSourceParamsBuilder<'a> {
        GetScriptSourceParamsBuilder {
            script_id: script_id,
        }
    }
    /// Id of the script to get source for.
    pub fn script_id(&self) -> &crate::runtime::ScriptId<'a> { &self.script_id }
}


pub struct GetScriptSourceParamsBuilder<'a> {
    script_id: crate::runtime::ScriptId<'a>,
}

impl<'a> GetScriptSourceParamsBuilder<'a> {
    pub fn build(self) -> GetScriptSourceParams<'a> {
        GetScriptSourceParams {
            script_id: self.script_id,
        }
    }
}

/// Returns source for the script with given id.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetScriptSourceReturns<'a> {
    /// Script source (empty in case of Wasm bytecode).
    #[serde(rename = "scriptSource")]
    script_source: Cow<'a, str>,
    /// Wasm bytecode. (Encoded as a base64 string when passed over JSON)
    #[serde(skip_serializing_if = "Option::is_none")]
    bytecode: Option<Cow<'a, str>>,
}

impl<'a> GetScriptSourceReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `script_source`: Script source (empty in case of Wasm bytecode).
    pub fn builder(script_source: impl Into<Cow<'a, str>>) -> GetScriptSourceReturnsBuilder<'a> {
        GetScriptSourceReturnsBuilder {
            script_source: script_source.into(),
            bytecode: None,
        }
    }
    /// Script source (empty in case of Wasm bytecode).
    pub fn script_source(&self) -> &str { self.script_source.as_ref() }
    /// Wasm bytecode. (Encoded as a base64 string when passed over JSON)
    pub fn bytecode(&self) -> Option<&str> { self.bytecode.as_deref() }
}


pub struct GetScriptSourceReturnsBuilder<'a> {
    script_source: Cow<'a, str>,
    bytecode: Option<Cow<'a, str>>,
}

impl<'a> GetScriptSourceReturnsBuilder<'a> {
    /// Wasm bytecode. (Encoded as a base64 string when passed over JSON)
    pub fn bytecode(mut self, bytecode: impl Into<Cow<'a, str>>) -> Self { self.bytecode = Some(bytecode.into()); self }
    pub fn build(self) -> GetScriptSourceReturns<'a> {
        GetScriptSourceReturns {
            script_source: self.script_source,
            bytecode: self.bytecode,
        }
    }
}

impl<'a> GetScriptSourceParams<'a> { pub const METHOD: &'static str = "Debugger.getScriptSource"; }

impl<'a> crate::CdpCommand<'a> for GetScriptSourceParams<'a> {
    const METHOD: &'static str = "Debugger.getScriptSource";
    type Response = GetScriptSourceReturns<'a>;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DisassembleWasmModuleParams<'a> {
    /// Id of the script to disassemble
    #[serde(rename = "scriptId")]
    script_id: crate::runtime::ScriptId<'a>,
}

impl<'a> DisassembleWasmModuleParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `script_id`: Id of the script to disassemble
    pub fn builder(script_id: crate::runtime::ScriptId<'a>) -> DisassembleWasmModuleParamsBuilder<'a> {
        DisassembleWasmModuleParamsBuilder {
            script_id: script_id,
        }
    }
    /// Id of the script to disassemble
    pub fn script_id(&self) -> &crate::runtime::ScriptId<'a> { &self.script_id }
}


pub struct DisassembleWasmModuleParamsBuilder<'a> {
    script_id: crate::runtime::ScriptId<'a>,
}

impl<'a> DisassembleWasmModuleParamsBuilder<'a> {
    pub fn build(self) -> DisassembleWasmModuleParams<'a> {
        DisassembleWasmModuleParams {
            script_id: self.script_id,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct DisassembleWasmModuleReturns<'a> {
    /// For large modules, return a stream from which additional chunks of
    /// disassembly can be read successively.
    #[serde(skip_serializing_if = "Option::is_none", rename = "streamId")]
    stream_id: Option<Cow<'a, str>>,
    /// The total number of lines in the disassembly text.
    #[serde(rename = "totalNumberOfLines")]
    total_number_of_lines: i64,
    /// The offsets of all function bodies, in the format \[start1, end1,
    /// start2, end2, ...\] where all ends are exclusive.
    #[serde(rename = "functionBodyOffsets")]
    function_body_offsets: Vec<i64>,
    /// The first chunk of disassembly.
    chunk: WasmDisassemblyChunk<'a>,
}

impl<'a> DisassembleWasmModuleReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `total_number_of_lines`: The total number of lines in the disassembly text.
    /// * `function_body_offsets`: The offsets of all function bodies, in the format \[start1, end1, start2, end2, ...\] where all ends are exclusive.
    /// * `chunk`: The first chunk of disassembly.
    pub fn builder(total_number_of_lines: i64, function_body_offsets: Vec<i64>, chunk: WasmDisassemblyChunk<'a>) -> DisassembleWasmModuleReturnsBuilder<'a> {
        DisassembleWasmModuleReturnsBuilder {
            stream_id: None,
            total_number_of_lines: total_number_of_lines,
            function_body_offsets: function_body_offsets,
            chunk: chunk,
        }
    }
    /// For large modules, return a stream from which additional chunks of
    /// disassembly can be read successively.
    pub fn stream_id(&self) -> Option<&str> { self.stream_id.as_deref() }
    /// The total number of lines in the disassembly text.
    pub fn total_number_of_lines(&self) -> i64 { self.total_number_of_lines }
    /// The offsets of all function bodies, in the format \[start1, end1,
    /// start2, end2, ...\] where all ends are exclusive.
    pub fn function_body_offsets(&self) -> &[i64] { &self.function_body_offsets }
    /// The first chunk of disassembly.
    pub fn chunk(&self) -> &WasmDisassemblyChunk<'a> { &self.chunk }
}


pub struct DisassembleWasmModuleReturnsBuilder<'a> {
    stream_id: Option<Cow<'a, str>>,
    total_number_of_lines: i64,
    function_body_offsets: Vec<i64>,
    chunk: WasmDisassemblyChunk<'a>,
}

impl<'a> DisassembleWasmModuleReturnsBuilder<'a> {
    /// For large modules, return a stream from which additional chunks of
    /// disassembly can be read successively.
    pub fn stream_id(mut self, stream_id: impl Into<Cow<'a, str>>) -> Self { self.stream_id = Some(stream_id.into()); self }
    pub fn build(self) -> DisassembleWasmModuleReturns<'a> {
        DisassembleWasmModuleReturns {
            stream_id: self.stream_id,
            total_number_of_lines: self.total_number_of_lines,
            function_body_offsets: self.function_body_offsets,
            chunk: self.chunk,
        }
    }
}

impl<'a> DisassembleWasmModuleParams<'a> { pub const METHOD: &'static str = "Debugger.disassembleWasmModule"; }

impl<'a> crate::CdpCommand<'a> for DisassembleWasmModuleParams<'a> {
    const METHOD: &'static str = "Debugger.disassembleWasmModule";
    type Response = DisassembleWasmModuleReturns<'a>;
}

/// Disassemble the next chunk of lines for the module corresponding to the
/// stream. If disassembly is complete, this API will invalidate the streamId
/// and return an empty chunk. Any subsequent calls for the now invalid stream
/// will return errors.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NextWasmDisassemblyChunkParams<'a> {
    #[serde(rename = "streamId")]
    stream_id: Cow<'a, str>,
}

impl<'a> NextWasmDisassemblyChunkParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `stream_id`: 
    pub fn builder(stream_id: impl Into<Cow<'a, str>>) -> NextWasmDisassemblyChunkParamsBuilder<'a> {
        NextWasmDisassemblyChunkParamsBuilder {
            stream_id: stream_id.into(),
        }
    }
    pub fn stream_id(&self) -> &str { self.stream_id.as_ref() }
}


pub struct NextWasmDisassemblyChunkParamsBuilder<'a> {
    stream_id: Cow<'a, str>,
}

impl<'a> NextWasmDisassemblyChunkParamsBuilder<'a> {
    pub fn build(self) -> NextWasmDisassemblyChunkParams<'a> {
        NextWasmDisassemblyChunkParams {
            stream_id: self.stream_id,
        }
    }
}

/// Disassemble the next chunk of lines for the module corresponding to the
/// stream. If disassembly is complete, this API will invalidate the streamId
/// and return an empty chunk. Any subsequent calls for the now invalid stream
/// will return errors.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct NextWasmDisassemblyChunkReturns<'a> {
    /// The next chunk of disassembly.
    chunk: WasmDisassemblyChunk<'a>,
}

impl<'a> NextWasmDisassemblyChunkReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `chunk`: The next chunk of disassembly.
    pub fn builder(chunk: WasmDisassemblyChunk<'a>) -> NextWasmDisassemblyChunkReturnsBuilder<'a> {
        NextWasmDisassemblyChunkReturnsBuilder {
            chunk: chunk,
        }
    }
    /// The next chunk of disassembly.
    pub fn chunk(&self) -> &WasmDisassemblyChunk<'a> { &self.chunk }
}


pub struct NextWasmDisassemblyChunkReturnsBuilder<'a> {
    chunk: WasmDisassemblyChunk<'a>,
}

impl<'a> NextWasmDisassemblyChunkReturnsBuilder<'a> {
    pub fn build(self) -> NextWasmDisassemblyChunkReturns<'a> {
        NextWasmDisassemblyChunkReturns {
            chunk: self.chunk,
        }
    }
}

impl<'a> NextWasmDisassemblyChunkParams<'a> { pub const METHOD: &'static str = "Debugger.nextWasmDisassemblyChunk"; }

impl<'a> crate::CdpCommand<'a> for NextWasmDisassemblyChunkParams<'a> {
    const METHOD: &'static str = "Debugger.nextWasmDisassemblyChunk";
    type Response = NextWasmDisassemblyChunkReturns<'a>;
}

/// This command is deprecated. Use getScriptSource instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetWasmBytecodeParams<'a> {
    /// Id of the Wasm script to get source for.
    #[serde(rename = "scriptId")]
    script_id: crate::runtime::ScriptId<'a>,
}

impl<'a> GetWasmBytecodeParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `script_id`: Id of the Wasm script to get source for.
    pub fn builder(script_id: crate::runtime::ScriptId<'a>) -> GetWasmBytecodeParamsBuilder<'a> {
        GetWasmBytecodeParamsBuilder {
            script_id: script_id,
        }
    }
    /// Id of the Wasm script to get source for.
    pub fn script_id(&self) -> &crate::runtime::ScriptId<'a> { &self.script_id }
}


pub struct GetWasmBytecodeParamsBuilder<'a> {
    script_id: crate::runtime::ScriptId<'a>,
}

impl<'a> GetWasmBytecodeParamsBuilder<'a> {
    pub fn build(self) -> GetWasmBytecodeParams<'a> {
        GetWasmBytecodeParams {
            script_id: self.script_id,
        }
    }
}

/// This command is deprecated. Use getScriptSource instead.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetWasmBytecodeReturns<'a> {
    /// Script source. (Encoded as a base64 string when passed over JSON)
    bytecode: Cow<'a, str>,
}

impl<'a> GetWasmBytecodeReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `bytecode`: Script source. (Encoded as a base64 string when passed over JSON)
    pub fn builder(bytecode: impl Into<Cow<'a, str>>) -> GetWasmBytecodeReturnsBuilder<'a> {
        GetWasmBytecodeReturnsBuilder {
            bytecode: bytecode.into(),
        }
    }
    /// Script source. (Encoded as a base64 string when passed over JSON)
    pub fn bytecode(&self) -> &str { self.bytecode.as_ref() }
}


pub struct GetWasmBytecodeReturnsBuilder<'a> {
    bytecode: Cow<'a, str>,
}

impl<'a> GetWasmBytecodeReturnsBuilder<'a> {
    pub fn build(self) -> GetWasmBytecodeReturns<'a> {
        GetWasmBytecodeReturns {
            bytecode: self.bytecode,
        }
    }
}

impl<'a> GetWasmBytecodeParams<'a> { pub const METHOD: &'static str = "Debugger.getWasmBytecode"; }

impl<'a> crate::CdpCommand<'a> for GetWasmBytecodeParams<'a> {
    const METHOD: &'static str = "Debugger.getWasmBytecode";
    type Response = GetWasmBytecodeReturns<'a>;
}

/// Returns stack trace with given 'stackTraceId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetStackTraceParams<'a> {
    #[serde(rename = "stackTraceId")]
    stack_trace_id: crate::runtime::StackTraceId<'a>,
}

impl<'a> GetStackTraceParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `stack_trace_id`: 
    pub fn builder(stack_trace_id: crate::runtime::StackTraceId<'a>) -> GetStackTraceParamsBuilder<'a> {
        GetStackTraceParamsBuilder {
            stack_trace_id: stack_trace_id,
        }
    }
    pub fn stack_trace_id(&self) -> &crate::runtime::StackTraceId<'a> { &self.stack_trace_id }
}


pub struct GetStackTraceParamsBuilder<'a> {
    stack_trace_id: crate::runtime::StackTraceId<'a>,
}

impl<'a> GetStackTraceParamsBuilder<'a> {
    pub fn build(self) -> GetStackTraceParams<'a> {
        GetStackTraceParams {
            stack_trace_id: self.stack_trace_id,
        }
    }
}

/// Returns stack trace with given 'stackTraceId'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetStackTraceReturns<'a> {
    #[serde(rename = "stackTrace")]
    stack_trace: crate::runtime::StackTrace<'a>,
}

impl<'a> GetStackTraceReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `stack_trace`: 
    pub fn builder(stack_trace: crate::runtime::StackTrace<'a>) -> GetStackTraceReturnsBuilder<'a> {
        GetStackTraceReturnsBuilder {
            stack_trace: stack_trace,
        }
    }
    pub fn stack_trace(&self) -> &crate::runtime::StackTrace<'a> { &self.stack_trace }
}


pub struct GetStackTraceReturnsBuilder<'a> {
    stack_trace: crate::runtime::StackTrace<'a>,
}

impl<'a> GetStackTraceReturnsBuilder<'a> {
    pub fn build(self) -> GetStackTraceReturns<'a> {
        GetStackTraceReturns {
            stack_trace: self.stack_trace,
        }
    }
}

impl<'a> GetStackTraceParams<'a> { pub const METHOD: &'static str = "Debugger.getStackTrace"; }

impl<'a> crate::CdpCommand<'a> for GetStackTraceParams<'a> {
    const METHOD: &'static str = "Debugger.getStackTrace";
    type Response = GetStackTraceReturns<'a>;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PauseParams {}

impl PauseParams { pub const METHOD: &'static str = "Debugger.pause"; }

impl<'a> crate::CdpCommand<'a> for PauseParams {
    const METHOD: &'static str = "Debugger.pause";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PauseOnAsyncCallParams<'a> {
    /// Debugger will pause when async call with given stack trace is started.
    #[serde(rename = "parentStackTraceId")]
    parent_stack_trace_id: crate::runtime::StackTraceId<'a>,
}

impl<'a> PauseOnAsyncCallParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `parent_stack_trace_id`: Debugger will pause when async call with given stack trace is started.
    pub fn builder(parent_stack_trace_id: crate::runtime::StackTraceId<'a>) -> PauseOnAsyncCallParamsBuilder<'a> {
        PauseOnAsyncCallParamsBuilder {
            parent_stack_trace_id: parent_stack_trace_id,
        }
    }
    /// Debugger will pause when async call with given stack trace is started.
    pub fn parent_stack_trace_id(&self) -> &crate::runtime::StackTraceId<'a> { &self.parent_stack_trace_id }
}


pub struct PauseOnAsyncCallParamsBuilder<'a> {
    parent_stack_trace_id: crate::runtime::StackTraceId<'a>,
}

impl<'a> PauseOnAsyncCallParamsBuilder<'a> {
    pub fn build(self) -> PauseOnAsyncCallParams<'a> {
        PauseOnAsyncCallParams {
            parent_stack_trace_id: self.parent_stack_trace_id,
        }
    }
}

impl<'a> PauseOnAsyncCallParams<'a> { pub const METHOD: &'static str = "Debugger.pauseOnAsyncCall"; }

impl<'a> crate::CdpCommand<'a> for PauseOnAsyncCallParams<'a> {
    const METHOD: &'static str = "Debugger.pauseOnAsyncCall";
    type Response = crate::EmptyReturns;
}

/// Removes JavaScript breakpoint.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RemoveBreakpointParams<'a> {
    #[serde(rename = "breakpointId")]
    breakpoint_id: BreakpointId<'a>,
}

impl<'a> RemoveBreakpointParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `breakpoint_id`: 
    pub fn builder(breakpoint_id: impl Into<BreakpointId<'a>>) -> RemoveBreakpointParamsBuilder<'a> {
        RemoveBreakpointParamsBuilder {
            breakpoint_id: breakpoint_id.into(),
        }
    }
    pub fn breakpoint_id(&self) -> &BreakpointId<'a> { &self.breakpoint_id }
}


pub struct RemoveBreakpointParamsBuilder<'a> {
    breakpoint_id: BreakpointId<'a>,
}

impl<'a> RemoveBreakpointParamsBuilder<'a> {
    pub fn build(self) -> RemoveBreakpointParams<'a> {
        RemoveBreakpointParams {
            breakpoint_id: self.breakpoint_id,
        }
    }
}

impl<'a> RemoveBreakpointParams<'a> { pub const METHOD: &'static str = "Debugger.removeBreakpoint"; }

impl<'a> crate::CdpCommand<'a> for RemoveBreakpointParams<'a> {
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
pub struct RestartFrameParams<'a> {
    /// Call frame identifier to evaluate on.
    #[serde(rename = "callFrameId")]
    call_frame_id: CallFrameId<'a>,
    /// The 'mode' parameter must be present and set to 'StepInto', otherwise
    /// 'restartFrame' will error out.
    #[serde(skip_serializing_if = "Option::is_none")]
    mode: Option<Cow<'a, str>>,
}

impl<'a> RestartFrameParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `call_frame_id`: Call frame identifier to evaluate on.
    pub fn builder(call_frame_id: impl Into<CallFrameId<'a>>) -> RestartFrameParamsBuilder<'a> {
        RestartFrameParamsBuilder {
            call_frame_id: call_frame_id.into(),
            mode: None,
        }
    }
    /// Call frame identifier to evaluate on.
    pub fn call_frame_id(&self) -> &CallFrameId<'a> { &self.call_frame_id }
    /// The 'mode' parameter must be present and set to 'StepInto', otherwise
    /// 'restartFrame' will error out.
    pub fn mode(&self) -> Option<&str> { self.mode.as_deref() }
}


pub struct RestartFrameParamsBuilder<'a> {
    call_frame_id: CallFrameId<'a>,
    mode: Option<Cow<'a, str>>,
}

impl<'a> RestartFrameParamsBuilder<'a> {
    /// The 'mode' parameter must be present and set to 'StepInto', otherwise
    /// 'restartFrame' will error out.
    pub fn mode(mut self, mode: impl Into<Cow<'a, str>>) -> Self { self.mode = Some(mode.into()); self }
    pub fn build(self) -> RestartFrameParams<'a> {
        RestartFrameParams {
            call_frame_id: self.call_frame_id,
            mode: self.mode,
        }
    }
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
pub struct RestartFrameReturns<'a> {
    /// New stack trace.
    #[serde(rename = "callFrames")]
    call_frames: Vec<CallFrame<'a>>,
    /// Async stack trace, if any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "asyncStackTrace")]
    async_stack_trace: Option<crate::runtime::StackTrace<'a>>,
    /// Async stack trace, if any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "asyncStackTraceId")]
    async_stack_trace_id: Option<crate::runtime::StackTraceId<'a>>,
}

impl<'a> RestartFrameReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `call_frames`: New stack trace.
    pub fn builder(call_frames: Vec<CallFrame<'a>>) -> RestartFrameReturnsBuilder<'a> {
        RestartFrameReturnsBuilder {
            call_frames: call_frames,
            async_stack_trace: None,
            async_stack_trace_id: None,
        }
    }
    /// New stack trace.
    pub fn call_frames(&self) -> &[CallFrame<'a>] { &self.call_frames }
    /// Async stack trace, if any.
    pub fn async_stack_trace(&self) -> Option<&crate::runtime::StackTrace<'a>> { self.async_stack_trace.as_ref() }
    /// Async stack trace, if any.
    pub fn async_stack_trace_id(&self) -> Option<&crate::runtime::StackTraceId<'a>> { self.async_stack_trace_id.as_ref() }
}


pub struct RestartFrameReturnsBuilder<'a> {
    call_frames: Vec<CallFrame<'a>>,
    async_stack_trace: Option<crate::runtime::StackTrace<'a>>,
    async_stack_trace_id: Option<crate::runtime::StackTraceId<'a>>,
}

impl<'a> RestartFrameReturnsBuilder<'a> {
    /// Async stack trace, if any.
    pub fn async_stack_trace(mut self, async_stack_trace: crate::runtime::StackTrace<'a>) -> Self { self.async_stack_trace = Some(async_stack_trace); self }
    /// Async stack trace, if any.
    pub fn async_stack_trace_id(mut self, async_stack_trace_id: crate::runtime::StackTraceId<'a>) -> Self { self.async_stack_trace_id = Some(async_stack_trace_id); self }
    pub fn build(self) -> RestartFrameReturns<'a> {
        RestartFrameReturns {
            call_frames: self.call_frames,
            async_stack_trace: self.async_stack_trace,
            async_stack_trace_id: self.async_stack_trace_id,
        }
    }
}

impl<'a> RestartFrameParams<'a> { pub const METHOD: &'static str = "Debugger.restartFrame"; }

impl<'a> crate::CdpCommand<'a> for RestartFrameParams<'a> {
    const METHOD: &'static str = "Debugger.restartFrame";
    type Response = RestartFrameReturns<'a>;
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
    #[serde(skip_serializing_if = "Option::is_none", rename = "terminateOnResume")]
    terminate_on_resume: Option<bool>,
}

impl ResumeParams {
    /// Creates a builder for this type.
    pub fn builder() -> ResumeParamsBuilder {
        ResumeParamsBuilder {
            terminate_on_resume: None,
        }
    }
    /// Set to true to terminate execution upon resuming execution. In contrast
    /// to Runtime.terminateExecution, this will allows to execute further
    /// JavaScript (i.e. via evaluation) until execution of the paused code
    /// is actually resumed, at which point termination is triggered.
    /// If execution is currently not paused, this parameter has no effect.
    pub fn terminate_on_resume(&self) -> Option<bool> { self.terminate_on_resume }
}

#[derive(Default)]
pub struct ResumeParamsBuilder {
    terminate_on_resume: Option<bool>,
}

impl ResumeParamsBuilder {
    /// Set to true to terminate execution upon resuming execution. In contrast
    /// to Runtime.terminateExecution, this will allows to execute further
    /// JavaScript (i.e. via evaluation) until execution of the paused code
    /// is actually resumed, at which point termination is triggered.
    /// If execution is currently not paused, this parameter has no effect.
    pub fn terminate_on_resume(mut self, terminate_on_resume: bool) -> Self { self.terminate_on_resume = Some(terminate_on_resume); self }
    pub fn build(self) -> ResumeParams {
        ResumeParams {
            terminate_on_resume: self.terminate_on_resume,
        }
    }
}

impl ResumeParams { pub const METHOD: &'static str = "Debugger.resume"; }

impl<'a> crate::CdpCommand<'a> for ResumeParams {
    const METHOD: &'static str = "Debugger.resume";
    type Response = crate::EmptyReturns;
}

/// Searches for given string in script content.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchInContentParams<'a> {
    /// Id of the script to search in.
    #[serde(rename = "scriptId")]
    script_id: crate::runtime::ScriptId<'a>,
    /// String to search for.
    query: Cow<'a, str>,
    /// If true, search is case sensitive.
    #[serde(skip_serializing_if = "Option::is_none", rename = "caseSensitive")]
    case_sensitive: Option<bool>,
    /// If true, treats string parameter as regex.
    #[serde(skip_serializing_if = "Option::is_none", rename = "isRegex")]
    is_regex: Option<bool>,
}

impl<'a> SearchInContentParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `script_id`: Id of the script to search in.
    /// * `query`: String to search for.
    pub fn builder(script_id: crate::runtime::ScriptId<'a>, query: impl Into<Cow<'a, str>>) -> SearchInContentParamsBuilder<'a> {
        SearchInContentParamsBuilder {
            script_id: script_id,
            query: query.into(),
            case_sensitive: None,
            is_regex: None,
        }
    }
    /// Id of the script to search in.
    pub fn script_id(&self) -> &crate::runtime::ScriptId<'a> { &self.script_id }
    /// String to search for.
    pub fn query(&self) -> &str { self.query.as_ref() }
    /// If true, search is case sensitive.
    pub fn case_sensitive(&self) -> Option<bool> { self.case_sensitive }
    /// If true, treats string parameter as regex.
    pub fn is_regex(&self) -> Option<bool> { self.is_regex }
}


pub struct SearchInContentParamsBuilder<'a> {
    script_id: crate::runtime::ScriptId<'a>,
    query: Cow<'a, str>,
    case_sensitive: Option<bool>,
    is_regex: Option<bool>,
}

impl<'a> SearchInContentParamsBuilder<'a> {
    /// If true, search is case sensitive.
    pub fn case_sensitive(mut self, case_sensitive: bool) -> Self { self.case_sensitive = Some(case_sensitive); self }
    /// If true, treats string parameter as regex.
    pub fn is_regex(mut self, is_regex: bool) -> Self { self.is_regex = Some(is_regex); self }
    pub fn build(self) -> SearchInContentParams<'a> {
        SearchInContentParams {
            script_id: self.script_id,
            query: self.query,
            case_sensitive: self.case_sensitive,
            is_regex: self.is_regex,
        }
    }
}

/// Searches for given string in script content.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SearchInContentReturns<'a> {
    /// List of search matches.
    result: Vec<SearchMatch<'a>>,
}

impl<'a> SearchInContentReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `result`: List of search matches.
    pub fn builder(result: Vec<SearchMatch<'a>>) -> SearchInContentReturnsBuilder<'a> {
        SearchInContentReturnsBuilder {
            result: result,
        }
    }
    /// List of search matches.
    pub fn result(&self) -> &[SearchMatch<'a>] { &self.result }
}


pub struct SearchInContentReturnsBuilder<'a> {
    result: Vec<SearchMatch<'a>>,
}

impl<'a> SearchInContentReturnsBuilder<'a> {
    pub fn build(self) -> SearchInContentReturns<'a> {
        SearchInContentReturns {
            result: self.result,
        }
    }
}

impl<'a> SearchInContentParams<'a> { pub const METHOD: &'static str = "Debugger.searchInContent"; }

impl<'a> crate::CdpCommand<'a> for SearchInContentParams<'a> {
    const METHOD: &'static str = "Debugger.searchInContent";
    type Response = SearchInContentReturns<'a>;
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

impl SetAsyncCallStackDepthParams { pub const METHOD: &'static str = "Debugger.setAsyncCallStackDepth"; }

impl<'a> crate::CdpCommand<'a> for SetAsyncCallStackDepthParams {
    const METHOD: &'static str = "Debugger.setAsyncCallStackDepth";
    type Response = crate::EmptyReturns;
}

/// Replace previous blackbox execution contexts with passed ones. Forces backend to skip
/// stepping/pausing in scripts in these execution contexts. VM will try to leave blackboxed script by
/// performing 'step in' several times, finally resorting to 'step out' if unsuccessful.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetBlackboxExecutionContextsParams<'a> {
    /// Array of execution context unique ids for the debugger to ignore.
    #[serde(rename = "uniqueIds")]
    unique_ids: Vec<Cow<'a, str>>,
}

impl<'a> SetBlackboxExecutionContextsParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `unique_ids`: Array of execution context unique ids for the debugger to ignore.
    pub fn builder(unique_ids: Vec<Cow<'a, str>>) -> SetBlackboxExecutionContextsParamsBuilder<'a> {
        SetBlackboxExecutionContextsParamsBuilder {
            unique_ids: unique_ids,
        }
    }
    /// Array of execution context unique ids for the debugger to ignore.
    pub fn unique_ids(&self) -> &[Cow<'a, str>] { &self.unique_ids }
}


pub struct SetBlackboxExecutionContextsParamsBuilder<'a> {
    unique_ids: Vec<Cow<'a, str>>,
}

impl<'a> SetBlackboxExecutionContextsParamsBuilder<'a> {
    pub fn build(self) -> SetBlackboxExecutionContextsParams<'a> {
        SetBlackboxExecutionContextsParams {
            unique_ids: self.unique_ids,
        }
    }
}

impl<'a> SetBlackboxExecutionContextsParams<'a> { pub const METHOD: &'static str = "Debugger.setBlackboxExecutionContexts"; }

impl<'a> crate::CdpCommand<'a> for SetBlackboxExecutionContextsParams<'a> {
    const METHOD: &'static str = "Debugger.setBlackboxExecutionContexts";
    type Response = crate::EmptyReturns;
}

/// Replace previous blackbox patterns with passed ones. Forces backend to skip stepping/pausing in
/// scripts with url matching one of the patterns. VM will try to leave blackboxed script by
/// performing 'step in' several times, finally resorting to 'step out' if unsuccessful.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetBlackboxPatternsParams<'a> {
    /// Array of regexps that will be used to check script url for blackbox state.
    patterns: Vec<Cow<'a, str>>,
    /// If true, also ignore scripts with no source url.
    #[serde(skip_serializing_if = "Option::is_none", rename = "skipAnonymous")]
    skip_anonymous: Option<bool>,
}

impl<'a> SetBlackboxPatternsParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `patterns`: Array of regexps that will be used to check script url for blackbox state.
    pub fn builder(patterns: Vec<Cow<'a, str>>) -> SetBlackboxPatternsParamsBuilder<'a> {
        SetBlackboxPatternsParamsBuilder {
            patterns: patterns,
            skip_anonymous: None,
        }
    }
    /// Array of regexps that will be used to check script url for blackbox state.
    pub fn patterns(&self) -> &[Cow<'a, str>] { &self.patterns }
    /// If true, also ignore scripts with no source url.
    pub fn skip_anonymous(&self) -> Option<bool> { self.skip_anonymous }
}


pub struct SetBlackboxPatternsParamsBuilder<'a> {
    patterns: Vec<Cow<'a, str>>,
    skip_anonymous: Option<bool>,
}

impl<'a> SetBlackboxPatternsParamsBuilder<'a> {
    /// If true, also ignore scripts with no source url.
    pub fn skip_anonymous(mut self, skip_anonymous: bool) -> Self { self.skip_anonymous = Some(skip_anonymous); self }
    pub fn build(self) -> SetBlackboxPatternsParams<'a> {
        SetBlackboxPatternsParams {
            patterns: self.patterns,
            skip_anonymous: self.skip_anonymous,
        }
    }
}

impl<'a> SetBlackboxPatternsParams<'a> { pub const METHOD: &'static str = "Debugger.setBlackboxPatterns"; }

impl<'a> crate::CdpCommand<'a> for SetBlackboxPatternsParams<'a> {
    const METHOD: &'static str = "Debugger.setBlackboxPatterns";
    type Response = crate::EmptyReturns;
}

/// Makes backend skip steps in the script in blackboxed ranges. VM will try leave blacklisted
/// scripts by performing 'step in' several times, finally resorting to 'step out' if unsuccessful.
/// Positions array contains positions where blackbox state is changed. First interval isn't
/// blackboxed. Array should be sorted.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetBlackboxedRangesParams<'a> {
    /// Id of the script.
    #[serde(rename = "scriptId")]
    script_id: crate::runtime::ScriptId<'a>,
    positions: Vec<ScriptPosition>,
}

impl<'a> SetBlackboxedRangesParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `script_id`: Id of the script.
    /// * `positions`: 
    pub fn builder(script_id: crate::runtime::ScriptId<'a>, positions: Vec<ScriptPosition>) -> SetBlackboxedRangesParamsBuilder<'a> {
        SetBlackboxedRangesParamsBuilder {
            script_id: script_id,
            positions: positions,
        }
    }
    /// Id of the script.
    pub fn script_id(&self) -> &crate::runtime::ScriptId<'a> { &self.script_id }
    pub fn positions(&self) -> &[ScriptPosition] { &self.positions }
}


pub struct SetBlackboxedRangesParamsBuilder<'a> {
    script_id: crate::runtime::ScriptId<'a>,
    positions: Vec<ScriptPosition>,
}

impl<'a> SetBlackboxedRangesParamsBuilder<'a> {
    pub fn build(self) -> SetBlackboxedRangesParams<'a> {
        SetBlackboxedRangesParams {
            script_id: self.script_id,
            positions: self.positions,
        }
    }
}

impl<'a> SetBlackboxedRangesParams<'a> { pub const METHOD: &'static str = "Debugger.setBlackboxedRanges"; }

impl<'a> crate::CdpCommand<'a> for SetBlackboxedRangesParams<'a> {
    const METHOD: &'static str = "Debugger.setBlackboxedRanges";
    type Response = crate::EmptyReturns;
}

/// Sets JavaScript breakpoint at a given location.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakpointParams<'a> {
    /// Location to set breakpoint in.
    location: Location<'a>,
    /// Expression to use as a breakpoint condition. When specified, debugger will only stop on the
    /// breakpoint if this expression evaluates to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<Cow<'a, str>>,
}

impl<'a> SetBreakpointParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `location`: Location to set breakpoint in.
    pub fn builder(location: Location<'a>) -> SetBreakpointParamsBuilder<'a> {
        SetBreakpointParamsBuilder {
            location: location,
            condition: None,
        }
    }
    /// Location to set breakpoint in.
    pub fn location(&self) -> &Location<'a> { &self.location }
    /// Expression to use as a breakpoint condition. When specified, debugger will only stop on the
    /// breakpoint if this expression evaluates to true.
    pub fn condition(&self) -> Option<&str> { self.condition.as_deref() }
}


pub struct SetBreakpointParamsBuilder<'a> {
    location: Location<'a>,
    condition: Option<Cow<'a, str>>,
}

impl<'a> SetBreakpointParamsBuilder<'a> {
    /// Expression to use as a breakpoint condition. When specified, debugger will only stop on the
    /// breakpoint if this expression evaluates to true.
    pub fn condition(mut self, condition: impl Into<Cow<'a, str>>) -> Self { self.condition = Some(condition.into()); self }
    pub fn build(self) -> SetBreakpointParams<'a> {
        SetBreakpointParams {
            location: self.location,
            condition: self.condition,
        }
    }
}

/// Sets JavaScript breakpoint at a given location.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakpointReturns<'a> {
    /// Id of the created breakpoint for further reference.
    #[serde(rename = "breakpointId")]
    breakpoint_id: BreakpointId<'a>,
    /// Location this breakpoint resolved into.
    #[serde(rename = "actualLocation")]
    actual_location: Location<'a>,
}

impl<'a> SetBreakpointReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `breakpoint_id`: Id of the created breakpoint for further reference.
    /// * `actual_location`: Location this breakpoint resolved into.
    pub fn builder(breakpoint_id: impl Into<BreakpointId<'a>>, actual_location: Location<'a>) -> SetBreakpointReturnsBuilder<'a> {
        SetBreakpointReturnsBuilder {
            breakpoint_id: breakpoint_id.into(),
            actual_location: actual_location,
        }
    }
    /// Id of the created breakpoint for further reference.
    pub fn breakpoint_id(&self) -> &BreakpointId<'a> { &self.breakpoint_id }
    /// Location this breakpoint resolved into.
    pub fn actual_location(&self) -> &Location<'a> { &self.actual_location }
}


pub struct SetBreakpointReturnsBuilder<'a> {
    breakpoint_id: BreakpointId<'a>,
    actual_location: Location<'a>,
}

impl<'a> SetBreakpointReturnsBuilder<'a> {
    pub fn build(self) -> SetBreakpointReturns<'a> {
        SetBreakpointReturns {
            breakpoint_id: self.breakpoint_id,
            actual_location: self.actual_location,
        }
    }
}

impl<'a> SetBreakpointParams<'a> { pub const METHOD: &'static str = "Debugger.setBreakpoint"; }

impl<'a> crate::CdpCommand<'a> for SetBreakpointParams<'a> {
    const METHOD: &'static str = "Debugger.setBreakpoint";
    type Response = SetBreakpointReturns<'a>;
}

/// Sets instrumentation breakpoint.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetInstrumentationBreakpointParams<'a> {
    /// Instrumentation name.
    instrumentation: Cow<'a, str>,
}

impl<'a> SetInstrumentationBreakpointParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `instrumentation`: Instrumentation name.
    pub fn builder(instrumentation: impl Into<Cow<'a, str>>) -> SetInstrumentationBreakpointParamsBuilder<'a> {
        SetInstrumentationBreakpointParamsBuilder {
            instrumentation: instrumentation.into(),
        }
    }
    /// Instrumentation name.
    pub fn instrumentation(&self) -> &str { self.instrumentation.as_ref() }
}


pub struct SetInstrumentationBreakpointParamsBuilder<'a> {
    instrumentation: Cow<'a, str>,
}

impl<'a> SetInstrumentationBreakpointParamsBuilder<'a> {
    pub fn build(self) -> SetInstrumentationBreakpointParams<'a> {
        SetInstrumentationBreakpointParams {
            instrumentation: self.instrumentation,
        }
    }
}

/// Sets instrumentation breakpoint.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetInstrumentationBreakpointReturns<'a> {
    /// Id of the created breakpoint for further reference.
    #[serde(rename = "breakpointId")]
    breakpoint_id: BreakpointId<'a>,
}

impl<'a> SetInstrumentationBreakpointReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `breakpoint_id`: Id of the created breakpoint for further reference.
    pub fn builder(breakpoint_id: impl Into<BreakpointId<'a>>) -> SetInstrumentationBreakpointReturnsBuilder<'a> {
        SetInstrumentationBreakpointReturnsBuilder {
            breakpoint_id: breakpoint_id.into(),
        }
    }
    /// Id of the created breakpoint for further reference.
    pub fn breakpoint_id(&self) -> &BreakpointId<'a> { &self.breakpoint_id }
}


pub struct SetInstrumentationBreakpointReturnsBuilder<'a> {
    breakpoint_id: BreakpointId<'a>,
}

impl<'a> SetInstrumentationBreakpointReturnsBuilder<'a> {
    pub fn build(self) -> SetInstrumentationBreakpointReturns<'a> {
        SetInstrumentationBreakpointReturns {
            breakpoint_id: self.breakpoint_id,
        }
    }
}

impl<'a> SetInstrumentationBreakpointParams<'a> { pub const METHOD: &'static str = "Debugger.setInstrumentationBreakpoint"; }

impl<'a> crate::CdpCommand<'a> for SetInstrumentationBreakpointParams<'a> {
    const METHOD: &'static str = "Debugger.setInstrumentationBreakpoint";
    type Response = SetInstrumentationBreakpointReturns<'a>;
}

/// Sets JavaScript breakpoint at given location specified either by URL or URL regex. Once this
/// command is issued, all existing parsed scripts will have breakpoints resolved and returned in
/// 'locations' property. Further matching script parsing will result in subsequent
/// 'breakpointResolved' events issued. This logical breakpoint will survive page reloads.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakpointByUrlParams<'a> {
    /// Line number to set breakpoint at.
    #[serde(rename = "lineNumber")]
    line_number: i64,
    /// URL of the resources to set breakpoint on.
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<Cow<'a, str>>,
    /// Regex pattern for the URLs of the resources to set breakpoints on. Either 'url' or
    /// 'urlRegex' must be specified.
    #[serde(skip_serializing_if = "Option::is_none", rename = "urlRegex")]
    url_regex: Option<Cow<'a, str>>,
    /// Script hash of the resources to set breakpoint on.
    #[serde(skip_serializing_if = "Option::is_none", rename = "scriptHash")]
    script_hash: Option<Cow<'a, str>>,
    /// Offset in the line to set breakpoint at.
    #[serde(skip_serializing_if = "Option::is_none", rename = "columnNumber")]
    column_number: Option<i64>,
    /// Expression to use as a breakpoint condition. When specified, debugger will only stop on the
    /// breakpoint if this expression evaluates to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<Cow<'a, str>>,
}

impl<'a> SetBreakpointByUrlParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `line_number`: Line number to set breakpoint at.
    pub fn builder(line_number: i64) -> SetBreakpointByUrlParamsBuilder<'a> {
        SetBreakpointByUrlParamsBuilder {
            line_number: line_number,
            url: None,
            url_regex: None,
            script_hash: None,
            column_number: None,
            condition: None,
        }
    }
    /// Line number to set breakpoint at.
    pub fn line_number(&self) -> i64 { self.line_number }
    /// URL of the resources to set breakpoint on.
    pub fn url(&self) -> Option<&str> { self.url.as_deref() }
    /// Regex pattern for the URLs of the resources to set breakpoints on. Either 'url' or
    /// 'urlRegex' must be specified.
    pub fn url_regex(&self) -> Option<&str> { self.url_regex.as_deref() }
    /// Script hash of the resources to set breakpoint on.
    pub fn script_hash(&self) -> Option<&str> { self.script_hash.as_deref() }
    /// Offset in the line to set breakpoint at.
    pub fn column_number(&self) -> Option<i64> { self.column_number }
    /// Expression to use as a breakpoint condition. When specified, debugger will only stop on the
    /// breakpoint if this expression evaluates to true.
    pub fn condition(&self) -> Option<&str> { self.condition.as_deref() }
}


pub struct SetBreakpointByUrlParamsBuilder<'a> {
    line_number: i64,
    url: Option<Cow<'a, str>>,
    url_regex: Option<Cow<'a, str>>,
    script_hash: Option<Cow<'a, str>>,
    column_number: Option<i64>,
    condition: Option<Cow<'a, str>>,
}

impl<'a> SetBreakpointByUrlParamsBuilder<'a> {
    /// URL of the resources to set breakpoint on.
    pub fn url(mut self, url: impl Into<Cow<'a, str>>) -> Self { self.url = Some(url.into()); self }
    /// Regex pattern for the URLs of the resources to set breakpoints on. Either 'url' or
    /// 'urlRegex' must be specified.
    pub fn url_regex(mut self, url_regex: impl Into<Cow<'a, str>>) -> Self { self.url_regex = Some(url_regex.into()); self }
    /// Script hash of the resources to set breakpoint on.
    pub fn script_hash(mut self, script_hash: impl Into<Cow<'a, str>>) -> Self { self.script_hash = Some(script_hash.into()); self }
    /// Offset in the line to set breakpoint at.
    pub fn column_number(mut self, column_number: i64) -> Self { self.column_number = Some(column_number); self }
    /// Expression to use as a breakpoint condition. When specified, debugger will only stop on the
    /// breakpoint if this expression evaluates to true.
    pub fn condition(mut self, condition: impl Into<Cow<'a, str>>) -> Self { self.condition = Some(condition.into()); self }
    pub fn build(self) -> SetBreakpointByUrlParams<'a> {
        SetBreakpointByUrlParams {
            line_number: self.line_number,
            url: self.url,
            url_regex: self.url_regex,
            script_hash: self.script_hash,
            column_number: self.column_number,
            condition: self.condition,
        }
    }
}

/// Sets JavaScript breakpoint at given location specified either by URL or URL regex. Once this
/// command is issued, all existing parsed scripts will have breakpoints resolved and returned in
/// 'locations' property. Further matching script parsing will result in subsequent
/// 'breakpointResolved' events issued. This logical breakpoint will survive page reloads.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakpointByUrlReturns<'a> {
    /// Id of the created breakpoint for further reference.
    #[serde(rename = "breakpointId")]
    breakpoint_id: BreakpointId<'a>,
    /// List of the locations this breakpoint resolved into upon addition.
    locations: Vec<Location<'a>>,
}

impl<'a> SetBreakpointByUrlReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `breakpoint_id`: Id of the created breakpoint for further reference.
    /// * `locations`: List of the locations this breakpoint resolved into upon addition.
    pub fn builder(breakpoint_id: impl Into<BreakpointId<'a>>, locations: Vec<Location<'a>>) -> SetBreakpointByUrlReturnsBuilder<'a> {
        SetBreakpointByUrlReturnsBuilder {
            breakpoint_id: breakpoint_id.into(),
            locations: locations,
        }
    }
    /// Id of the created breakpoint for further reference.
    pub fn breakpoint_id(&self) -> &BreakpointId<'a> { &self.breakpoint_id }
    /// List of the locations this breakpoint resolved into upon addition.
    pub fn locations(&self) -> &[Location<'a>] { &self.locations }
}


pub struct SetBreakpointByUrlReturnsBuilder<'a> {
    breakpoint_id: BreakpointId<'a>,
    locations: Vec<Location<'a>>,
}

impl<'a> SetBreakpointByUrlReturnsBuilder<'a> {
    pub fn build(self) -> SetBreakpointByUrlReturns<'a> {
        SetBreakpointByUrlReturns {
            breakpoint_id: self.breakpoint_id,
            locations: self.locations,
        }
    }
}

impl<'a> SetBreakpointByUrlParams<'a> { pub const METHOD: &'static str = "Debugger.setBreakpointByUrl"; }

impl<'a> crate::CdpCommand<'a> for SetBreakpointByUrlParams<'a> {
    const METHOD: &'static str = "Debugger.setBreakpointByUrl";
    type Response = SetBreakpointByUrlReturns<'a>;
}

/// Sets JavaScript breakpoint before each call to the given function.
/// If another function was created from the same source as a given one,
/// calling it will also trigger the breakpoint.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakpointOnFunctionCallParams<'a> {
    /// Function object id.
    #[serde(rename = "objectId")]
    object_id: crate::runtime::RemoteObjectId<'a>,
    /// Expression to use as a breakpoint condition. When specified, debugger will
    /// stop on the breakpoint if this expression evaluates to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<Cow<'a, str>>,
}

impl<'a> SetBreakpointOnFunctionCallParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `object_id`: Function object id.
    pub fn builder(object_id: crate::runtime::RemoteObjectId<'a>) -> SetBreakpointOnFunctionCallParamsBuilder<'a> {
        SetBreakpointOnFunctionCallParamsBuilder {
            object_id: object_id,
            condition: None,
        }
    }
    /// Function object id.
    pub fn object_id(&self) -> &crate::runtime::RemoteObjectId<'a> { &self.object_id }
    /// Expression to use as a breakpoint condition. When specified, debugger will
    /// stop on the breakpoint if this expression evaluates to true.
    pub fn condition(&self) -> Option<&str> { self.condition.as_deref() }
}


pub struct SetBreakpointOnFunctionCallParamsBuilder<'a> {
    object_id: crate::runtime::RemoteObjectId<'a>,
    condition: Option<Cow<'a, str>>,
}

impl<'a> SetBreakpointOnFunctionCallParamsBuilder<'a> {
    /// Expression to use as a breakpoint condition. When specified, debugger will
    /// stop on the breakpoint if this expression evaluates to true.
    pub fn condition(mut self, condition: impl Into<Cow<'a, str>>) -> Self { self.condition = Some(condition.into()); self }
    pub fn build(self) -> SetBreakpointOnFunctionCallParams<'a> {
        SetBreakpointOnFunctionCallParams {
            object_id: self.object_id,
            condition: self.condition,
        }
    }
}

/// Sets JavaScript breakpoint before each call to the given function.
/// If another function was created from the same source as a given one,
/// calling it will also trigger the breakpoint.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakpointOnFunctionCallReturns<'a> {
    /// Id of the created breakpoint for further reference.
    #[serde(rename = "breakpointId")]
    breakpoint_id: BreakpointId<'a>,
}

impl<'a> SetBreakpointOnFunctionCallReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `breakpoint_id`: Id of the created breakpoint for further reference.
    pub fn builder(breakpoint_id: impl Into<BreakpointId<'a>>) -> SetBreakpointOnFunctionCallReturnsBuilder<'a> {
        SetBreakpointOnFunctionCallReturnsBuilder {
            breakpoint_id: breakpoint_id.into(),
        }
    }
    /// Id of the created breakpoint for further reference.
    pub fn breakpoint_id(&self) -> &BreakpointId<'a> { &self.breakpoint_id }
}


pub struct SetBreakpointOnFunctionCallReturnsBuilder<'a> {
    breakpoint_id: BreakpointId<'a>,
}

impl<'a> SetBreakpointOnFunctionCallReturnsBuilder<'a> {
    pub fn build(self) -> SetBreakpointOnFunctionCallReturns<'a> {
        SetBreakpointOnFunctionCallReturns {
            breakpoint_id: self.breakpoint_id,
        }
    }
}

impl<'a> SetBreakpointOnFunctionCallParams<'a> { pub const METHOD: &'static str = "Debugger.setBreakpointOnFunctionCall"; }

impl<'a> crate::CdpCommand<'a> for SetBreakpointOnFunctionCallParams<'a> {
    const METHOD: &'static str = "Debugger.setBreakpointOnFunctionCall";
    type Response = SetBreakpointOnFunctionCallReturns<'a>;
}

/// Activates / deactivates all breakpoints on the page.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetBreakpointsActiveParams {
    /// New value for breakpoints active state.
    active: bool,
}

impl SetBreakpointsActiveParams {
    /// Creates a builder for this type with the required parameters:
    /// * `active`: New value for breakpoints active state.
    pub fn builder(active: bool) -> SetBreakpointsActiveParamsBuilder {
        SetBreakpointsActiveParamsBuilder {
            active: active,
        }
    }
    /// New value for breakpoints active state.
    pub fn active(&self) -> bool { self.active }
}


pub struct SetBreakpointsActiveParamsBuilder {
    active: bool,
}

impl SetBreakpointsActiveParamsBuilder {
    pub fn build(self) -> SetBreakpointsActiveParams {
        SetBreakpointsActiveParams {
            active: self.active,
        }
    }
}

impl SetBreakpointsActiveParams { pub const METHOD: &'static str = "Debugger.setBreakpointsActive"; }

impl<'a> crate::CdpCommand<'a> for SetBreakpointsActiveParams {
    const METHOD: &'static str = "Debugger.setBreakpointsActive";
    type Response = crate::EmptyReturns;
}

/// Defines pause on exceptions state. Can be set to stop on all exceptions, uncaught exceptions,
/// or caught exceptions, no exceptions. Initial pause on exceptions state is 'none'.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetPauseOnExceptionsParams<'a> {
    /// Pause on exceptions mode.
    state: Cow<'a, str>,
}

impl<'a> SetPauseOnExceptionsParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `state`: Pause on exceptions mode.
    pub fn builder(state: impl Into<Cow<'a, str>>) -> SetPauseOnExceptionsParamsBuilder<'a> {
        SetPauseOnExceptionsParamsBuilder {
            state: state.into(),
        }
    }
    /// Pause on exceptions mode.
    pub fn state(&self) -> &str { self.state.as_ref() }
}


pub struct SetPauseOnExceptionsParamsBuilder<'a> {
    state: Cow<'a, str>,
}

impl<'a> SetPauseOnExceptionsParamsBuilder<'a> {
    pub fn build(self) -> SetPauseOnExceptionsParams<'a> {
        SetPauseOnExceptionsParams {
            state: self.state,
        }
    }
}

impl<'a> SetPauseOnExceptionsParams<'a> { pub const METHOD: &'static str = "Debugger.setPauseOnExceptions"; }

impl<'a> crate::CdpCommand<'a> for SetPauseOnExceptionsParams<'a> {
    const METHOD: &'static str = "Debugger.setPauseOnExceptions";
    type Response = crate::EmptyReturns;
}

/// Changes return value in top frame. Available only at return break position.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetReturnValueParams<'a> {
    /// New return value.
    #[serde(rename = "newValue")]
    new_value: crate::runtime::CallArgument<'a>,
}

impl<'a> SetReturnValueParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `new_value`: New return value.
    pub fn builder(new_value: crate::runtime::CallArgument<'a>) -> SetReturnValueParamsBuilder<'a> {
        SetReturnValueParamsBuilder {
            new_value: new_value,
        }
    }
    /// New return value.
    pub fn new_value(&self) -> &crate::runtime::CallArgument<'a> { &self.new_value }
}


pub struct SetReturnValueParamsBuilder<'a> {
    new_value: crate::runtime::CallArgument<'a>,
}

impl<'a> SetReturnValueParamsBuilder<'a> {
    pub fn build(self) -> SetReturnValueParams<'a> {
        SetReturnValueParams {
            new_value: self.new_value,
        }
    }
}

impl<'a> SetReturnValueParams<'a> { pub const METHOD: &'static str = "Debugger.setReturnValue"; }

impl<'a> crate::CdpCommand<'a> for SetReturnValueParams<'a> {
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
pub struct SetScriptSourceParams<'a> {
    /// Id of the script to edit.
    #[serde(rename = "scriptId")]
    script_id: crate::runtime::ScriptId<'a>,
    /// New content of the script.
    #[serde(rename = "scriptSource")]
    script_source: Cow<'a, str>,
    /// If true the change will not actually be applied. Dry run may be used to get result
    /// description without actually modifying the code.
    #[serde(skip_serializing_if = "Option::is_none", rename = "dryRun")]
    dry_run: Option<bool>,
    /// If true, then 'scriptSource' is allowed to change the function on top of the stack
    /// as long as the top-most stack frame is the only activation of that function.
    #[serde(skip_serializing_if = "Option::is_none", rename = "allowTopFrameEditing")]
    allow_top_frame_editing: Option<bool>,
}

impl<'a> SetScriptSourceParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `script_id`: Id of the script to edit.
    /// * `script_source`: New content of the script.
    pub fn builder(script_id: crate::runtime::ScriptId<'a>, script_source: impl Into<Cow<'a, str>>) -> SetScriptSourceParamsBuilder<'a> {
        SetScriptSourceParamsBuilder {
            script_id: script_id,
            script_source: script_source.into(),
            dry_run: None,
            allow_top_frame_editing: None,
        }
    }
    /// Id of the script to edit.
    pub fn script_id(&self) -> &crate::runtime::ScriptId<'a> { &self.script_id }
    /// New content of the script.
    pub fn script_source(&self) -> &str { self.script_source.as_ref() }
    /// If true the change will not actually be applied. Dry run may be used to get result
    /// description without actually modifying the code.
    pub fn dry_run(&self) -> Option<bool> { self.dry_run }
    /// If true, then 'scriptSource' is allowed to change the function on top of the stack
    /// as long as the top-most stack frame is the only activation of that function.
    pub fn allow_top_frame_editing(&self) -> Option<bool> { self.allow_top_frame_editing }
}


pub struct SetScriptSourceParamsBuilder<'a> {
    script_id: crate::runtime::ScriptId<'a>,
    script_source: Cow<'a, str>,
    dry_run: Option<bool>,
    allow_top_frame_editing: Option<bool>,
}

impl<'a> SetScriptSourceParamsBuilder<'a> {
    /// If true the change will not actually be applied. Dry run may be used to get result
    /// description without actually modifying the code.
    pub fn dry_run(mut self, dry_run: bool) -> Self { self.dry_run = Some(dry_run); self }
    /// If true, then 'scriptSource' is allowed to change the function on top of the stack
    /// as long as the top-most stack frame is the only activation of that function.
    pub fn allow_top_frame_editing(mut self, allow_top_frame_editing: bool) -> Self { self.allow_top_frame_editing = Some(allow_top_frame_editing); self }
    pub fn build(self) -> SetScriptSourceParams<'a> {
        SetScriptSourceParams {
            script_id: self.script_id,
            script_source: self.script_source,
            dry_run: self.dry_run,
            allow_top_frame_editing: self.allow_top_frame_editing,
        }
    }
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
pub struct SetScriptSourceReturns<'a> {
    /// New stack trace in case editing has happened while VM was stopped.
    #[serde(skip_serializing_if = "Option::is_none", rename = "callFrames")]
    call_frames: Option<Vec<CallFrame<'a>>>,
    /// Whether current call stack  was modified after applying the changes.
    #[serde(skip_serializing_if = "Option::is_none", rename = "stackChanged")]
    stack_changed: Option<bool>,
    /// Async stack trace, if any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "asyncStackTrace")]
    async_stack_trace: Option<crate::runtime::StackTrace<'a>>,
    /// Async stack trace, if any.
    #[serde(skip_serializing_if = "Option::is_none", rename = "asyncStackTraceId")]
    async_stack_trace_id: Option<crate::runtime::StackTraceId<'a>>,
    /// Whether the operation was successful or not. Only 'Ok' denotes a
    /// successful live edit while the other enum variants denote why
    /// the live edit failed.
    status: Cow<'a, str>,
    /// Exception details if any. Only present when 'status' is 'CompileError'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "exceptionDetails")]
    exception_details: Option<crate::runtime::ExceptionDetails<'a>>,
}

impl<'a> SetScriptSourceReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `status`: Whether the operation was successful or not. Only `Ok` denotes a successful live edit while the other enum variants denote why the live edit failed.
    pub fn builder(status: impl Into<Cow<'a, str>>) -> SetScriptSourceReturnsBuilder<'a> {
        SetScriptSourceReturnsBuilder {
            call_frames: None,
            stack_changed: None,
            async_stack_trace: None,
            async_stack_trace_id: None,
            status: status.into(),
            exception_details: None,
        }
    }
    /// New stack trace in case editing has happened while VM was stopped.
    pub fn call_frames(&self) -> Option<&[CallFrame<'a>]> { self.call_frames.as_deref() }
    /// Whether current call stack  was modified after applying the changes.
    pub fn stack_changed(&self) -> Option<bool> { self.stack_changed }
    /// Async stack trace, if any.
    pub fn async_stack_trace(&self) -> Option<&crate::runtime::StackTrace<'a>> { self.async_stack_trace.as_ref() }
    /// Async stack trace, if any.
    pub fn async_stack_trace_id(&self) -> Option<&crate::runtime::StackTraceId<'a>> { self.async_stack_trace_id.as_ref() }
    /// Whether the operation was successful or not. Only 'Ok' denotes a
    /// successful live edit while the other enum variants denote why
    /// the live edit failed.
    pub fn status(&self) -> &str { self.status.as_ref() }
    /// Exception details if any. Only present when 'status' is 'CompileError'.
    pub fn exception_details(&self) -> Option<&crate::runtime::ExceptionDetails<'a>> { self.exception_details.as_ref() }
}


pub struct SetScriptSourceReturnsBuilder<'a> {
    call_frames: Option<Vec<CallFrame<'a>>>,
    stack_changed: Option<bool>,
    async_stack_trace: Option<crate::runtime::StackTrace<'a>>,
    async_stack_trace_id: Option<crate::runtime::StackTraceId<'a>>,
    status: Cow<'a, str>,
    exception_details: Option<crate::runtime::ExceptionDetails<'a>>,
}

impl<'a> SetScriptSourceReturnsBuilder<'a> {
    /// New stack trace in case editing has happened while VM was stopped.
    pub fn call_frames(mut self, call_frames: Vec<CallFrame<'a>>) -> Self { self.call_frames = Some(call_frames); self }
    /// Whether current call stack  was modified after applying the changes.
    pub fn stack_changed(mut self, stack_changed: bool) -> Self { self.stack_changed = Some(stack_changed); self }
    /// Async stack trace, if any.
    pub fn async_stack_trace(mut self, async_stack_trace: crate::runtime::StackTrace<'a>) -> Self { self.async_stack_trace = Some(async_stack_trace); self }
    /// Async stack trace, if any.
    pub fn async_stack_trace_id(mut self, async_stack_trace_id: crate::runtime::StackTraceId<'a>) -> Self { self.async_stack_trace_id = Some(async_stack_trace_id); self }
    /// Exception details if any. Only present when 'status' is 'CompileError'.
    pub fn exception_details(mut self, exception_details: crate::runtime::ExceptionDetails<'a>) -> Self { self.exception_details = Some(exception_details); self }
    pub fn build(self) -> SetScriptSourceReturns<'a> {
        SetScriptSourceReturns {
            call_frames: self.call_frames,
            stack_changed: self.stack_changed,
            async_stack_trace: self.async_stack_trace,
            async_stack_trace_id: self.async_stack_trace_id,
            status: self.status,
            exception_details: self.exception_details,
        }
    }
}

impl<'a> SetScriptSourceParams<'a> { pub const METHOD: &'static str = "Debugger.setScriptSource"; }

impl<'a> crate::CdpCommand<'a> for SetScriptSourceParams<'a> {
    const METHOD: &'static str = "Debugger.setScriptSource";
    type Response = SetScriptSourceReturns<'a>;
}

/// Makes page not interrupt on any pauses (breakpoint, exception, dom exception etc).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetSkipAllPausesParams {
    /// New value for skip pauses state.
    skip: bool,
}

impl SetSkipAllPausesParams {
    /// Creates a builder for this type with the required parameters:
    /// * `skip`: New value for skip pauses state.
    pub fn builder(skip: bool) -> SetSkipAllPausesParamsBuilder {
        SetSkipAllPausesParamsBuilder {
            skip: skip,
        }
    }
    /// New value for skip pauses state.
    pub fn skip(&self) -> bool { self.skip }
}


pub struct SetSkipAllPausesParamsBuilder {
    skip: bool,
}

impl SetSkipAllPausesParamsBuilder {
    pub fn build(self) -> SetSkipAllPausesParams {
        SetSkipAllPausesParams {
            skip: self.skip,
        }
    }
}

impl SetSkipAllPausesParams { pub const METHOD: &'static str = "Debugger.setSkipAllPauses"; }

impl<'a> crate::CdpCommand<'a> for SetSkipAllPausesParams {
    const METHOD: &'static str = "Debugger.setSkipAllPauses";
    type Response = crate::EmptyReturns;
}

/// Changes value of variable in a callframe. Object-based scopes are not supported and must be
/// mutated manually.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetVariableValueParams<'a> {
    /// 0-based number of scope as was listed in scope chain. Only 'local', 'closure' and 'catch'
    /// scope types are allowed. Other scopes could be manipulated manually.
    #[serde(rename = "scopeNumber")]
    scope_number: i64,
    /// Variable name.
    #[serde(rename = "variableName")]
    variable_name: Cow<'a, str>,
    /// New variable value.
    #[serde(rename = "newValue")]
    new_value: crate::runtime::CallArgument<'a>,
    /// Id of callframe that holds variable.
    #[serde(rename = "callFrameId")]
    call_frame_id: CallFrameId<'a>,
}

impl<'a> SetVariableValueParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `scope_number`: 0-based number of scope as was listed in scope chain. Only 'local', 'closure' and 'catch' scope types are allowed. Other scopes could be manipulated manually.
    /// * `variable_name`: Variable name.
    /// * `new_value`: New variable value.
    /// * `call_frame_id`: Id of callframe that holds variable.
    pub fn builder(scope_number: i64, variable_name: impl Into<Cow<'a, str>>, new_value: crate::runtime::CallArgument<'a>, call_frame_id: impl Into<CallFrameId<'a>>) -> SetVariableValueParamsBuilder<'a> {
        SetVariableValueParamsBuilder {
            scope_number: scope_number,
            variable_name: variable_name.into(),
            new_value: new_value,
            call_frame_id: call_frame_id.into(),
        }
    }
    /// 0-based number of scope as was listed in scope chain. Only 'local', 'closure' and 'catch'
    /// scope types are allowed. Other scopes could be manipulated manually.
    pub fn scope_number(&self) -> i64 { self.scope_number }
    /// Variable name.
    pub fn variable_name(&self) -> &str { self.variable_name.as_ref() }
    /// New variable value.
    pub fn new_value(&self) -> &crate::runtime::CallArgument<'a> { &self.new_value }
    /// Id of callframe that holds variable.
    pub fn call_frame_id(&self) -> &CallFrameId<'a> { &self.call_frame_id }
}


pub struct SetVariableValueParamsBuilder<'a> {
    scope_number: i64,
    variable_name: Cow<'a, str>,
    new_value: crate::runtime::CallArgument<'a>,
    call_frame_id: CallFrameId<'a>,
}

impl<'a> SetVariableValueParamsBuilder<'a> {
    pub fn build(self) -> SetVariableValueParams<'a> {
        SetVariableValueParams {
            scope_number: self.scope_number,
            variable_name: self.variable_name,
            new_value: self.new_value,
            call_frame_id: self.call_frame_id,
        }
    }
}

impl<'a> SetVariableValueParams<'a> { pub const METHOD: &'static str = "Debugger.setVariableValue"; }

impl<'a> crate::CdpCommand<'a> for SetVariableValueParams<'a> {
    const METHOD: &'static str = "Debugger.setVariableValue";
    type Response = crate::EmptyReturns;
}

/// Steps into the function call.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StepIntoParams<'a> {
    /// Debugger will pause on the execution of the first async task which was scheduled
    /// before next pause.
    #[serde(skip_serializing_if = "Option::is_none", rename = "breakOnAsyncCall")]
    break_on_async_call: Option<bool>,
    /// The skipList specifies location ranges that should be skipped on step into.
    #[serde(skip_serializing_if = "Option::is_none", rename = "skipList")]
    skip_list: Option<Vec<LocationRange<'a>>>,
}

impl<'a> StepIntoParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> StepIntoParamsBuilder<'a> {
        StepIntoParamsBuilder {
            break_on_async_call: None,
            skip_list: None,
        }
    }
    /// Debugger will pause on the execution of the first async task which was scheduled
    /// before next pause.
    pub fn break_on_async_call(&self) -> Option<bool> { self.break_on_async_call }
    /// The skipList specifies location ranges that should be skipped on step into.
    pub fn skip_list(&self) -> Option<&[LocationRange<'a>]> { self.skip_list.as_deref() }
}

#[derive(Default)]
pub struct StepIntoParamsBuilder<'a> {
    break_on_async_call: Option<bool>,
    skip_list: Option<Vec<LocationRange<'a>>>,
}

impl<'a> StepIntoParamsBuilder<'a> {
    /// Debugger will pause on the execution of the first async task which was scheduled
    /// before next pause.
    pub fn break_on_async_call(mut self, break_on_async_call: bool) -> Self { self.break_on_async_call = Some(break_on_async_call); self }
    /// The skipList specifies location ranges that should be skipped on step into.
    pub fn skip_list(mut self, skip_list: Vec<LocationRange<'a>>) -> Self { self.skip_list = Some(skip_list); self }
    pub fn build(self) -> StepIntoParams<'a> {
        StepIntoParams {
            break_on_async_call: self.break_on_async_call,
            skip_list: self.skip_list,
        }
    }
}

impl<'a> StepIntoParams<'a> { pub const METHOD: &'static str = "Debugger.stepInto"; }

impl<'a> crate::CdpCommand<'a> for StepIntoParams<'a> {
    const METHOD: &'static str = "Debugger.stepInto";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StepOutParams {}

impl StepOutParams { pub const METHOD: &'static str = "Debugger.stepOut"; }

impl<'a> crate::CdpCommand<'a> for StepOutParams {
    const METHOD: &'static str = "Debugger.stepOut";
    type Response = crate::EmptyReturns;
}

/// Steps over the statement.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StepOverParams<'a> {
    /// The skipList specifies location ranges that should be skipped on step over.
    #[serde(skip_serializing_if = "Option::is_none", rename = "skipList")]
    skip_list: Option<Vec<LocationRange<'a>>>,
}

impl<'a> StepOverParams<'a> {
    /// Creates a builder for this type.
    pub fn builder() -> StepOverParamsBuilder<'a> {
        StepOverParamsBuilder {
            skip_list: None,
        }
    }
    /// The skipList specifies location ranges that should be skipped on step over.
    pub fn skip_list(&self) -> Option<&[LocationRange<'a>]> { self.skip_list.as_deref() }
}

#[derive(Default)]
pub struct StepOverParamsBuilder<'a> {
    skip_list: Option<Vec<LocationRange<'a>>>,
}

impl<'a> StepOverParamsBuilder<'a> {
    /// The skipList specifies location ranges that should be skipped on step over.
    pub fn skip_list(mut self, skip_list: Vec<LocationRange<'a>>) -> Self { self.skip_list = Some(skip_list); self }
    pub fn build(self) -> StepOverParams<'a> {
        StepOverParams {
            skip_list: self.skip_list,
        }
    }
}

impl<'a> StepOverParams<'a> { pub const METHOD: &'static str = "Debugger.stepOver"; }

impl<'a> crate::CdpCommand<'a> for StepOverParams<'a> {
    const METHOD: &'static str = "Debugger.stepOver";
    type Response = crate::EmptyReturns;
}
