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
pub use job::Job;
pub use statistics::Statistics;


#[derive(Serialize, Deserialize, Default, Debug)]
pub struct IngressExternalServiceTemplateEntry<'a> {
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

    
    #[serde(rename="ACLTemplateName")]
    pub acl_template_name: Option<String>,
    
    #[serde(rename="ICMPCode")]
    pub icmp_code: Option<String>,
    
    #[serde(rename="ICMPType")]
    pub icmp_type: Option<String>,
    
    #[serde(rename="IPv6AddressOverride")]
    pub ipv6_address_override: Option<String>,
    
    #[serde(rename="DSCP")]
    pub dscp: Option<String>,
    
    pub name: Option<String>,
    
    #[serde(rename="lastUpdatedBy")]
    pub last_updated_by: Option<String>,
    
    pub action: Option<String>,
    
    #[serde(rename="addressOverride")]
    pub address_override: Option<String>,
    
    #[serde(rename="redirectExternalServiceEndPointID")]
    pub redirect_external_service_end_point_id: Option<String>,
    
    pub description: Option<String>,
    
    #[serde(rename="destinationPort")]
    pub destination_port: Option<String>,
    
    #[serde(rename="networkID")]
    pub network_id: Option<String>,
    
    #[serde(rename="networkType")]
    pub network_type: Option<String>,
    
    #[serde(rename="mirrorDestinationID")]
    pub mirror_destination_id: Option<String>,
    
    #[serde(rename="flowLoggingEnabled")]
    pub flow_logging_enabled: bool,
    
    #[serde(rename="enterpriseName")]
    pub enterprise_name: Option<String>,
    
    #[serde(rename="entityScope")]
    pub entity_scope: Option<String>,
    
    #[serde(rename="locationID")]
    pub location_id: Option<String>,
    
    #[serde(rename="locationType")]
    pub location_type: Option<String>,
    
    #[serde(rename="policyState")]
    pub policy_state: Option<String>,
    
    #[serde(rename="domainName")]
    pub domain_name: Option<String>,
    
    #[serde(rename="sourcePort")]
    pub source_port: Option<String>,
    
    pub priority: u64,
    
    pub protocol: Option<String>,
    
    #[serde(rename="associatedApplicationID")]
    pub associated_application_id: Option<String>,
    
    #[serde(rename="associatedApplicationObjectID")]
    pub associated_application_object_id: Option<String>,
    
    #[serde(rename="associatedApplicationObjectType")]
    pub associated_application_object_type: Option<String>,
    
    #[serde(rename="associatedLiveEntityID")]
    pub associated_live_entity_id: Option<String>,
    
    #[serde(rename="statsID")]
    pub stats_id: Option<String>,
    
    #[serde(rename="statsLoggingEnabled")]
    pub stats_logging_enabled: bool,
    
    #[serde(rename="etherType")]
    pub ether_type: Option<String>,
    
    #[serde(rename="externalID")]
    pub external_id: Option<String>,
    
}

impl<'a> RestEntity<'a> for IngressExternalServiceTemplateEntry<'a> {
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
        "ingressexternalserviceentrytemplate"
    }

    fn group_path() -> &'static str {
        "ingressexternalserviceentrytemplates"
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

impl<'a> IngressExternalServiceTemplateEntry<'a> {

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

    pub fn fetch_jobs(&self) -> Result<Vec<Job>, Error> {
        let mut jobs = Vec::<Job>::new();
        let _ = self.fetch_children(&mut jobs)?;
        Ok(jobs)
    }

    pub fn fetch_statistics(&self) -> Result<Vec<Statistics>, Error> {
        let mut statistics = Vec::<Statistics>::new();
        let _ = self.fetch_children(&mut statistics)?;
        Ok(statistics)
    }
}