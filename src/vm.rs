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


pub use vmresync::VMResync;
pub use metadata::Metadata;
pub use alarm::Alarm;
pub use globalmetadata::GlobalMetadata;
pub use vminterface::VMInterface;
pub use vrs::VRS;
pub use eventlog::EventLog;


#[derive(Serialize, Deserialize, Default, Debug)]
pub struct VM<'a> {
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

    
    #[serde(rename="l2DomainIDs")]
    pub l2_domain_ids: Vec<Option<String>>,
    
    #[serde(rename="VRSID")]
    pub vrsid: Option<String>,
    
    #[serde(rename="UUID")]
    pub uuid: Option<String>,
    
    pub name: Option<String>,
    
    #[serde(rename="lastUpdatedBy")]
    pub last_updated_by: Option<String>,
    
    #[serde(rename="reasonType")]
    pub reason_type: Option<String>,
    
    #[serde(rename="deleteExpiry")]
    pub delete_expiry: u64,
    
    #[serde(rename="deleteMode")]
    pub delete_mode: Option<String>,
    
    #[serde(rename="resyncInfo")]
    pub resync_info: BTreeMap<String, serde_json::Value>,
    
    #[serde(rename="siteIdentifier")]
    pub site_identifier: Option<String>,
    
    pub interfaces: Vec<BTreeMap<String, serde_json::Value>>,
    
    #[serde(rename="enterpriseID")]
    pub enterprise_id: Option<String>,
    
    #[serde(rename="enterpriseName")]
    pub enterprise_name: Option<String>,
    
    #[serde(rename="entityScope")]
    pub entity_scope: Option<String>,
    
    #[serde(rename="domainIDs")]
    pub domain_ids: Vec<Option<String>>,
    
    #[serde(rename="zoneIDs")]
    pub zone_ids: Vec<Option<String>>,
    
    #[serde(rename="orchestrationID")]
    pub orchestration_id: Option<String>,
    
    #[serde(rename="userID")]
    pub user_id: Option<String>,
    
    #[serde(rename="userName")]
    pub user_name: Option<String>,
    
    pub status: Option<String>,
    
    #[serde(rename="subnetIDs")]
    pub subnet_ids: Vec<Option<String>>,
    
    #[serde(rename="externalID")]
    pub external_id: Option<String>,
    
    #[serde(rename="hypervisorIP")]
    pub hypervisor_ip: Option<String>,
    
}

impl<'a> RestEntity<'a> for VM<'a> {
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
        "vm"
    }

    fn group_path() -> &'static str {
        "vms"
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

impl<'a> VM<'a> {

    pub fn fetch_resync(&self) -> Result<Vec<VMResync>, Error> {
        let mut resync = Vec::<VMResync>::new();
        let _ = self.fetch_children(&mut resync)?;
        Ok(resync)
    }

    pub fn fetch_metadatas(&self) -> Result<Vec<Metadata>, Error> {
        let mut metadatas = Vec::<Metadata>::new();
        let _ = self.fetch_children(&mut metadatas)?;
        Ok(metadatas)
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

    pub fn fetch_vminterfaces(&self) -> Result<Vec<VMInterface>, Error> {
        let mut vminterfaces = Vec::<VMInterface>::new();
        let _ = self.fetch_children(&mut vminterfaces)?;
        Ok(vminterfaces)
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