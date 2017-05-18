// Copyright (c) 2015 Alcatel-Lucent, (c) 2016 Nokia
//
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


use bambou::{Error, RestEntity, Session};
use reqwest::Response;
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


#[derive(Serialize, Deserialize, Default, Debug)]
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

    
    pub name: Option<String>,
    
    #[serde(rename="managementIP")]
    pub management_ip: Option<String>,
    
    #[serde(rename="lastStateChange")]
    pub last_state_change: u64,
    
    #[serde(rename="lastUpdatedBy")]
    pub last_updated_by: Option<String>,
    
    pub address: Option<String>,
    
    #[serde(rename="peakCPUUsage")]
    pub peak_cpuusage: f64,
    
    #[serde(rename="peakMemoryUsage")]
    pub peak_memory_usage: f64,
    
    pub description: Option<String>,
    
    pub messages: Vec<Option<String>>,
    
    pub disks: Vec<BTreeMap<String, serde_json::Value>>,
    
    #[serde(rename="alreadyMarkedForUnavailable")]
    pub already_marked_for_unavailable: bool,
    
    #[serde(rename="unavailableTimestamp")]
    pub unavailable_timestamp: u64,
    
    #[serde(rename="entityScope")]
    pub entity_scope: Option<String>,
    
    pub location: Option<String>,
    
    #[serde(rename="productVersion")]
    pub product_version: Option<String>,
    
    pub vsds: Vec<Option<String>>,
    
    pub status: Option<String>,
    
    #[serde(rename="currentCPUUsage")]
    pub current_cpuusage: f64,
    
    #[serde(rename="currentMemoryUsage")]
    pub current_memory_usage: f64,
    
    #[serde(rename="averageCPUUsage")]
    pub average_cpuusage: f64,
    
    #[serde(rename="averageMemoryUsage")]
    pub average_memory_usage: f64,
    
    #[serde(rename="externalID")]
    pub external_id: Option<String>,
    
}

impl<'a> RestEntity<'a> for VSC<'a> {
    fn fetch(&mut self) -> Result<Response, Error> {
        match self._session {
            Some(session) => session.fetch_entity(self),
            None => Err(Error::NoSession),
        }
    }

    fn save(&mut self) -> Result<Response, Error> {
        match self._session {
            Some(session) => session.save(self),
            None => Err(Error::NoSession),
        }
    }

    fn delete(self) -> Result<Response, Error> {
        match self._session {
            Some(session) => session.delete(self),
            None => Err(Error::NoSession),
        }
    }

    fn create_child<C>(&self, child: &mut C) -> Result<Response, Error>
        where C: RestEntity<'a>
    {
        match self._session {
            Some(session) => session.create_child(self, child),
            None => Err(Error::NoSession),
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

    fn fetch_children<R>(&self, children: &mut Vec<R>) -> Result<Response, Error>
        where R: RestEntity<'a>
    {
        match self._session {
            Some(session) => session.fetch_children(self, children),
            None => Err(Error::NoSession),
        }
    }

    fn get_session(&self) -> Option<&Session> {
        self._session
    }

    fn set_session(&mut self, session: &'a Session) {
        self._session = Some(session);
    }
}

impl<'a> VSC<'a> {

    pub fn fetch_metadatas(&self) -> Result<Vec<Metadata>, Error> {
        let mut metadatas = Vec::<Metadata>::new();
        let _ = self.fetch_children(&mut metadatas)?;
        Ok(metadatas)
    }

    pub fn fetch_bgppeers(&self) -> Result<Vec<BGPPeer>, Error> {
        let mut bgppeers = Vec::<BGPPeer>::new();
        let _ = self.fetch_children(&mut bgppeers)?;
        Ok(bgppeers)
    }

    pub fn fetch_alarms(&self) -> Result<Vec<Alarm>, Error> {
        let mut alarms = Vec::<Alarm>::new();
        let _ = self.fetch_children(&mut alarms)?;
        Ok(alarms)
    }

    pub fn fetch_globalmetadatas(&self) -> Result<Vec<GlobalMetadata>, Error> {
        let mut globalmetadatas = Vec::<GlobalMetadata>::new();
        let _ = self.fetch_children(&mut globalmetadatas)?;
        Ok(globalmetadatas)
    }

    pub fn fetch_jobs(&self) -> Result<Vec<Job>, Error> {
        let mut jobs = Vec::<Job>::new();
        let _ = self.fetch_children(&mut jobs)?;
        Ok(jobs)
    }

    pub fn fetch_monitoringports(&self) -> Result<Vec<MonitoringPort>, Error> {
        let mut monitoringports = Vec::<MonitoringPort>::new();
        let _ = self.fetch_children(&mut monitoringports)?;
        Ok(monitoringports)
    }

    pub fn fetch_vrss(&self) -> Result<Vec<VRS>, Error> {
        let mut vrss = Vec::<VRS>::new();
        let _ = self.fetch_children(&mut vrss)?;
        Ok(vrss)
    }

    pub fn fetch_eventlogs(&self) -> Result<Vec<EventLog>, Error> {
        let mut eventlogs = Vec::<EventLog>::new();
        let _ = self.fetch_children(&mut eventlogs)?;
        Ok(eventlogs)
    }
}