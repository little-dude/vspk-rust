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


pub use patnatpool::PATNATPool;
pub use permission::Permission;
pub use metadata::Metadata;
pub use bgpneighbor::BGPNeighbor;
pub use ikegatewayconnection::IKEGatewayConnection;
pub use alarm::Alarm;
pub use globalmetadata::GlobalMetadata;
pub use enterprisepermission::EnterprisePermission;
pub use uplinkconnection::UplinkConnection;
pub use brconnection::BRConnection;
pub use eventlog::EventLog;


#[derive(Serialize, Deserialize)]
pub struct VLAN<'a> {
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
    value: u64,
    
    #[serde(rename="lastUpdatedBy")]
    last_updated_by: Option<String>,
    
    #[serde(rename="gatewayID")]
    gateway_id: Option<String>,
    readonly: bool,
    
    #[serde(rename="templateID")]
    template_id: Option<String>,
    
    #[serde(rename="permittedAction")]
    permitted_action: Option<String>,
    description: Option<String>,
    restricted: bool,
    
    #[serde(rename="entityScope")]
    entity_scope: Option<String>,
    
    #[serde(rename="vportID")]
    vport_id: Option<String>,
    
    #[serde(rename="useUserMnemonic")]
    use_user_mnemonic: bool,
    
    #[serde(rename="userMnemonic")]
    user_mnemonic: Option<String>,
    
    #[serde(rename="associatedBGPProfileID")]
    associated_bgp_profile_id: Option<String>,
    
    #[serde(rename="associatedEgressQOSPolicyID")]
    associated_egress_qos_policy_id: Option<String>,
    
    #[serde(rename="associatedUplinkConnectionID")]
    associated_uplink_connection_id: Option<String>,
    
    #[serde(rename="associatedVSCProfileID")]
    associated_vsc_profile_id: Option<String>,
    status: Option<String>,
    
    #[serde(rename="externalID")]
    external_id: Option<String>,
    
}

impl<'a> RestEntity<'a> for VLAN<'a> {
    fn fetch(&mut self) -> Result<Response, BambouError> {
        match self._session {
            Some(session) => session.fetch(self),
            None => Err(BambouError::NoSession),
        }
    }

    fn path() -> &'static str {
        "vlan"
    }

    fn group_path() -> &'static str {
        "vlans"
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

impl<'a> VLAN<'a> {

    fn fetch_patnatpools(&self) -> Result<Vec<PATNATPool>, BambouError> {
        let mut patnatpools = Vec::<PATNATPool>::new();
        try!(self.fetch_children(&mut patnatpools));
        Ok(patnatpools)
    }

    fn fetch_permissions(&self) -> Result<Vec<Permission>, BambouError> {
        let mut permissions = Vec::<Permission>::new();
        try!(self.fetch_children(&mut permissions));
        Ok(permissions)
    }

    fn fetch_metadatas(&self) -> Result<Vec<Metadata>, BambouError> {
        let mut metadatas = Vec::<Metadata>::new();
        try!(self.fetch_children(&mut metadatas));
        Ok(metadatas)
    }

    fn fetch_bgpneighbors(&self) -> Result<Vec<BGPNeighbor>, BambouError> {
        let mut bgpneighbors = Vec::<BGPNeighbor>::new();
        try!(self.fetch_children(&mut bgpneighbors));
        Ok(bgpneighbors)
    }

    fn fetch_ikegatewayconnections(&self) -> Result<Vec<IKEGatewayConnection>, BambouError> {
        let mut ikegatewayconnections = Vec::<IKEGatewayConnection>::new();
        try!(self.fetch_children(&mut ikegatewayconnections));
        Ok(ikegatewayconnections)
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

    fn fetch_uplinkconnections(&self) -> Result<Vec<UplinkConnection>, BambouError> {
        let mut uplinkconnections = Vec::<UplinkConnection>::new();
        try!(self.fetch_children(&mut uplinkconnections));
        Ok(uplinkconnections)
    }

    fn fetch_brconnections(&self) -> Result<Vec<BRConnection>, BambouError> {
        let mut brconnections = Vec::<BRConnection>::new();
        try!(self.fetch_children(&mut brconnections));
        Ok(brconnections)
    }

    fn fetch_eventlogs(&self) -> Result<Vec<EventLog>, BambouError> {
        let mut eventlogs = Vec::<EventLog>::new();
        try!(self.fetch_children(&mut eventlogs));
        Ok(eventlogs)
    }
}
