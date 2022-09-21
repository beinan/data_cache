use std::collections::HashMap;
use std::io::Cursor;

use alluxio_common::exception::AlluxioException;
use alluxio_common::settings::Settings;
use alluxio_grpc::alluxio::grpc::block::block_worker_client::BlockWorkerClient;
use alluxio_grpc::alluxio::grpc::block::ReadRequest;
use alluxio_grpc::alluxio::grpc::file::{FileInfo, OpenFilePOptions};
use alluxio_grpc::grpc_client::Client;

use async_trait::async_trait;
use futures::stream;
use tonic::Request;
use tonic::{metadata::MetadataValue, transport::Channel};

// Not Ready
trait FileInStream: InputStream + BoundedStream + PositionedReadable + Seekable {}

// Not Ready
#[async_trait]
trait InputStream {
    async fn read(&mut self, b: &mut Vec<u8>, off: i64, len: i64) -> Result<i64, AlluxioException>;
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
        // if self.get_length() <= output_buf.len() {
        //     output_buf.clone_from_slice(&self.buffer.get_ref());
        // } else {
        //     output_buf.clone_from_slice(&self.buffer.get_ref()[..output_buf.len() - 1]);
        // }
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
pub struct AlluxioFileInStream<T, B>
where
    T: DataReader,
    B: BlockLocationPolicy,
{
    status: URIStatus,
    options: InStreamOptions<B>,
    block_store: BlockStoreClient,
    context: FileSystemContext,
    passive_caching_enabled: bool,
    // Convenience values derived from mStatus, use these instead of querying mStatus.
    // Length of the file in bytes.
    length: i64,
    block_size: i64,
    // Underlying stream and associated bookkeeping.
    // Current offset in the file.
    position: i64,
    // Underlying block stream, null if a position change has invalidated the previous stream.
    block_in_stream: BlockInStream<T>,
    // Cached block stream for the positioned read API.
    cached_positioned_read_stream: BlockInStream<T>,
    // The last block id for which async cache was triggered.
    last_block_id_cached: i64,
    // A map of worker addresses to the most recent epoch time when client fails to read from it.
    failed_workers: HashMap<WorkerNetAddress, i64>,
}
impl<T, B> AlluxioFileInStream<T, B>
where
    T: DataReader,
    B: BlockLocationPolicy,
{
    // pub fn new(
    //     options: InStreamOptions<B>,
    //     context: FileSystemContext,
    // ) -> AlluxioFileInStream<T, B> {
    //     AlluxioFileInStream {
    //         status: options.status,
    //         options,
    //         block_store: BlockStoreClient {}, // tmp
    //         context,
    //         passive_caching_enabled: false, //tmp
    //         length: options.status.file_info.length(),
    //         block_size: options.status.file_info.block_size_bytes(),
    //         position: 0,
    //         block_in_stream: BlockInStream::create(
    //             context,
    //             block_info,
    //             data_source: WorkerNetAddress {"localhost".to_string()},
    //             data_source_type: BlockInStreamSource::NODE_LOCAL,
    //             options,
    //         ),
    //     }
    // }
}

// Not Ready
impl<T, B> FileInStream for AlluxioFileInStream<T, B>
where
    T: DataReader,
    B: BlockLocationPolicy,
{
}

// Not Ready
#[async_trait]
impl<T, B> InputStream for AlluxioFileInStream<T, B>
where
    T: DataReader,
    B: BlockLocationPolicy,
{
    async fn read(&mut self, b: &mut Vec<u8>, off: i64, len: i64) -> Result<i64, AlluxioException> {
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
            let bytes_read = self
                .block_in_stream
                .read(b, current_offset, bytes_left)
                .await;
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
impl<T, B> BoundedStream for AlluxioFileInStream<T, B>
where
    T: DataReader,
    B: BlockLocationPolicy,
{
}

// Not Ready
impl<T, B> PositionedReadable for AlluxioFileInStream<T, B>
where
    T: DataReader,
    B: BlockLocationPolicy,
{
}

// Not Ready
impl<T, B> Seekable for AlluxioFileInStream<T, B>
where
    T: DataReader,
    B: BlockLocationPolicy,
{
}

struct BlockStoreClient {}

// Not Ready
struct BlockInStream<T>
where
    T: DataReader,
{
    // The size in bytes of the block.
    length: i64,
    // Current position of the stream, relative to the start of the block.
    position: i64,
    EOF: bool,
    data_reader: T,
    current_chunk: DefaultDataBuffer,
}
impl<T> BlockInStream<T>
where
    T: DataReader,
{
    pub fn create<B>(
        context: FileSystemContext,
        block_info: BlockInfo,
        data_source: WorkerNetAddress,
        data_source_type: BlockInStreamSource,
        options: InStreamOptions<B>,
    ) -> BlockInStream<GrpcDataReader>
    where
        B: BlockLocationPolicy,
    {
        let block_id = block_info.block_id;
        let block_size = block_info.length;
        let block_in_stream = Self::create_grpc_block_in_stream(
            context,
            data_source,
            data_source_type,
            block_id,
            block_size,
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
        context: FileSystemContext,
        address: WorkerNetAddress,
        block_source: BlockInStreamSource,
        block_size: i64,
        block_id: i64,
        options: InStreamOptions<B>,
    ) -> BlockInStream<GrpcDataReader>
    where
        B: BlockLocationPolicy,
    {
        let length = 0;
        let position = 0;
        let EOF = true;
        let read_request = ReadRequest {
            block_id: Some(100),
            chunk_size: Some(1000),
            length: Some(50),
            offset: Some(5),
            offset_received: None,
            open_ufs_block_options: None,
            position_short: None,
            promote: None,
        };
        let address = WorkerNetAddress {
            host: "localhost".to_string(),
        };
        let data_reader = GrpcDataReader::create(read_request, address);
        let buffer = Cursor::new(vec![1, 2, 3]);
        let current_chunk = DefaultDataBuffer { buffer };
        BlockInStream {
            length,
            position,
            EOF,
            data_reader,
            current_chunk,
        }
    }

    async fn read(&mut self, b: &Vec<u8>, off: i64, len: i64) -> i64 {
        if len == 0 {
            return 0;
        }
        if self.position == self.length {
            return -1;
        }
        self.read_chunck();
        // if (mCurrentChunk == null) { // mCurrentChunk是DataBuffer
        //   mEOF = true;
        // }
        // if (mEOF) {
        //   closeDataReader();
        //   Preconditions
        //       .checkState(mPos >= mLength, PreconditionMessage.BLOCK_LENGTH_INCONSISTENT.toString(),
        //           mId, mLength, mPos);
        //   return -1;
        // }
        // int toRead = Math.min(len, mCurrentChunk.readableBytes());
        // byteBuffer.position(off).limit(off + toRead);
        // mCurrentChunk.readBytes(byteBuffer);
        // mPos += toRead;
        // if (mPos == mLength) {
        //   // a performance improvement introduced by https://github.com/Alluxio/alluxio/issues/14020
        //   closeDataReader();
        // }
        // return toRead;
        0
    }

    async fn read_chunck(&mut self) -> Result<(), AlluxioException> {
        let data = self.data_reader.read_chunk().await?;
        let buf = self.current_chunk.get_buffer();
        buf.get_mut().copy_from_slice(data.buffer.get_ref());
        Ok(())
    }
}

// Not Ready
#[async_trait]
impl<T> InputStream for BlockInStream<T>
where
    T: DataReader,
{
    ///    Reads up to len bytes of data from the input stream into the byte buffer.
    ///
    ///    @param b the buffer into which the data is read
    ///    @param off the start offset in the buffer at which the data is written
    ///    @param len the maximum number of bytes to read
    ///    @return the total number of bytes read into the buffer, or -1 if there is no more data because
    ///            the end of the stream has been reached
    async fn read(&mut self, b: &mut Vec<u8>, off: i64, len: i64) -> Result<i64, AlluxioException> {
        if len == 0 {
            return Ok(0);
        }
        if self.length == self.position {
            return Ok(-1);
        }
        let data = &mut self.data_reader.read_chunk().await?;
        b.clone_from_slice(data.get_buffer().get_ref());
        Ok(data.get_buffer().get_ref().len() as i64)
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
