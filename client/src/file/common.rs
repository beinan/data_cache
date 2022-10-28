use std::collections::HashMap;
use std::io::Cursor;

use alluxio_common::exception::AlluxioException;
use alluxio_common::settings::Settings;
use alluxio_grpc::alluxio::grpc::block::block_worker_client::BlockWorkerClient;
use alluxio_grpc::alluxio::grpc::block::ReadRequest;
use alluxio_grpc::alluxio::grpc::file::{FileBlockInfo, FileInfo, OpenFilePOptions};
use alluxio_grpc::grpc_client::Client;

use async_trait::async_trait;
use futures::stream;
use tonic::Request;

// WIP
#[async_trait]
trait FileInStream: InputStream + BoundedStream + PositionedReadable + Seekable {}

// Not Ready
#[async_trait]
pub trait InputStream {
    async fn read(
        &mut self,
        b: &mut Cursor<Vec<u8>>,
        off: i64,
        len: i64,
    ) -> Result<i64, AlluxioException>;
}

// Not Ready
trait BoundedStream {}

// WIP
pub trait DataBuffer {
    fn get_length(&self) -> usize;
    fn get_buffer(&mut self) -> &mut Cursor<Vec<u8>>;
    fn read_bytes(&mut self, output_buf: &mut Vec<u8>) -> Result<(), AlluxioException>;
}

// WIP
pub struct DefaultDataBuffer {
    buffer: Cursor<Vec<u8>>,
}
impl DataBuffer for DefaultDataBuffer {
    fn get_length(&self) -> usize {
        self.buffer.get_ref().len()
    }

    fn get_buffer(&mut self) -> &mut Cursor<Vec<u8>> {
        &mut self.buffer
    }

    fn read_bytes(&mut self, output_buf: &mut Vec<u8>) -> Result<(), AlluxioException> {
        output_buf.extend_from_slice(&self.buffer.get_ref());
        Ok(())
    }
}

// WIP
#[async_trait]
pub trait DataReader: Send + Sync {
    fn create(read_request: ReadRequest, address: WorkerNetAddress) -> Self;
    async fn read_chunk(&self) -> Result<DefaultDataBuffer, AlluxioException>;
}

// WIP
pub struct GrpcDataReader {
    pos_to_read: i64,
    read_request: ReadRequest,
    address: WorkerNetAddress,
}

// WIP
#[async_trait]
impl DataReader for GrpcDataReader {
    fn create(read_request: ReadRequest, address: WorkerNetAddress) -> Self {
        GrpcDataReader {
            pos_to_read: read_request.offset(),
            read_request,
            address,
        }
    }

    async fn read_chunk(&self) -> Result<DefaultDataBuffer, AlluxioException> {
        match Settings::new() {
            Ok(settings) => {
                let read_block_fun =
                    async move |client: Client| -> Result<DefaultDataBuffer, AlluxioException> {
                        let mut block_worker_client = BlockWorkerClient::with_interceptor(
                            client.channel,
                            move |mut req: Request<()>| {
                                req.metadata_mut()
                                    .insert("channel-id", client.interceptor.token.clone());
                                Ok(req)
                            },
                        );
                        // read block
                        let mut blocks = vec![];
                        blocks.push(self.read_request.clone());
                        let request = Request::new(stream::iter(blocks));
                        match block_worker_client.read_block(request).await {
                            Ok(response) => {
                                let mut stream = response.into_inner();
                                let mut buffer = Cursor::new(vec![]);
                                while let Ok(Some(r)) = stream.message().await {
                                    if let Some(chunk) = r.chunk {
                                        if let Some(mut data) = chunk.data {
                                            buffer.get_mut().append(&mut data);
                                        }
                                    }
                                }
                                return Ok(DefaultDataBuffer { buffer });
                            }
                            Err(err) => {
                                println!("{:?}", err);
                                return Err(AlluxioException::UnexpectedAlluxioException {
                                    message: err.to_string(),
                                });
                            }
                        };
                    };
                match Client::connect_with_simple_auth(settings.master, 29999, read_block_fun).await
                {
                    Ok(result) => result,
                    Err(err) => {
                        Err(AlluxioException::UnexpectedAlluxioException { message: (err) })
                    }
                }
            }
            Err(config_error) => Err(AlluxioException::UnexpectedAlluxioException {
                message: config_error.to_string(),
            }),
        }
    }
}

// Not Ready
trait PositionedReadable {}

// Not Ready
trait Seekable {}

// struct LocalCacheFileInStream {}
// impl FileInStream for LocalCacheFileInStream {}

// Not Ready
pub struct AlluxioFileInStream<'a, B>
where
    B: BlockLocationPolicy,
{
    status: &'a URIStatus,
    file_info_map: HashMap<i64, &'a FileBlockInfo>,
    options: &'a InStreamOptions<B>,
    block_store: BlockStoreClient,
    context: &'a FileSystemContext,
    passive_caching_enabled: bool,
    // Convenience values derived from mStatus, use these instead of querying mStatus.
    // Length of the file in bytes.
    length: i64,
    block_size: i64,
    // Underlying stream and associated bookkeeping.
    // Current offset in the file.
    position: i64,
    // The last block id for which async cache was triggered.
    failed_workers: HashMap<WorkerNetAddress, i64>,
}
impl<'a, B> AlluxioFileInStream<'a, B>
where
    B: BlockLocationPolicy,
{
    pub fn new(
        options: &'a InStreamOptions<B>,
        context: &'a FileSystemContext,
    ) -> Result<AlluxioFileInStream<'a, B>, AlluxioException> {
        let failed_workers: HashMap<WorkerNetAddress, i64> = HashMap::new();
        let mut file_info_map: HashMap<i64, &'a FileBlockInfo> = HashMap::new();
        let block_infos = &options.status.file_info.file_block_infos;
        for file_block_info in block_infos {
            match &file_block_info.block_info {
                Some(block_info) => file_info_map.insert(block_info.block_id(), &file_block_info),
                None => {
                    return Err(AlluxioException::AccessControlException {
                        message: "file block info get faild".to_string(),
                    })
                }
            };
        }
        let instream = AlluxioFileInStream {
            status: &options.status,
            file_info_map,
            options,
            block_store: BlockStoreClient {}, // tmp
            context,
            passive_caching_enabled: false, //tmp
            length: options.status.file_info.length(),
            block_size: options.status.file_info.block_size_bytes(),
            position: 0,
            failed_workers,
        };
        Ok(instream)
    }
    fn create_block_instream(
        &self,
        block_info: &alluxio_grpc::alluxio::grpc::BlockInfo,
    ) -> BlockInStream {
        let data_source = WorkerNetAddress {
            host: "localhost".to_string(),
        };
        let data_source_type = BlockInStreamSource::NODE_LOCAL;

        BlockInStream::create(
            &self.context,
            block_info,
            data_source,
            data_source_type,
            &self.options,
        )
    }
    fn closeBlockInStream(&self) {}
}

// Not Ready
impl<'a, B> FileInStream for AlluxioFileInStream<'a, B> where B: BlockLocationPolicy {}

// Not Ready
#[async_trait]
impl<'a, B> InputStream for AlluxioFileInStream<'a, B>
where
    B: BlockLocationPolicy,
{
    /**
     * Reads up to len bytes of data from the input stream into the cursor.
     *
     * @param b the buffer into which the data is read
     * @param off the start offset in the buffer at which the data is written
     * @param len the maximum number of bytes to read
     * @return the total number of bytes read into the buffer, or -1 if there is no more
     *         data because the end of the stream has been reached
     */
    async fn read(
        &mut self,
        b: &mut Cursor<Vec<u8>>,
        off: i64,
        len: i64,
    ) -> Result<i64, AlluxioException> {
        if len == 0 {
            return Ok(0);
        }
        // at end of file
        if self.position == self.length {
            return Ok(-1);
        }
        let mut bytes_left = len;
        let mut current_offset = off;
        while bytes_left > 0 && self.position != self.length {
            let index: usize = (self.position / self.block_size) as usize;
            let block_id = self.status.file_info.block_ids.get(index).unwrap();
            let file_block_info = self.file_info_map.get(block_id).unwrap();
            let block_info = file_block_info.block_info.as_ref().unwrap();
            let mut block_in_stream = self.create_block_instream(block_info);
            let block_size = self.status.file_info.block_size_bytes();
            let len_to_read = if bytes_left > block_size {
                block_size
            } else {
                bytes_left
            };
            let bytes_read = block_in_stream.read(b, current_offset, len_to_read).await?;

            if bytes_read > 0 {
                bytes_left -= bytes_read;
                current_offset += bytes_read;
                self.position += bytes_read;
            }
        }
        return Ok(len - bytes_left);
    }
}

// Not Ready
impl<'a, B> BoundedStream for AlluxioFileInStream<'a, B> where B: BlockLocationPolicy {}

// Not Ready
impl<'a, B> PositionedReadable for AlluxioFileInStream<'a, B> where B: BlockLocationPolicy {}

// Not Ready
impl<'a, B> Seekable for AlluxioFileInStream<'a, B> where B: BlockLocationPolicy {}

struct BlockStoreClient {}

// Not Ready
struct BlockInStream {
    block_id: i64,
    current_chunk: DefaultDataBuffer,
}
impl BlockInStream {
    pub fn create<B>(
        context: &FileSystemContext,
        block_info: &alluxio_grpc::alluxio::grpc::BlockInfo,
        data_source: WorkerNetAddress,
        data_source_type: BlockInStreamSource,
        options: &InStreamOptions<B>,
    ) -> BlockInStream
    where
        B: BlockLocationPolicy,
    {
        let block_id = block_info.block_id.unwrap();
        let block_size = block_info.length.unwrap();
        let block_in_stream = Self::create_grpc_block_in_stream(
            context,
            data_source,
            data_source_type,
            block_size,
            block_id,
            options,
        );
        return block_in_stream;
    }

    /**
     * Creates a {@link BlockInStream} to read from a gRPC data server.
     *
     * @param context the file system context
     * @param address the address of the gRPC data server
     * @param blockSource the source location of the block
     * @param blockSize the block size
     * @param blockId the block id
     * @return the {@link BlockInStream} created
     */
    fn create_grpc_block_in_stream<B>(
        context: &FileSystemContext,
        address: WorkerNetAddress,
        block_source: BlockInStreamSource,
        block_size: i64,
        block_id: i64,
        options: &InStreamOptions<B>,
    ) -> BlockInStream
    where
        B: BlockLocationPolicy,
    {
        let buffer = Cursor::new(vec![1, 2, 3]);
        let current_chunk = DefaultDataBuffer { buffer };
        BlockInStream {
            block_id,
            current_chunk,
        }
    }

    // 此offset非文件的offset，应该是block的offset
    async fn read_chunck(&mut self, off: i64, len: i64) -> Result<(), AlluxioException> {
        // TODO: add more DataReader
        let data_reader: GrpcDataReader = self.get_data_reader(off, len);
        let data = data_reader.read_chunk().await?;
        self.current_chunk = data;
        Ok(())
    }

    pub fn remaining(&self) -> usize {
        0
    }

    fn get_data_reader<T>(&self, offset: i64, length: i64) -> T
    where
        T: DataReader,
    {
        let read_request = ReadRequest {
            block_id: Some(self.block_id),
            chunk_size: Some(1024 * 1024),
            length: Some(length),
            offset: Some(offset),
            offset_received: None,
            open_ufs_block_options: None,
            position_short: None,
            promote: None,
        };
        let address = WorkerNetAddress {
            host: "localhost".to_string(),
        };
        T::create(read_request, address)
    }
}

// Not Ready
#[async_trait]
impl InputStream for BlockInStream {
    ///    Reads up to len bytes of data from the input stream into the byte buffer.
    ///
    ///    @param b the buffer into which the data is read
    ///    @param off the start offset in the buffer at which the data is written
    ///    @param len the maximum number of bytes to read
    ///    @return the total number of bytes read into the buffer, or -1 if there is no more data because
    ///            the end of the stream has been reached
    async fn read(
        &mut self,
        b: &mut Cursor<Vec<u8>>,
        off: i64,
        len: i64,
    ) -> Result<i64, AlluxioException> {
        if len == 0 {
            return Ok(0);
        }
        let length_before = b.get_ref().len();
        let mut remaining = len;
        let mut pos = 0;
        let mut data;
        while remaining > 0 {
            self.read_chunck(pos, len).await?;
            data = self.current_chunk.get_buffer().get_ref().as_slice();
            b.get_mut().extend_from_slice(data);
            pos += data.len() as i64;
            remaining -= pos;
        }
        let length_after = b.get_ref().len();
        let increase = length_after - length_before;
        Ok(increase as i64)
    }
}

enum BlockInStreamSource {
    // The block is from a worker in the same process
    PROCESS_LOCAL,
    // The block is from a separate worker process on the same node
    NODE_LOCAL,
    // The block is from a remote worker
    REMOTE,
    // The block is in UFS
    UFS,
}

struct BlockInfo {
    block_id: i64,
    length: i64,
    locations: Vec<BlockLocation>,
}

struct BlockLocation {}

pub trait BlockLocationPolicy: Send + Sync {
    fn getWorker() -> WorkerNetAddress;
}

pub struct LocalFirstPolicy {}

impl BlockLocationPolicy for LocalFirstPolicy {
    fn getWorker() -> WorkerNetAddress {
        WorkerNetAddress {
            host: "localhost".to_string(),
        }
    }
}

// TODO
pub struct FileSystemContext {}

pub struct InStreamOptions<T: BlockLocationPolicy> {
    pub status: URIStatus,
    pub options: OpenFilePOptions,
    pub ufs_read_location_policy: T,
    pub position_short: bool,
}

// TODO: add CacheContext
pub struct URIStatus {
    pub file_info: FileInfo,
}

pub struct WorkerNetAddress {
    pub host: String,
}

#[cfg(test)]
mod tests {

    // GrpcDataReader
    #[test]
    fn read_chuck_works() {
        assert_eq!(2 + 2, 4);
    }
}
