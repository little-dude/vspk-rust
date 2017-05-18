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


pub use patipentry::PATIPEntry;
pub use addressrange::AddressRange;
pub use metadata::Metadata;
pub use dhcpoption::DHCPOption;
pub use globalmetadata::GlobalMetadata;
pub use enterprisepermission::EnterprisePermission;
pub use vpnconnection::VPNConnection;
pub use staticroute::StaticRoute;


#[derive(Serialize, Deserialize, Default)]
pub struct SharedNetworkResource<'a> {
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

    
    #[serde(rename="ECMPCount")]
    pub ecmp_count: u64,
    
    #[serde(rename="DHCPManaged")]
    pub dhcp_managed: bool,
    
    #[serde(rename="backHaulRouteDistinguisher")]
    pub back_haul_route_distinguisher: Option<String>,
    
    #[serde(rename="backHaulRouteTarget")]
    pub back_haul_route_target: Option<String>,
    
    #[serde(rename="backHaulVNID")]
    pub back_haul_vnid: u64,
    
    pub name: Option<String>,
    
    #[serde(rename="lastUpdatedBy")]
    pub last_updated_by: Option<String>,
    
    pub gateway: Option<String>,
    
    #[serde(rename="gatewayMACAddress")]
    pub gateway_mac_address: Option<String>,
    
    #[serde(rename="accessRestrictionEnabled")]
    pub access_restriction_enabled: bool,
    
    pub address: Option<String>,
    
    #[serde(rename="permittedActionType")]
    pub permitted_action_type: Option<String>,
    
    pub description: Option<String>,
    
    pub netmask: Option<String>,
    
    #[serde(rename="sharedResourceParentID")]
    pub shared_resource_parent_id: Option<String>,
    
    #[serde(rename="vnID")]
    pub vn_id: u64,
    
    pub underlay: bool,
    
    #[serde(rename="entityScope")]
    pub entity_scope: Option<String>,
    
    #[serde(rename="domainRouteDistinguisher")]
    pub domain_route_distinguisher: Option<String>,
    
    #[serde(rename="domainRouteTarget")]
    pub domain_route_target: Option<String>,
    
    #[serde(rename="uplinkGWVlanAttachmentID")]
    pub uplink_gw_vlan_attachment_id: Option<String>,
    
    #[serde(rename="uplinkInterfaceIP")]
    pub uplink_interface_ip: Option<String>,
    
    #[serde(rename="uplinkInterfaceMAC")]
    pub uplink_interface_mac: Option<String>,
    
    #[serde(rename="uplinkVPortName")]
    pub uplink_vport_name: Option<String>,
    
    #[serde(rename="useGlobalMAC")]
    pub use_global_mac: Option<String>,
    
    #[serde(rename="associatedPATMapperID")]
    pub associated_pat_mapper_id: Option<String>,
    
    #[serde(rename="externalID")]
    pub external_id: Option<String>,
    
    #[serde(rename="dynamicPATAllocationEnabled")]
    pub dynamic_pat_allocation_enabled: bool,
    
    #[serde(rename="type")]
    pub type_: Option<String>,
    
}

impl<'a> RestEntity<'a> for SharedNetworkResource<'a> {
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
        "sharednetworkresource"
    }

    fn group_path() -> &'static str {
        "sharednetworkresources"
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

impl<'a> SharedNetworkResource<'a> {

    pub fn fetch_patipentries(&self) -> Result<Vec<PATIPEntry>, Error> {
        let mut patipentries = Vec::<PATIPEntry>::new();
        let _ = self.fetch_children(&mut patipentries)?;
        Ok(patipentries)
    }

    pub fn fetch_addressranges(&self) -> Result<Vec<AddressRange>, Error> {
        let mut addressranges = Vec::<AddressRange>::new();
        let _ = self.fetch_children(&mut addressranges)?;
        Ok(addressranges)
    }

    pub fn fetch_metadatas(&self) -> Result<Vec<Metadata>, Error> {
        let mut metadatas = Vec::<Metadata>::new();
        let _ = self.fetch_children(&mut metadatas)?;
        Ok(metadatas)
    }

    pub fn fetch_dhcpoptions(&self) -> Result<Vec<DHCPOption>, Error> {
        let mut dhcpoptions = Vec::<DHCPOption>::new();
        let _ = self.fetch_children(&mut dhcpoptions)?;
        Ok(dhcpoptions)
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

    pub fn fetch_vpnconnections(&self) -> Result<Vec<VPNConnection>, Error> {
        let mut vpnconnections = Vec::<VPNConnection>::new();
        let _ = self.fetch_children(&mut vpnconnections)?;
        Ok(vpnconnections)
    }

    pub fn fetch_staticroutes(&self) -> Result<Vec<StaticRoute>, Error> {
        let mut staticroutes = Vec::<StaticRoute>::new();
        let _ = self.fetch_children(&mut staticroutes)?;
        Ok(staticroutes)
    }
}