use std::collections::HashMap;

use crate::errs::FlymodelError;

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Perm: u32 {
        const R = 0b00000000;
        const W = 0b00000001;
    }
}

impl TryFrom<&str> for Perm {
    type Error = FlymodelError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "read" => Ok(Self::R),
            "write" => Ok(Self::W),
            _ => Err(FlymodelError::InvalidPermission(value.to_string())),
        }
    }
}

impl ToString for Perm {
    fn to_string(&self) -> String {
        match self {
            &Self::R => "read",
            &Self::W => "write",
            _ => unreachable!(),
        }
        .to_string()
    }
}

/**
    we expect permissions to be handled by e.g. oidc (vendor
    however, we want a common representation across auth styles for
    our own internal resolutions across access layers.
*/
pub enum Permission {
    Global { perm: Perm },
    Namespace { perm: Perm, id: i64 },
    Model { perm: Perm, id: i64 },
}

impl ToString for Permission {
    fn to_string(&self) -> String {
        match self {
            Self::Global { perm } => {
                format!("global:{}", perm.to_string())
            }
            Self::Namespace { perm, id } => {
                format!("namespace:{}:{}", id, perm.to_string())
            }
            Self::Model { perm, id } => {
                format!("model:{}:{}", id.to_string(), perm.to_string())
            }
        }
    }
}

impl TryFrom<&str> for Permission {
    type Error = FlymodelError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let sp = value.split(":");
        if sp.clone().count() == 1 {
            return Err(FlymodelError::InvalidPermission(value.to_string()));
        }
        let sp = sp.collect::<Vec<&str>>();
        Ok(match sp[..] {
            ["global", perm] => Permission::Global {
                perm: perm.try_into()?,
            },
            ["namespace", id, perm] => Permission::Namespace {
                id: id.parse().map_err(FlymodelError::IdParsingError)?,
                perm: perm.try_into()?,
            },
            ["model", id, perm] => Permission::Model {
                id: id.parse().map_err(FlymodelError::IdParsingError)?,
                perm: perm.try_into()?,
            },
            _ => return Err(FlymodelError::InvalidPermission(value.to_string())),
        })
    }
}

pub struct Permissions(Vec<Permission>);

impl Permissions {
    pub fn new(perms: Vec<Permission>) -> Self {
        Self(perms)
    }

    pub fn namespace_permissions(&self, ns: Vec<i64>) -> HashMap<i64, Option<Perm>> {
        let mut perms = HashMap::new();
        ns.iter().for_each(|ns| {
            let mut found = false;
            for perm in self.as_ref() {
                match perm {
                    Permission::Global { perm } => {
                        if let Some(other) = perms.get_mut(ns) {
                            if let Some(other) = other {
                                *other |= *perm;
                            }
                        } else {
                            perms.insert(*ns, Some(*perm));
                        }
                        found = true;
                    }
                    Permission::Namespace { perm, id } => {
                        if ns == id {
                            if let Some(other) = perms.get_mut(ns) {
                                if let Some(other) = other {
                                    *other &= *perm;
                                }
                            } else {
                                perms.insert(*ns, Some(*perm));
                            }
                            found = true;
                        }
                    }
                    Permission::Model { .. } => (),
                }
            }
            if !found {
                perms.insert(*ns, None);
            }
        });
        perms
    }
}

impl AsRef<[Permission]> for Permissions {
    fn as_ref(&self) -> &[Permission] {
        &self.0
    }
}

impl Into<Vec<String>> for Permissions {
    fn into(self) -> Vec<String> {
        self.0.into_iter().map(|p| p.to_string()).collect()
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::{Perm, Permission};

    #[test]
    fn test_perm() {
        let r = Perm::R & Perm::W;
        assert_eq!(r, Perm::R);
        let w = Perm::R | Perm::W;
        assert_eq!(w, Perm::W);
    }

    fn expected_ns_perms(perms: Vec<(i64, Option<Perm>)>) -> HashMap<i64, Option<Perm>> {
        HashMap::from_iter(perms.into_iter())
    }

    #[test]
    fn test_global_read_perms() {
        let perms = super::Permissions::new(vec![Permission::Global { perm: Perm::R }]);
        let found = perms.namespace_permissions(vec![1]);
        let expect = expected_ns_perms(vec![(1, Some(Perm::R))]);
        assert_eq!(expect, found);
    }

    #[test]
    fn test_global_write_perms() {
        let perms = super::Permissions::new(vec![Permission::Global { perm: Perm::W }]);
        let found = perms.namespace_permissions(vec![1]);
        let expect = expected_ns_perms(vec![(1, Some(Perm::W))]);
        assert_eq!(expect, found);
    }

    #[test]
    fn test_namespace_perm_segregation() {
        let mut perms = super::Permissions::new(vec![
            Permission::Namespace {
                perm: Perm::R,
                id: 1,
            },
            Permission::Namespace {
                perm: Perm::W,
                id: 2,
            },
        ]);

        let found = perms.namespace_permissions(vec![1, 2, 3]);
        let expect = expected_ns_perms(vec![(1, Some(Perm::R)), (2, Some(Perm::W)), (3, None)]);

        assert_eq!(expect, found);

        perms.0.push(Permission::Global { perm: Perm::W });

        let found = perms.namespace_permissions(vec![1, 2, 3]);
        let expect = expected_ns_perms(vec![
            (1, Some(Perm::W)),
            (2, Some(Perm::W)),
            (3, Some(Perm::W)),
        ]);

        assert_eq!(expect, found);
    }

    #[test]
    fn test_least_nonglobal_permissions_in_ns() {
        let perms = super::Permissions::new(vec![
            Permission::Namespace {
                perm: Perm::R,
                id: 1,
            },
            Permission::Namespace {
                perm: Perm::W,
                id: 1,
            },
        ]);

        let found = perms.namespace_permissions(vec![1]);
        let expect = expected_ns_perms(vec![(1, Some(Perm::R))]);

        assert_eq!(found, expect);
    }
}
