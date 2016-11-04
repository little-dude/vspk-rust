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
pub use vm::VM;
pub use ingressaclentrytemplate::IngressACLEntryTemplate;
pub use job::Job;
pub use container::Container;
pub use eventlog::EventLog;


#[derive(Serialize, Deserialize)]
pub struct IngressACLTemplate<'a> {
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
    active: bool,
    
    #[serde(rename="defaultAllowIP")]
    default_allow_ip: bool,
    
    #[serde(rename="defaultAllowNonIP")]
    default_allow_non_ip: bool,
    description: Option<String>,
    
    #[serde(rename="allowAddressSpoof")]
    allow_address_spoof: bool,
    
    #[serde(rename="allowL2AddressSpoof")]
    allow_l2_address_spoof: bool,
    
    #[serde(rename="entityScope")]
    entity_scope: Option<String>,
    
    #[serde(rename="policyState")]
    policy_state: Option<String>,
    priority: u64,
    
    #[serde(rename="priorityType")]
    priority_type: Option<String>,
    
    #[serde(rename="assocAclTemplateId")]
    assoc_acl_template_id: Option<String>,
    
    #[serde(rename="associatedLiveEntityID")]
    associated_live_entity_id: Option<String>,
    
    #[serde(rename="externalID")]
    external_id: Option<String>,
    
}

impl<'a> RestEntity<'a> for IngressACLTemplate<'a> {
    fn fetch(&mut self) -> Result<Response, BambouError> {
        match self._session {
            Some(session) => session.fetch(self),
            None => Err(BambouError::NoSession),
        }
    }

    fn path() -> &'static str {
        "ingressacltemplate"
    }

    fn group_path() -> &'static str {
        "ingressacltemplates"
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

impl<'a> IngressACLTemplate<'a> {

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

    fn fetch_vms(&self) -> Result<Vec<VM>, BambouError> {
        let mut vms = Vec::<VM>::new();
        try!(self.fetch_children(&mut vms));
        Ok(vms)
    }

    fn fetch_ingressaclentrytemplates(&self) -> Result<Vec<IngressACLEntryTemplate>, BambouError> {
        let mut ingressaclentrytemplates = Vec::<IngressACLEntryTemplate>::new();
        try!(self.fetch_children(&mut ingressaclentrytemplates));
        Ok(ingressaclentrytemplates)
    }

    fn fetch_jobs(&self) -> Result<Vec<Job>, BambouError> {
        let mut jobs = Vec::<Job>::new();
        try!(self.fetch_children(&mut jobs));
        Ok(jobs)
    }

    fn fetch_containers(&self) -> Result<Vec<Container>, BambouError> {
        let mut containers = Vec::<Container>::new();
        try!(self.fetch_children(&mut containers));
        Ok(containers)
    }

    fn fetch_eventlogs(&self) -> Result<Vec<EventLog>, BambouError> {
        let mut eventlogs = Vec::<EventLog>::new();
        try!(self.fetch_children(&mut eventlogs));
        Ok(eventlogs)
    }
}
