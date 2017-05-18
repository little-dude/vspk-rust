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


pub use l2domain::L2Domain;
pub use addressrange::AddressRange;
pub use redirectiontargettemplate::RedirectionTargetTemplate;
pub use permission::Permission;
pub use metadata::Metadata;
pub use egressacltemplate::EgressACLTemplate;
pub use globalmetadata::GlobalMetadata;
pub use ingressacltemplate::IngressACLTemplate;
pub use ingressadvfwdtemplate::IngressAdvFwdTemplate;
pub use ingressexternalservicetemplate::IngressExternalServiceTemplate;
pub use job::Job;
pub use policygrouptemplate::PolicyGroupTemplate;
pub use qos::QOS;
pub use group::Group;
pub use eventlog::EventLog;


#[derive(Serialize, Deserialize, Default, Debug)]
pub struct L2DomainTemplate<'a> {
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

    
    #[serde(rename="DHCPManaged")]
    pub dhcp_managed: bool,
    
    #[serde(rename="DPI")]
    pub dpi: Option<String>,
    
    #[serde(rename="IPType")]
    pub ip_type: Option<String>,
    
    pub name: Option<String>,
    
    #[serde(rename="lastUpdatedBy")]
    pub last_updated_by: Option<String>,
    
    pub gateway: Option<String>,
    
    #[serde(rename="gatewayMACAddress")]
    pub gateway_mac_address: Option<String>,
    
    pub address: Option<String>,
    
    pub description: Option<String>,
    
    pub netmask: Option<String>,
    
    pub encryption: Option<String>,
    
    #[serde(rename="entityScope")]
    pub entity_scope: Option<String>,
    
    #[serde(rename="policyChangeStatus")]
    pub policy_change_status: Option<String>,
    
    #[serde(rename="useGlobalMAC")]
    pub use_global_mac: Option<String>,
    
    #[serde(rename="associatedMulticastChannelMapID")]
    pub associated_multicast_channel_map_id: Option<String>,
    
    pub multicast: Option<String>,
    
    #[serde(rename="externalID")]
    pub external_id: Option<String>,
    
}

impl<'a> RestEntity<'a> for L2DomainTemplate<'a> {
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
        "l2domaintemplate"
    }

    fn group_path() -> &'static str {
        "l2domaintemplates"
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

impl<'a> L2DomainTemplate<'a> {

    pub fn fetch_l2domains(&self) -> Result<Vec<L2Domain>, Error> {
        let mut l2domains = Vec::<L2Domain>::new();
        let _ = self.fetch_children(&mut l2domains)?;
        Ok(l2domains)
    }

    pub fn fetch_addressranges(&self) -> Result<Vec<AddressRange>, Error> {
        let mut addressranges = Vec::<AddressRange>::new();
        let _ = self.fetch_children(&mut addressranges)?;
        Ok(addressranges)
    }

    pub fn fetch_redirectiontargettemplates(&self) -> Result<Vec<RedirectionTargetTemplate>, Error> {
        let mut redirectiontargettemplates = Vec::<RedirectionTargetTemplate>::new();
        let _ = self.fetch_children(&mut redirectiontargettemplates)?;
        Ok(redirectiontargettemplates)
    }

    pub fn fetch_permissions(&self) -> Result<Vec<Permission>, Error> {
        let mut permissions = Vec::<Permission>::new();
        let _ = self.fetch_children(&mut permissions)?;
        Ok(permissions)
    }

    pub fn fetch_metadatas(&self) -> Result<Vec<Metadata>, Error> {
        let mut metadatas = Vec::<Metadata>::new();
        let _ = self.fetch_children(&mut metadatas)?;
        Ok(metadatas)
    }

    pub fn fetch_egressacltemplates(&self) -> Result<Vec<EgressACLTemplate>, Error> {
        let mut egressacltemplates = Vec::<EgressACLTemplate>::new();
        let _ = self.fetch_children(&mut egressacltemplates)?;
        Ok(egressacltemplates)
    }

    pub fn fetch_globalmetadatas(&self) -> Result<Vec<GlobalMetadata>, Error> {
        let mut globalmetadatas = Vec::<GlobalMetadata>::new();
        let _ = self.fetch_children(&mut globalmetadatas)?;
        Ok(globalmetadatas)
    }

    pub fn fetch_ingressacltemplates(&self) -> Result<Vec<IngressACLTemplate>, Error> {
        let mut ingressacltemplates = Vec::<IngressACLTemplate>::new();
        let _ = self.fetch_children(&mut ingressacltemplates)?;
        Ok(ingressacltemplates)
    }

    pub fn fetch_ingressadvfwdtemplates(&self) -> Result<Vec<IngressAdvFwdTemplate>, Error> {
        let mut ingressadvfwdtemplates = Vec::<IngressAdvFwdTemplate>::new();
        let _ = self.fetch_children(&mut ingressadvfwdtemplates)?;
        Ok(ingressadvfwdtemplates)
    }

    pub fn fetch_ingressexternalservicetemplates(&self) -> Result<Vec<IngressExternalServiceTemplate>, Error> {
        let mut ingressexternalservicetemplates = Vec::<IngressExternalServiceTemplate>::new();
        let _ = self.fetch_children(&mut ingressexternalservicetemplates)?;
        Ok(ingressexternalservicetemplates)
    }

    pub fn fetch_jobs(&self) -> Result<Vec<Job>, Error> {
        let mut jobs = Vec::<Job>::new();
        let _ = self.fetch_children(&mut jobs)?;
        Ok(jobs)
    }

    pub fn fetch_policygrouptemplates(&self) -> Result<Vec<PolicyGroupTemplate>, Error> {
        let mut policygrouptemplates = Vec::<PolicyGroupTemplate>::new();
        let _ = self.fetch_children(&mut policygrouptemplates)?;
        Ok(policygrouptemplates)
    }

    pub fn fetch_qos(&self) -> Result<Vec<QOS>, Error> {
        let mut qos = Vec::<QOS>::new();
        let _ = self.fetch_children(&mut qos)?;
        Ok(qos)
    }

    pub fn fetch_groups(&self) -> Result<Vec<Group>, Error> {
        let mut groups = Vec::<Group>::new();
        let _ = self.fetch_children(&mut groups)?;
        Ok(groups)
    }

    pub fn fetch_eventlogs(&self) -> Result<Vec<EventLog>, Error> {
        let mut eventlogs = Vec::<EventLog>::new();
        let _ = self.fetch_children(&mut eventlogs)?;
        Ok(eventlogs)
    }
}