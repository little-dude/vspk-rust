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


pub use natmapentry::NATMapEntry;
pub use addressmap::AddressMap;
pub use metadata::Metadata;
pub use globalmetadata::GlobalMetadata;
pub use enterprisepermission::EnterprisePermission;
pub use statistics::Statistics;
pub use statisticspolicy::StatisticsPolicy;
pub use bulkstatistics::BulkStatistics;


#[derive(Serialize, Deserialize)]
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
    name: Option<String>,
    
    #[serde(rename="lastUpdatedBy")]
    last_updated_by: Option<String>,
    
    #[serde(rename="addressRange")]
    address_range: Option<String>,
    
    #[serde(rename="defaultPATIP")]
    default_patip: Option<String>,
    
    #[serde(rename="permittedAction")]
    permitted_action: Option<String>,
    description: Option<String>,
    
    #[serde(rename="endAddressRange")]
    end_address_range: Option<String>,
    
    #[serde(rename="endSourceAddress")]
    end_source_address: Option<String>,
    
    #[serde(rename="entityScope")]
    entity_scope: Option<String>,
    
    #[serde(rename="associatedGatewayId")]
    associated_gateway_id: Option<String>,
    
    #[serde(rename="associatedGatewayType")]
    associated_gateway_type: Option<String>,
    
    #[serde(rename="associatedSubnetId")]
    associated_subnet_id: Option<String>,
    
    #[serde(rename="associatedVlanId")]
    associated_vlan_id: Option<String>,
    
    #[serde(rename="startAddressRange")]
    start_address_range: Option<String>,
    
    #[serde(rename="startSourceAddress")]
    start_source_address: Option<String>,
    
    #[serde(rename="externalID")]
    external_id: Option<String>,
    
    #[serde(rename="dynamicSourceEnabled")]
    dynamic_source_enabled: bool,
    
}

impl<'a> RestEntity<'a> for PATNATPool<'a> {
    fn fetch(&mut self) -> Result<Response, BambouError> {
        match self._session {
            Some(session) => session.fetch(self),
            None => Err(BambouError::NoSession),
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

impl<'a> PATNATPool<'a> {

    fn fetch_natmapentries(&self) -> Result<Vec<NATMapEntry>, BambouError> {
        let mut natmapentries = Vec::<NATMapEntry>::new();
        try!(self.fetch_children(&mut natmapentries));
        Ok(natmapentries)
    }

    fn fetch_addressmaps(&self) -> Result<Vec<AddressMap>, BambouError> {
        let mut addressmaps = Vec::<AddressMap>::new();
        try!(self.fetch_children(&mut addressmaps));
        Ok(addressmaps)
    }

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

    fn fetch_enterprisepermissions(&self) -> Result<Vec<EnterprisePermission>, BambouError> {
        let mut enterprisepermissions = Vec::<EnterprisePermission>::new();
        try!(self.fetch_children(&mut enterprisepermissions));
        Ok(enterprisepermissions)
    }

    fn fetch_statistics(&self) -> Result<Vec<Statistics>, BambouError> {
        let mut statistics = Vec::<Statistics>::new();
        try!(self.fetch_children(&mut statistics));
        Ok(statistics)
    }

    fn fetch_statisticspolicies(&self) -> Result<Vec<StatisticsPolicy>, BambouError> {
        let mut statisticspolicies = Vec::<StatisticsPolicy>::new();
        try!(self.fetch_children(&mut statisticspolicies));
        Ok(statisticspolicies)
    }

    fn fetch_bulkstatistics(&self) -> Result<Vec<BulkStatistics>, BambouError> {
        let mut bulkstatistics = Vec::<BulkStatistics>::new();
        try!(self.fetch_children(&mut bulkstatistics));
        Ok(bulkstatistics)
    }
}
