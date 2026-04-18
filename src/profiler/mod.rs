use serde::{Serialize, Deserialize};

/// Profile node. Holds callsite information, execution statistics and child nodes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProfileNode {
    /// Unique id of the node.

    pub id: u64,
    /// Function location.

    pub callFrame: crate::runtime::CallFrame,
    /// Number of samples where this node was on top of the call stack.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hitCount: Option<u64>,
    /// Child node ids.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<i64>>,
    /// The reason of being not optimized. The function may be deoptimized or marked as don't
    /// optimize.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deoptReason: Option<String>,
    /// An array of source position ticks.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub positionTicks: Option<Vec<PositionTickInfo>>,
}

/// Profile.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    /// The list of profile nodes. First item is the root node.

    pub nodes: Vec<ProfileNode>,
    /// Profiling start timestamp in microseconds.

    pub startTime: f64,
    /// Profiling end timestamp in microseconds.

    pub endTime: f64,
    /// Ids of samples top nodes.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub samples: Option<Vec<i64>>,
    /// Time intervals between adjacent samples in microseconds. The first delta is relative to the
    /// profile startTime.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeDeltas: Option<Vec<i64>>,
}

/// Specifies a number of samples attributed to a certain source position.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PositionTickInfo {
    /// Source line number (1-based).

    pub line: i32,
    /// Number of samples attributed to the source line.

    pub ticks: i64,
}

/// Coverage data for a source range.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CoverageRange {
    /// JavaScript script source offset for the range start.

    pub startOffset: i32,
    /// JavaScript script source offset for the range end.

    pub endOffset: i32,
    /// Collected execution count of the source range.

    pub count: u64,
}

/// Coverage data for a JavaScript function.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct FunctionCoverage {
    /// JavaScript function name.

    pub functionName: String,
    /// Source ranges inside the function with coverage data.

    pub ranges: Vec<CoverageRange>,
    /// Whether coverage data for this function has block granularity.

    pub isBlockCoverage: bool,
}

/// Coverage data for a JavaScript script.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ScriptCoverage {
    /// JavaScript script id.

    pub scriptId: crate::runtime::ScriptId,
    /// JavaScript script name or url.

    pub url: String,
    /// Functions contained in the script that has coverage data.

    pub functions: Vec<FunctionCoverage>,
}

/// Collect coverage data for the current isolate. The coverage data may be incomplete due to
/// garbage collection.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetBestEffortCoverageReturns {
    /// Coverage data for the current isolate.

    pub result: Vec<ScriptCoverage>,
}

/// Changes CPU profiler sampling interval. Must be called before CPU profiles recording started.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetSamplingIntervalParams {
    /// New sampling interval in microseconds.

    pub interval: i64,
}

/// Enable precise code coverage. Coverage data for JavaScript executed before enabling precise code
/// coverage may be incomplete. Enabling prevents running optimized code and resets execution
/// counters.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartPreciseCoverageParams {
    /// Collect accurate call counts beyond simple 'covered' or 'not covered'.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub callCount: Option<bool>,
    /// Collect block-based coverage.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub detailed: Option<bool>,
    /// Allow the backend to send updates on its own initiative

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowTriggeredUpdates: Option<bool>,
}

/// Enable precise code coverage. Coverage data for JavaScript executed before enabling precise code
/// coverage may be incomplete. Enabling prevents running optimized code and resets execution
/// counters.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartPreciseCoverageReturns {
    /// Monotonically increasing time (in seconds) when the coverage update was taken in the backend.

    pub timestamp: f64,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StopReturns {
    /// Recorded profile.

    pub profile: Profile,
}

/// Collect coverage data for the current isolate, and resets execution counters. Precise code
/// coverage needs to have started.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TakePreciseCoverageReturns {
    /// Coverage data for the current isolate.

    pub result: Vec<ScriptCoverage>,
    /// Monotonically increasing time (in seconds) when the coverage update was taken in the backend.

    pub timestamp: f64,
}
