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




#[derive(Serialize, Deserialize, Default, Debug)]
pub struct NSGInfo<'a> {
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

    
    #[serde(rename="MACAddress")]
    pub mac_address: Option<String>,
    
    #[serde(rename="SKU")]
    pub sku: Option<String>,
    
    #[serde(rename="TPMStatus")]
    pub tpm_status: Option<String>,
    
    #[serde(rename="CPUType")]
    pub cpu_type: Option<String>,
    
    #[serde(rename="NSGVersion")]
    pub nsg_version: Option<String>,
    
    #[serde(rename="UUID")]
    pub uuid: Option<String>,
    
    pub family: Option<String>,
    
    #[serde(rename="serialNumber")]
    pub serial_number: Option<String>,
    
    pub libraries: Option<String>,
    
    #[serde(rename="entityScope")]
    pub entity_scope: Option<String>,
    
    #[serde(rename="associatedNSGatewayID")]
    pub associated_ns_gateway_id: Option<String>,
    
    #[serde(rename="externalID")]
    pub external_id: Option<String>,
    
}

impl<'a> RestEntity<'a> for NSGInfo<'a> {
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
        "nsginfo"
    }

    fn group_path() -> &'static str {
        "nsginfos"
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

impl<'a> NSGInfo<'a> {
}