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


pub use patipentry::PATIPEntry;
pub use addressrange::AddressRange;
pub use metadata::Metadata;
pub use dhcpoption::DHCPOption;
pub use globalmetadata::GlobalMetadata;
pub use enterprisepermission::EnterprisePermission;
pub use vpnconnection::VPNConnection;
pub use staticroute::StaticRoute;


#[derive(Serialize, Deserialize)]
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
    ecmp_count: u64,
    
    #[serde(rename="DHCPManaged")]
    dhcp_managed: bool,
    
    #[serde(rename="backHaulRouteDistinguisher")]
    back_haul_route_distinguisher: Option<String>,
    
    #[serde(rename="backHaulRouteTarget")]
    back_haul_route_target: Option<String>,
    
    #[serde(rename="backHaulVNID")]
    back_haul_vnid: u64,
    name: Option<String>,
    
    #[serde(rename="lastUpdatedBy")]
    last_updated_by: Option<String>,
    gateway: Option<String>,
    
    #[serde(rename="gatewayMACAddress")]
    gateway_mac_address: Option<String>,
    
    #[serde(rename="accessRestrictionEnabled")]
    access_restriction_enabled: bool,
    address: Option<String>,
    
    #[serde(rename="permittedActionType")]
    permitted_action_type: Option<String>,
    description: Option<String>,
    netmask: Option<String>,
    
    #[serde(rename="sharedResourceParentID")]
    shared_resource_parent_id: Option<String>,
    
    #[serde(rename="vnID")]
    vn_id: u64,
    underlay: bool,
    
    #[serde(rename="entityScope")]
    entity_scope: Option<String>,
    
    #[serde(rename="domainRouteDistinguisher")]
    domain_route_distinguisher: Option<String>,
    
    #[serde(rename="domainRouteTarget")]
    domain_route_target: Option<String>,
    
    #[serde(rename="uplinkGWVlanAttachmentID")]
    uplink_gw_vlan_attachment_id: Option<String>,
    
    #[serde(rename="uplinkInterfaceIP")]
    uplink_interface_ip: Option<String>,
    
    #[serde(rename="uplinkInterfaceMAC")]
    uplink_interface_mac: Option<String>,
    
    #[serde(rename="uplinkVPortName")]
    uplink_vport_name: Option<String>,
    
    #[serde(rename="useGlobalMAC")]
    use_global_mac: Option<String>,
    
    #[serde(rename="associatedPATMapperID")]
    associated_pat_mapper_id: Option<String>,
    
    #[serde(rename="externalID")]
    external_id: Option<String>,
    
    #[serde(rename="dynamicPATAllocationEnabled")]
    dynamic_pat_allocation_enabled: bool,
    
    #[serde(rename="type")]
    type_: Option<String>,
    
}

impl<'a> RestEntity<'a> for SharedNetworkResource<'a> {
    fn fetch(&mut self) -> Result<Response, BambouError> {
        match self._session {
            Some(session) => session.fetch(self),
            None => Err(BambouError::NoSession),
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

impl<'a> SharedNetworkResource<'a> {

    fn fetch_patipentries(&self) -> Result<Vec<PATIPEntry>, BambouError> {
        let mut patipentries = Vec::<PATIPEntry>::new();
        try!(self.fetch_children(&mut patipentries));
        Ok(patipentries)
    }

    fn fetch_addressranges(&self) -> Result<Vec<AddressRange>, BambouError> {
        let mut addressranges = Vec::<AddressRange>::new();
        try!(self.fetch_children(&mut addressranges));
        Ok(addressranges)
    }

    fn fetch_metadatas(&self) -> Result<Vec<Metadata>, BambouError> {
        let mut metadatas = Vec::<Metadata>::new();
        try!(self.fetch_children(&mut metadatas));
        Ok(metadatas)
    }

    fn fetch_dhcpoptions(&self) -> Result<Vec<DHCPOption>, BambouError> {
        let mut dhcpoptions = Vec::<DHCPOption>::new();
        try!(self.fetch_children(&mut dhcpoptions));
        Ok(dhcpoptions)
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

    fn fetch_vpnconnections(&self) -> Result<Vec<VPNConnection>, BambouError> {
        let mut vpnconnections = Vec::<VPNConnection>::new();
        try!(self.fetch_children(&mut vpnconnections));
        Ok(vpnconnections)
    }

    fn fetch_staticroutes(&self) -> Result<Vec<StaticRoute>, BambouError> {
        let mut staticroutes = Vec::<StaticRoute>::new();
        try!(self.fetch_children(&mut staticroutes));
        Ok(staticroutes)
    }
}
