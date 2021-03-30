use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::fs::read;
use std::io::Write;
use std::net::ToSocketAddrs;
use std::str::FromStr;

use strong_xml::xmlparser::{ElementEnd, Token};
use strong_xml::{XmlRead, XmlReader, XmlResult, XmlWrite, XmlWriter};

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "applications")]
pub struct Applications {
    #[xml(flatten_text = "versions__delta")]
    pub versions_delta: Option<String>,
    #[xml(flatten_text = "apps__hashcode")]
    pub apps_hashcode: Option<String>,
    #[xml(child = "application")]
    pub applications: Vec<Application>,
}

#[derive(XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "application")]
pub struct Application {
    #[xml(flatten_text = "name")]
    pub name: String,
    #[xml(child = "instance")]
    pub instances: Vec<Instance>,
}

#[derive(Clone, XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "instance")]
pub struct Instance {
    #[xml(flatten_text = "hostName")]
    pub host_name: String,
    #[xml(flatten_text = "instanceId")]
    pub instance_id: Option<String>,
    #[xml(flatten_text = "app")]
    pub app: String,
    #[xml(flatten_text = "ipAddr")]
    pub ip_addr: String,
    #[xml(flatten_text = "vipAddress")]
    pub vip_address: String,
    #[xml(flatten_text = "secureVipAddress")]
    pub secure_vip_address: String,
    #[xml(flatten_text = "status")]
    pub status: StatusType,
    #[xml(child = "port")]
    pub port: PortData,
    #[xml(child = "securePort")]
    pub secure_port: SecurePort,
    #[xml(flatten_text = "homePageUrl")]
    pub home_page_url: String,
    #[xml(flatten_text = "statusPageUrl")]
    pub status_page_url: String,
    #[xml(flatten_text = "healthCheckUrl")]
    pub health_check_url: String,
    #[xml(child = "dataCenterInfo")]
    pub data_center_info: DataCenterInfo,
    #[xml(child = "leaseInfo")]
    pub lease_info: Option<LeaseInfo>,
    #[xml(child = "metadata")]
    pub metadata: Option<AppMetaDataType>,
}

impl Default for Instance {
    fn default() -> Self {
        Instance {
            host_name: "localhost".to_string(),
            instance_id: None,
            app: env!("CARGO_PKG_NAME").to_string(),
            ip_addr: "127.0.0.1".to_string(),
            vip_address: env!("CARGO_PKG_NAME").to_string(),
            secure_vip_address: env!("CARGO_PKG_NAME").to_string(),
            status: StatusType::Starting,
            port: PortData::default(),
            secure_port: SecurePort::default(),
            home_page_url: "".to_string(),
            status_page_url: "".to_string(),
            health_check_url: "".to_string(),
            data_center_info: DataCenterInfo::default(),
            lease_info: None,
            metadata: None,
        }
    }
}

#[derive(Clone, XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "port")]
pub struct PortData {
    #[xml(attr = "enabled")]
    pub enabled: bool,
    #[xml(default, text)]
    pub value: u16,
}

impl Default for PortData {
    fn default() -> Self {
        PortData {
            enabled: true,
            value: 80,
        }
    }
}

impl PortData {
    pub fn new(port: u16, enabled: bool) -> Self {
        Self {
            value: port,
            enabled,
        }
    }
}

#[derive(Clone, XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "securePort")]
pub struct SecurePort {
    #[xml(attr = "enabled")]
    pub enabled: bool,
    #[xml(default, text)]
    pub value: u16,
}

impl Default for SecurePort {
    fn default() -> Self {
        SecurePort {
            enabled: false,
            value: 443,
        }
    }
}

impl SecurePort {
    pub fn new(port: u16, enabled: bool) -> Self {
        Self {
            value: port,
            enabled,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum DcNameType {
    MyOwn,
    Amazon,
}

impl Display for DcNameType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::MyOwn => write!(f, "MyOwn"),
            Self::Amazon => write!(f, "Amazon"),
        }
    }
}

impl FromStr for DcNameType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "MyOwn" => Ok(Self::MyOwn),
            "Amazon" => Ok(Self::Amazon),
            _ => Err("Invalid dcNameType".to_string()),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum StatusType {
    Up,
    Down,
    Starting,
    OutOfService,
    Unknown,
}

impl Display for StatusType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::Up => write!(f, "UP"),
            Self::Down => write!(f, "DOWN"),
            Self::Starting => write!(f, "STARTING"),
            Self::OutOfService => write!(f, "OUT_OF_SERVICE"),
            Self::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

impl FromStr for StatusType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "UP" => Ok(Self::Up),
            "DOWN" => Ok(Self::Down),
            "OUT_OF_SERVICE" => Ok(Self::OutOfService),
            "UNKNOWN" => Ok(Self::Unknown),
            _ => Err("Invalid statusType".to_string()),
        }
    }
}

#[derive(Clone, XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "metadata")]
pub struct AmazonMetaDataType {
    #[xml(flatten_text = "ami-launch-index")]
    pub ami_launch_index: String,
    #[xml(flatten_text = "local-hostname")]
    pub local_hostname: String,
    #[xml(flatten_text = "availability-zone")]
    pub availability_zone: String,
    #[xml(flatten_text = "instance-id")]
    pub instance_id: String,
    #[xml(flatten_text = "public-ipv4")]
    pub public_ipv4: String,
    #[xml(flatten_text = "public-hostname")]
    pub public_hostname: String,
    #[xml(flatten_text = "ami-manifest-patch")]
    pub ami_manifest_patch: String,
    #[xml(flatten_text = "local-ipv4")]
    pub local_ipv4: String,
    #[xml(flatten_text = "hostname")]
    pub hostname: String,
    #[xml(flatten_text = "ami-id")]
    pub ami_id: String,
    #[xml(flatten_text = "instance-type")]
    pub instance_type: String,
}

#[derive(Clone, XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "dataCenterInfo")]
pub struct DataCenterInfo {
    #[xml(attr = "class")]
    pub class: Option<String>,
    #[xml(flatten_text = "name")]
    pub name: DcNameType,
    #[xml(child = "metadata")]
    pub metadata: Option<AmazonMetaDataType>,
}

impl Default for DataCenterInfo {
    fn default() -> Self {
        DataCenterInfo {
            class: None,
            name: DcNameType::MyOwn,
            metadata: None,
        }
    }
}

#[derive(Clone, XmlWrite, XmlRead, PartialEq, Debug)]
#[xml(tag = "leaseInfo")]
pub struct LeaseInfo {
    #[xml(flatten_text = "evictionDurationInSecs")]
    pub eviction_duration_in_secs: Option<usize>,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct AppMetaDataType {
    pub class: Option<String>,
    pub map: HashMap<String, String>,
}

impl AppMetaDataType {
    pub const TAG: &'static str = "metadata";
}

impl<'a> XmlRead<'a> for AppMetaDataType {
    fn from_reader(reader: &mut XmlReader<'a>) -> XmlResult<Self> {
        let mut metadata = Self::default();
        reader.read_till_element_start(AppMetaDataType::TAG)?;

        // attr class
        while let Some((attr_name, attr_val)) = reader.find_attribute()? {
            if attr_name.eq("class") {
                metadata.class = Some(attr_val.to_string());
            }
        }

        // end?
        let next = reader.next().unwrap()?;
        if let Token::ElementEnd {
            end: ElementEnd::Empty,
            ..
        } = next
        {
            return Ok(metadata);
        }

        // child
        while let Some(child_key) = reader.find_element_start(Some(AppMetaDataType::TAG))? {
            reader.next();
            let child_value = reader.read_text(child_key)?;
            metadata
                .map
                .insert(child_key.to_string(), child_value.to_string());
        }

        Ok(metadata)
    }
}

impl XmlWrite for AppMetaDataType {
    fn to_writer<W: Write>(&self, writer: &mut XmlWriter<W>) -> XmlResult<()> {
        writer.write_element_start(AppMetaDataType::TAG);
        if let Some(v) = &self.class {
            writer.write_attribute("class", v.as_str())?;
        }
        writer.write_element_end_open()?;
        for (k, v) in &self.map {
            writer.write_flatten_text(k.as_str(), v.as_str(), false);
        }
        writer.write_element_end_close(AppMetaDataType::TAG);
        Ok(())
    }
}

mod tests {
    use std::str::FromStr;

    use strong_xml::{XmlRead, XmlResult};

    use super::*;

    #[derive(XmlRead, PartialEq, Debug)]
    #[xml(tag = "root")]
    struct R {
        #[xml(attr = "bar")]
        bar: Option<String>,
        #[xml(attr = "baz")]
        baz: Option<usize>,
    }

    #[test]
    fn test_xml_r() -> XmlResult<()> {
        let xml = r#"<root bar="bar" baz="123"/>"#;
        let r = R::from_str(xml)?;
        Ok(())
    }

    #[test]
    fn test_xml_applications() -> XmlResult<()> {
        let xml = r#"<applications></applications>"#;
        let applications = Applications::from_str(xml)?;
        Ok(())
    }

    #[test]
    fn test_xml_application() -> XmlResult<()> {
        let xml = r#"<application><name>abcd</name></application>"#;
        let application = Application::from_str(xml)?;
        Ok(())
    }

    #[test]
    fn test_xml_instance() -> XmlResult<()> {
        let xml = r#"<instance>
      <hostName>localhost</hostName>
      <app>BENCH</app>
      <ipAddr>127.0.0.1</ipAddr>
      <status>UP</status>
      <overriddenstatus>UP</overriddenstatus>
      <port enabled="true">8080</port>
      <securePort enabled="false">443</securePort>
      <countryId>1</countryId>
      <dataCenterInfo class="com.netflix.appinfo.InstanceInfo$DefaultDataCenterInfo">
        <name>MyOwn</name>
      </dataCenterInfo>
      <leaseInfo>
        <renewalIntervalInSecs>30</renewalIntervalInSecs>
        <durationInSecs>90</durationInSecs>
        <registrationTimestamp>1616761261538</registrationTimestamp>
        <lastRenewalTimestamp>1616761921820</lastRenewalTimestamp>
        <evictionTimestamp>0</evictionTimestamp>
        <serviceUpTimestamp>1616761261439</serviceUpTimestamp>
      </leaseInfo>
      <metadata class="java.util.Collections$EmptyMap"/>
      <homePageUrl>/eureka</homePageUrl>
      <statusPageUrl></statusPageUrl>
      <healthCheckUrl></healthCheckUrl>
      <vipAddress>bench</vipAddress>
      <secureVipAddress>bench</secureVipAddress>
      <isCoordinatingDiscoveryServer>false</isCoordinatingDiscoveryServer>
      <lastUpdatedTimestamp>1616761261538</lastUpdatedTimestamp>
      <lastDirtyTimestamp>1616761261439</lastDirtyTimestamp>
      <actionType>ADDED</actionType>
    </instance>"#;

        let instance: Instance = Instance::from_str(xml)?;
        assert_eq!(instance.host_name, "localhost");
        assert_eq!(instance.app, "BENCH");
        assert_eq!(instance.ip_addr, "127.0.0.1");

        Ok(())
    }

    #[test]
    fn test_xml_port() -> XmlResult<()> {
        let xml = r#"<port enabled="false">80</port>"#;
        let port = PortData::from_str(xml)?;
        assert_eq!(port.enabled, false);
        assert_eq!(port.value, 80);
        Ok(())
    }

    #[test]
    fn test_xml_data_center_info() -> XmlResult<()> {
        let xml = r#"<dataCenterInfo class="com.netflix.appinfo.InstanceInfo$DefaultDataCenterInfo">
        <name>MyOwn</name>
      </dataCenterInfo>"#;
        let data_center_info = DataCenterInfo::from_str(xml)?;
        assert_eq!(data_center_info.name, DcNameType::MyOwn);
        assert_eq!(
            data_center_info.class,
            Some("com.netflix.appinfo.InstanceInfo$DefaultDataCenterInfo".to_string())
        );

        Ok(())
    }

    #[test]
    fn test_xml_lease_info() -> XmlResult<()> {
        let xml = r#"<leaseInfo></leaseInfo>"#;
        let lease_info = LeaseInfo::from_str(xml)?;
        assert_eq!(lease_info.eviction_duration_in_secs, None);
        Ok(())
    }

    #[test]
    fn test_xml_app_meta_data() -> XmlResult<()> {
        let xml = r#"<metadata class="java.util.Collections$EmptyMap"><a>hello</a></metadata>"#;
        let metadata = AppMetaDataType::from_str(xml)?;
        assert_eq!(
            metadata.class,
            Some("java.util.Collections$EmptyMap".to_string())
        );
        assert_eq!(metadata.map.get("a").unwrap(), "hello");

        let s = metadata.to_string()?;
        assert_eq!(s, xml);

        Ok(())
    }

    #[test]
    fn test_xml_full() -> XmlResult<()> {
        let xml = r#"<applications>
  <versions__delta>1</versions__delta>
  <apps__hashcode>UP_2_</apps__hashcode>
  <application>
    <name>BENCH</name>
    <instance>
      <hostName>localhost</hostName>
      <app>BENCH</app>
      <ipAddr>127.0.0.1</ipAddr>
      <status>UP</status>
      <overriddenstatus>UP</overriddenstatus>
      <port enabled="true">8080</port>
      <securePort enabled="false">443</securePort>
      <countryId>1</countryId>
      <dataCenterInfo class="com.netflix.appinfo.InstanceInfo$DefaultDataCenterInfo">
        <name>MyOwn</name>
      </dataCenterInfo>
      <leaseInfo>
        <renewalIntervalInSecs>30</renewalIntervalInSecs>
        <durationInSecs>90</durationInSecs>
        <registrationTimestamp>1616761261538</registrationTimestamp>
        <lastRenewalTimestamp>1616761921820</lastRenewalTimestamp>
        <evictionTimestamp>0</evictionTimestamp>
        <serviceUpTimestamp>1616761261439</serviceUpTimestamp>
      </leaseInfo>
      <metadata class="java.util.Collections$EmptyMap"/>
      <homePageUrl>/eureka</homePageUrl>
      <statusPageUrl></statusPageUrl>
      <healthCheckUrl></healthCheckUrl>
      <vipAddress>bench</vipAddress>
      <secureVipAddress>bench</secureVipAddress>
      <isCoordinatingDiscoveryServer>false</isCoordinatingDiscoveryServer>
      <lastUpdatedTimestamp>1616761261538</lastUpdatedTimestamp>
      <lastDirtyTimestamp>1616761261439</lastDirtyTimestamp>
      <actionType>ADDED</actionType>
    </instance>
    <instance>
      <hostName>localhost2</hostName>
      <app>BENCH</app>
      <ipAddr>127.0.0.1</ipAddr>
      <status>UP</status>
      <overriddenstatus>UP</overriddenstatus>
      <port enabled="true">8081</port>
      <securePort enabled="false">443</securePort>
      <countryId>1</countryId>
      <dataCenterInfo class="com.netflix.appinfo.InstanceInfo$DefaultDataCenterInfo">
        <name>MyOwn</name>
      </dataCenterInfo>
      <leaseInfo>
        <renewalIntervalInSecs>30</renewalIntervalInSecs>
        <durationInSecs>90</durationInSecs>
        <registrationTimestamp>1616761233029</registrationTimestamp>
        <lastRenewalTimestamp>1616761900818</lastRenewalTimestamp>
        <evictionTimestamp>0</evictionTimestamp>
        <serviceUpTimestamp>1616761232774</serviceUpTimestamp>
      </leaseInfo>
      <metadata class="java.util.Collections$EmptyMap"/>
      <homePageUrl>/eureka</homePageUrl>
      <statusPageUrl></statusPageUrl>
      <healthCheckUrl></healthCheckUrl>
      <vipAddress>bench</vipAddress>
      <secureVipAddress>bench</secureVipAddress>
      <isCoordinatingDiscoveryServer>false</isCoordinatingDiscoveryServer>
      <lastUpdatedTimestamp>1616761233030</lastUpdatedTimestamp>
      <lastDirtyTimestamp>1616761232774</lastDirtyTimestamp>
      <actionType>ADDED</actionType>
    </instance>
  </application>
</applications>"#;

        let application = Applications::from_str(xml)?;
        Ok(())
    }
}
