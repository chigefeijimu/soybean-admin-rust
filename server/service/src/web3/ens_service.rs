//! ENS Service
//! Ethereum Name Service resolution and registration

use serde::{Deserialize, Serialize};

/// ENS record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnsRecord {
    pub name: String,
    pub address: Option<String>,
    pub avatar: Option<String>,
    pub email: Option<String>,
    pub url: Option<String>,
    pub bio: Option<String>,
    pub twitter: Option<String>,
    pub github: Option<String>,
    pub resolver: String,
    pub registration_date: Option<String>,
    pub expires_at: Option<String>,
}

/// Reverse resolution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReverseRecord {
    pub address: String,
    pub name: Option<String>,
    pub avatar: Option<String>,
}

/// Domain availability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainAvailability {
    pub name: String,
    pub available: bool,
    pub price: Option<String>,
    pub expires_in_days: Option<u32>,
}

/// ENS Service
pub struct EnsService;

impl EnsService {
    /// Resolve ENS name to address
    pub fn resolve(&self, name: &str) -> Option<EnsRecord> {
        // Mock ENS resolution
        let mock_records = vec![
            ("vitalik.eth", "0xd8dA6BF26964aF9D7eEd09eE47c44cBda2CCECEC"),
            ("uniswap.eth", "0x1a7b0a5c2b0e5d7d8c9b0a5c2b0e5d7d8c9b0a5c"),
            ("aave.eth", "0x7Fc66500c84A76Ad7e9c93437bFc5Ac33E2DDaE9"),
        ];
        
        for (ens_name, address) in mock_records {
            if name.to_lowercase() == ens_name {
                return Some(EnsRecord {
                    name: name.to_string(),
                    address: Some(address.to_string()),
                    avatar: Some(format!("https://metadata.ens.domains/mainnet/avatar/{}", name)),
                    email: None,
                    url: Some(format!("https://{}.eth.link", name)),
                    bio: None,
                    twitter: None,
                    github: None,
                    resolver: "0x4976fb03C32e5B8c51c9a5C5F3d3a2D7E5C6D7E5".to_string(),
                    registration_date: Some("2021-01-15".to_string()),
                    expires_at: Some("2031-01-15".to_string()),
                });
            }
        }
        
        None
    }

    /// Reverse resolve address to ENS name
    pub fn reverse_resolve(&self, address: &str) -> Option<ReverseRecord> {
        let mock_reverse = vec![
            ("0xd8dA6BF26964aF9D7eEd09eE47c44cBda2CCECEC", "vitalik.eth"),
            ("0x7Fc66500c84A76Ad7e9c93437bFc5Ac33E2DDaE9", "aave.eth"),
        ];
        
        for (addr, name) in mock_reverse {
            if address.to_lowercase() == addr.to_lowercase() {
                return Some(ReverseRecord {
                    address: address.to_string(),
                    name: Some(name.to_string()),
                    avatar: Some(format!("https://metadata.ens.domains/mainnet/avatar/{}", name)),
                });
            }
        }
        
        None
    }

    /// Check domain availability
    pub fn check_availability(&self, name: &str) -> DomainAvailability {
        // Mock availability check
        let unavailable = vec![
            "vitalik", "uniswap", "aave", "ethereum", "maker", "compound", "curve"
        ];
        
        let name_lower = name.to_lowercase().trim_end_matches(".eth").to_string();
        let available = !unavailable.contains(&name_lower.as_str());
        
        DomainAvailability {
            name: name.to_string(),
            available,
            price: if available {
                Some("$5.00/year".to_string())
            } else {
                None
            },
            expires_in_days: None,
        }
    }

    /// Get all records for a name
    pub fn get_all_records(&self, name: &str) -> Option<EnsRecord> {
        self.resolve(name)
    }

    /// Search ENS names
    pub fn search(&self, query: &str) -> Vec<EnsRecord> {
        let mock_names = vec![
            "vitalik.eth",
            "uniswap.eth", 
            "aave.eth",
            "compound.eth",
            "curve.eth",
            "maker.eth",
            "chainlink.eth",
        ];
        
        let query_lower = query.to_lowercase();
        mock_names
            .into_iter()
            .filter(|name| name.contains(&query_lower))
            .filter_map(|name| self.resolve(name))
            .collect()
    }

    /// Get popular ENS domains
    pub fn get_popular(&self) -> Vec<EnsRecord> {
        vec![
            self.resolve("uniswap.eth").unwrap(),
            self.resolve("aave.eth").unwrap(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve() {
        let service = EnsService;
        let record = service.resolve("vitalik.eth");
        assert!(record.is_some());
        assert_eq!(record.unwrap().name, "vitalik.eth");
    }

    #[test]
    fn test_reverse() {
        let service = EnsService;
        let record = service.reverse_resolve("0xd8dA6BF26964aF9D7eEd09eE47c44cBda2CCECEC");
        assert!(record.is_some());
    }

    #[test]
    fn test_availability() {
        let service = EnsService;
        let avail = service.check_availability("newdomain.eth");
        assert!(avail.available);
    }
}
