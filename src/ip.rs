use obfstr::obfstr;
use serde::Deserialize;
use anyhow::Result;


#[allow(non_snake_case)]
#[derive(Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct IpInfo {
    #[serde(rename = "query")]
    pub Ip: String,
    pub Continent: String,
    pub Country: String,
    pub City: String,
    pub District: String,
    #[serde(rename = "regionName")]
    pub Region: String,
    #[serde(rename = "zip")]
    pub ZipCode: String,
    pub Isp: String,
}

pub async fn get_ip_info() -> Result<IpInfo> {

    let ip_info = reqwest::get(obfstr!("http://ip-api.com/json/?fields=continent,country,regionName,city,district,zip,isp,query"))
        .await?
        .json::<IpInfo>()
        .await?;

    Ok(ip_info)
}