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
pub use egressaclentrytemplate::EgressACLEntryTemplate;
pub use globalmetadata::GlobalMetadata;
pub use ingressaclentrytemplate::IngressACLEntryTemplate;
pub use ingressadvfwdentrytemplate::IngressAdvFwdEntryTemplate;
pub use vportmirror::VPortMirror;


#[derive(Serialize, Deserialize)]
pub struct MirrorDestination<'a> {
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
    
    #[serde(rename="serviceId")]
    service_id: u64,
    
    #[serde(rename="destinationIp")]
    destination_ip: Option<String>,
    
    #[serde(rename="entityScope")]
    entity_scope: Option<String>,
    
    #[serde(rename="externalID")]
    external_id: Option<String>,
    
}

impl<'a> RestEntity<'a> for MirrorDestination<'a> {
    fn fetch(&mut self) -> Result<Response, BambouError> {
        match self._session {
            Some(session) => session.fetch(self),
            None => Err(BambouError::NoSession),
        }
    }

    fn path() -> &'static str {
        "mirrordestination"
    }

    fn group_path() -> &'static str {
        "mirrordestinations"
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

impl<'a> MirrorDestination<'a> {

    fn fetch_metadatas(&self) -> Result<Vec<Metadata>, BambouError> {
        let mut metadatas = Vec::<Metadata>::new();
        try!(self.fetch_children(&mut metadatas));
        Ok(metadatas)
    }

    fn fetch_egressaclentrytemplates(&self) -> Result<Vec<EgressACLEntryTemplate>, BambouError> {
        let mut egressaclentrytemplates = Vec::<EgressACLEntryTemplate>::new();
        try!(self.fetch_children(&mut egressaclentrytemplates));
        Ok(egressaclentrytemplates)
    }

    fn fetch_globalmetadatas(&self) -> Result<Vec<GlobalMetadata>, BambouError> {
        let mut globalmetadatas = Vec::<GlobalMetadata>::new();
        try!(self.fetch_children(&mut globalmetadatas));
        Ok(globalmetadatas)
    }

    fn fetch_ingressaclentrytemplates(&self) -> Result<Vec<IngressACLEntryTemplate>, BambouError> {
        let mut ingressaclentrytemplates = Vec::<IngressACLEntryTemplate>::new();
        try!(self.fetch_children(&mut ingressaclentrytemplates));
        Ok(ingressaclentrytemplates)
    }

    fn fetch_ingressadvfwdentrytemplates(&self) -> Result<Vec<IngressAdvFwdEntryTemplate>, BambouError> {
        let mut ingressadvfwdentrytemplates = Vec::<IngressAdvFwdEntryTemplate>::new();
        try!(self.fetch_children(&mut ingressadvfwdentrytemplates));
        Ok(ingressadvfwdentrytemplates)
    }

    fn fetch_vportmirrors(&self) -> Result<Vec<VPortMirror>, BambouError> {
        let mut vportmirrors = Vec::<VPortMirror>::new();
        try!(self.fetch_children(&mut vportmirrors));
        Ok(vportmirrors)
    }
}
