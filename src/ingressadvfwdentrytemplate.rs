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


pub use metadata::Metadata;
pub use globalmetadata::GlobalMetadata;
pub use job::Job;
pub use statistics::Statistics;


#[derive(Serialize, Deserialize)]
pub struct IngressAdvFwdEntryTemplate<'a> {
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
    acl_template_name: Option<String>,
    
    #[serde(rename="ICMPCode")]
    icmp_code: Option<String>,
    
    #[serde(rename="ICMPType")]
    icmp_type: Option<String>,
    
    #[serde(rename="FCOverride")]
    fc_override: Option<String>,
    
    #[serde(rename="IPv6AddressOverride")]
    ipv6_address_override: Option<String>,
    
    #[serde(rename="DSCP")]
    dscp: Option<String>,
    name: Option<String>,
    
    #[serde(rename="lastUpdatedBy")]
    last_updated_by: Option<String>,
    action: Option<String>,
    
    #[serde(rename="addressOverride")]
    address_override: Option<String>,
    
    #[serde(rename="redirectVPortTagID")]
    redirect_vport_tag_id: Option<String>,
    description: Option<String>,
    
    #[serde(rename="destinationPort")]
    destination_port: Option<String>,
    
    #[serde(rename="networkID")]
    network_id: Option<String>,
    
    #[serde(rename="networkType")]
    network_type: Option<String>,
    
    #[serde(rename="mirrorDestinationID")]
    mirror_destination_id: Option<String>,
    
    #[serde(rename="flowLoggingEnabled")]
    flow_logging_enabled: bool,
    
    #[serde(rename="enterpriseName")]
    enterprise_name: Option<String>,
    
    #[serde(rename="entityScope")]
    entity_scope: Option<String>,
    
    #[serde(rename="locationID")]
    location_id: Option<String>,
    
    #[serde(rename="locationType")]
    location_type: Option<String>,
    
    #[serde(rename="policyState")]
    policy_state: Option<String>,
    
    #[serde(rename="domainName")]
    domain_name: Option<String>,
    
    #[serde(rename="sourcePort")]
    source_port: Option<String>,
    
    #[serde(rename="uplinkPreference")]
    uplink_preference: Option<String>,
    priority: u64,
    protocol: Option<String>,
    
    #[serde(rename="associatedApplicationID")]
    associated_application_id: Option<String>,
    
    #[serde(rename="associatedApplicationObjectID")]
    associated_application_object_id: Option<String>,
    
    #[serde(rename="associatedApplicationObjectType")]
    associated_application_object_type: Option<String>,
    
    #[serde(rename="associatedLiveEntityID")]
    associated_live_entity_id: Option<String>,
    
    #[serde(rename="statsID")]
    stats_id: Option<String>,
    
    #[serde(rename="statsLoggingEnabled")]
    stats_logging_enabled: bool,
    
    #[serde(rename="etherType")]
    ether_type: Option<String>,
    
    #[serde(rename="externalID")]
    external_id: Option<String>,
    
}

impl<'a> RestEntity<'a> for IngressAdvFwdEntryTemplate<'a> {
    fn fetch(&mut self) -> Result<Response, BambouError> {
        match self._session {
            Some(session) => session.fetch(self),
            None => Err(BambouError::NoSession),
        }
    }

    fn path() -> &'static str {
        "ingressadvfwdentrytemplate"
    }

    fn group_path() -> &'static str {
        "ingressadvfwdentrytemplates"
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

impl<'a> IngressAdvFwdEntryTemplate<'a> {

    fn fetch_metadatas(&self) -> Result<Vec<Metadata>, BambouError> {
        let mut metadatas = Vec::<Metadata>::new();
        try!(self.fetch_children(&mut metadatas));
        Ok(metadatas)
    }

    fn fetch_globalmetadatas(&self) -> Result<Vec<GlobalMetadata>, BambouError> {
        let mut globalmetadatas = Vec::<GlobalMetadata>::new();
        try!(self.fetch_children(&mut globalmetadatas));
        Ok(globalmetadatas)
    }

    fn fetch_jobs(&self) -> Result<Vec<Job>, BambouError> {
        let mut jobs = Vec::<Job>::new();
        try!(self.fetch_children(&mut jobs));
        Ok(jobs)
    }

    fn fetch_statistics(&self) -> Result<Vec<Statistics>, BambouError> {
        let mut statistics = Vec::<Statistics>::new();
        try!(self.fetch_children(&mut statistics));
        Ok(statistics)
    }
}