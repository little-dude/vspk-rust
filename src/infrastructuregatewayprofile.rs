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


#[derive(Serialize, Deserialize, Default)]
pub struct InfrastructureGatewayProfile<'a> {
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

    
    #[serde(rename="NTPServerKey")]
    pub ntp_server_key: Option<String>,
    
    #[serde(rename="NTPServerKeyID")]
    pub ntp_server_key_id: u64,
    
    pub name: Option<String>,
    
    #[serde(rename="lastUpdatedBy")]
    pub last_updated_by: Option<String>,
    
    #[serde(rename="datapathSyncTimeout")]
    pub datapath_sync_timeout: u64,
    
    #[serde(rename="deadTimer")]
    pub dead_timer: Option<String>,
    
    #[serde(rename="deadTimerEnabled")]
    pub dead_timer_enabled: bool,
    
    #[serde(rename="remoteLogMode")]
    pub remote_log_mode: Option<String>,
    
    #[serde(rename="remoteLogServerAddress")]
    pub remote_log_server_address: Option<String>,
    
    #[serde(rename="remoteLogServerPort")]
    pub remote_log_server_port: u64,
    
    pub description: Option<String>,
    
    #[serde(rename="metadataUpgradePath")]
    pub metadata_upgrade_path: Option<String>,
    
    #[serde(rename="flowEvictionThreshold")]
    pub flow_eviction_threshold: u64,
    
    #[serde(rename="enterpriseID")]
    pub enterprise_id: Option<String>,
    
    #[serde(rename="entityScope")]
    pub entity_scope: Option<String>,
    
    #[serde(rename="controllerLessDuration")]
    pub controller_less_duration: Option<String>,
    
    #[serde(rename="controllerLessEnabled")]
    pub controller_less_enabled: bool,
    
    #[serde(rename="controllerLessForwardingMode")]
    pub controller_less_forwarding_mode: Option<String>,
    
    #[serde(rename="controllerLessRemoteDuration")]
    pub controller_less_remote_duration: Option<String>,
    
    #[serde(rename="forceImmediateSystemSync")]
    pub force_immediate_system_sync: bool,
    
    #[serde(rename="openFlowAuditTimer")]
    pub open_flow_audit_timer: u64,
    
    #[serde(rename="upgradeAction")]
    pub upgrade_action: Option<String>,
    
    #[serde(rename="proxyDNSName")]
    pub proxy_dns_name: Option<String>,
    
    #[serde(rename="useTwoFactor")]
    pub use_two_factor: bool,
    
    #[serde(rename="statsCollectorPort")]
    pub stats_collector_port: u64,
    
    #[serde(rename="externalID")]
    pub external_id: Option<String>,
    
    #[serde(rename="systemSyncScheduler")]
    pub system_sync_scheduler: Option<String>,
    
}

impl<'a> RestEntity<'a> for InfrastructureGatewayProfile<'a> {
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
        "infrastructuregatewayprofile"
    }

    fn group_path() -> &'static str {
        "infrastructuregatewayprofiles"
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

impl<'a> InfrastructureGatewayProfile<'a> {

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
}