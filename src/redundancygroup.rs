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


pub use gateway::Gateway;
pub use permission::Permission;
pub use wanservice::WANService;
pub use metadata::Metadata;
pub use alarm::Alarm;
pub use globalmetadata::GlobalMetadata;
pub use enterprisepermission::EnterprisePermission;
pub use port::Port;
pub use vsgredundantport::VsgRedundantPort;
pub use eventlog::EventLog;


#[derive(Serialize, Deserialize, Default, Debug)]
pub struct RedundancyGroup<'a> {
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
    
    #[serde(rename="gatewayPeer1AutodiscoveredGatewayID")]
    pub gateway_peer1_autodiscovered_gateway_id: Option<String>,
    
    #[serde(rename="gatewayPeer1ID")]
    pub gateway_peer1_id: Option<String>,
    
    #[serde(rename="gatewayPeer1Name")]
    pub gateway_peer1_name: Option<String>,
    
    #[serde(rename="gatewayPeer2AutodiscoveredGatewayID")]
    pub gateway_peer2_autodiscovered_gateway_id: Option<String>,
    
    #[serde(rename="gatewayPeer2ID")]
    pub gateway_peer2_id: Option<String>,
    
    #[serde(rename="gatewayPeer2Name")]
    pub gateway_peer2_name: Option<String>,
    
    #[serde(rename="redundantGatewayStatus")]
    pub redundant_gateway_status: Option<String>,
    
    #[serde(rename="permittedAction")]
    pub permitted_action: Option<String>,
    
    pub personality: Option<String>,
    
    pub description: Option<String>,
    
    #[serde(rename="enterpriseID")]
    pub enterprise_id: Option<String>,
    
    #[serde(rename="entityScope")]
    pub entity_scope: Option<String>,
    
    pub vtep: Option<String>,
    
    #[serde(rename="externalID")]
    pub external_id: Option<String>,
    
}

impl<'a> RestEntity<'a> for RedundancyGroup<'a> {
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
        "redundancygroup"
    }

    fn group_path() -> &'static str {
        "redundancygroups"
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

impl<'a> RedundancyGroup<'a> {

    pub fn fetch_gateways(&self) -> Result<Vec<Gateway>, Error> {
        let mut gateways = Vec::<Gateway>::new();
        let _ = self.fetch_children(&mut gateways)?;
        Ok(gateways)
    }

    pub fn fetch_permissions(&self) -> Result<Vec<Permission>, Error> {
        let mut permissions = Vec::<Permission>::new();
        let _ = self.fetch_children(&mut permissions)?;
        Ok(permissions)
    }

    pub fn fetch_services(&self) -> Result<Vec<WANService>, Error> {
        let mut services = Vec::<WANService>::new();
        let _ = self.fetch_children(&mut services)?;
        Ok(services)
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

    pub fn fetch_enterprisepermissions(&self) -> Result<Vec<EnterprisePermission>, Error> {
        let mut enterprisepermissions = Vec::<EnterprisePermission>::new();
        let _ = self.fetch_children(&mut enterprisepermissions)?;
        Ok(enterprisepermissions)
    }

    pub fn fetch_ports(&self) -> Result<Vec<Port>, Error> {
        let mut ports = Vec::<Port>::new();
        let _ = self.fetch_children(&mut ports)?;
        Ok(ports)
    }

    pub fn fetch_vsgredundantports(&self) -> Result<Vec<VsgRedundantPort>, Error> {
        let mut vsgredundantports = Vec::<VsgRedundantPort>::new();
        let _ = self.fetch_children(&mut vsgredundantports)?;
        Ok(vsgredundantports)
    }

    pub fn fetch_eventlogs(&self) -> Result<Vec<EventLog>, Error> {
        let mut eventlogs = Vec::<EventLog>::new();
        let _ = self.fetch_children(&mut eventlogs)?;
        Ok(eventlogs)
    }
}