use serde::{Serialize, Deserialize};
use serde_json::Value as JsonValue;
use std::borrow::Cow;

/// Heap snapshot object id.

pub type HeapSnapshotObjectId<'a> = Cow<'a, str>;

/// Sampling Heap Profile node. Holds callsite information, allocation statistics and child nodes.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SamplingHeapProfileNode<'a> {
    /// Function location.
    #[serde(rename = "callFrame")]
    call_frame: crate::runtime::CallFrame<'a>,
    /// Allocations size in bytes for the node excluding children.
    #[serde(rename = "selfSize")]
    self_size: f64,
    /// Node id. Ids are unique across all profiles collected between startSampling and stopSampling.
    id: u64,
    /// Child nodes.
    children: Vec<Box<SamplingHeapProfileNode<'a>>>,
}

impl<'a> SamplingHeapProfileNode<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `call_frame`: Function location.
    /// * `self_size`: Allocations size in bytes for the node excluding children.
    /// * `id`: Node id. Ids are unique across all profiles collected between startSampling and stopSampling.
    /// * `children`: Child nodes.
    pub fn builder(call_frame: crate::runtime::CallFrame<'a>, self_size: f64, id: u64, children: Vec<Box<SamplingHeapProfileNode<'a>>>) -> SamplingHeapProfileNodeBuilder<'a> {
        SamplingHeapProfileNodeBuilder {
            call_frame: call_frame,
            self_size: self_size,
            id: id,
            children: children,
        }
    }
    /// Function location.
    pub fn call_frame(&self) -> &crate::runtime::CallFrame<'a> { &self.call_frame }
    /// Allocations size in bytes for the node excluding children.
    pub fn self_size(&self) -> f64 { self.self_size }
    /// Node id. Ids are unique across all profiles collected between startSampling and stopSampling.
    pub fn id(&self) -> u64 { self.id }
    /// Child nodes.
    pub fn children(&self) -> &[Box<SamplingHeapProfileNode<'a>>] { &self.children }
}


pub struct SamplingHeapProfileNodeBuilder<'a> {
    call_frame: crate::runtime::CallFrame<'a>,
    self_size: f64,
    id: u64,
    children: Vec<Box<SamplingHeapProfileNode<'a>>>,
}

impl<'a> SamplingHeapProfileNodeBuilder<'a> {
    pub fn build(self) -> SamplingHeapProfileNode<'a> {
        SamplingHeapProfileNode {
            call_frame: self.call_frame,
            self_size: self.self_size,
            id: self.id,
            children: self.children,
        }
    }
}

/// A single sample from a sampling profile.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SamplingHeapProfileSample {
    /// Allocation size in bytes attributed to the sample.
    size: f64,
    /// Id of the corresponding profile tree node.
    #[serde(rename = "nodeId")]
    node_id: u64,
    /// Time-ordered sample ordinal number. It is unique across all profiles retrieved
    /// between startSampling and stopSampling.
    ordinal: f64,
}

impl SamplingHeapProfileSample {
    /// Creates a builder for this type with the required parameters:
    /// * `size`: Allocation size in bytes attributed to the sample.
    /// * `node_id`: Id of the corresponding profile tree node.
    /// * `ordinal`: Time-ordered sample ordinal number. It is unique across all profiles retrieved between startSampling and stopSampling.
    pub fn builder(size: f64, node_id: u64, ordinal: f64) -> SamplingHeapProfileSampleBuilder {
        SamplingHeapProfileSampleBuilder {
            size: size,
            node_id: node_id,
            ordinal: ordinal,
        }
    }
    /// Allocation size in bytes attributed to the sample.
    pub fn size(&self) -> f64 { self.size }
    /// Id of the corresponding profile tree node.
    pub fn node_id(&self) -> u64 { self.node_id }
    /// Time-ordered sample ordinal number. It is unique across all profiles retrieved
    /// between startSampling and stopSampling.
    pub fn ordinal(&self) -> f64 { self.ordinal }
}


pub struct SamplingHeapProfileSampleBuilder {
    size: f64,
    node_id: u64,
    ordinal: f64,
}

impl SamplingHeapProfileSampleBuilder {
    pub fn build(self) -> SamplingHeapProfileSample {
        SamplingHeapProfileSample {
            size: self.size,
            node_id: self.node_id,
            ordinal: self.ordinal,
        }
    }
}

/// Sampling profile.

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SamplingHeapProfile<'a> {
    head: SamplingHeapProfileNode<'a>,
    samples: Vec<SamplingHeapProfileSample>,
}

impl<'a> SamplingHeapProfile<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `head`: 
    /// * `samples`: 
    pub fn builder(head: SamplingHeapProfileNode<'a>, samples: Vec<SamplingHeapProfileSample>) -> SamplingHeapProfileBuilder<'a> {
        SamplingHeapProfileBuilder {
            head: head,
            samples: samples,
        }
    }
    pub fn head(&self) -> &SamplingHeapProfileNode<'a> { &self.head }
    pub fn samples(&self) -> &[SamplingHeapProfileSample] { &self.samples }
}


pub struct SamplingHeapProfileBuilder<'a> {
    head: SamplingHeapProfileNode<'a>,
    samples: Vec<SamplingHeapProfileSample>,
}

impl<'a> SamplingHeapProfileBuilder<'a> {
    pub fn build(self) -> SamplingHeapProfile<'a> {
        SamplingHeapProfile {
            head: self.head,
            samples: self.samples,
        }
    }
}

/// Enables console to refer to the node with given id via $x (see Command Line API for more details
/// $x functions).

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AddInspectedHeapObjectParams<'a> {
    /// Heap snapshot object id to be accessible by means of $x command line API.
    #[serde(rename = "heapObjectId")]
    heap_object_id: HeapSnapshotObjectId<'a>,
}

impl<'a> AddInspectedHeapObjectParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `heap_object_id`: Heap snapshot object id to be accessible by means of $x command line API.
    pub fn builder(heap_object_id: impl Into<HeapSnapshotObjectId<'a>>) -> AddInspectedHeapObjectParamsBuilder<'a> {
        AddInspectedHeapObjectParamsBuilder {
            heap_object_id: heap_object_id.into(),
        }
    }
    /// Heap snapshot object id to be accessible by means of $x command line API.
    pub fn heap_object_id(&self) -> &HeapSnapshotObjectId<'a> { &self.heap_object_id }
}


pub struct AddInspectedHeapObjectParamsBuilder<'a> {
    heap_object_id: HeapSnapshotObjectId<'a>,
}

impl<'a> AddInspectedHeapObjectParamsBuilder<'a> {
    pub fn build(self) -> AddInspectedHeapObjectParams<'a> {
        AddInspectedHeapObjectParams {
            heap_object_id: self.heap_object_id,
        }
    }
}

impl<'a> AddInspectedHeapObjectParams<'a> { pub const METHOD: &'static str = "HeapProfiler.addInspectedHeapObject"; }

impl<'a> crate::CdpCommand<'a> for AddInspectedHeapObjectParams<'a> {
    const METHOD: &'static str = "HeapProfiler.addInspectedHeapObject";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CollectGarbageParams {}

impl CollectGarbageParams { pub const METHOD: &'static str = "HeapProfiler.collectGarbage"; }

impl<'a> crate::CdpCommand<'a> for CollectGarbageParams {
    const METHOD: &'static str = "HeapProfiler.collectGarbage";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisableParams {}

impl DisableParams { pub const METHOD: &'static str = "HeapProfiler.disable"; }

impl<'a> crate::CdpCommand<'a> for DisableParams {
    const METHOD: &'static str = "HeapProfiler.disable";
    type Response = crate::EmptyReturns;
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnableParams {}

impl EnableParams { pub const METHOD: &'static str = "HeapProfiler.enable"; }

impl<'a> crate::CdpCommand<'a> for EnableParams {
    const METHOD: &'static str = "HeapProfiler.enable";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetHeapObjectIdParams<'a> {
    /// Identifier of the object to get heap object id for.
    #[serde(rename = "objectId")]
    object_id: crate::runtime::RemoteObjectId<'a>,
}

impl<'a> GetHeapObjectIdParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `object_id`: Identifier of the object to get heap object id for.
    pub fn builder(object_id: crate::runtime::RemoteObjectId<'a>) -> GetHeapObjectIdParamsBuilder<'a> {
        GetHeapObjectIdParamsBuilder {
            object_id: object_id,
        }
    }
    /// Identifier of the object to get heap object id for.
    pub fn object_id(&self) -> &crate::runtime::RemoteObjectId<'a> { &self.object_id }
}


pub struct GetHeapObjectIdParamsBuilder<'a> {
    object_id: crate::runtime::RemoteObjectId<'a>,
}

impl<'a> GetHeapObjectIdParamsBuilder<'a> {
    pub fn build(self) -> GetHeapObjectIdParams<'a> {
        GetHeapObjectIdParams {
            object_id: self.object_id,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetHeapObjectIdReturns<'a> {
    /// Id of the heap snapshot object corresponding to the passed remote object id.
    #[serde(rename = "heapSnapshotObjectId")]
    heap_snapshot_object_id: HeapSnapshotObjectId<'a>,
}

impl<'a> GetHeapObjectIdReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `heap_snapshot_object_id`: Id of the heap snapshot object corresponding to the passed remote object id.
    pub fn builder(heap_snapshot_object_id: impl Into<HeapSnapshotObjectId<'a>>) -> GetHeapObjectIdReturnsBuilder<'a> {
        GetHeapObjectIdReturnsBuilder {
            heap_snapshot_object_id: heap_snapshot_object_id.into(),
        }
    }
    /// Id of the heap snapshot object corresponding to the passed remote object id.
    pub fn heap_snapshot_object_id(&self) -> &HeapSnapshotObjectId<'a> { &self.heap_snapshot_object_id }
}


pub struct GetHeapObjectIdReturnsBuilder<'a> {
    heap_snapshot_object_id: HeapSnapshotObjectId<'a>,
}

impl<'a> GetHeapObjectIdReturnsBuilder<'a> {
    pub fn build(self) -> GetHeapObjectIdReturns<'a> {
        GetHeapObjectIdReturns {
            heap_snapshot_object_id: self.heap_snapshot_object_id,
        }
    }
}

impl<'a> GetHeapObjectIdParams<'a> { pub const METHOD: &'static str = "HeapProfiler.getHeapObjectId"; }

impl<'a> crate::CdpCommand<'a> for GetHeapObjectIdParams<'a> {
    const METHOD: &'static str = "HeapProfiler.getHeapObjectId";
    type Response = GetHeapObjectIdReturns<'a>;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetObjectByHeapObjectIdParams<'a> {
    #[serde(rename = "objectId")]
    object_id: HeapSnapshotObjectId<'a>,
    /// Symbolic group name that can be used to release multiple objects.
    #[serde(skip_serializing_if = "Option::is_none", rename = "objectGroup")]
    object_group: Option<Cow<'a, str>>,
}

impl<'a> GetObjectByHeapObjectIdParams<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `object_id`: 
    pub fn builder(object_id: impl Into<HeapSnapshotObjectId<'a>>) -> GetObjectByHeapObjectIdParamsBuilder<'a> {
        GetObjectByHeapObjectIdParamsBuilder {
            object_id: object_id.into(),
            object_group: None,
        }
    }
    pub fn object_id(&self) -> &HeapSnapshotObjectId<'a> { &self.object_id }
    /// Symbolic group name that can be used to release multiple objects.
    pub fn object_group(&self) -> Option<&str> { self.object_group.as_deref() }
}


pub struct GetObjectByHeapObjectIdParamsBuilder<'a> {
    object_id: HeapSnapshotObjectId<'a>,
    object_group: Option<Cow<'a, str>>,
}

impl<'a> GetObjectByHeapObjectIdParamsBuilder<'a> {
    /// Symbolic group name that can be used to release multiple objects.
    pub fn object_group(mut self, object_group: impl Into<Cow<'a, str>>) -> Self { self.object_group = Some(object_group.into()); self }
    pub fn build(self) -> GetObjectByHeapObjectIdParams<'a> {
        GetObjectByHeapObjectIdParams {
            object_id: self.object_id,
            object_group: self.object_group,
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetObjectByHeapObjectIdReturns<'a> {
    /// Evaluation result.
    result: crate::runtime::RemoteObject<'a>,
}

impl<'a> GetObjectByHeapObjectIdReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `result`: Evaluation result.
    pub fn builder(result: crate::runtime::RemoteObject<'a>) -> GetObjectByHeapObjectIdReturnsBuilder<'a> {
        GetObjectByHeapObjectIdReturnsBuilder {
            result: result,
        }
    }
    /// Evaluation result.
    pub fn result(&self) -> &crate::runtime::RemoteObject<'a> { &self.result }
}


pub struct GetObjectByHeapObjectIdReturnsBuilder<'a> {
    result: crate::runtime::RemoteObject<'a>,
}

impl<'a> GetObjectByHeapObjectIdReturnsBuilder<'a> {
    pub fn build(self) -> GetObjectByHeapObjectIdReturns<'a> {
        GetObjectByHeapObjectIdReturns {
            result: self.result,
        }
    }
}

impl<'a> GetObjectByHeapObjectIdParams<'a> { pub const METHOD: &'static str = "HeapProfiler.getObjectByHeapObjectId"; }

impl<'a> crate::CdpCommand<'a> for GetObjectByHeapObjectIdParams<'a> {
    const METHOD: &'static str = "HeapProfiler.getObjectByHeapObjectId";
    type Response = GetObjectByHeapObjectIdReturns<'a>;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetSamplingProfileReturns<'a> {
    /// Return the sampling profile being collected.
    profile: SamplingHeapProfile<'a>,
}

impl<'a> GetSamplingProfileReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `profile`: Return the sampling profile being collected.
    pub fn builder(profile: SamplingHeapProfile<'a>) -> GetSamplingProfileReturnsBuilder<'a> {
        GetSamplingProfileReturnsBuilder {
            profile: profile,
        }
    }
    /// Return the sampling profile being collected.
    pub fn profile(&self) -> &SamplingHeapProfile<'a> { &self.profile }
}


pub struct GetSamplingProfileReturnsBuilder<'a> {
    profile: SamplingHeapProfile<'a>,
}

impl<'a> GetSamplingProfileReturnsBuilder<'a> {
    pub fn build(self) -> GetSamplingProfileReturns<'a> {
        GetSamplingProfileReturns {
            profile: self.profile,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetSamplingProfileParams {}

impl GetSamplingProfileParams { pub const METHOD: &'static str = "HeapProfiler.getSamplingProfile"; }

impl<'a> crate::CdpCommand<'a> for GetSamplingProfileParams {
    const METHOD: &'static str = "HeapProfiler.getSamplingProfile";
    type Response = GetSamplingProfileReturns<'a>;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartSamplingParams {
    /// Average sample interval in bytes. Poisson distribution is used for the intervals. The
    /// default value is 32768 bytes.
    #[serde(skip_serializing_if = "Option::is_none", rename = "samplingInterval")]
    sampling_interval: Option<f64>,
    /// Maximum stack depth. The default value is 128.
    #[serde(skip_serializing_if = "Option::is_none", rename = "stackDepth")]
    stack_depth: Option<f64>,
    /// By default, the sampling heap profiler reports only objects which are
    /// still alive when the profile is returned via getSamplingProfile or
    /// stopSampling, which is useful for determining what functions contribute
    /// the most to steady-state memory usage. This flag instructs the sampling
    /// heap profiler to also include information about objects discarded by
    /// major GC, which will show which functions cause large temporary memory
    /// usage or long GC pauses.
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeObjectsCollectedByMajorGC")]
    include_objects_collected_by_major_gc: Option<bool>,
    /// By default, the sampling heap profiler reports only objects which are
    /// still alive when the profile is returned via getSamplingProfile or
    /// stopSampling, which is useful for determining what functions contribute
    /// the most to steady-state memory usage. This flag instructs the sampling
    /// heap profiler to also include information about objects discarded by
    /// minor GC, which is useful when tuning a latency-sensitive application
    /// for minimal GC activity.
    #[serde(skip_serializing_if = "Option::is_none", rename = "includeObjectsCollectedByMinorGC")]
    include_objects_collected_by_minor_gc: Option<bool>,
}

impl StartSamplingParams {
    /// Creates a builder for this type.
    pub fn builder() -> StartSamplingParamsBuilder {
        StartSamplingParamsBuilder {
            sampling_interval: None,
            stack_depth: None,
            include_objects_collected_by_major_gc: None,
            include_objects_collected_by_minor_gc: None,
        }
    }
    /// Average sample interval in bytes. Poisson distribution is used for the intervals. The
    /// default value is 32768 bytes.
    pub fn sampling_interval(&self) -> Option<f64> { self.sampling_interval }
    /// Maximum stack depth. The default value is 128.
    pub fn stack_depth(&self) -> Option<f64> { self.stack_depth }
    /// By default, the sampling heap profiler reports only objects which are
    /// still alive when the profile is returned via getSamplingProfile or
    /// stopSampling, which is useful for determining what functions contribute
    /// the most to steady-state memory usage. This flag instructs the sampling
    /// heap profiler to also include information about objects discarded by
    /// major GC, which will show which functions cause large temporary memory
    /// usage or long GC pauses.
    pub fn include_objects_collected_by_major_gc(&self) -> Option<bool> { self.include_objects_collected_by_major_gc }
    /// By default, the sampling heap profiler reports only objects which are
    /// still alive when the profile is returned via getSamplingProfile or
    /// stopSampling, which is useful for determining what functions contribute
    /// the most to steady-state memory usage. This flag instructs the sampling
    /// heap profiler to also include information about objects discarded by
    /// minor GC, which is useful when tuning a latency-sensitive application
    /// for minimal GC activity.
    pub fn include_objects_collected_by_minor_gc(&self) -> Option<bool> { self.include_objects_collected_by_minor_gc }
}

#[derive(Default)]
pub struct StartSamplingParamsBuilder {
    sampling_interval: Option<f64>,
    stack_depth: Option<f64>,
    include_objects_collected_by_major_gc: Option<bool>,
    include_objects_collected_by_minor_gc: Option<bool>,
}

impl StartSamplingParamsBuilder {
    /// Average sample interval in bytes. Poisson distribution is used for the intervals. The
    /// default value is 32768 bytes.
    pub fn sampling_interval(mut self, sampling_interval: f64) -> Self { self.sampling_interval = Some(sampling_interval); self }
    /// Maximum stack depth. The default value is 128.
    pub fn stack_depth(mut self, stack_depth: f64) -> Self { self.stack_depth = Some(stack_depth); self }
    /// By default, the sampling heap profiler reports only objects which are
    /// still alive when the profile is returned via getSamplingProfile or
    /// stopSampling, which is useful for determining what functions contribute
    /// the most to steady-state memory usage. This flag instructs the sampling
    /// heap profiler to also include information about objects discarded by
    /// major GC, which will show which functions cause large temporary memory
    /// usage or long GC pauses.
    pub fn include_objects_collected_by_major_gc(mut self, include_objects_collected_by_major_gc: bool) -> Self { self.include_objects_collected_by_major_gc = Some(include_objects_collected_by_major_gc); self }
    /// By default, the sampling heap profiler reports only objects which are
    /// still alive when the profile is returned via getSamplingProfile or
    /// stopSampling, which is useful for determining what functions contribute
    /// the most to steady-state memory usage. This flag instructs the sampling
    /// heap profiler to also include information about objects discarded by
    /// minor GC, which is useful when tuning a latency-sensitive application
    /// for minimal GC activity.
    pub fn include_objects_collected_by_minor_gc(mut self, include_objects_collected_by_minor_gc: bool) -> Self { self.include_objects_collected_by_minor_gc = Some(include_objects_collected_by_minor_gc); self }
    pub fn build(self) -> StartSamplingParams {
        StartSamplingParams {
            sampling_interval: self.sampling_interval,
            stack_depth: self.stack_depth,
            include_objects_collected_by_major_gc: self.include_objects_collected_by_major_gc,
            include_objects_collected_by_minor_gc: self.include_objects_collected_by_minor_gc,
        }
    }
}

impl StartSamplingParams { pub const METHOD: &'static str = "HeapProfiler.startSampling"; }

impl<'a> crate::CdpCommand<'a> for StartSamplingParams {
    const METHOD: &'static str = "HeapProfiler.startSampling";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StartTrackingHeapObjectsParams {
    #[serde(skip_serializing_if = "Option::is_none", rename = "trackAllocations")]
    track_allocations: Option<bool>,
}

impl StartTrackingHeapObjectsParams {
    /// Creates a builder for this type.
    pub fn builder() -> StartTrackingHeapObjectsParamsBuilder {
        StartTrackingHeapObjectsParamsBuilder {
            track_allocations: None,
        }
    }
    pub fn track_allocations(&self) -> Option<bool> { self.track_allocations }
}

#[derive(Default)]
pub struct StartTrackingHeapObjectsParamsBuilder {
    track_allocations: Option<bool>,
}

impl StartTrackingHeapObjectsParamsBuilder {
    pub fn track_allocations(mut self, track_allocations: bool) -> Self { self.track_allocations = Some(track_allocations); self }
    pub fn build(self) -> StartTrackingHeapObjectsParams {
        StartTrackingHeapObjectsParams {
            track_allocations: self.track_allocations,
        }
    }
}

impl StartTrackingHeapObjectsParams { pub const METHOD: &'static str = "HeapProfiler.startTrackingHeapObjects"; }

impl<'a> crate::CdpCommand<'a> for StartTrackingHeapObjectsParams {
    const METHOD: &'static str = "HeapProfiler.startTrackingHeapObjects";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StopSamplingReturns<'a> {
    /// Recorded sampling heap profile.
    profile: SamplingHeapProfile<'a>,
}

impl<'a> StopSamplingReturns<'a> {
    /// Creates a builder for this type with the required parameters:
    /// * `profile`: Recorded sampling heap profile.
    pub fn builder(profile: SamplingHeapProfile<'a>) -> StopSamplingReturnsBuilder<'a> {
        StopSamplingReturnsBuilder {
            profile: profile,
        }
    }
    /// Recorded sampling heap profile.
    pub fn profile(&self) -> &SamplingHeapProfile<'a> { &self.profile }
}


pub struct StopSamplingReturnsBuilder<'a> {
    profile: SamplingHeapProfile<'a>,
}

impl<'a> StopSamplingReturnsBuilder<'a> {
    pub fn build(self) -> StopSamplingReturns<'a> {
        StopSamplingReturns {
            profile: self.profile,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StopSamplingParams {}

impl StopSamplingParams { pub const METHOD: &'static str = "HeapProfiler.stopSampling"; }

impl<'a> crate::CdpCommand<'a> for StopSamplingParams {
    const METHOD: &'static str = "HeapProfiler.stopSampling";
    type Response = StopSamplingReturns<'a>;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct StopTrackingHeapObjectsParams {
    /// If true 'reportHeapSnapshotProgress' events will be generated while snapshot is being taken
    /// when the tracking is stopped.
    #[serde(skip_serializing_if = "Option::is_none", rename = "reportProgress")]
    report_progress: Option<bool>,
    /// Deprecated in favor of 'exposeInternals'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "treatGlobalObjectsAsRoots")]
    treat_global_objects_as_roots: Option<bool>,
    /// If true, numerical values are included in the snapshot
    #[serde(skip_serializing_if = "Option::is_none", rename = "captureNumericValue")]
    capture_numeric_value: Option<bool>,
    /// If true, exposes internals of the snapshot.
    #[serde(skip_serializing_if = "Option::is_none", rename = "exposeInternals")]
    expose_internals: Option<bool>,
}

impl StopTrackingHeapObjectsParams {
    /// Creates a builder for this type.
    pub fn builder() -> StopTrackingHeapObjectsParamsBuilder {
        StopTrackingHeapObjectsParamsBuilder {
            report_progress: None,
            treat_global_objects_as_roots: None,
            capture_numeric_value: None,
            expose_internals: None,
        }
    }
    /// If true 'reportHeapSnapshotProgress' events will be generated while snapshot is being taken
    /// when the tracking is stopped.
    pub fn report_progress(&self) -> Option<bool> { self.report_progress }
    /// Deprecated in favor of 'exposeInternals'.
    pub fn treat_global_objects_as_roots(&self) -> Option<bool> { self.treat_global_objects_as_roots }
    /// If true, numerical values are included in the snapshot
    pub fn capture_numeric_value(&self) -> Option<bool> { self.capture_numeric_value }
    /// If true, exposes internals of the snapshot.
    pub fn expose_internals(&self) -> Option<bool> { self.expose_internals }
}

#[derive(Default)]
pub struct StopTrackingHeapObjectsParamsBuilder {
    report_progress: Option<bool>,
    treat_global_objects_as_roots: Option<bool>,
    capture_numeric_value: Option<bool>,
    expose_internals: Option<bool>,
}

impl StopTrackingHeapObjectsParamsBuilder {
    /// If true 'reportHeapSnapshotProgress' events will be generated while snapshot is being taken
    /// when the tracking is stopped.
    pub fn report_progress(mut self, report_progress: bool) -> Self { self.report_progress = Some(report_progress); self }
    /// Deprecated in favor of 'exposeInternals'.
    pub fn treat_global_objects_as_roots(mut self, treat_global_objects_as_roots: bool) -> Self { self.treat_global_objects_as_roots = Some(treat_global_objects_as_roots); self }
    /// If true, numerical values are included in the snapshot
    pub fn capture_numeric_value(mut self, capture_numeric_value: bool) -> Self { self.capture_numeric_value = Some(capture_numeric_value); self }
    /// If true, exposes internals of the snapshot.
    pub fn expose_internals(mut self, expose_internals: bool) -> Self { self.expose_internals = Some(expose_internals); self }
    pub fn build(self) -> StopTrackingHeapObjectsParams {
        StopTrackingHeapObjectsParams {
            report_progress: self.report_progress,
            treat_global_objects_as_roots: self.treat_global_objects_as_roots,
            capture_numeric_value: self.capture_numeric_value,
            expose_internals: self.expose_internals,
        }
    }
}

impl StopTrackingHeapObjectsParams { pub const METHOD: &'static str = "HeapProfiler.stopTrackingHeapObjects"; }

impl<'a> crate::CdpCommand<'a> for StopTrackingHeapObjectsParams {
    const METHOD: &'static str = "HeapProfiler.stopTrackingHeapObjects";
    type Response = crate::EmptyReturns;
}


#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct TakeHeapSnapshotParams {
    /// If true 'reportHeapSnapshotProgress' events will be generated while snapshot is being taken.
    #[serde(skip_serializing_if = "Option::is_none", rename = "reportProgress")]
    report_progress: Option<bool>,
    /// If true, a raw snapshot without artificial roots will be generated.
    /// Deprecated in favor of 'exposeInternals'.
    #[serde(skip_serializing_if = "Option::is_none", rename = "treatGlobalObjectsAsRoots")]
    treat_global_objects_as_roots: Option<bool>,
    /// If true, numerical values are included in the snapshot
    #[serde(skip_serializing_if = "Option::is_none", rename = "captureNumericValue")]
    capture_numeric_value: Option<bool>,
    /// If true, exposes internals of the snapshot.
    #[serde(skip_serializing_if = "Option::is_none", rename = "exposeInternals")]
    expose_internals: Option<bool>,
}

impl TakeHeapSnapshotParams {
    /// Creates a builder for this type.
    pub fn builder() -> TakeHeapSnapshotParamsBuilder {
        TakeHeapSnapshotParamsBuilder {
            report_progress: None,
            treat_global_objects_as_roots: None,
            capture_numeric_value: None,
            expose_internals: None,
        }
    }
    /// If true 'reportHeapSnapshotProgress' events will be generated while snapshot is being taken.
    pub fn report_progress(&self) -> Option<bool> { self.report_progress }
    /// If true, a raw snapshot without artificial roots will be generated.
    /// Deprecated in favor of 'exposeInternals'.
    pub fn treat_global_objects_as_roots(&self) -> Option<bool> { self.treat_global_objects_as_roots }
    /// If true, numerical values are included in the snapshot
    pub fn capture_numeric_value(&self) -> Option<bool> { self.capture_numeric_value }
    /// If true, exposes internals of the snapshot.
    pub fn expose_internals(&self) -> Option<bool> { self.expose_internals }
}

#[derive(Default)]
pub struct TakeHeapSnapshotParamsBuilder {
    report_progress: Option<bool>,
    treat_global_objects_as_roots: Option<bool>,
    capture_numeric_value: Option<bool>,
    expose_internals: Option<bool>,
}

impl TakeHeapSnapshotParamsBuilder {
    /// If true 'reportHeapSnapshotProgress' events will be generated while snapshot is being taken.
    pub fn report_progress(mut self, report_progress: bool) -> Self { self.report_progress = Some(report_progress); self }
    /// If true, a raw snapshot without artificial roots will be generated.
    /// Deprecated in favor of 'exposeInternals'.
    pub fn treat_global_objects_as_roots(mut self, treat_global_objects_as_roots: bool) -> Self { self.treat_global_objects_as_roots = Some(treat_global_objects_as_roots); self }
    /// If true, numerical values are included in the snapshot
    pub fn capture_numeric_value(mut self, capture_numeric_value: bool) -> Self { self.capture_numeric_value = Some(capture_numeric_value); self }
    /// If true, exposes internals of the snapshot.
    pub fn expose_internals(mut self, expose_internals: bool) -> Self { self.expose_internals = Some(expose_internals); self }
    pub fn build(self) -> TakeHeapSnapshotParams {
        TakeHeapSnapshotParams {
            report_progress: self.report_progress,
            treat_global_objects_as_roots: self.treat_global_objects_as_roots,
            capture_numeric_value: self.capture_numeric_value,
            expose_internals: self.expose_internals,
        }
    }
}

impl TakeHeapSnapshotParams { pub const METHOD: &'static str = "HeapProfiler.takeHeapSnapshot"; }

impl<'a> crate::CdpCommand<'a> for TakeHeapSnapshotParams {
    const METHOD: &'static str = "HeapProfiler.takeHeapSnapshot";
    type Response = crate::EmptyReturns;
}
