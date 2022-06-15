use glommio::{
    io::{DmaFile, DmaStreamReaderBuilder},
    LocalExecutor,
};
use std::rc::Rc;
use std::str;
//use bytes::Bytes;
use bytes::Buf;

extern crate quick_protobuf;
mod osmformat;
mod osm;
use quick_protobuf::Reader;
use osm::BlobHeader;
use osm::Blob;
use osmformat::HeaderBlock;
use quick_protobuf::{MessageRead, BytesReader};
//use bytes::BufMut;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

// hey this code looks dirty but it's gonna get cleaned up when I get some sense what I am doing.. ha!

fn main() {
    let ex = LocalExecutor::default();
    ex.run(async {
        let file = Rc::new(DmaFile::open("../planet_150.388,-34.25_151.613,-33.436.osm.pbf").await.unwrap());
        let _reader = DmaStreamReaderBuilder::from_rc(file.clone()).build();

        let mut l_offset = 0;
        
        // issue random I/O now, even though a stream is open.
        glommio::spawn_local(async move {
            
            let a = file.read_at(l_offset, 4).await.unwrap();
            let mut b = a.chunks_exact(4);

            let mut c = b.next().unwrap();
            let hdr_len = &c.get_int(4);

            // BlobHeader
            let hdr = file.read_at(l_offset + 4, *hdr_len as usize).await.unwrap();
            let mut hdr_chunk = hdr.chunks_exact(*hdr_len as usize);
            let hdr_v = hdr_chunk.next().unwrap();
                
            dbg!(&hdr_v);

            let mut reader = BytesReader::from_bytes(&hdr_v);
            let blobHdr = BlobHeader::from_reader(&mut reader, &hdr_v).expect("foobar");
            dbg!(&blobHdr);

            println!("{}", format!("Datasize: {:?}", &blobHdr.datasize));
            
            // OSMHeader
            if blobHdr.type_pb == "OSMHeader" {
                let blob_offset: u64 = l_offset + 4 + *hdr_len as u64;
                let blob_read = file.read_at(blob_offset, blobHdr.datasize as usize).await.unwrap();
                let mut blob_chunk = blob_read.chunks_exact(blobHdr.datasize as usize);
                let blob_v = blob_chunk.next().unwrap();
                let mut reader = BytesReader::from_bytes(&blob_v);
                let blob_compressed = Blob::from_reader(&mut reader, &blob_v).expect("foobar");
                //dbg!(&blob_compressed);
                dbg!(&blob_compressed.raw_size);
                let mut flate = flate2::Decompress::new(true);
                //let mut blob_uncompressed = bytes::BytesMut::with_capacity(blob_compressed.raw_size.unwrap() as usize);
                match blob_compressed.data {
                    osm::mod_Blob::OneOfdata::zlib_data(compressed) => {
                        let laa = compressed.into_owned();
                        let sss: &[_] = &laa;
                        print_type_of(&sss);
                        dbg!(&sss.len());
                        let uncompressed_size = blob_compressed.raw_size.unwrap() as usize;
                        
                        let mut blob_uncompressed = vec![0; uncompressed_size];
                        //let u = flate.decompress_vec(sss, &mut blob_uncompressed, flate2::FlushDecompress::Finish);
                        let mut libdef =  libdeflater::Decompressor::new();
                        let u = libdef.zlib_decompress(sss, &mut blob_uncompressed);
                        print_type_of(&blob_uncompressed);
                        dbg!(&u);

                        let aaa: &[_] = &blob_uncompressed;
                        dbg!(&aaa.len());
                        let mut reader = BytesReader::from_bytes(&aaa);
                        let foo = HeaderBlock::from_reader(&mut reader, &blob_uncompressed).expect("foobar");
                        dbg!(&foo);
                        //dbg!(&u);
                        /*dbg!(&u);
                        dbg!(u.as_ref().unwrap());
                        dbg!(flate.total_out());
                        dbg!(flate.total_in()); */
                    },
                    _ => todo!(),
                }
            }
            
            //let mut d = Bytes::copy_from_slice(&c);
            //c.get_int(4);
            //let num = Bytes::from(a).get_int(4);
            //dbg!(num);
            
            //dbg!(str::from_utf8(&_a.chunks_exact(8)));
            //dbg!(Bytes::from(&_a.chunks_exact(4)).get_int(4));
        }).await;

    });
}
