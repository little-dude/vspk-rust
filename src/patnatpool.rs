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


pub use natmapentry::NATMapEntry;
pub use addressmap::AddressMap;
pub use metadata::Metadata;
pub use globalmetadata::GlobalMetadata;
pub use enterprisepermission::EnterprisePermission;
pub use statistics::Statistics;
pub use statisticspolicy::StatisticsPolicy;
pub use bulkstatistics::BulkStatistics;


#[derive(Serialize, Deserialize, Default)]
pub struct PATNATPool<'a> {
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

    
    pub name: Option<String>,
    
    #[serde(rename="lastUpdatedBy")]
    pub last_updated_by: Option<String>,
    
    #[serde(rename="addressRange")]
    pub address_range: Option<String>,
    
    #[serde(rename="defaultPATIP")]
    pub default_patip: Option<String>,
    
    #[serde(rename="permittedAction")]
    pub permitted_action: Option<String>,
    
    pub description: Option<String>,
    
    #[serde(rename="endAddressRange")]
    pub end_address_range: Option<String>,
    
    #[serde(rename="endSourceAddress")]
    pub end_source_address: Option<String>,
    
    #[serde(rename="entityScope")]
    pub entity_scope: Option<String>,
    
    #[serde(rename="associatedGatewayId")]
    pub associated_gateway_id: Option<String>,
    
    #[serde(rename="associatedGatewayType")]
    pub associated_gateway_type: Option<String>,
    
    #[serde(rename="associatedSubnetId")]
    pub associated_subnet_id: Option<String>,
    
    #[serde(rename="associatedVlanId")]
    pub associated_vlan_id: Option<String>,
    
    #[serde(rename="startAddressRange")]
    pub start_address_range: Option<String>,
    
    #[serde(rename="startSourceAddress")]
    pub start_source_address: Option<String>,
    
    #[serde(rename="externalID")]
    pub external_id: Option<String>,
    
    #[serde(rename="dynamicSourceEnabled")]
    pub dynamic_source_enabled: bool,
    
}

impl<'a> RestEntity<'a> for PATNATPool<'a> {
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
        "patnatpool"
    }

    fn group_path() -> &'static str {
        "patnatpools"
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

impl<'a> PATNATPool<'a> {

    pub fn fetch_natmapentries(&self) -> Result<Vec<NATMapEntry>, Error> {
        let mut natmapentries = Vec::<NATMapEntry>::new();
        let _ = self.fetch_children(&mut natmapentries)?;
        Ok(natmapentries)
    }

    pub fn fetch_addressmaps(&self) -> Result<Vec<AddressMap>, Error> {
        let mut addressmaps = Vec::<AddressMap>::new();
        let _ = self.fetch_children(&mut addressmaps)?;
        Ok(addressmaps)
    }

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

    pub fn fetch_enterprisepermissions(&self) -> Result<Vec<EnterprisePermission>, Error> {
        let mut enterprisepermissions = Vec::<EnterprisePermission>::new();
        let _ = self.fetch_children(&mut enterprisepermissions)?;
        Ok(enterprisepermissions)
    }

    pub fn fetch_statistics(&self) -> Result<Vec<Statistics>, Error> {
        let mut statistics = Vec::<Statistics>::new();
        let _ = self.fetch_children(&mut statistics)?;
        Ok(statistics)
    }

    pub fn fetch_statisticspolicies(&self) -> Result<Vec<StatisticsPolicy>, Error> {
        let mut statisticspolicies = Vec::<StatisticsPolicy>::new();
        let _ = self.fetch_children(&mut statisticspolicies)?;
        Ok(statisticspolicies)
    }

    pub fn fetch_bulkstatistics(&self) -> Result<Vec<BulkStatistics>, Error> {
        let mut bulkstatistics = Vec::<BulkStatistics>::new();
        let _ = self.fetch_children(&mut bulkstatistics)?;
        Ok(bulkstatistics)
    }
}