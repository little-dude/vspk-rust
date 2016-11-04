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


#[derive(Serialize, Deserialize)]
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
    name: Option<String>,
    
    #[serde(rename="lastUpdatedBy")]
    last_updated_by: Option<String>,
    
    #[serde(rename="gatewayPeer1AutodiscoveredGatewayID")]
    gateway_peer1_autodiscovered_gateway_id: Option<String>,
    
    #[serde(rename="gatewayPeer1ID")]
    gateway_peer1_id: Option<String>,
    
    #[serde(rename="gatewayPeer1Name")]
    gateway_peer1_name: Option<String>,
    
    #[serde(rename="gatewayPeer2AutodiscoveredGatewayID")]
    gateway_peer2_autodiscovered_gateway_id: Option<String>,
    
    #[serde(rename="gatewayPeer2ID")]
    gateway_peer2_id: Option<String>,
    
    #[serde(rename="gatewayPeer2Name")]
    gateway_peer2_name: Option<String>,
    
    #[serde(rename="redundantGatewayStatus")]
    redundant_gateway_status: Option<String>,
    
    #[serde(rename="permittedAction")]
    permitted_action: Option<String>,
    personality: Option<String>,
    description: Option<String>,
    
    #[serde(rename="enterpriseID")]
    enterprise_id: Option<String>,
    
    #[serde(rename="entityScope")]
    entity_scope: Option<String>,
    vtep: Option<String>,
    
    #[serde(rename="externalID")]
    external_id: Option<String>,
    
}

impl<'a> RestEntity<'a> for RedundancyGroup<'a> {
    fn fetch(&mut self) -> Result<Response, BambouError> {
        match self._session {
            Some(session) => session.fetch(self),
            None => Err(BambouError::NoSession),
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

impl<'a> RedundancyGroup<'a> {

    fn fetch_gateways(&self) -> Result<Vec<Gateway>, BambouError> {
        let mut gateways = Vec::<Gateway>::new();
        try!(self.fetch_children(&mut gateways));
        Ok(gateways)
    }

    fn fetch_permissions(&self) -> Result<Vec<Permission>, BambouError> {
        let mut permissions = Vec::<Permission>::new();
        try!(self.fetch_children(&mut permissions));
        Ok(permissions)
    }

    fn fetch_services(&self) -> Result<Vec<WANService>, BambouError> {
        let mut services = Vec::<WANService>::new();
        try!(self.fetch_children(&mut services));
        Ok(services)
    }

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

    fn fetch_enterprisepermissions(&self) -> Result<Vec<EnterprisePermission>, BambouError> {
        let mut enterprisepermissions = Vec::<EnterprisePermission>::new();
        try!(self.fetch_children(&mut enterprisepermissions));
        Ok(enterprisepermissions)
    }

    fn fetch_ports(&self) -> Result<Vec<Port>, BambouError> {
        let mut ports = Vec::<Port>::new();
        try!(self.fetch_children(&mut ports));
        Ok(ports)
    }

    fn fetch_vsgredundantports(&self) -> Result<Vec<VsgRedundantPort>, BambouError> {
        let mut vsgredundantports = Vec::<VsgRedundantPort>::new();
        try!(self.fetch_children(&mut vsgredundantports));
        Ok(vsgredundantports)
    }

    fn fetch_eventlogs(&self) -> Result<Vec<EventLog>, BambouError> {
        let mut eventlogs = Vec::<EventLog>::new();
        try!(self.fetch_children(&mut eventlogs));
        Ok(eventlogs)
    }
}