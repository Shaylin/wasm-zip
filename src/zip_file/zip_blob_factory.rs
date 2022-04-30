use std::collections::HashMap;
use std::iter::Zip;
use wasm_bindgen::JsValue;
use crate::CrcCalculator;
use crate::date_time_retriever::DosDateTimeRetriever;
use crate::zip_file::zip_file_entry::ZipFileEntry;
use crate::zip_file::ZipBlobFactory;

pub struct ZipBlobFactoryAdapter {
    crc_calculator: Box<dyn CrcCalculator>,
    date_time_retriever: Box<dyn DosDateTimeRetriever>,
}

impl ZipBlobFactory for ZipBlobFactoryAdapter {
    fn create_zip_blob(&self, directory_mapping: HashMap<String, Vec<u8>>) -> Box<[u8]> {
        todo!()
    }
}

impl ZipBlobFactoryAdapter {
    fn create_zip_file_entries(&self, directory_mapping: HashMap<String, Vec<u8>>) -> Vec<ZipFileEntry> {
        let mut zip_file_entries: Vec<ZipFileEntry> = Vec::new();

        let mut file_header_offset: u32 = 0;

        for (file_name, file_body) in directory_mapping.into_iter() {
            let zip_entry = self.create_zip_file_entry(file_name, file_body, file_header_offset);
            file_header_offset += self.get_zip_file_size(&zip_entry);
            zip_file_entries.push(zip_entry);
        }

        zip_file_entries
    }

    fn create_zip_file_entry(&self, zip_file_name: String, file_body: Vec<u8>, header_offset: u32) -> ZipFileEntry {
        let file_crc = self.calculate_file_crc(&file_body);

        ZipFileEntry {
            body: file_body,
            crc: file_crc,
            file_name: zip_file_name,
            dos_time: self.date_time_retriever.get_current_dos_time(),
            dos_date: self.date_time_retriever.get_current_dos_date(),
            entry_offset: header_offset,
        }
    }

    fn calculate_file_crc(&self, file_contents: &Vec<u8>) -> u32 {
        let contents_byte_slice = &file_contents[..];

        self.crc_calculator.calculate_crc32(contents_byte_slice)
    }

    fn get_zip_file_size(&self, zip_file: &ZipFileEntry) -> u32 {
        let local_file_header_size = zip_file.get_local_file_header_size() as u32;
        let body_size = zip_file.body.len() as u32;

        local_file_header_size + body_size
    }

    fn get_end_of_central_directory_record() -> Vec<u8> {
        let mut end_of_central_directory_record: Vec<u8> = Vec::with_capacity(22);


        end_of_central_directory_record
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FakeCrcCalculator {}

    impl CrcCalculator for FakeCrcCalculator {
        fn calculate_crc32(&self, data: &[u8]) -> u32 {
            0x11223344
        }
    }

    struct FakeDosDateTimeRetriever {}

    impl DosDateTimeRetriever for FakeDosDateTimeRetriever {
        fn get_current_dos_time(&self) -> u16 {
            0x1133
        }

        fn get_current_dos_date(&self) -> u16 {
            0x9988
        }
    }

    #[test]
    fn zip_file_size_should_be_the_sum_of_body_header_lengths() {
        let blob_factory_adapter = ZipBlobFactoryAdapter {
            crc_calculator: Box::new(FakeCrcCalculator {}),
            date_time_retriever: Box::new(FakeDosDateTimeRetriever {}),
        };

        let fake_file_entry = ZipFileEntry {
            body: vec![0;293],
            crc: 0,
            file_name: String::from("blab"),
            dos_time: 0,
            dos_date: 0,
            entry_offset: 0
        };

        assert_eq!(327, blob_factory_adapter.get_zip_file_size(&fake_file_entry));
    }

    #[test]
    fn calculate_file_crc_should_delegate_to_crc_calculator() {
        let blob_factory_adapter = ZipBlobFactoryAdapter {
            crc_calculator: Box::new(FakeCrcCalculator {}),
            date_time_retriever: Box::new(FakeDosDateTimeRetriever {}),
        };

        let fake_file_body: Vec<u8> = vec![0;33];

        assert_eq!(0x11223344, blob_factory_adapter.calculate_file_crc(&fake_file_body));
    }

    #[test]
    fn create_zip_file_entry_should_create_zip_entry_with_given_data() {
        let blob_factory_adapter = ZipBlobFactoryAdapter {
            crc_calculator: Box::new(FakeCrcCalculator {}),
            date_time_retriever: Box::new(FakeDosDateTimeRetriever {}),
        };

        let file_name = String::from("BugCat");
        let file_body: Vec<u8> = vec![0;33];
        let header_offset: u32 = 98;

        let created_file_entry = blob_factory_adapter.create_zip_file_entry(file_name.clone(), file_body.clone(), header_offset);

        assert_eq!(file_name, created_file_entry.file_name);
        assert_eq!(file_body, created_file_entry.body);
        assert_eq!(header_offset, created_file_entry.entry_offset);
        assert_eq!(0x11223344, created_file_entry.crc);
        assert_eq!(0x1133, created_file_entry.dos_time);
        assert_eq!(0x9988, created_file_entry.dos_date);
    }

    #[test]
    fn creating_multiple_entries_from_directory_hash_map_with_sequential_header_offsets() {
        let blob_factory_adapter = ZipBlobFactoryAdapter {
            crc_calculator: Box::new(FakeCrcCalculator {}),
            date_time_retriever: Box::new(FakeDosDateTimeRetriever {}),
        };

        let given_hash_map:HashMap<String, Vec<u8>> = HashMap::from([
            (String::from("BugCat.txt"), vec![2;16]),
            (String::from("MyFolder/FoamCat.txt"), vec![0;11])
        ]);

        let created_file_entries = blob_factory_adapter.create_zip_file_entries(given_hash_map);

        //The consuming iterator used to create the vector has arbitrary ordering
        if created_file_entries[0].file_name == String::from("BugCat.txt") {
            assert_eq!(vec![2;16], created_file_entries[0].body);
            assert_eq!(0, created_file_entries[0].entry_offset);

            assert_eq!(vec![0;11], created_file_entries[1].body);
            assert_eq!(56, created_file_entries[1].entry_offset);
        } else {
            assert_eq!(vec![0;11], created_file_entries[0].body);
            assert_eq!(0, created_file_entries[0].entry_offset);

            assert_eq!(vec![2;16], created_file_entries[1].body);
            assert_eq!(61, created_file_entries[1].entry_offset);
        }
    }
}