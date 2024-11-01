use crate::domain::flags::entities::Flag;


trait SendingDomain: Send + Sync {
    type SendingError;
    /// Method for get flags for sending and change flag status to waiting
    fn get_flags(&self, limit: i32) -> Result<Vec<Flag>, Self::SendingError>;
}
