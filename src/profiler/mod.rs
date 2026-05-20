use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// Profile node. Holds callsite information, execution statistics and child nodes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProfileNode<'a> {
    /// Unique id of the node.
    id: u64,
    /// Function location.
    #[serde(rename = "callFrame")]
    call_frame: crate::runtime::CallFrame<'a>,
    /// Number of samples where this node was on top of the call stack.
    #[serde(skip_serializing_if = "Option::is_none", rename = "hitCount")]
    hit_count: Option<u64>,
    /// Child node ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    children: Option<Vec<i64>>,
    /// The reason of being not optimized. The function may be deoptimized or marked as don't
    /// optimize.
    #[serde(skip_serializing_if = "Option::is_none", rename = "deoptReason")]
    deopt_reason: Option<Cow<'a, str>>,
    /// An array of source position ticks.
    #[serde(skip_serializing_if = "Option::is_none", rename = "positionTicks")]
    position_ticks: Option<Vec<PositionTickInfo>>,
}

impl<'a> ProfileNode<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `id`: Unique id of the node.
    /// * `call_frame`: Function location.
    pub fn builder(id: u64, call_frame: crate::runtime::CallFrame<'a>) -> ProfileNodeBuilder<'a> {
        ProfileNodeBuilder {
            id: id,
            call_frame: call_frame,
            hit_count: None,
            children: None,
            deopt_reason: None,
            position_ticks: None,
        }
    }
    /// Unique id of the node.
    pub fn id(&self) -> u64 { self.id }
    /// Function location.
    pub fn call_frame(&self) -> &crate::runtime::CallFrame<'a> { &self.call_frame }
    /// Number of samples where this node was on top of the call stack.
    pub fn hit_count(&self) -> Option<u64> { self.hit_count }
    /// Child node ids.
    pub fn children(&self) -> Option<&[i64]> { self.children.as_deref() }
    /// The reason of being not optimized. The function may be deoptimized or marked as don't
    /// optimize.
    pub fn deopt_reason(&self) -> Option<&str> { self.deopt_reason.as_deref() }
    /// An array of source position ticks.
    pub fn position_ticks(&self) -> Option<&[PositionTickInfo]> { self.position_ticks.as_deref() }
}


pub struct ProfileNodeBuilder<'a> {
    id: u64,
    call_frame: crate::runtime::CallFrame<'a>,
    hit_count: Option<u64>,
    children: Option<Vec<i64>>,
    deopt_reason: Option<Cow<'a, str>>,
    position_ticks: Option<Vec<PositionTickInfo>>,
}

impl<'a> ProfileNodeBuilder<'a> {
    /// Number of samples where this node was on top of the call stack.
    pub fn hit_count(mut self, hit_count: u64) -> Self { self.hit_count = Some(hit_count); self }
    /// Child node ids.
    pub fn children(mut self, children: Vec<i64>) -> Self { self.children = Some(children); self }
    /// The reason of being not optimized. The function may be deoptimized or marked as don't
    /// optimize.
    pub fn deopt_reason(mut self, deopt_reason: impl Into<Cow<'a, str>>) -> Self { self.deopt_reason = Some(deopt_reason.into()); self }
    /// An array of source position ticks.
    pub fn position_ticks(mut self, position_ticks: Vec<PositionTickInfo>) -> Self { self.position_ticks = Some(position_ticks); self }
    pub fn build(self) -> ProfileNode<'a> {
        ProfileNode {
            id: self.id,
            call_frame: self.call_frame,
            hit_count: self.hit_count,
            children: self.children,
            deopt_reason: self.deopt_reason,
            position_ticks: self.position_ticks,
        }
    }
}

/// Profile.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Profile<'a> {
    /// The list of profile nodes. First item is the root node.
    nodes: Vec<ProfileNode<'a>>,
    /// Profiling start timestamp in microseconds.
    #[serde(rename = "startTime")]
    start_time: f64,
    /// Profiling end timestamp in microseconds.
    #[serde(rename = "endTime")]
    end_time: f64,
    /// Ids of samples top nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    samples: Option<Vec<i64>>,
    /// Time intervals between adjacent samples in microseconds. The first delta is relative to the
    /// profile startTime.
    #[serde(skip_serializing_if = "Option::is_none", rename = "timeDeltas")]
    time_deltas: Option<Vec<i64>>,
}

impl<'a> Profile<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `nodes`: The list of profile nodes. First item is the root node.
    /// * `start_time`: Profiling start timestamp in microseconds.
    /// * `end_time`: Profiling end timestamp in microseconds.
    pub fn builder(nodes: Vec<ProfileNode<'a>>, start_time: f64, end_time: f64) -> ProfileBuilder<'a> {
        ProfileBuilder {
            nodes: nodes,
            start_time: start_time,
            end_time: end_time,
            samples: None,
            time_deltas: None,
        }
    }
    /// The list of profile nodes. First item is the root node.
    pub fn nodes(&self) -> &[ProfileNode<'a>] { &self.nodes }
    /// Profiling start timestamp in microseconds.
    pub fn start_time(&self) -> f64 { self.start_time }
    /// Profiling end timestamp in microseconds.
    pub fn end_time(&self) -> f64 { self.end_time }
    /// Ids of samples top nodes.
    pub fn samples(&self) -> Option<&[i64]> { self.samples.as_deref() }
    /// Time intervals between adjacent samples in microseconds. The first delta is relative to the
    /// profile startTime.
    pub fn time_deltas(&self) -> Option<&[i64]> { self.time_deltas.as_deref() }
}


pub struct ProfileBuilder<'a> {
    nodes: Vec<ProfileNode<'a>>,
    start_time: f64,
    end_time: f64,
    samples: Option<Vec<i64>>,
    time_deltas: Option<Vec<i64>>,
}

impl<'a> ProfileBuilder<'a> {
    /// Ids of samples top nodes.
    pub fn samples(mut self, samples: Vec<i64>) -> Self { self.samples = Some(samples); self }
    /// Time intervals between adjacent samples in microseconds. The first delta is relative to the
    /// profile startTime.
    pub fn time_deltas(mut self, time_deltas: Vec<i64>) -> Self { self.time_deltas = Some(time_deltas); self }
    pub fn build(self) -> Profile<'a> {
        Profile {
            nodes: self.nodes,
            start_time: self.start_time,
            end_time: self.end_time,
            samples: self.samples,
            time_deltas: self.time_deltas,
        }
    }
}

/// Specifies a number of samples attributed to a certain source position.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PositionTickInfo {
    /// Source line number (1-based).
    line: i64,
    /// Number of samples attributed to the source line.
    ticks: i64,
}

impl PositionTickInfo {
    /// Creates a builder for this type with the required parameters:
    /// * `line`: Source line number (1-based).
    /// * `ticks`: Number of samples attributed to the source line.
    pub fn builder(line: i64, ticks: i64) -> PositionTickInfoBuilder {
        PositionTickInfoBuilder {
            line: line,
            ticks: ticks,
        }
    }
    /// Source line number (1-based).
    pub fn line(&self) -> i64 { self.line }
    /// Number of samples attributed to the source line.
    pub fn ticks(&self) -> i64 { self.ticks }
}


pub struct PositionTickInfoBuilder {
    line: i64,
    ticks: i64,
}

impl PositionTickInfoBuilder {
    pub fn build(self) -> PositionTickInfo {
        PositionTickInfo {
            line: self.line,
            ticks: self.ticks,
        }
    }
}

/// Coverage data for a source range.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CoverageRange {
    /// JavaScript script source offset for the range start.
    #[serde(rename = "startOffset")]
    start_offset: i32,
    /// JavaScript script source offset for the range end.
    #[serde(rename = "endOffset")]
    end_offset: i32,
    /// Collected execution count of the source range.
    count: u64,
}

impl CoverageRange {
    /// Creates a builder for this type with the required parameters:
    /// * `start_offset`: JavaScript script source offset for the range start.
    /// * `end_offset`: JavaScript script source offset for the range end.
    /// * `count`: Collected execution count of the source range.
    pub fn builder(start_offset: i32, end_offset: i32, count: u64) -> CoverageRangeBuilder {
        CoverageRangeBuilder {
            start_offset: start_offset,
            end_offset: end_offset,
            count: count,
        }
    }
    /// JavaScript script source offset for the range start.
    pub fn start_offset(&self) -> i32 { self.start_offset }
    /// JavaScript script source offset for the range end.
    pub fn end_offset(&self) -> i32 { self.end_offset }
    /// Collected execution count of the source range.
    pub fn count(&self) -> u64 { self.count }
}


pub struct CoverageRangeBuilder {
    start_offset: i32,
    end_offset: i32,
    count: u64,
}

impl CoverageRangeBuilder {
    pub fn build(self) -> CoverageRange {
        CoverageRange {
            start_offset: self.start_offset,
            end_offset: self.end_offset,
            count: self.count,
        }
    }
}

/// Coverage data for a JavaScript function.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCoverage<'a> {
    /// JavaScript function name.
    #[serde(rename = "functionName")]
    function_name: Cow<'a, str>,
    /// Source ranges inside the function with coverage data.
    ranges: Vec<CoverageRange>,
    /// Whether coverage data for this function has block granularity.
    #[serde(rename = "isBlockCoverage")]
    is_block_coverage: bool,
}

impl<'a> FunctionCoverage<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `function_name`: JavaScript function name.
    /// * `ranges`: Source ranges inside the function with coverage data.
    /// * `is_block_coverage`: Whether coverage data for this function has block granularity.
    pub fn builder(function_name: impl Into<Cow<'a, str>>, ranges: Vec<CoverageRange>, is_block_coverage: bool) -> FunctionCoverageBuilder<'a> {
        FunctionCoverageBuilder {
            function_name: function_name.into(),
            ranges: ranges,
            is_block_coverage: is_block_coverage,
        }
    }
    /// JavaScript function name.
    pub fn function_name(&self) -> &str { self.function_name.as_ref() }
    /// Source ranges inside the function with coverage data.
    pub fn ranges(&self) -> &[CoverageRange] { &self.ranges }
    /// Whether coverage data for this function has block granularity.
    pub fn is_block_coverage(&self) -> bool { self.is_block_coverage }
}


pub struct FunctionCoverageBuilder<'a> {
    function_name: Cow<'a, str>,
    ranges: Vec<CoverageRange>,
    is_block_coverage: bool,
}

impl<'a> FunctionCoverageBuilder<'a> {
    pub fn build(self) -> FunctionCoverage<'a> {
        FunctionCoverage {
            function_name: self.function_name,
            ranges: self.ranges,
            is_block_coverage: self.is_block_coverage,
        }
    }
}

/// Coverage data for a JavaScript script.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScriptCoverage<'a> {
    /// JavaScript script id.
    #[serde(rename = "scriptId")]
    script_id: crate::runtime::ScriptId<'a>,
    /// JavaScript script name or url.
    url: Cow<'a, str>,
    /// Functions contained in the script that has coverage data.
    functions: Vec<FunctionCoverage<'a>>,
}

impl<'a> ScriptCoverage<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `script_id`: JavaScript script id.
    /// * `url`: JavaScript script name or url.
    /// * `functions`: Functions contained in the script that has coverage data.
    pub fn builder(script_id: crate::runtime::ScriptId<'a>, url: impl Into<Cow<'a, str>>, functions: Vec<FunctionCoverage<'a>>) -> ScriptCoverageBuilder<'a> {
        ScriptCoverageBuilder {
            script_id: script_id,
            url: url.into(),
            functions: functions,
        }
    }
    /// JavaScript script id.
    pub fn script_id(&self) -> &crate::runtime::ScriptId<'a> { &self.script_id }
    /// JavaScript script name or url.
    pub fn url(&self) -> &str { self.url.as_ref() }
    /// Functions contained in the script that has coverage data.
    pub fn functions(&self) -> &[FunctionCoverage<'a>] { &self.functions }
}


pub struct ScriptCoverageBuilder<'a> {
    script_id: crate::runtime::ScriptId<'a>,
    url: Cow<'a, str>,
    functions: Vec<FunctionCoverage<'a>>,
}

impl<'a> ScriptCoverageBuilder<'a> {
    pub fn build(self) -> ScriptCoverage<'a> {
        ScriptCoverage {
            script_id: self.script_id,
            url: self.url,
            functions: self.functions,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "Profiler.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "Profiler.disable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams { pub const METHOD: &'static str = "Profiler.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "Profiler.enable";
    type Response = crate::EmptyReturns;
}

/// Collect coverage data for the current isolate. The coverage data may be incomplete due to
/// garbage collection.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetBestEffortCoverageReturns<'a> {
    /// Coverage data for the current isolate.
    result: Vec<ScriptCoverage<'a>>,
}

impl<'a> GetBestEffortCoverageReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `result`: Coverage data for the current isolate.
    pub fn builder(result: Vec<ScriptCoverage<'a>>) -> GetBestEffortCoverageReturnsBuilder<'a> {
        GetBestEffortCoverageReturnsBuilder {
            result: result,
        }
    }
    /// Coverage data for the current isolate.
    pub fn result(&self) -> &[ScriptCoverage<'a>] { &self.result }
}


pub struct GetBestEffortCoverageReturnsBuilder<'a> {
    result: Vec<ScriptCoverage<'a>>,
}

impl<'a> GetBestEffortCoverageReturnsBuilder<'a> {
    pub fn build(self) -> GetBestEffortCoverageReturns<'a> {
        GetBestEffortCoverageReturns {
            result: self.result,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetBestEffortCoverageParams {}

impl GetBestEffortCoverageParams { pub const METHOD: &'static str = "Profiler.getBestEffortCoverage"; }

impl<'a> crate::CdpCommand<'a> for GetBestEffortCoverageParams {
    const METHOD: &'static str = "Profiler.getBestEffortCoverage";
    type Response = GetBestEffortCoverageReturns<'a>;
}

/// Changes CPU profiler sampling interval. Must be called before CPU profiles recording started.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetSamplingIntervalParams {
    /// New sampling interval in microseconds.
    interval: i64,
}

impl SetSamplingIntervalParams {
    /// Creates a builder for this type with the required parameters:
    /// * `interval`: New sampling interval in microseconds.
    pub fn builder(interval: i64) -> SetSamplingIntervalParamsBuilder {
        SetSamplingIntervalParamsBuilder {
            interval: interval,
        }
    }
    /// New sampling interval in microseconds.
    pub fn interval(&self) -> i64 { self.interval }
}


pub struct SetSamplingIntervalParamsBuilder {
    interval: i64,
}

impl SetSamplingIntervalParamsBuilder {
    pub fn build(self) -> SetSamplingIntervalParams {
        SetSamplingIntervalParams {
            interval: self.interval,
        }
    }
}

impl SetSamplingIntervalParams { pub const METHOD: &'static str = "Profiler.setSamplingInterval"; }

impl<'a> crate::CdpCommand<'a> for SetSamplingIntervalParams {
    const METHOD: &'static str = "Profiler.setSamplingInterval";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StartParams {}

impl StartParams { pub const METHOD: &'static str = "Profiler.start"; }

impl<'a> crate::CdpCommand<'a> for StartParams {
    const METHOD: &'static str = "Profiler.start";
    type Response = crate::EmptyReturns;
}

/// Enable precise code coverage. Coverage data for JavaScript executed before enabling precise code
/// coverage may be incomplete. Enabling prevents running optimized code and resets execution
/// counters.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartPreciseCoverageParams {
    /// Collect accurate call counts beyond simple 'covered' or 'not covered'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "callCount")]
    call_count: Option<bool>,
    /// Collect block-based coverage.
    #[serde(skip_serializing_if = "Option::is_none")]
    detailed: Option<bool>,
    /// Allow the backend to send updates on its own initiative
    #[serde(skip_serializing_if = "Option::is_none", rename = "allowTriggeredUpdates")]
    allow_triggered_updates: Option<bool>,
}

impl StartPreciseCoverageParams {
    /// Creates a builder for this type.
    pub fn builder() -> StartPreciseCoverageParamsBuilder {
        StartPreciseCoverageParamsBuilder {
            call_count: None,
            detailed: None,
            allow_triggered_updates: None,
        }
    }
    /// Collect accurate call counts beyond simple 'covered' or 'not covered'.
    pub fn call_count(&self) -> Option<bool> { self.call_count }
    /// Collect block-based coverage.
    pub fn detailed(&self) -> Option<bool> { self.detailed }
    /// Allow the backend to send updates on its own initiative
    pub fn allow_triggered_updates(&self) -> Option<bool> { self.allow_triggered_updates }
}

#[derive(Default)]
pub struct StartPreciseCoverageParamsBuilder {
    call_count: Option<bool>,
    detailed: Option<bool>,
    allow_triggered_updates: Option<bool>,
}

impl StartPreciseCoverageParamsBuilder {
    /// Collect accurate call counts beyond simple 'covered' or 'not covered'.
    pub fn call_count(mut self, call_count: bool) -> Self { self.call_count = Some(call_count); self }
    /// Collect block-based coverage.
    pub fn detailed(mut self, detailed: bool) -> Self { self.detailed = Some(detailed); self }
    /// Allow the backend to send updates on its own initiative
    pub fn allow_triggered_updates(mut self, allow_triggered_updates: bool) -> Self { self.allow_triggered_updates = Some(allow_triggered_updates); self }
    pub fn build(self) -> StartPreciseCoverageParams {
        StartPreciseCoverageParams {
            call_count: self.call_count,
            detailed: self.detailed,
            allow_triggered_updates: self.allow_triggered_updates,
        }
    }
}

/// Enable precise code coverage. Coverage data for JavaScript executed before enabling precise code
/// coverage may be incomplete. Enabling prevents running optimized code and resets execution
/// counters.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartPreciseCoverageReturns {
    /// Monotonically increasing time (in seconds) when the coverage update was taken in the backend.
    timestamp: f64,
}

impl StartPreciseCoverageReturns {
    /// Creates a builder for this type with the required parameters:
    /// * `timestamp`: Monotonically increasing time (in seconds) when the coverage update was taken in the backend.
    pub fn builder(timestamp: f64) -> StartPreciseCoverageReturnsBuilder {
        StartPreciseCoverageReturnsBuilder {
            timestamp: timestamp,
        }
    }
    /// Monotonically increasing time (in seconds) when the coverage update was taken in the backend.
    pub fn timestamp(&self) -> f64 { self.timestamp }
}


pub struct StartPreciseCoverageReturnsBuilder {
    timestamp: f64,
}

impl StartPreciseCoverageReturnsBuilder {
    pub fn build(self) -> StartPreciseCoverageReturns {
        StartPreciseCoverageReturns {
            timestamp: self.timestamp,
        }
    }
}

impl StartPreciseCoverageParams { pub const METHOD: &'static str = "Profiler.startPreciseCoverage"; }

impl<'a> crate::CdpCommand<'a> for StartPreciseCoverageParams {
    const METHOD: &'static str = "Profiler.startPreciseCoverage";
    type Response = StartPreciseCoverageReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StopReturns<'a> {
    /// Recorded profile.
    profile: Profile<'a>,
}

impl<'a> StopReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `profile`: Recorded profile.
    pub fn builder(profile: Profile<'a>) -> StopReturnsBuilder<'a> {
        StopReturnsBuilder {
            profile: profile,
        }
    }
    /// Recorded profile.
    pub fn profile(&self) -> &Profile<'a> { &self.profile }
}


pub struct StopReturnsBuilder<'a> {
    profile: Profile<'a>,
}

impl<'a> StopReturnsBuilder<'a> {
    pub fn build(self) -> StopReturns<'a> {
        StopReturns {
            profile: self.profile,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StopParams {}

impl StopParams { pub const METHOD: &'static str = "Profiler.stop"; }

impl<'a> crate::CdpCommand<'a> for StopParams {
    const METHOD: &'static str = "Profiler.stop";
    type Response = StopReturns<'a>;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StopPreciseCoverageParams {}

impl StopPreciseCoverageParams { pub const METHOD: &'static str = "Profiler.stopPreciseCoverage"; }

impl<'a> crate::CdpCommand<'a> for StopPreciseCoverageParams {
    const METHOD: &'static str = "Profiler.stopPreciseCoverage";
    type Response = crate::EmptyReturns;
}

/// Collect coverage data for the current isolate, and resets execution counters. Precise code
/// coverage needs to have started.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TakePreciseCoverageReturns<'a> {
    /// Coverage data for the current isolate.
    result: Vec<ScriptCoverage<'a>>,
    /// Monotonically increasing time (in seconds) when the coverage update was taken in the backend.
    timestamp: f64,
}

impl<'a> TakePreciseCoverageReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `result`: Coverage data for the current isolate.
    /// * `timestamp`: Monotonically increasing time (in seconds) when the coverage update was taken in the backend.
    pub fn builder(result: Vec<ScriptCoverage<'a>>, timestamp: f64) -> TakePreciseCoverageReturnsBuilder<'a> {
        TakePreciseCoverageReturnsBuilder {
            result: result,
            timestamp: timestamp,
        }
    }
    /// Coverage data for the current isolate.
    pub fn result(&self) -> &[ScriptCoverage<'a>] { &self.result }
    /// Monotonically increasing time (in seconds) when the coverage update was taken in the backend.
    pub fn timestamp(&self) -> f64 { self.timestamp }
}


pub struct TakePreciseCoverageReturnsBuilder<'a> {
    result: Vec<ScriptCoverage<'a>>,
    timestamp: f64,
}

impl<'a> TakePreciseCoverageReturnsBuilder<'a> {
    pub fn build(self) -> TakePreciseCoverageReturns<'a> {
        TakePreciseCoverageReturns {
            result: self.result,
            timestamp: self.timestamp,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TakePreciseCoverageParams {}

impl TakePreciseCoverageParams { pub const METHOD: &'static str = "Profiler.takePreciseCoverage"; }

impl<'a> crate::CdpCommand<'a> for TakePreciseCoverageParams {
    const METHOD: &'static str = "Profiler.takePreciseCoverage";
    type Response = TakePreciseCoverageReturns<'a>;
}
