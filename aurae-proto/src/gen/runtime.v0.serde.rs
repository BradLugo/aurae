// @generated
impl serde::Serialize for Cell {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.cpu_cpus.is_empty() {
            len += 1;
        }
        if self.cpu_shares != 0 {
            len += 1;
        }
        if !self.cpu_mems.is_empty() {
            len += 1;
        }
        if self.cpu_quota != 0 {
            len += 1;
        }
        if self.ns_share_mount {
            len += 1;
        }
        if self.ns_share_uts {
            len += 1;
        }
        if self.ns_share_ipc {
            len += 1;
        }
        if self.ns_share_pid {
            len += 1;
        }
        if self.ns_share_net {
            len += 1;
        }
        if self.ns_share_cgroup {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("runtime.v0.Cell", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.cpu_cpus.is_empty() {
            struct_ser.serialize_field("cpuCpus", &self.cpu_cpus)?;
        }
        if self.cpu_shares != 0 {
            struct_ser.serialize_field("cpuShares", ToString::to_string(&self.cpu_shares).as_str())?;
        }
        if !self.cpu_mems.is_empty() {
            struct_ser.serialize_field("cpuMems", &self.cpu_mems)?;
        }
        if self.cpu_quota != 0 {
            struct_ser.serialize_field("cpuQuota", ToString::to_string(&self.cpu_quota).as_str())?;
        }
        if self.ns_share_mount {
            struct_ser.serialize_field("nsShareMount", &self.ns_share_mount)?;
        }
        if self.ns_share_uts {
            struct_ser.serialize_field("nsShareUts", &self.ns_share_uts)?;
        }
        if self.ns_share_ipc {
            struct_ser.serialize_field("nsShareIpc", &self.ns_share_ipc)?;
        }
        if self.ns_share_pid {
            struct_ser.serialize_field("nsSharePid", &self.ns_share_pid)?;
        }
        if self.ns_share_net {
            struct_ser.serialize_field("nsShareNet", &self.ns_share_net)?;
        }
        if self.ns_share_cgroup {
            struct_ser.serialize_field("nsShareCgroup", &self.ns_share_cgroup)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Cell {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "cpu_cpus",
            "cpuCpus",
            "cpu_shares",
            "cpuShares",
            "cpu_mems",
            "cpuMems",
            "cpu_quota",
            "cpuQuota",
            "ns_share_mount",
            "nsShareMount",
            "ns_share_uts",
            "nsShareUts",
            "ns_share_ipc",
            "nsShareIpc",
            "ns_share_pid",
            "nsSharePid",
            "ns_share_net",
            "nsShareNet",
            "ns_share_cgroup",
            "nsShareCgroup",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            CpuCpus,
            CpuShares,
            CpuMems,
            CpuQuota,
            NsShareMount,
            NsShareUts,
            NsShareIpc,
            NsSharePid,
            NsShareNet,
            NsShareCgroup,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "cpuCpus" | "cpu_cpus" => Ok(GeneratedField::CpuCpus),
                            "cpuShares" | "cpu_shares" => Ok(GeneratedField::CpuShares),
                            "cpuMems" | "cpu_mems" => Ok(GeneratedField::CpuMems),
                            "cpuQuota" | "cpu_quota" => Ok(GeneratedField::CpuQuota),
                            "nsShareMount" | "ns_share_mount" => Ok(GeneratedField::NsShareMount),
                            "nsShareUts" | "ns_share_uts" => Ok(GeneratedField::NsShareUts),
                            "nsShareIpc" | "ns_share_ipc" => Ok(GeneratedField::NsShareIpc),
                            "nsSharePid" | "ns_share_pid" => Ok(GeneratedField::NsSharePid),
                            "nsShareNet" | "ns_share_net" => Ok(GeneratedField::NsShareNet),
                            "nsShareCgroup" | "ns_share_cgroup" => Ok(GeneratedField::NsShareCgroup),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Cell;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct runtime.v0.Cell")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Cell, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut cpu_cpus__ = None;
                let mut cpu_shares__ = None;
                let mut cpu_mems__ = None;
                let mut cpu_quota__ = None;
                let mut ns_share_mount__ = None;
                let mut ns_share_uts__ = None;
                let mut ns_share_ipc__ = None;
                let mut ns_share_pid__ = None;
                let mut ns_share_net__ = None;
                let mut ns_share_cgroup__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::CpuCpus => {
                            if cpu_cpus__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cpuCpus"));
                            }
                            cpu_cpus__ = Some(map.next_value()?);
                        }
                        GeneratedField::CpuShares => {
                            if cpu_shares__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cpuShares"));
                            }
                            cpu_shares__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::CpuMems => {
                            if cpu_mems__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cpuMems"));
                            }
                            cpu_mems__ = Some(map.next_value()?);
                        }
                        GeneratedField::CpuQuota => {
                            if cpu_quota__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cpuQuota"));
                            }
                            cpu_quota__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NsShareMount => {
                            if ns_share_mount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nsShareMount"));
                            }
                            ns_share_mount__ = Some(map.next_value()?);
                        }
                        GeneratedField::NsShareUts => {
                            if ns_share_uts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nsShareUts"));
                            }
                            ns_share_uts__ = Some(map.next_value()?);
                        }
                        GeneratedField::NsShareIpc => {
                            if ns_share_ipc__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nsShareIpc"));
                            }
                            ns_share_ipc__ = Some(map.next_value()?);
                        }
                        GeneratedField::NsSharePid => {
                            if ns_share_pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nsSharePid"));
                            }
                            ns_share_pid__ = Some(map.next_value()?);
                        }
                        GeneratedField::NsShareNet => {
                            if ns_share_net__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nsShareNet"));
                            }
                            ns_share_net__ = Some(map.next_value()?);
                        }
                        GeneratedField::NsShareCgroup => {
                            if ns_share_cgroup__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nsShareCgroup"));
                            }
                            ns_share_cgroup__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Cell {
                    name: name__.unwrap_or_default(),
                    cpu_cpus: cpu_cpus__.unwrap_or_default(),
                    cpu_shares: cpu_shares__.unwrap_or_default(),
                    cpu_mems: cpu_mems__.unwrap_or_default(),
                    cpu_quota: cpu_quota__.unwrap_or_default(),
                    ns_share_mount: ns_share_mount__.unwrap_or_default(),
                    ns_share_uts: ns_share_uts__.unwrap_or_default(),
                    ns_share_ipc: ns_share_ipc__.unwrap_or_default(),
                    ns_share_pid: ns_share_pid__.unwrap_or_default(),
                    ns_share_net: ns_share_net__.unwrap_or_default(),
                    ns_share_cgroup: ns_share_cgroup__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("runtime.v0.Cell", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CellServiceAllocateRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.cell.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("runtime.v0.CellServiceAllocateRequest", len)?;
        if let Some(v) = self.cell.as_ref() {
            struct_ser.serialize_field("cell", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CellServiceAllocateRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cell",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Cell,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "cell" => Ok(GeneratedField::Cell),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CellServiceAllocateRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct runtime.v0.CellServiceAllocateRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CellServiceAllocateRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cell__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Cell => {
                            if cell__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cell"));
                            }
                            cell__ = map.next_value()?;
                        }
                    }
                }
                Ok(CellServiceAllocateRequest {
                    cell: cell__,
                })
            }
        }
        deserializer.deserialize_struct("runtime.v0.CellServiceAllocateRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CellServiceAllocateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cell_name.is_empty() {
            len += 1;
        }
        if self.cgroup_v2 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("runtime.v0.CellServiceAllocateResponse", len)?;
        if !self.cell_name.is_empty() {
            struct_ser.serialize_field("cellName", &self.cell_name)?;
        }
        if self.cgroup_v2 {
            struct_ser.serialize_field("cgroupV2", &self.cgroup_v2)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CellServiceAllocateResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cell_name",
            "cellName",
            "cgroup_v2",
            "cgroupV2",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CellName,
            CgroupV2,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "cellName" | "cell_name" => Ok(GeneratedField::CellName),
                            "cgroupV2" | "cgroup_v2" => Ok(GeneratedField::CgroupV2),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CellServiceAllocateResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct runtime.v0.CellServiceAllocateResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CellServiceAllocateResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cell_name__ = None;
                let mut cgroup_v2__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CellName => {
                            if cell_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cellName"));
                            }
                            cell_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::CgroupV2 => {
                            if cgroup_v2__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cgroupV2"));
                            }
                            cgroup_v2__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CellServiceAllocateResponse {
                    cell_name: cell_name__.unwrap_or_default(),
                    cgroup_v2: cgroup_v2__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("runtime.v0.CellServiceAllocateResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CellServiceFreeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cell_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("runtime.v0.CellServiceFreeRequest", len)?;
        if !self.cell_name.is_empty() {
            struct_ser.serialize_field("cellName", &self.cell_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CellServiceFreeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cell_name",
            "cellName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CellName,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "cellName" | "cell_name" => Ok(GeneratedField::CellName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CellServiceFreeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct runtime.v0.CellServiceFreeRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CellServiceFreeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cell_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CellName => {
                            if cell_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cellName"));
                            }
                            cell_name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CellServiceFreeRequest {
                    cell_name: cell_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("runtime.v0.CellServiceFreeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CellServiceFreeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("runtime.v0.CellServiceFreeResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CellServiceFreeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CellServiceFreeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct runtime.v0.CellServiceFreeResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CellServiceFreeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(CellServiceFreeResponse {
                })
            }
        }
        deserializer.deserialize_struct("runtime.v0.CellServiceFreeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CellServiceStartRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cell_name.is_empty() {
            len += 1;
        }
        if self.executable.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("runtime.v0.CellServiceStartRequest", len)?;
        if !self.cell_name.is_empty() {
            struct_ser.serialize_field("cellName", &self.cell_name)?;
        }
        if let Some(v) = self.executable.as_ref() {
            struct_ser.serialize_field("executable", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CellServiceStartRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cell_name",
            "cellName",
            "executable",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CellName,
            Executable,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "cellName" | "cell_name" => Ok(GeneratedField::CellName),
                            "executable" => Ok(GeneratedField::Executable),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CellServiceStartRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct runtime.v0.CellServiceStartRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CellServiceStartRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cell_name__ = None;
                let mut executable__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CellName => {
                            if cell_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cellName"));
                            }
                            cell_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Executable => {
                            if executable__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executable"));
                            }
                            executable__ = map.next_value()?;
                        }
                    }
                }
                Ok(CellServiceStartRequest {
                    cell_name: cell_name__.unwrap_or_default(),
                    executable: executable__,
                })
            }
        }
        deserializer.deserialize_struct("runtime.v0.CellServiceStartRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CellServiceStartResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pid != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("runtime.v0.CellServiceStartResponse", len)?;
        if self.pid != 0 {
            struct_ser.serialize_field("pid", &self.pid)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CellServiceStartResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pid",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pid,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "pid" => Ok(GeneratedField::Pid),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CellServiceStartResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct runtime.v0.CellServiceStartResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CellServiceStartResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut pid__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Pid => {
                            if pid__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pid"));
                            }
                            pid__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CellServiceStartResponse {
                    pid: pid__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("runtime.v0.CellServiceStartResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CellServiceStopRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.cell_name.is_empty() {
            len += 1;
        }
        if !self.executable_name.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("runtime.v0.CellServiceStopRequest", len)?;
        if !self.cell_name.is_empty() {
            struct_ser.serialize_field("cellName", &self.cell_name)?;
        }
        if !self.executable_name.is_empty() {
            struct_ser.serialize_field("executableName", &self.executable_name)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CellServiceStopRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "cell_name",
            "cellName",
            "executable_name",
            "executableName",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CellName,
            ExecutableName,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "cellName" | "cell_name" => Ok(GeneratedField::CellName),
                            "executableName" | "executable_name" => Ok(GeneratedField::ExecutableName),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CellServiceStopRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct runtime.v0.CellServiceStopRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CellServiceStopRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut cell_name__ = None;
                let mut executable_name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CellName => {
                            if cell_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cellName"));
                            }
                            cell_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::ExecutableName => {
                            if executable_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executableName"));
                            }
                            executable_name__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CellServiceStopRequest {
                    cell_name: cell_name__.unwrap_or_default(),
                    executable_name: executable_name__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("runtime.v0.CellServiceStopRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CellServiceStopResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("runtime.v0.CellServiceStopResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CellServiceStopResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CellServiceStopResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct runtime.v0.CellServiceStopResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CellServiceStopResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(CellServiceStopResponse {
                })
            }
        }
        deserializer.deserialize_struct("runtime.v0.CellServiceStopResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Container {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("runtime.v0.Container", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Container {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Container;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct runtime.v0.Container")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Container, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(Container {
                })
            }
        }
        deserializer.deserialize_struct("runtime.v0.Container", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Executable {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.command.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("runtime.v0.Executable", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.command.is_empty() {
            struct_ser.serialize_field("command", &self.command)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Executable {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "command",
            "description",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            Command,
            Description,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "name" => Ok(GeneratedField::Name),
                            "command" => Ok(GeneratedField::Command),
                            "description" => Ok(GeneratedField::Description),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Executable;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct runtime.v0.Executable")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Executable, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut command__ = None;
                let mut description__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Command => {
                            if command__.is_some() {
                                return Err(serde::de::Error::duplicate_field("command"));
                            }
                            command__ = Some(map.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(serde::de::Error::duplicate_field("description"));
                            }
                            description__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Executable {
                    name: name__.unwrap_or_default(),
                    command: command__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("runtime.v0.Executable", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Pod {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("runtime.v0.Pod", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Pod {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Pod;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct runtime.v0.Pod")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Pod, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(Pod {
                })
            }
        }
        deserializer.deserialize_struct("runtime.v0.Pod", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PodServiceAllocateRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("runtime.v0.PodServiceAllocateRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PodServiceAllocateRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PodServiceAllocateRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct runtime.v0.PodServiceAllocateRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PodServiceAllocateRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PodServiceAllocateRequest {
                })
            }
        }
        deserializer.deserialize_struct("runtime.v0.PodServiceAllocateRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PodServiceAllocateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("runtime.v0.PodServiceAllocateResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PodServiceAllocateResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PodServiceAllocateResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct runtime.v0.PodServiceAllocateResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PodServiceAllocateResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PodServiceAllocateResponse {
                })
            }
        }
        deserializer.deserialize_struct("runtime.v0.PodServiceAllocateResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PodServiceFreeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("runtime.v0.PodServiceFreeRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PodServiceFreeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PodServiceFreeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct runtime.v0.PodServiceFreeRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PodServiceFreeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PodServiceFreeRequest {
                })
            }
        }
        deserializer.deserialize_struct("runtime.v0.PodServiceFreeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PodServiceFreeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("runtime.v0.PodServiceFreeResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PodServiceFreeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PodServiceFreeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct runtime.v0.PodServiceFreeResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PodServiceFreeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PodServiceFreeResponse {
                })
            }
        }
        deserializer.deserialize_struct("runtime.v0.PodServiceFreeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PodServiceStartRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("runtime.v0.PodServiceStartRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PodServiceStartRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PodServiceStartRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct runtime.v0.PodServiceStartRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PodServiceStartRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PodServiceStartRequest {
                })
            }
        }
        deserializer.deserialize_struct("runtime.v0.PodServiceStartRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PodServiceStartResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("runtime.v0.PodServiceStartResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PodServiceStartResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PodServiceStartResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct runtime.v0.PodServiceStartResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PodServiceStartResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PodServiceStartResponse {
                })
            }
        }
        deserializer.deserialize_struct("runtime.v0.PodServiceStartResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PodServiceStopRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("runtime.v0.PodServiceStopRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PodServiceStopRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PodServiceStopRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct runtime.v0.PodServiceStopRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PodServiceStopRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PodServiceStopRequest {
                })
            }
        }
        deserializer.deserialize_struct("runtime.v0.PodServiceStopRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PodServiceStopResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("runtime.v0.PodServiceStopResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PodServiceStopResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PodServiceStopResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct runtime.v0.PodServiceStopResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PodServiceStopResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PodServiceStopResponse {
                })
            }
        }
        deserializer.deserialize_struct("runtime.v0.PodServiceStopResponse", FIELDS, GeneratedVisitor)
    }
}