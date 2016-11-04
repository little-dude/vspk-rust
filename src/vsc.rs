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
pub use bgppeer::BGPPeer;
pub use alarm::Alarm;
pub use globalmetadata::GlobalMetadata;
pub use job::Job;
pub use monitoringport::MonitoringPort;
pub use vrs::VRS;
pub use eventlog::EventLog;


#[derive(Serialize, Deserialize)]
pub struct VSC<'a> {
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
    name: Option<String>,
    
    #[serde(rename="managementIP")]
    management_ip: Option<String>,
    
    #[serde(rename="lastStateChange")]
    last_state_change: u64,
    
    #[serde(rename="lastUpdatedBy")]
    last_updated_by: Option<String>,
    address: Option<String>,
    
    #[serde(rename="peakCPUUsage")]
    peak_cpuusage: f64,
    
    #[serde(rename="peakMemoryUsage")]
    peak_memory_usage: f64,
    description: Option<String>,
    messages: Vec<Option<String>>,
    disks: Vec<BTreeMap<String, serde_json::Value>>,
    
    #[serde(rename="alreadyMarkedForUnavailable")]
    already_marked_for_unavailable: bool,
    
    #[serde(rename="unavailableTimestamp")]
    unavailable_timestamp: u64,
    
    #[serde(rename="entityScope")]
    entity_scope: Option<String>,
    location: Option<String>,
    
    #[serde(rename="productVersion")]
    product_version: Option<String>,
    vsds: Vec<Option<String>>,
    status: Option<String>,
    
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
    
}

impl<'a> RestEntity<'a> for VSC<'a> {
    fn fetch(&mut self) -> Result<Response, BambouError> {
        match self._session {
            Some(session) => session.fetch(self),
            None => Err(BambouError::NoSession),
        }
    }

    fn path() -> &'static str {
        "vsc"
    }

    fn group_path() -> &'static str {
        "vscs"
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

impl<'a> VSC<'a> {

    fn fetch_metadatas(&self) -> Result<Vec<Metadata>, BambouError> {
        let mut metadatas = Vec::<Metadata>::new();
        try!(self.fetch_children(&mut metadatas));
        Ok(metadatas)
    }

    fn fetch_bgppeers(&self) -> Result<Vec<BGPPeer>, BambouError> {
        let mut bgppeers = Vec::<BGPPeer>::new();
        try!(self.fetch_children(&mut bgppeers));
        Ok(bgppeers)
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

    fn fetch_vrss(&self) -> Result<Vec<VRS>, BambouError> {
        let mut vrss = Vec::<VRS>::new();
        try!(self.fetch_children(&mut vrss));
        Ok(vrss)
    }

    fn fetch_eventlogs(&self) -> Result<Vec<EventLog>, BambouError> {
        let mut eventlogs = Vec::<EventLog>::new();
        try!(self.fetch_children(&mut eventlogs));
        Ok(eventlogs)
    }
}
