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
pub use ltestatistics::Ltestatistics;
pub use eventlog::EventLog;


#[derive(Serialize, Deserialize, Default)]
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

    
    pub value: u64,
    
    #[serde(rename="lastUpdatedBy")]
    pub last_updated_by: Option<String>,
    
    #[serde(rename="gatewayID")]
    pub gateway_id: Option<String>,
    
    pub readonly: bool,
    
    #[serde(rename="templateID")]
    pub template_id: Option<String>,
    
    #[serde(rename="permittedAction")]
    pub permitted_action: Option<String>,
    
    pub description: Option<String>,
    
    pub restricted: bool,
    
    #[serde(rename="entityScope")]
    pub entity_scope: Option<String>,
    
    #[serde(rename="vportID")]
    pub vport_id: Option<String>,
    
    #[serde(rename="useUserMnemonic")]
    pub use_user_mnemonic: bool,
    
    #[serde(rename="userMnemonic")]
    pub user_mnemonic: Option<String>,
    
    #[serde(rename="associatedBGPProfileID")]
    pub associated_bgp_profile_id: Option<String>,
    
    #[serde(rename="associatedEgressQOSPolicyID")]
    pub associated_egress_qos_policy_id: Option<String>,
    
    #[serde(rename="associatedUplinkConnectionID")]
    pub associated_uplink_connection_id: Option<String>,
    
    #[serde(rename="associatedVSCProfileID")]
    pub associated_vsc_profile_id: Option<String>,
    
    pub status: Option<String>,
    
    #[serde(rename="ducVlan")]
    pub duc_vlan: bool,
    
    #[serde(rename="externalID")]
    pub external_id: Option<String>,
    
    #[serde(rename="type")]
    pub type_: Option<String>,
    
}

impl<'a> RestEntity<'a> for VLAN<'a> {
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

impl<'a> VLAN<'a> {

    pub fn fetch_patnatpools(&self) -> Result<Vec<PATNATPool>, Error> {
        let mut patnatpools = Vec::<PATNATPool>::new();
        let _ = self.fetch_children(&mut patnatpools)?;
        Ok(patnatpools)
    }

    pub fn fetch_permissions(&self) -> Result<Vec<Permission>, Error> {
        let mut permissions = Vec::<Permission>::new();
        let _ = self.fetch_children(&mut permissions)?;
        Ok(permissions)
    }

    pub fn fetch_metadatas(&self) -> Result<Vec<Metadata>, Error> {
        let mut metadatas = Vec::<Metadata>::new();
        let _ = self.fetch_children(&mut metadatas)?;
        Ok(metadatas)
    }

    pub fn fetch_bgpneighbors(&self) -> Result<Vec<BGPNeighbor>, Error> {
        let mut bgpneighbors = Vec::<BGPNeighbor>::new();
        let _ = self.fetch_children(&mut bgpneighbors)?;
        Ok(bgpneighbors)
    }

    pub fn fetch_ikegatewayconnections(&self) -> Result<Vec<IKEGatewayConnection>, Error> {
        let mut ikegatewayconnections = Vec::<IKEGatewayConnection>::new();
        let _ = self.fetch_children(&mut ikegatewayconnections)?;
        Ok(ikegatewayconnections)
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

    pub fn fetch_uplinkconnections(&self) -> Result<Vec<UplinkConnection>, Error> {
        let mut uplinkconnections = Vec::<UplinkConnection>::new();
        let _ = self.fetch_children(&mut uplinkconnections)?;
        Ok(uplinkconnections)
    }

    pub fn fetch_brconnections(&self) -> Result<Vec<BRConnection>, Error> {
        let mut brconnections = Vec::<BRConnection>::new();
        let _ = self.fetch_children(&mut brconnections)?;
        Ok(brconnections)
    }

    pub fn fetch_ltestatistics(&self) -> Result<Vec<Ltestatistics>, Error> {
        let mut ltestatistics = Vec::<Ltestatistics>::new();
        let _ = self.fetch_children(&mut ltestatistics)?;
        Ok(ltestatistics)
    }

    pub fn fetch_eventlogs(&self) -> Result<Vec<EventLog>, Error> {
        let mut eventlogs = Vec::<EventLog>::new();
        let _ = self.fetch_children(&mut eventlogs)?;
        Ok(eventlogs)
    }
}