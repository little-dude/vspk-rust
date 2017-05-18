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




#[derive(Serialize, Deserialize, Default)]
pub struct FirewallRule<'a> {
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
    pub acl_template_name: Option<String>,
    
    #[serde(rename="ICMPCode")]
    pub icmp_code: Option<String>,
    
    #[serde(rename="ICMPType")]
    pub icmp_type: Option<String>,
    
    #[serde(rename="IPv6AddressOverride")]
    pub ipv6_address_override: Option<String>,
    
    #[serde(rename="DSCP")]
    pub dscp: Option<String>,
    
    pub action: Option<String>,
    
    #[serde(rename="addressOverride")]
    pub address_override: Option<String>,
    
    pub description: Option<String>,
    
    #[serde(rename="destNetwork")]
    pub dest_network: Option<String>,
    
    #[serde(rename="destPgId")]
    pub dest_pg_id: Option<String>,
    
    #[serde(rename="destPgType")]
    pub dest_pg_type: Option<String>,
    
    #[serde(rename="destinationIpv6Value")]
    pub destination_ipv6_value: Option<String>,
    
    #[serde(rename="destinationPort")]
    pub destination_port: Option<String>,
    
    #[serde(rename="destinationType")]
    pub destination_type: Option<String>,
    
    #[serde(rename="destinationValue")]
    pub destination_value: Option<String>,
    
    #[serde(rename="networkID")]
    pub network_id: Option<String>,
    
    #[serde(rename="networkType")]
    pub network_type: Option<String>,
    
    #[serde(rename="mirrorDestinationID")]
    pub mirror_destination_id: Option<String>,
    
    #[serde(rename="flowLoggingEnabled")]
    pub flow_logging_enabled: bool,
    
    #[serde(rename="enterpriseName")]
    pub enterprise_name: Option<String>,
    
    #[serde(rename="locationID")]
    pub location_id: Option<String>,
    
    #[serde(rename="locationType")]
    pub location_type: Option<String>,
    
    #[serde(rename="domainName")]
    pub domain_name: Option<String>,
    
    #[serde(rename="sourceIpv6Value")]
    pub source_ipv6_value: Option<String>,
    
    #[serde(rename="sourceNetwork")]
    pub source_network: Option<String>,
    
    #[serde(rename="sourcePgId")]
    pub source_pg_id: Option<String>,
    
    #[serde(rename="sourcePgType")]
    pub source_pg_type: Option<String>,
    
    #[serde(rename="sourcePort")]
    pub source_port: Option<String>,
    
    #[serde(rename="sourceType")]
    pub source_type: Option<String>,
    
    #[serde(rename="sourceValue")]
    pub source_value: Option<String>,
    
    pub priority: Option<String>,
    
    #[serde(rename="associatedApplicationID")]
    pub associated_application_id: Option<String>,
    
    #[serde(rename="associatedApplicationObjectID")]
    pub associated_application_object_id: Option<String>,
    
    #[serde(rename="associatedfirewallACLID")]
    pub associatedfirewall_aclid: Option<String>,
    
    pub stateful: bool,
    
    #[serde(rename="statsID")]
    pub stats_id: Option<String>,
    
    #[serde(rename="statsLoggingEnabled")]
    pub stats_logging_enabled: bool,
    
    #[serde(rename="etherType")]
    pub ether_type: Option<String>,
    
}

impl<'a> RestEntity<'a> for FirewallRule<'a> {
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
        "firewallrule"
    }

    fn group_path() -> &'static str {
        "firewallrules"
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

impl<'a> FirewallRule<'a> {
}