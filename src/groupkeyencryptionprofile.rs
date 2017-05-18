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
pub struct GroupKeyEncryptionProfile<'a> {
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

    
    #[serde(rename="SEKGenerationInterval")]
    pub sek_generation_interval: u64,
    
    #[serde(rename="SEKLifetime")]
    pub sek_lifetime: u64,
    
    #[serde(rename="SEKPayloadEncryptionAlgorithm")]
    pub sek_payload_encryption_algorithm: Option<String>,
    
    #[serde(rename="SEKPayloadEncryptionBCAlgorithm")]
    pub sek_payload_encryption_bc_algorithm: Option<String>,
    
    #[serde(rename="SEKPayloadEncryptionKeyLength")]
    pub sek_payload_encryption_key_length: u64,
    
    #[serde(rename="SEKPayloadSigningAlgorithm")]
    pub sek_payload_signing_algorithm: Option<String>,
    
    pub name: Option<String>,
    
    #[serde(rename="lastUpdatedBy")]
    pub last_updated_by: Option<String>,
    
    #[serde(rename="seedGenerationInterval")]
    pub seed_generation_interval: u64,
    
    #[serde(rename="seedLifetime")]
    pub seed_lifetime: u64,
    
    #[serde(rename="seedPayloadAuthenticationAlgorithm")]
    pub seed_payload_authentication_algorithm: Option<String>,
    
    #[serde(rename="seedPayloadAuthenticationBCAlgorithm")]
    pub seed_payload_authentication_bc_algorithm: Option<String>,
    
    #[serde(rename="seedPayloadAuthenticationKeyLength")]
    pub seed_payload_authentication_key_length: u64,
    
    #[serde(rename="seedPayloadEncryptionAlgorithm")]
    pub seed_payload_encryption_algorithm: Option<String>,
    
    #[serde(rename="seedPayloadEncryptionBCAlgorithm")]
    pub seed_payload_encryption_bc_algorithm: Option<String>,
    
    #[serde(rename="seedPayloadEncryptionKeyLength")]
    pub seed_payload_encryption_key_length: u64,
    
    #[serde(rename="seedPayloadSigningAlgorithm")]
    pub seed_payload_signing_algorithm: Option<String>,
    
    pub description: Option<String>,
    
    #[serde(rename="entityScope")]
    pub entity_scope: Option<String>,
    
    #[serde(rename="trafficAuthenticationAlgorithm")]
    pub traffic_authentication_algorithm: Option<String>,
    
    #[serde(rename="trafficEncryptionAlgorithm")]
    pub traffic_encryption_algorithm: Option<String>,
    
    #[serde(rename="trafficEncryptionKeyLifetime")]
    pub traffic_encryption_key_lifetime: u64,
    
    #[serde(rename="associatedEnterpriseID")]
    pub associated_enterprise_id: Option<String>,
    
    #[serde(rename="externalID")]
    pub external_id: Option<String>,
    
}

impl<'a> RestEntity<'a> for GroupKeyEncryptionProfile<'a> {
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
        "groupkeyencryptionprofile"
    }

    fn group_path() -> &'static str {
        "groupkeyencryptionprofiles"
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

impl<'a> GroupKeyEncryptionProfile<'a> {

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