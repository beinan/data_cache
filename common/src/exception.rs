use thiserror::Error;

#[derive(Error, Debug)]
pub enum AlluxioException {
    #[error("{message}")]
    AccessControlException { message: String },
    #[error("{message}")]
    AggregateException { message: String },
    #[error("{message}")]
    BackupAbortedException { message: String },
    #[error("{message}")]
    BackupDelegationException { message: String },
    #[error("{message}")]
    BackupException { message: String },
    #[error("{message}")]
    BlockAlreadyExistsException { message: String },
    #[error("{message}")]
    BlockDoesNotExistException { message: String },
    #[error("{message}")]
    BlockInfoException { message: String },
    #[error("{message}")]
    ConnectionFailedException { message: String },
    #[error("{message}")]
    DependencyDoesNotExistException { message: String },
    #[error("{message}")]
    DirectoryNotEmptyException { message: String },
    #[error("{message}")]
    FailedToAcquireRegisterLeaseException { message: String },
    #[error("{message}")]
    FailedToCheckpointException { message: String },
    #[error("{message}")]
    FileAlreadyCompletedException { message: String },
    #[error("{message}")]
    FileAlreadyExistsException { message: String },
    #[error("{message}")]
    FileDoesNotExistException { message: String },
    #[error("{message}")]
    FileIncompleteException { message: String },
    #[error("{message}")]
    InvalidFileSizeException { message: String },
    #[error("{message}")]
    InvalidJournalEntryException { message: String },
    #[error("{message}")]
    InvalidPathException { message: String },
    #[error("{message}")]
    InvalidWorkerStateException { message: String },
    #[error("{message}")]
    JobDoesNotExistException { message: String },
    #[error("{message}")]
    JournalClosedException { message: String },
    #[error("{message}")]
    OpenDirectoryException { message: String },
    #[error("{message}")]
    PageNotFoundException { message: String },
    #[error("{message}")]
    RegisterLeaseNotFoundException { message: String },
    #[error("{message}")]
    ServiceNotFoundException { message: String },
    #[error("{message}")]
    UfsBlockAccessTokenUnavailableException { message: String },
    #[error("{message}")]
    UnexpectedAlluxioException { message: String },
    #[error("{message}")]
    WorkerOutOfSpaceException { message: String },
}

pub enum ExceptionMessage {
    // general
    INVALID_PREFIX,
    NOT_SUPPORTED,
    PATH_DOES_NOT_EXIST,
    PATH_MUST_BE_FILE,
    PATH_MUST_BE_MOUNT_POINT,
    PATH_INVALID,
    STATE_LOCK_TIMED_OUT,

    // general block
    BLOCK_UNAVAILABLE,
    BLOCK_SIZE_INVALID,
    CANNOT_REQUEST_SPACE,
    NO_SPACE_FOR_BLOCK_ON_WORKER,
    NO_WORKER_AVAILABLE,
}
impl ExceptionMessage {
    fn value(&self) -> &str {
        match *self {
            // general
            ExceptionMessage::INVALID_PREFIX => "Parent path \"{0}\" is not a prefix of child {1}.",
            ExceptionMessage::NOT_SUPPORTED => "This method is not supported.",
            ExceptionMessage::PATH_DOES_NOT_EXIST => "Path \"{0}\" does not exist.",
            ExceptionMessage::PATH_MUST_BE_FILE => "Path \"{0}\" must be a file.",
            ExceptionMessage::PATH_MUST_BE_MOUNT_POINT => "Path \"{0}\" must be a mount point.",
            ExceptionMessage::PATH_INVALID => "Path \"{0}\" is invalid.",
            ExceptionMessage::STATE_LOCK_TIMED_OUT => "Failed to acquire the lock after {0}ms",

            // general block
            ExceptionMessage::BLOCK_UNAVAILABLE => {
                "Block {0} is unavailable in both Alluxio and UFS."
            }
            ExceptionMessage::BLOCK_SIZE_INVALID => {
                "Block size of {0} is invalid. Block size must be > 0 bytes."
            }
            ExceptionMessage::CANNOT_REQUEST_SPACE => {
                "Not enough space left on worker {0} to store blockId {1,number,#}."
            }
            ExceptionMessage::NO_SPACE_FOR_BLOCK_ON_WORKER => {
                "There is no worker with enough space for a new block of size {0}"
            }
            ExceptionMessage::NO_WORKER_AVAILABLE => "No available Alluxio worker found",
        }
    }
}

pub enum PreconditionMessage {}
