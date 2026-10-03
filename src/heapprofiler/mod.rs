use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;
use crate::{CdpBuilder, CdpCommand, CdpEvent};

/// Heap snapshot object id.

pub type HeapSnapshotObjectId<'a> = Cow<'a, str>;

/// Sampling Heap Profile node. Holds callsite information, allocation statistics and child nodes.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SamplingHeapProfileNode<'a> {
    /// Function location.
    #[serde(rename = "callFrame")]
    pub call_frame: crate::runtime::CallFrame<'a>,
    /// Allocations size in bytes for the node excluding children.
    #[serde(rename = "selfSize")]
    pub self_size: f64,
    /// Node id. Ids are unique across all profiles collected between startSampling and stopSampling.
    pub id: u64,
    /// Child nodes.
    pub children: Vec<Box<SamplingHeapProfileNode<'a>>>,
}
/// A single sample from a sampling profile.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SamplingHeapProfileSample {
    /// Allocation size in bytes attributed to the sample.
    pub size: f64,
    /// Id of the corresponding profile tree node.
    #[serde(rename = "nodeId")]
    pub node_id: u64,
    /// Time-ordered sample ordinal number. It is unique across all profiles retrieved
    /// between startSampling and stopSampling.
    pub ordinal: f64,
}
/// Sampling profile.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct SamplingHeapProfile<'a> {
    pub head: SamplingHeapProfileNode<'a>,
    pub samples: Vec<SamplingHeapProfileSample>,
}
/// Enables console to refer to the node with given id via $x (see Command Line API for more details
/// $x functions).

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "HeapProfiler.addInspectedHeapObject")]
pub struct AddInspectedHeapObjectParams<'a> {
    /// Heap snapshot object id to be accessible by means of $x command line API.
    #[serde(rename = "heapObjectId")]
    pub heap_object_id: HeapSnapshotObjectId<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "HeapProfiler.collectGarbage")]
pub struct CollectGarbageParams {

}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "HeapProfiler.disable")]
pub struct DisableParams {

}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "HeapProfiler.enable")]
pub struct EnableParams {

}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "HeapProfiler.getHeapObjectId", response = "GetHeapObjectIdReturns<'a>")]
pub struct GetHeapObjectIdParams<'a> {
    /// Identifier of the object to get heap object id for.
    #[serde(rename = "objectId")]
    pub object_id: crate::runtime::RemoteObjectId<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetHeapObjectIdReturns<'a> {
    /// Id of the heap snapshot object corresponding to the passed remote object id.
    #[serde(rename = "heapSnapshotObjectId")]
    pub heap_snapshot_object_id: HeapSnapshotObjectId<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "HeapProfiler.getObjectByHeapObjectId", response = "GetObjectByHeapObjectIdReturns<'a>")]
pub struct GetObjectByHeapObjectIdParams<'a> {
    #[serde(rename = "objectId")]
    pub object_id: HeapSnapshotObjectId<'a>,
    /// Symbolic group name that can be used to release multiple objects.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectGroup")]
    pub object_group: Option<Cow<'a, str>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetObjectByHeapObjectIdReturns<'a> {
    /// Evaluation result.
    pub result: crate::runtime::RemoteObject<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "HeapProfiler.getSamplingProfile", response = "GetSamplingProfileReturns<'a>")]
pub struct GetSamplingProfileParams {

}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct GetSamplingProfileReturns<'a> {
    /// Return the sampling profile being collected.
    pub profile: SamplingHeapProfile<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "HeapProfiler.startSampling")]
pub struct StartSamplingParams {
    /// Average sample interval in bytes. Poisson distribution is used for the intervals. The
    /// default value is 32768 bytes.
    #[serde(skip_serializing_if = "Option::is_none", rename = "samplingInterval")]
    pub sampling_interval: Option<f64>,
    /// Maximum stack depth. The default value is 128.
    #[serde(skip_serializing_if = "Option::is_none", rename = "stackDepth")]
    pub stack_depth: Option<f64>,
    /// By default, the sampling heap profiler reports only objects which are
    /// still alive when the profile is returned via getSamplingProfile or
    /// stopSampling, which is useful for determining what functions contribute
    /// the most to steady-state memory usage. This flag instructs the sampling
    /// heap profiler to also include information about objects discarded by
    /// major GC, which will show which functions cause large temporary memory
    /// usage or long GC pauses.
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeObjectsCollectedByMajorGC")]
    pub include_objects_collected_by_major_gc: Option<bool>,
    /// By default, the sampling heap profiler reports only objects which are
    /// still alive when the profile is returned via getSamplingProfile or
    /// stopSampling, which is useful for determining what functions contribute
    /// the most to steady-state memory usage. This flag instructs the sampling
    /// heap profiler to also include information about objects discarded by
    /// minor GC, which is useful when tuning a latency-sensitive application
    /// for minimal GC activity.
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeObjectsCollectedByMinorGC")]
    pub include_objects_collected_by_minor_gc: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "HeapProfiler.startTrackingHeapObjects")]
pub struct StartTrackingHeapObjectsParams {
    #[serde(skip_serializing_if = "Option::is_none", rename = "trackAllocations")]
    pub track_allocations: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "HeapProfiler.stopSampling", response = "StopSamplingReturns<'a>")]
pub struct StopSamplingParams {

}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder)]
#[serde(rename_all = "camelCase")]
pub struct StopSamplingReturns<'a> {
    /// Recorded sampling heap profile.
    pub profile: SamplingHeapProfile<'a>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "HeapProfiler.stopTrackingHeapObjects")]
pub struct StopTrackingHeapObjectsParams {
    /// If true 'reportHeapSnapshotProgress' events will be generated while snapshot is being taken
    /// when the tracking is stopped.
    #[serde(skip_serializing_if = "Option::is_none", rename = "reportProgress")]
    pub report_progress: Option<bool>,
    /// Deprecated in favor of 'exposeInternals'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "treatGlobalObjectsAsRoots")]
    pub treat_global_objects_as_roots: Option<bool>,
    /// If true, numerical values are included in the snapshot
    #[serde(skip_serializing_if = "Option::is_none", rename = "captureNumericValue")]
    pub capture_numeric_value: Option<bool>,
    /// If true, exposes internals of the snapshot.
    #[serde(skip_serializing_if = "Option::is_none", rename = "exposeInternals")]
    pub expose_internals: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpCommand)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "HeapProfiler.takeHeapSnapshot")]
pub struct TakeHeapSnapshotParams {
    /// If true 'reportHeapSnapshotProgress' events will be generated while snapshot is being taken.
    #[serde(skip_serializing_if = "Option::is_none", rename = "reportProgress")]
    pub report_progress: Option<bool>,
    /// If true, a raw snapshot without artificial roots will be generated.
    /// Deprecated in favor of 'exposeInternals'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "treatGlobalObjectsAsRoots")]
    pub treat_global_objects_as_roots: Option<bool>,
    /// If true, numerical values are included in the snapshot
    #[serde(skip_serializing_if = "Option::is_none", rename = "captureNumericValue")]
    pub capture_numeric_value: Option<bool>,
    /// If true, exposes internals of the snapshot.
    #[serde(skip_serializing_if = "Option::is_none", rename = "exposeInternals")]
    pub expose_internals: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "HeapProfiler.addHeapSnapshotChunk")]
pub struct AddHeapSnapshotChunk<'a> {
    pub chunk: Cow<'a, str>,
}
/// If heap objects tracking has been started then backend may send update for one or more fragments

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "HeapProfiler.heapStatsUpdate")]
pub struct HeapStatsUpdate {
    /// An array of triplets. Each triplet describes a fragment. The first integer is the fragment
    /// index, the second integer is a total count of objects for the fragment, the third integer is
    /// a total size of the objects for the fragment.
    #[serde(rename = "statsUpdate")]
    pub stats_update: Vec<i64>,
}
/// If heap objects tracking has been started then backend regularly sends a current value for last
/// seen object id and corresponding timestamp. If the were changes in the heap since last event
/// then one or more heapStatsUpdate events will be sent before a new lastSeenObjectId event.

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "HeapProfiler.lastSeenObjectId")]
pub struct LastSeenObjectId {
    #[serde(rename = "lastSeenObjectId")]
    pub last_seen_object_id: u64,
    pub timestamp: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "HeapProfiler.reportHeapSnapshotProgress")]
pub struct ReportHeapSnapshotProgress {
    pub done: i64,
    pub total: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, CdpBuilder, CdpEvent)]
#[serde(rename_all = "camelCase")]
#[cdp(method = "HeapProfiler.resetProfiles")]
pub struct ResetProfiles {

}