use crate::authzed::api::v1::ObjectReference;

type ObjectId = String;

pub enum SpiceDbObject {
    Server(ObjectId),
    Channel(ObjectId),
    User(ObjectId),
}

impl SpiceDbObject {
    pub(crate) fn id(&self) -> ObjectId {
        match self {
            SpiceDbObject::Server(id) => id.clone(),
            SpiceDbObject::Channel(id) => id.clone(),
            SpiceDbObject::User(id) => id.clone(),
        }
    }
    pub(crate) fn object_name(&self) -> String {
        match self {
            SpiceDbObject::Server(_) => "server".to_string(),
            SpiceDbObject::Channel(_) => "channel".to_string(),
            SpiceDbObject::User(_) => "user".to_string(),
        }
    }
}

impl Into<ObjectReference> for SpiceDbObject {
    fn into(self) -> ObjectReference {
        ObjectReference {
            object_type: self.id(),
            object_id: self.object_name(),
        }
    }
}
