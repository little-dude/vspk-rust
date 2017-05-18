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


pub use tca::TCA;
pub use metadata::Metadata;
pub use globalmetadata::GlobalMetadata;
pub use vm::VM;
pub use container::Container;
pub use vport::VPort;
pub use statistics::Statistics;
pub use statisticspolicy::StatisticsPolicy;
pub use eventlog::EventLog;


#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Tier<'a> {
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
    
    #[serde(rename="lastUpdatedBy")]
    pub last_updated_by: Option<String>,
    
    pub gateway: Option<String>,
    
    pub address: Option<String>,
    
    pub description: Option<String>,
    
    pub metadata: Option<String>,
    
    pub netmask: Option<String>,
    
    #[serde(rename="entityScope")]
    pub entity_scope: Option<String>,
    
    #[serde(rename="associatedApplicationID")]
    pub associated_application_id: Option<String>,
    
    #[serde(rename="associatedFloatingIPPoolID")]
    pub associated_floating_ip_pool_id: Option<String>,
    
    #[serde(rename="associatedNetworkMacroID")]
    pub associated_network_macro_id: Option<String>,
    
    #[serde(rename="associatedNetworkObjectID")]
    pub associated_network_object_id: Option<String>,
    
    #[serde(rename="associatedNetworkObjectType")]
    pub associated_network_object_type: Option<String>,
    
    #[serde(rename="externalID")]
    pub external_id: Option<String>,
    
    #[serde(rename="type")]
    pub type_: Option<String>,
    
}

impl<'a> RestEntity<'a> for Tier<'a> {
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
        "tier"
    }

    fn group_path() -> &'static str {
        "tiers"
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

impl<'a> Tier<'a> {

    pub fn fetch_tcas(&self) -> Result<Vec<TCA>, Error> {
        let mut tcas = Vec::<TCA>::new();
        let _ = self.fetch_children(&mut tcas)?;
        Ok(tcas)
    }

    pub fn fetch_metadatas(&self) -> Result<Vec<Metadata>, Error> {
        let mut metadatas = Vec::<Metadata>::new();
        let _ = self.fetch_children(&mut metadatas)?;
        Ok(metadatas)
    }

    pub fn fetch_globalmetadatas(&self) -> Result<Vec<GlobalMetadata>, Error> {
        let mut globalmetadatas = Vec::<GlobalMetadata>::new();
        let _ = self.fetch_children(&mut globalmetadatas)?;
        Ok(globalmetadatas)
    }

    pub fn fetch_vms(&self) -> Result<Vec<VM>, Error> {
        let mut vms = Vec::<VM>::new();
        let _ = self.fetch_children(&mut vms)?;
        Ok(vms)
    }

    pub fn fetch_containers(&self) -> Result<Vec<Container>, Error> {
        let mut containers = Vec::<Container>::new();
        let _ = self.fetch_children(&mut containers)?;
        Ok(containers)
    }

    pub fn fetch_vports(&self) -> Result<Vec<VPort>, Error> {
        let mut vports = Vec::<VPort>::new();
        let _ = self.fetch_children(&mut vports)?;
        Ok(vports)
    }

    pub fn fetch_statistics(&self) -> Result<Vec<Statistics>, Error> {
        let mut statistics = Vec::<Statistics>::new();
        let _ = self.fetch_children(&mut statistics)?;
        Ok(statistics)
    }

    pub fn fetch_statisticspolicies(&self) -> Result<Vec<StatisticsPolicy>, Error> {
        let mut statisticspolicies = Vec::<StatisticsPolicy>::new();
        let _ = self.fetch_children(&mut statisticspolicies)?;
        Ok(statisticspolicies)
    }

    pub fn fetch_eventlogs(&self) -> Result<Vec<EventLog>, Error> {
        let mut eventlogs = Vec::<EventLog>::new();
        let _ = self.fetch_children(&mut eventlogs)?;
        Ok(eventlogs)
    }
}