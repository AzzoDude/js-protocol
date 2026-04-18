use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;

/// Heap snapshot object id.

pub type HeapSnapshotObjectId = String;

/// Sampling Heap Profile node. Holds callsite information, allocation statistics and child nodes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SamplingHeapProfileNode {
    /// Function location.

    pub callFrame: crate::runtime::CallFrame,
    /// Allocations size in bytes for the node excluding children.

    pub selfSize: f64,
    /// Node id. Ids are unique across all profiles collected between startSampling and stopSampling.

    pub id: u64,
    /// Child nodes.

    pub children: Vec<SamplingHeapProfileNode>,
}

/// A single sample from a sampling profile.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SamplingHeapProfileSample {
    /// Allocation size in bytes attributed to the sample.

    pub size: f64,
    /// Id of the corresponding profile tree node.

    pub nodeId: u64,
    /// Time-ordered sample ordinal number. It is unique across all profiles retrieved
    /// between startSampling and stopSampling.

    pub ordinal: f64,
}

/// Sampling profile.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SamplingHeapProfile {

    pub head: SamplingHeapProfileNode,

    pub samples: Vec<SamplingHeapProfileSample>,
}

/// Enables console to refer to the node with given id via $x (see Command Line API for more details
/// $x functions).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddInspectedHeapObjectParams {
    /// Heap snapshot object id to be accessible by means of $x command line API.

    pub heapObjectId: HeapSnapshotObjectId,
}

impl AddInspectedHeapObjectParams { pub const METHOD: &'static str = "HeapProfiler.addInspectedHeapObject"; }

impl crate::CdpCommand for AddInspectedHeapObjectParams {
    const METHOD: &'static str = "HeapProfiler.addInspectedHeapObject";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CollectGarbageParams {}

impl CollectGarbageParams { pub const METHOD: &'static str = "HeapProfiler.collectGarbage"; }

impl crate::CdpCommand for CollectGarbageParams {
    const METHOD: &'static str = "HeapProfiler.collectGarbage";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "HeapProfiler.disable"; }

impl crate::CdpCommand for DisableParams {
    const METHOD: &'static str = "HeapProfiler.disable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams { pub const METHOD: &'static str = "HeapProfiler.enable"; }

impl crate::CdpCommand for EnableParams {
    const METHOD: &'static str = "HeapProfiler.enable";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetHeapObjectIdParams {
    /// Identifier of the object to get heap object id for.

    pub objectId: crate::runtime::RemoteObjectId,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetHeapObjectIdReturns {
    /// Id of the heap snapshot object corresponding to the passed remote object id.

    pub heapSnapshotObjectId: HeapSnapshotObjectId,
}

impl GetHeapObjectIdParams { pub const METHOD: &'static str = "HeapProfiler.getHeapObjectId"; }

impl crate::CdpCommand for GetHeapObjectIdParams {
    const METHOD: &'static str = "HeapProfiler.getHeapObjectId";
    type Response = GetHeapObjectIdReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetObjectByHeapObjectIdParams {

    pub objectId: HeapSnapshotObjectId,
    /// Symbolic group name that can be used to release multiple objects.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub objectGroup: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetObjectByHeapObjectIdReturns {
    /// Evaluation result.

    pub result: crate::runtime::RemoteObject,
}

impl GetObjectByHeapObjectIdParams { pub const METHOD: &'static str = "HeapProfiler.getObjectByHeapObjectId"; }

impl crate::CdpCommand for GetObjectByHeapObjectIdParams {
    const METHOD: &'static str = "HeapProfiler.getObjectByHeapObjectId";
    type Response = GetObjectByHeapObjectIdReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSamplingProfileReturns {
    /// Return the sampling profile being collected.

    pub profile: SamplingHeapProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetSamplingProfileParams {}

impl GetSamplingProfileParams { pub const METHOD: &'static str = "HeapProfiler.getSamplingProfile"; }

impl crate::CdpCommand for GetSamplingProfileParams {
    const METHOD: &'static str = "HeapProfiler.getSamplingProfile";
    type Response = GetSamplingProfileReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartSamplingParams {
    /// Average sample interval in bytes. Poisson distribution is used for the intervals. The
    /// default value is 32768 bytes.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub samplingInterval: Option<f64>,
    /// Maximum stack depth. The default value is 128.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stackDepth: Option<f64>,
    /// By default, the sampling heap profiler reports only objects which are
    /// still alive when the profile is returned via getSamplingProfile or
    /// stopSampling, which is useful for determining what functions contribute
    /// the most to steady-state memory usage. This flag instructs the sampling
    /// heap profiler to also include information about objects discarded by
    /// major GC, which will show which functions cause large temporary memory
    /// usage or long GC pauses.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub includeObjectsCollectedByMajorGC: Option<bool>,
    /// By default, the sampling heap profiler reports only objects which are
    /// still alive when the profile is returned via getSamplingProfile or
    /// stopSampling, which is useful for determining what functions contribute
    /// the most to steady-state memory usage. This flag instructs the sampling
    /// heap profiler to also include information about objects discarded by
    /// minor GC, which is useful when tuning a latency-sensitive application
    /// for minimal GC activity.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub includeObjectsCollectedByMinorGC: Option<bool>,
}

impl StartSamplingParams { pub const METHOD: &'static str = "HeapProfiler.startSampling"; }

impl crate::CdpCommand for StartSamplingParams {
    const METHOD: &'static str = "HeapProfiler.startSampling";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartTrackingHeapObjectsParams {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trackAllocations: Option<bool>,
}

impl StartTrackingHeapObjectsParams { pub const METHOD: &'static str = "HeapProfiler.startTrackingHeapObjects"; }

impl crate::CdpCommand for StartTrackingHeapObjectsParams {
    const METHOD: &'static str = "HeapProfiler.startTrackingHeapObjects";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StopSamplingReturns {
    /// Recorded sampling heap profile.

    pub profile: SamplingHeapProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StopSamplingParams {}

impl StopSamplingParams { pub const METHOD: &'static str = "HeapProfiler.stopSampling"; }

impl crate::CdpCommand for StopSamplingParams {
    const METHOD: &'static str = "HeapProfiler.stopSampling";
    type Response = StopSamplingReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StopTrackingHeapObjectsParams {
    /// If true 'reportHeapSnapshotProgress' events will be generated while snapshot is being taken
    /// when the tracking is stopped.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reportProgress: Option<bool>,
    /// Deprecated in favor of 'exposeInternals'.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatGlobalObjectsAsRoots: Option<bool>,
    /// If true, numerical values are included in the snapshot

    #[serde(skip_serializing_if = "Option::is_none")]
    pub captureNumericValue: Option<bool>,
    /// If true, exposes internals of the snapshot.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposeInternals: Option<bool>,
}

impl StopTrackingHeapObjectsParams { pub const METHOD: &'static str = "HeapProfiler.stopTrackingHeapObjects"; }

impl crate::CdpCommand for StopTrackingHeapObjectsParams {
    const METHOD: &'static str = "HeapProfiler.stopTrackingHeapObjects";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TakeHeapSnapshotParams {
    /// If true 'reportHeapSnapshotProgress' events will be generated while snapshot is being taken.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reportProgress: Option<bool>,
    /// If true, a raw snapshot without artificial roots will be generated.
    /// Deprecated in favor of 'exposeInternals'.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub treatGlobalObjectsAsRoots: Option<bool>,
    /// If true, numerical values are included in the snapshot

    #[serde(skip_serializing_if = "Option::is_none")]
    pub captureNumericValue: Option<bool>,
    /// If true, exposes internals of the snapshot.

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exposeInternals: Option<bool>,
}

impl TakeHeapSnapshotParams { pub const METHOD: &'static str = "HeapProfiler.takeHeapSnapshot"; }

impl crate::CdpCommand for TakeHeapSnapshotParams {
    const METHOD: &'static str = "HeapProfiler.takeHeapSnapshot";
    type Response = crate::EmptyReturns;
}
