// Copyright (c) 2015-2016, Nokia Inc
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//     * Redistributions of source code must retain the above copyright
//       notice, this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above copyright
//       notice, this list of conditions and the following disclaimer in the
//       documentation and/or other materials provided with the distribution.
//     * Neither the name of the copyright holder nor the names of its contributors
//       may be used to endorse or promote products derived from this software without
//       specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
// (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
// LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
// ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.


use bambou::{BambouError, RestEntity, Session, SessionConfig};
use hyper::client::{Response};
use std::collections::BTreeMap;
use serde_json;


pub use metadata::Metadata;
pub use alarm::Alarm;
pub use globalmetadata::GlobalMetadata;
pub use vm::VM;
pub use job::Job;
pub use monitoringport::MonitoringPort;
pub use container::Container;
pub use vport::VPort;
pub use hsc::HSC;
pub use vsc::VSC;
pub use multinicvport::MultiNICVPort;
pub use eventlog::EventLog;


#[derive(Serialize, Deserialize)]
pub struct VRS<'a> {
    #[serde(skip_serializing)]
    #[serde(skip_deserializing)]
    _session: Option<&'a Session>,
    #[serde(rename="ID")]
    id: Option<String>,
    
    #[serde(rename="parentID")]
    parent_id: Option<String>,
    #[serde(rename="parentType")]
    parent_type: Option<String>,
    owner: Option<String>,
    
    #[serde(rename="JSONRPCConnectionState")]
    jsonrpc_connection_state: Option<String>,
    name: Option<String>,
    
    #[serde(rename="managementIP")]
    management_ip: Option<String>,
    
    #[serde(rename="parentIDs")]
    parent_ids: Vec<Option<String>>,
    
    #[serde(rename="lastEventName")]
    last_event_name: Option<String>,
    
    #[serde(rename="lastEventObject")]
    last_event_object: Option<String>,
    
    #[serde(rename="lastEventTimestamp")]
    last_event_timestamp: u64,
    
    #[serde(rename="lastStateChange")]
    last_state_change: u64,
    
    #[serde(rename="lastUpdatedBy")]
    last_updated_by: Option<String>,
    
    #[serde(rename="dbSynced")]
    db_synced: bool,
    address: Option<String>,
    
    #[serde(rename="peakCPUUsage")]
    peak_cpuusage: f64,
    
    #[serde(rename="peakMemoryUsage")]
    peak_memory_usage: f64,
    peer: Option<String>,
    personality: Option<String>,
    description: Option<String>,
    messages: Vec<Option<String>>,
    
    #[serde(rename="revertBehaviorEnabled")]
    revert_behavior_enabled: bool,
    
    #[serde(rename="revertCompleted")]
    revert_completed: bool,
    
    #[serde(rename="revertCount")]
    revert_count: u64,
    
    #[serde(rename="revertFailedCount")]
    revert_failed_count: u64,
    
    #[serde(rename="licensedState")]
    licensed_state: Option<String>,
    disks: Vec<BTreeMap<String, serde_json::Value>>,
    
    #[serde(rename="clusterNodeRole")]
    cluster_node_role: Option<String>,
    
    #[serde(rename="entityScope")]
    entity_scope: Option<String>,
    location: Option<String>,
    role: Option<String>,
    uptime: u64,
    
    #[serde(rename="primaryVSCConnectionLost")]
    primary_vsc_connection_lost: bool,
    
    #[serde(rename="productVersion")]
    product_version: Option<String>,
    
    #[serde(rename="isResilient")]
    is_resilient: bool,
    
    #[serde(rename="vscConfigState")]
    vsc_config_state: Option<String>,
    
    #[serde(rename="vscCurrentState")]
    vsc_current_state: Option<String>,
    status: Option<String>,
    
    #[serde(rename="multiNICVPortEnabled")]
    multi_nic_vport_enabled: bool,
    
    #[serde(rename="numberOfBridgeInterfaces")]
    number_of_bridge_interfaces: u64,
    
    #[serde(rename="numberOfContainers")]
    number_of_containers: u64,
    
    #[serde(rename="numberOfHostInterfaces")]
    number_of_host_interfaces: u64,
    
    #[serde(rename="numberOfVirtualMachines")]
    number_of_virtual_machines: u64,
    
    #[serde(rename="currentCPUUsage")]
    current_cpuusage: f64,
    
    #[serde(rename="currentMemoryUsage")]
    current_memory_usage: f64,
    
    #[serde(rename="averageCPUUsage")]
    average_cpuusage: f64,
    
    #[serde(rename="averageMemoryUsage")]
    average_memory_usage: f64,
    
    #[serde(rename="externalID")]
    external_id: Option<String>,
    dynamic: bool,
    
    #[serde(rename="hypervisorConnectionState")]
    hypervisor_connection_state: Option<String>,
    
    #[serde(rename="hypervisorIdentifier")]
    hypervisor_identifier: Option<String>,
    
    #[serde(rename="hypervisorName")]
    hypervisor_name: Option<String>,
    
    #[serde(rename="hypervisorType")]
    hypervisor_type: Option<String>,
    
}

impl<'a> RestEntity<'a> for VRS<'a> {
    fn fetch(&mut self) -> Result<Response, BambouError> {
        match self._session {
            Some(session) => session.fetch(self),
            None => Err(BambouError::NoSession),
        }
    }

    fn path() -> &'static str {
        "vrs"
    }

    fn group_path() -> &'static str {
        "vrss"
    }

    fn is_root(&self) -> bool {
        false
    }

    fn id(&self) -> Option<&str> {
        self.id.as_ref().and_then(|id| Some(id.as_str()))
    }

    fn fetch_children<R>(&self, children: &mut Vec<R>) -> Result<Response, BambouError>
        where R: RestEntity<'a>
    {
        match self._session {
            Some(session) => session.fetch_children(self, children),
            None => Err(BambouError::NoSession),
        }
    }

    fn get_session(&self) -> Option<&Session> {
        self._session
    }

    fn set_session(&mut self, session: &'a Session) {
        self._session = Some(session);
    }

    fn save(&mut self) -> Result<Response, BambouError> {
        match self._session {
            Some(session) => session.save(self),
            None => Err(BambouError::NoSession),
        }
    }

    fn delete(self) -> Result<Response, BambouError> {
        match self._session {
            Some(session) => session.delete(self),
            None => Err(BambouError::NoSession),
        }
    }

    fn create_child<C>(&self, child: &mut C) -> Result<Response, BambouError>
        where C: RestEntity<'a>
    {
        match self._session {
            Some(session) => session.create_child(self, child),
            None => Err(BambouError::NoSession),
        }
    }

}

impl<'a> VRS<'a> {

    fn fetch_metadatas(&self) -> Result<Vec<Metadata>, BambouError> {
        let mut metadatas = Vec::<Metadata>::new();
        try!(self.fetch_children(&mut metadatas));
        Ok(metadatas)
    }

    fn fetch_alarms(&self) -> Result<Vec<Alarm>, BambouError> {
        let mut alarms = Vec::<Alarm>::new();
        try!(self.fetch_children(&mut alarms));
        Ok(alarms)
    }

    fn fetch_globalmetadatas(&self) -> Result<Vec<GlobalMetadata>, BambouError> {
        let mut globalmetadatas = Vec::<GlobalMetadata>::new();
        try!(self.fetch_children(&mut globalmetadatas));
        Ok(globalmetadatas)
    }

    fn fetch_vms(&self) -> Result<Vec<VM>, BambouError> {
        let mut vms = Vec::<VM>::new();
        try!(self.fetch_children(&mut vms));
        Ok(vms)
    }

    fn fetch_jobs(&self) -> Result<Vec<Job>, BambouError> {
        let mut jobs = Vec::<Job>::new();
        try!(self.fetch_children(&mut jobs));
        Ok(jobs)
    }

    fn fetch_monitoringports(&self) -> Result<Vec<MonitoringPort>, BambouError> {
        let mut monitoringports = Vec::<MonitoringPort>::new();
        try!(self.fetch_children(&mut monitoringports));
        Ok(monitoringports)
    }

    fn fetch_containers(&self) -> Result<Vec<Container>, BambouError> {
        let mut containers = Vec::<Container>::new();
        try!(self.fetch_children(&mut containers));
        Ok(containers)
    }

    fn fetch_vports(&self) -> Result<Vec<VPort>, BambouError> {
        let mut vports = Vec::<VPort>::new();
        try!(self.fetch_children(&mut vports));
        Ok(vports)
    }

    fn fetch_hscs(&self) -> Result<Vec<HSC>, BambouError> {
        let mut hscs = Vec::<HSC>::new();
        try!(self.fetch_children(&mut hscs));
        Ok(hscs)
    }

    fn fetch_vscs(&self) -> Result<Vec<VSC>, BambouError> {
        let mut vscs = Vec::<VSC>::new();
        try!(self.fetch_children(&mut vscs));
        Ok(vscs)
    }

    fn fetch_multinicvports(&self) -> Result<Vec<MultiNICVPort>, BambouError> {
        let mut multinicvports = Vec::<MultiNICVPort>::new();
        try!(self.fetch_children(&mut multinicvports));
        Ok(multinicvports)
    }

    fn fetch_eventlogs(&self) -> Result<Vec<EventLog>, BambouError> {
        let mut eventlogs = Vec::<EventLog>::new();
        try!(self.fetch_children(&mut eventlogs));
        Ok(eventlogs)
    }
}
