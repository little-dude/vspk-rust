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
pub use globalmetadata::GlobalMetadata;
pub use enterprise::Enterprise;
pub use multicastlist::MultiCastList;
pub use eventlog::EventLog;


#[derive(Serialize, Deserialize, Default, Debug)]
pub struct EnterpriseProfile<'a> {
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

    
    #[serde(rename="BGPEnabled")]
    pub bgp_enabled: bool,
    
    #[serde(rename="DHCPLeaseInterval")]
    pub dhcp_lease_interval: u64,
    
    pub name: Option<String>,
    
    #[serde(rename="lastUpdatedBy")]
    pub last_updated_by: Option<String>,
    
    #[serde(rename="receiveMultiCastListID")]
    pub receive_multi_cast_list_id: Option<String>,
    
    #[serde(rename="sendMultiCastListID")]
    pub send_multi_cast_list_id: Option<String>,
    
    pub description: Option<String>,
    
    #[serde(rename="allowAdvancedQOSConfiguration")]
    pub allow_advanced_qos_configuration: bool,
    
    #[serde(rename="allowGatewayManagement")]
    pub allow_gateway_management: bool,
    
    #[serde(rename="allowTrustedForwardingClass")]
    pub allow_trusted_forwarding_class: bool,
    
    #[serde(rename="allowedForwardingClasses")]
    pub allowed_forwarding_classes: Vec<Option<String>>,
    
    #[serde(rename="floatingIPsQuota")]
    pub floating_ips_quota: u64,
    
    #[serde(rename="enableApplicationPerformanceManagement")]
    pub enable_application_performance_management: bool,
    
    #[serde(rename="encryptionManagementMode")]
    pub encryption_management_mode: Option<String>,
    
    #[serde(rename="entityScope")]
    pub entity_scope: Option<String>,
    
    #[serde(rename="externalID")]
    pub external_id: Option<String>,
    
}

impl<'a> RestEntity<'a> for EnterpriseProfile<'a> {
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
        "enterpriseprofile"
    }

    fn group_path() -> &'static str {
        "enterpriseprofiles"
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

impl<'a> EnterpriseProfile<'a> {

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

    pub fn fetch_enterprises(&self) -> Result<Vec<Enterprise>, Error> {
        let mut enterprises = Vec::<Enterprise>::new();
        let _ = self.fetch_children(&mut enterprises)?;
        Ok(enterprises)
    }

    pub fn fetch_multicastlists(&self) -> Result<Vec<MultiCastList>, Error> {
        let mut multicastlists = Vec::<MultiCastList>::new();
        let _ = self.fetch_children(&mut multicastlists)?;
        Ok(multicastlists)
    }

    pub fn fetch_eventlogs(&self) -> Result<Vec<EventLog>, Error> {
        let mut eventlogs = Vec::<EventLog>::new();
        let _ = self.fetch_children(&mut eventlogs)?;
        Ok(eventlogs)
    }
}