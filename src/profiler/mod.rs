use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Profile node. Holds callsite information, execution statistics and child nodes.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ProfileNode<'a> {
    /// Unique id of the node.
    pub id: u64,
    /// Function location.
    #[serde(rename = "callFrame")]
    pub call_frame: crate::runtime::CallFrame<'a>,
    /// Number of samples where this node was on top of the call stack.
    #[serde(skip_serializing_if = "Option::is_none", rename = "hitCount")]
    pub hit_count: Option<u64>,
    /// Child node ids.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<i64>>,
    /// The reason of being not optimized. The function may be deoptimized or marked as don't
    /// optimize.
    #[serde(skip_serializing_if = "Option::is_none", rename = "deoptReason")]
    pub deopt_reason: Option<Cow<'a, str>>,
    /// An array of source position ticks.
    #[serde(skip_serializing_if = "Option::is_none", rename = "positionTicks")]
    pub position_ticks: Option<Vec<PositionTickInfo>>,
}
/// Profile.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct Profile<'a> {
    /// The list of profile nodes. First item is the root node.
    pub nodes: Vec<ProfileNode<'a>>,
    /// Profiling start timestamp in microseconds.
    #[serde(rename = "startTime")]
    pub start_time: f64,
    /// Profiling end timestamp in microseconds.
    #[serde(rename = "endTime")]
    pub end_time: f64,
    /// Ids of samples top nodes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub samples: Option<Vec<i64>>,
    /// Time intervals between adjacent samples in microseconds. The first delta is relative to the
    /// profile startTime.
    #[serde(skip_serializing_if = "Option::is_none", rename = "timeDeltas")]
    pub time_deltas: Option<Vec<i64>>,
}
/// Specifies a number of samples attributed to a certain source position.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct PositionTickInfo {
    /// Source line number (1-based).
    pub line: i64,
    /// Number of samples attributed to the source line.
    pub ticks: i64,
}
/// Coverage data for a source range.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct CoverageRange {
    /// JavaScript script source offset for the range start.
    #[serde(rename = "startOffset")]
    pub start_offset: i32,
    /// JavaScript script source offset for the range end.
    #[serde(rename = "endOffset")]
    pub end_offset: i32,
    /// Collected execution count of the source range.
    pub count: u64,
}
/// Coverage data for a JavaScript function.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCoverage<'a> {
    /// JavaScript function name.
    #[serde(rename = "functionName")]
    pub function_name: Cow<'a, str>,
    /// Source ranges inside the function with coverage data.
    pub ranges: Vec<CoverageRange>,
    /// Whether coverage data for this function has block granularity.
    #[serde(rename = "isBlockCoverage")]
    pub is_block_coverage: bool,
}
/// Coverage data for a JavaScript script.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct ScriptCoverage<'a> {
    /// JavaScript script id.
    #[serde(rename = "scriptId")]
    pub script_id: crate::runtime::ScriptId<'a>,
    /// JavaScript script name or url.
    pub url: Cow<'a, str>,
    /// Functions contained in the script that has coverage data.
    pub functions: Vec<FunctionCoverage<'a>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Profiler.disable")]
pub struct DisableParams {

}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Profiler.enable")]
pub struct EnableParams {

}
/// Collect coverage data for the current isolate. The coverage data may be incomplete due to
/// garbage collection.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Profiler.getBestEffortCoverage", response = "GetBestEffortCoverageReturns<'a>")]
pub struct GetBestEffortCoverageParams {

}
/// Collect coverage data for the current isolate. The coverage data may be incomplete due to
/// garbage collection.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetBestEffortCoverageReturns<'a> {
    /// Coverage data for the current isolate.
    pub result: Vec<ScriptCoverage<'a>>,
}
/// Changes CPU profiler sampling interval. Must be called before CPU profiles recording started.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Profiler.setSamplingInterval")]
pub struct SetSamplingIntervalParams {
    /// New sampling interval in microseconds.
    pub interval: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Profiler.start")]
pub struct StartParams {

}
/// Enable precise code coverage. Coverage data for JavaScript executed before enabling precise code
/// coverage may be incomplete. Enabling prevents running optimized code and resets execution
/// counters.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Profiler.startPreciseCoverage", response = "StartPreciseCoverageReturns")]
pub struct StartPreciseCoverageParams {
    /// Collect accurate call counts beyond simple 'covered' or 'not covered'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "callCount")]
    pub call_count: Option<bool>,
    /// Collect block-based coverage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed: Option<bool>,
    /// Allow the backend to send updates on its own initiative
    #[serde(skip_serializing_if = "Option::is_none", rename = "allowTriggeredUpdates")]
    pub allow_triggered_updates: Option<bool>,
}
/// Enable precise code coverage. Coverage data for JavaScript executed before enabling precise code
/// coverage may be incomplete. Enabling prevents running optimized code and resets execution
/// counters.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct StartPreciseCoverageReturns {
    /// Monotonically increasing time (in seconds) when the coverage update was taken in the backend.
    pub timestamp: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Profiler.stop", response = "StopReturns<'a>")]
pub struct StopParams {

}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct StopReturns<'a> {
    /// Recorded profile.
    pub profile: Profile<'a>,
}
/// Disable precise code coverage. Disabling releases unnecessary execution count records and allows
/// executing optimized code.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Profiler.stopPreciseCoverage")]
pub struct StopPreciseCoverageParams {

}
/// Collect coverage data for the current isolate, and resets execution counters. Precise code
/// coverage needs to have started.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Profiler.takePreciseCoverage", response = "TakePreciseCoverageReturns<'a>")]
pub struct TakePreciseCoverageParams {

}
/// Collect coverage data for the current isolate, and resets execution counters. Precise code
/// coverage needs to have started.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct TakePreciseCoverageReturns<'a> {
    /// Coverage data for the current isolate.
    pub result: Vec<ScriptCoverage<'a>>,
    /// Monotonically increasing time (in seconds) when the coverage update was taken in the backend.
    pub timestamp: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Profiler.consoleProfileFinished")]
pub struct ConsoleProfileFinished<'a> {
    pub id: Cow<'a, str>,
    /// Location of console.profileEnd().
    pub location: crate::debugger::Location<'a>,
    pub profile: Profile<'a>,
    /// Profile title passed as an argument to console.profile().
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Cow<'a, str>>,
}
/// Sent when new profile recording is started using console.profile() call.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Profiler.consoleProfileStarted")]
pub struct ConsoleProfileStarted<'a> {
    pub id: Cow<'a, str>,
    /// Location of console.profile().
    pub location: crate::debugger::Location<'a>,
    /// Profile title passed as an argument to console.profile().
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Cow<'a, str>>,
}
/// Reports coverage delta since the last poll (either from an event like this, or from
/// 'takePreciseCoverage' for the current isolate. May only be sent if precise code
/// coverage has been started. This event can be trigged by the embedder to, for example,
/// trigger collection of coverage data immediately at a certain point in time.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "Profiler.preciseCoverageDeltaUpdate")]
pub struct PreciseCoverageDeltaUpdate<'a> {
    /// Monotonically increasing time (in seconds) when the coverage update was taken in the backend.
    pub timestamp: f64,
    /// Identifier for distinguishing coverage events.
    pub occasion: Cow<'a, str>,
    /// Coverage data for the current isolate.
    pub result: Vec<ScriptCoverage<'a>>,
}