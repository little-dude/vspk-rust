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


pub use monitorscope::Monitorscope;
pub use applicationbinding::ApplicationBinding;


#[derive(Serialize, Deserialize)]
pub struct Application<'a> {
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
    
    #[serde(rename="DSCP")]
    dscp: Option<String>,
    name: Option<String>,
    
    #[serde(rename="readOnly")]
    read_only: bool,
    
    #[serde(rename="performanceMonitorType")]
    performance_monitor_type: Option<String>,
    description: Option<String>,
    
    #[serde(rename="destinationIP")]
    destination_ip: Option<String>,
    
    #[serde(rename="destinationPort")]
    destination_port: Option<String>,
    
    #[serde(rename="enablePPS")]
    enable_pps: bool,
    
    #[serde(rename="oneWayDelay")]
    one_way_delay: u64,
    
    #[serde(rename="oneWayJitter")]
    one_way_jitter: u64,
    
    #[serde(rename="oneWayLoss")]
    one_way_loss: f64,
    
    #[serde(rename="postClassificationPath")]
    post_classification_path: Option<String>,
    
    #[serde(rename="sourceIP")]
    source_ip: Option<String>,
    
    #[serde(rename="sourcePort")]
    source_port: Option<String>,
    
    #[serde(rename="optimizePathSelection")]
    optimize_path_selection: Option<String>,
    
    #[serde(rename="preClassificationPath")]
    pre_classification_path: Option<String>,
    protocol: Option<String>,
    
    #[serde(rename="associatedL7ApplicationSignatureID")]
    associated_l7_application_signature_id: Option<String>,
    
    #[serde(rename="etherType")]
    ether_type: Option<String>,
    symmetry: bool,
    
}

impl<'a> RestEntity<'a> for Application<'a> {
    fn fetch(&mut self) -> Result<Response, BambouError> {
        match self._session {
            Some(session) => session.fetch(self),
            None => Err(BambouError::NoSession),
        }
    }

    fn path() -> &'static str {
        "application"
    }

    fn group_path() -> &'static str {
        "applications"
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

impl<'a> Application<'a> {

    fn fetch_monitorscopes(&self) -> Result<Vec<Monitorscope>, BambouError> {
        let mut monitorscopes = Vec::<Monitorscope>::new();
        try!(self.fetch_children(&mut monitorscopes));
        Ok(monitorscopes)
    }

    fn fetch_applicationbindings(&self) -> Result<Vec<ApplicationBinding>, BambouError> {
        let mut applicationbindings = Vec::<ApplicationBinding>::new();
        try!(self.fetch_children(&mut applicationbindings));
        Ok(applicationbindings)
    }
}
